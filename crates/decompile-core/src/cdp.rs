use anyhow::{Context, Result};
use dom_query::Document;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::LazyLock;
use std::time::Duration;
use url::Url;

static LAZY_CHUNK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"["']((?:\./|/|assets/|static/|_next/static/chunks/)[a-zA-Z0-9_./-]{3,90}\.m?js)["']"#).unwrap()
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpTargetInfo {
    pub id: String,
    pub title: String,
    pub url: String,
    #[serde(rename = "type")]
    pub target_type: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub ws_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestedWebBundle {
    pub target_url: String,
    pub page_title: String,
    pub html: String,
    pub scripts: Vec<HarvestedScript>,
    pub discovered_links: Vec<String>,
    pub cdp_targets: Vec<CdpTargetInfo>,
    pub live_endpoints_probed: Vec<ProbedEndpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestedScript {
    pub url: String,
    pub filename: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbedEndpoint {
    pub method: String,
    pub url: String,
    pub path: String,
    pub status: u16,
    pub content_type: String,
    pub sample_body: Option<Value>,
}

/// Checks if a CDP browser (Patchright / Chromium) is reachable on `:9122` or `:9222`.
pub async fn probe_cdp_targets(cdp_ports: &[u16]) -> Vec<(u16, Vec<CdpTargetInfo>)> {
    let client = Client::builder()
        .timeout(Duration::from_millis(1500))
        .build()
        .unwrap_or_default();

    let mut found = Vec::new();
    for &port in cdp_ports {
        let list_url = format!("http://127.0.0.1:{}/json/list", port);
        if let Ok(resp) = client.get(&list_url).send().await {
            if let Ok(targets) = resp.json::<Vec<CdpTargetInfo>>().await {
                found.push((port, targets));
            }
        }
    }
    found
}

/// Harvests HTML, linked `<script src>` bundles, lazy-loaded SPA route chunks, inline scripts,
/// and standard discovery endpoints (`/health`, `/openapi.json`) from a live URL.
pub async fn harvest_live_url(target_url: &str, cdp_port: Option<u16>) -> Result<HarvestedWebBundle> {
    let client = Client::builder()
        .timeout(Duration::from_secs(12))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .danger_accept_invalid_certs(true)
        .build()
        .context("build HTTP client")?;

    let base_url = Url::parse(target_url).context("invalid target URL")?;
    let ports = match cdp_port {
        Some(p) => vec![p],
        None => vec![9122, 9222],
    };
    let cdp_targets = probe_cdp_targets(&ports)
        .await
        .into_iter()
        .flat_map(|(_, t)| t)
        .collect::<Vec<_>>();

    let mut html = String::new();
    let mut page_title = base_url.host_str().unwrap_or("web-app").to_string();
    let mut scripts = Vec::new();
    let mut discovered_links = Vec::new();
    let mut live_endpoints_probed = Vec::new();
    let mut seen_script_urls = HashSet::new();

    if let Ok(resp) = client.get(target_url).send().await {
        let status = resp.status().as_u16();
        let ct = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        if let Ok(body_text) = resp.text().await {
            if ct.contains("application/json") {
                if let Ok(json_val) = serde_json::from_str::<Value>(&body_text) {
                    live_endpoints_probed.push(ProbedEndpoint {
                        method: "GET".into(),
                        url: target_url.to_string(),
                        path: base_url.path().to_string(),
                        status,
                        content_type: ct,
                        sample_body: Some(json_val),
                    });
                }
            } else {
                html = body_text;
            }
        }
    }

    if !html.is_empty() {
        let script_urls = {
            let doc = Document::from(html.as_str());
            let title_text = doc.select("title").text().to_string();
            if !title_text.trim().is_empty() {
                page_title = title_text.trim().to_string();
            }

            for a in doc.select("a[href]").iter() {
                if let Some(href) = a.attr("href") {
                    if let Ok(resolved) = base_url.join(&href) {
                        if resolved.host_str() == base_url.host_str() {
                            discovered_links.push(resolved.to_string());
                        }
                    }
                }
            }

            let mut urls = Vec::new();
            for (idx, script_node) in doc.select("script").iter().enumerate() {
                if let Some(src) = script_node.attr("src") {
                    if let Ok(resolved) = base_url.join(&src) {
                        if seen_script_urls.insert(resolved.to_string()) {
                            urls.push(resolved);
                        }
                    }
                } else {
                    let inline_code = script_node.text().to_string();
                    if inline_code.trim().len() > 48 {
                        scripts.push(HarvestedScript {
                            url: format!("{}#inline-{}", target_url, idx),
                            filename: format!("inline_{}.js", idx),
                            content: inline_code,
                        });
                    }
                }
            }
            urls
        };

        let mut lazy_chunk_urls = Vec::new();

        for resolved in script_urls.into_iter().take(20) {
            if let Ok(sresp) = client.get(resolved.as_str()).send().await {
                if sresp.status().is_success() {
                    if let Ok(code) = sresp.text().await {
                        // Scan entry bundle for lazy-loaded SPA chunks (`import("./Page-xyz.js")` or `"assets/..."`)
                        for cap in LAZY_CHUNK_RE.captures_iter(&code) {
                            let rel_chunk = cap.get(1).unwrap().as_str();
                            if let Ok(joined) = resolved.join(rel_chunk).or_else(|_| base_url.join(rel_chunk)) {
                                if joined.host_str() == base_url.host_str()
                                    && seen_script_urls.insert(joined.to_string())
                                    && lazy_chunk_urls.len() < 16
                                {
                                    lazy_chunk_urls.push(joined);
                                }
                            }
                        }

                        let fname = resolved
                            .path_segments()
                            .and_then(|s| s.last())
                            .filter(|s| !s.is_empty())
                            .unwrap_or("bundle.js")
                            .to_string();
                        scripts.push(HarvestedScript {
                            url: resolved.to_string(),
                            filename: fname,
                            content: code,
                        });
                    }
                }
            }
        }

        // Fetch discovered lazy route chunks (capped at 16)
        for lazy_url in lazy_chunk_urls {
            if let Ok(sresp) = client.get(lazy_url.as_str()).send().await {
                if sresp.status().is_success() {
                    if let Ok(code) = sresp.text().await {
                        let fname = lazy_url
                            .path_segments()
                            .and_then(|s| s.last())
                            .filter(|s| !s.is_empty())
                            .unwrap_or("chunk.js")
                            .to_string();
                        scripts.push(HarvestedScript {
                            url: lazy_url.to_string(),
                            filename: fname,
                            content: code,
                        });
                    }
                }
            }
        }
    }

    // Single polite check for well-known API discovery paths on the target origin
    for well_known in ["/health", "/openapi.json", "/api/health", "/api/status", "/status"] {
        if let Ok(probe_url) = base_url.join(well_known) {
            if probe_url.as_str() == target_url {
                continue;
            }
            if let Ok(resp) = client.get(probe_url.as_str()).send().await {
                let status = resp.status().as_u16();
                if (200..300).contains(&status) {
                    let ct = resp
                        .headers()
                        .get("content-type")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("")
                        .to_string();
                    if let Ok(text) = resp.text().await {
                        let sample_body = serde_json::from_str::<Value>(&text).ok();
                        if sample_body.is_some() {
                            live_endpoints_probed.push(ProbedEndpoint {
                                method: "GET".into(),
                                url: probe_url.to_string(),
                                path: well_known.to_string(),
                                status,
                                content_type: ct,
                                sample_body,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(HarvestedWebBundle {
        target_url: target_url.to_string(),
        page_title,
        html,
        scripts,
        discovered_links,
        cdp_targets,
        live_endpoints_probed,
    })
}
