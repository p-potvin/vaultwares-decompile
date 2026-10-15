use crate::model::{
    ApiRoute, CodeSlice, EmbeddedEntry, EmbeddingIndex, EntityModel, FeatureCluster, StateStore,
    UiViewSpec,
};
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub const DEFAULT_EMBED_URL: &str = "http://127.0.0.1:8791";
pub const DEFAULT_EMBED_DIM: usize = 256;

#[derive(Debug, Serialize)]
struct EmbedRequestItem {
    text: String,
    prompt: String,
}

#[derive(Debug, Serialize)]
struct EmbedRequest {
    inputs: Vec<EmbedRequestItem>,
    dim: usize,
}

#[derive(Debug, Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
    dim: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub score: f32,
    pub snippet: String,
}

/// Checks if EmbeddingGemma2 is reachable on `127.0.0.1:8791`.
pub async fn check_embedding_service(base_url: &str) -> bool {
    let client = Client::builder()
        .timeout(Duration::from_millis(1500))
        .build()
        .unwrap_or_default();
    let url = format!("{}/health", base_url.trim_end_matches('/'));
    client
        .get(&url)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

/// Embeds a batch of `(text, prompt)` pairs via EmbeddingGemma2 (`http://127.0.0.1:8791/embed`).
/// Respects the server's 1..=64 batch limit by chunking in groups of 32.
pub async fn embed_texts(
    base_url: &str,
    items: &[(String, String)],
    dim: usize,
) -> Result<Vec<Vec<f32>>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .context("build embedding client")?;

    let endpoint = format!("{}/embed", base_url.trim_end_matches('/'));
    let mut all_vectors = Vec::with_capacity(items.len());

    let capped = &items[..items.len().min(384)];
    for chunk in capped.chunks(32) {
        let req_body = EmbedRequest {
            inputs: chunk
                .iter()
                .map(|(text, prompt)| EmbedRequestItem {
                    text: text.chars().take(1200).collect(),
                    prompt: prompt.clone(),
                })
                .collect(),
            dim,
        };

        let resp = client
            .post(&endpoint)
            .json(&req_body)
            .send()
            .await
            .context("POST to EmbeddingGemma2 /embed")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_text = resp.text().await.unwrap_or_default();
            anyhow::bail!("EmbeddingGemma2 returned {}: {}", status, err_text);
        }

        let parsed: EmbedResponse = resp.json().await.context("parse EmbeddingGemma2 JSON")?;
        let _ = parsed.dim;
        all_vectors.extend(parsed.embeddings);
    }

    Ok(all_vectors)
}

/// Builds the full semantic index for all extracted routes, slices, entities, and stores,
/// cross-links routes to their most relevant code slices, and groups items into Feature Clusters.
pub async fn build_semantic_index_and_clusters(
    base_url: &str,
    routes: &mut [ApiRoute],
    slices: &mut [CodeSlice],
    entities: &[EntityModel],
    stores: &[StateStore],
    ui_views: &[UiViewSpec],
) -> Result<(Option<EmbeddingIndex>, Vec<FeatureCluster>)> {
    let mut raw_entries: Vec<(EmbeddedEntry, String)> = Vec::new();

    for r in routes.iter() {
        let text = format!(
            "API Route {} {} — {} (file: {})",
            r.method, r.pattern, r.summary, r.source_file
        );
        raw_entries.push((
            EmbeddedEntry {
                id: r.id.clone(),
                kind: "route".into(),
                title: format!("{} {}", r.method, r.pattern),
                subtitle: r.summary.clone(),
                source: format!("{}:{}", r.source_file, r.source_line),
                text: text.clone(),
                vector: Vec::new(),
            },
            "CodeRetrieval".into(),
        ));
    }

    // Select slices stratified across domains so no single large folder starves smaller subsystems
    let mut domain_buckets: HashMap<String, Vec<&CodeSlice>> = HashMap::new();
    for s in slices.iter() {
        let dom = infer_domain_key(&s.source_file, &s.category);
        domain_buckets.entry(dom).or_default().push(s);
    }
    let mut selected_slices = Vec::new();
    let max_slice_embeddings = 220usize;
    let mut round = 0usize;
    loop {
        let mut added_in_round = false;
        for bucket in domain_buckets.values() {
            if let Some(s) = bucket.get(round) {
                selected_slices.push(*s);
                added_in_round = true;
                if selected_slices.len() >= max_slice_embeddings {
                    break;
                }
            }
        }
        if !added_in_round || selected_slices.len() >= max_slice_embeddings {
            break;
        }
        round += 1;
    }

    for s in selected_slices {
        let text = format!(
            "{} ({}) in {}: {}\n{}",
            s.name, s.category, s.source_file, s.summary, s.signature_or_pseudo
        );
        raw_entries.push((
            EmbeddedEntry {
                id: s.id.clone(),
                kind: "slice".into(),
                title: s.name.clone(),
                subtitle: format!("{} · {}", s.category, s.summary),
                source: format!("{}:{}", s.source_file, s.start_line),
                text: text.clone(),
                vector: Vec::new(),
            },
            "CodeRetrieval".into(),
        ));
    }

    for e in entities.iter().take(40) {
        let field_names: Vec<&str> = e.fields.iter().map(|f| f.name.as_str()).collect();
        let text = format!(
            "Model {} ({}) in {}: fields [{}]",
            e.name,
            e.kind,
            e.source_file,
            field_names.join(", ")
        );
        raw_entries.push((
            EmbeddedEntry {
                id: e.id.clone(),
                kind: "entity".into(),
                title: e.name.clone(),
                subtitle: format!("{} ({} fields)", e.kind, e.fields.len()),
                source: format!("{}:{}", e.source_file, e.source_line),
                text: text.clone(),
                vector: Vec::new(),
            },
            "CodeRetrieval".into(),
        ));
    }

    for st in stores.iter().take(24) {
        let text = format!(
            "State Store {} ({}) in {}: keys [{}]",
            st.name,
            st.kind,
            st.source_file,
            st.state_keys.join(", ")
        );
        raw_entries.push((
            EmbeddedEntry {
                id: st.id.clone(),
                kind: "store".into(),
                title: st.name.clone(),
                subtitle: st.description.clone(),
                source: st.source_file.clone(),
                text: text.clone(),
                vector: Vec::new(),
            },
            "CodeRetrieval".into(),
        ));
    }

    for uv in ui_views.iter().take(36) {
        let text = format!(
            "UI View {} ({}) in {}: renders [{}] labels [{}] routes [{}]",
            uv.name,
            uv.kind,
            uv.source_file,
            uv.rendered_elements.join(", "),
            uv.visible_labels_and_copy.join(" | "),
            uv.bound_routes.join(", ")
        );
        raw_entries.push((
            EmbeddedEntry {
                id: uv.id.clone(),
                kind: "ui_view".into(),
                title: uv.name.clone(),
                subtitle: uv.summary.clone(),
                source: uv.source_file.clone(),
                text: text.clone(),
                vector: Vec::new(),
            },
            "CodeRetrieval".into(),
        ));
    }

    let embed_available = check_embedding_service(base_url).await;
    let mut index_opt = None;

    if embed_available && !raw_entries.is_empty() {
        let batch_inputs: Vec<(String, String)> = raw_entries
            .iter()
            .map(|(e, prompt)| (e.text.clone(), prompt.clone()))
            .collect();

        if let Ok(vectors) = embed_texts(base_url, &batch_inputs, DEFAULT_EMBED_DIM).await {
            let mut entries = Vec::new();
            for ((mut entry, _), vec) in raw_entries.into_iter().zip(vectors.into_iter()) {
                entry.vector = vec;
                entries.push(entry);
            }

            // Cross-link each ApiRoute to its top-3 semantically matching CodeSlices
            let slice_vecs: Vec<(&str, &[f32])> = entries
                .iter()
                .filter(|e| e.kind == "slice")
                .map(|e| (e.id.as_str(), e.vector.as_slice()))
                .collect();

            let route_vecs: HashMap<String, Vec<f32>> = entries
                .iter()
                .filter(|e| e.kind == "route")
                .map(|e| (e.id.clone(), e.vector.clone()))
                .collect();

            for r in routes.iter_mut() {
                if let Some(rv) = route_vecs.get(&r.id) {
                    let mut scored: Vec<(&str, f32)> = slice_vecs
                        .iter()
                        .filter(|(sid, _)| !sid.ends_with(":do_GET") && !sid.ends_with(":do_POST") && !sid.ends_with(":main"))
                        .map(|(sid, sv)| (*sid, cosine_similarity(rv, sv)))
                        .filter(|(_, sim)| *sim > 0.50)
                        .collect();
                    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    for (sid, _) in scored.into_iter().take(3) {
                        if !r.related_slices.iter().any(|x| x == sid) && r.related_slices.len() < 4 {
                            r.related_slices.push(sid.to_string());
                        }
                    }
                }
            }

            index_opt = Some(EmbeddingIndex {
                model: "embeddinggemma-2".into(),
                dim: DEFAULT_EMBED_DIM,
                entries,
            });
        }
    }

    let clusters = build_feature_clusters(routes, slices, entities, index_opt.as_ref());
    Ok((index_opt, clusters))
}

/// Runs a live semantic search query against an existing `EmbeddingIndex` using `EmbeddingGemma2` (`prompt: "SearchQuery"`).
pub async fn semantic_search(
    base_url: &str,
    index: &EmbeddingIndex,
    query: &str,
    top_k: usize,
) -> Result<Vec<SearchHit>> {
    let inputs = vec![(query.to_string(), "SearchQuery".to_string())];
    let vecs = embed_texts(base_url, &inputs, index.dim).await?;
    let Some(qvec) = vecs.first() else {
        return Ok(Vec::new());
    };

    let mut hits: Vec<SearchHit> = index
        .entries
        .iter()
        .map(|e| SearchHit {
            id: e.id.clone(),
            kind: e.kind.clone(),
            title: e.title.clone(),
            subtitle: e.subtitle.clone(),
            source: e.source.clone(),
            score: cosine_similarity(qvec, &e.vector),
            snippet: e.text.clone(),
        })
        .collect();

    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits.truncate(top_k);
    Ok(hits)
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for (&x, &y) in a.iter().zip(b.iter()) {
        dot += x * y;
        na += x * x;
        nb += y * y;
    }
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na.sqrt() * nb.sqrt())
    }
}

fn build_feature_clusters(
    routes: &[ApiRoute],
    slices: &mut [CodeSlice],
    entities: &[EntityModel],
    embed_index: Option<&EmbeddingIndex>,
) -> Vec<FeatureCluster> {
    // Group initially by top-level subsystem directory or logical domain, then refine with embedding similarity
    let mut groups: HashMap<String, (Vec<String>, Vec<String>, Vec<String>)> = HashMap::new();

    for s in slices.iter_mut() {
        let domain = infer_domain_key(&s.source_file, &s.category);
        let cluster_id = format!("cluster_{}", crate::analyzer::sanitise_id(&domain));
        s.cluster_id = Some(cluster_id);
        groups
            .entry(domain)
            .or_default()
            .0
            .push(s.id.clone());
    }

    for r in routes {
        let domain = infer_domain_key(&r.source_file, "api");
        groups
            .entry(domain)
            .or_default()
            .1
            .push(r.id.clone());
    }

    for e in entities {
        let domain = infer_domain_key(&e.source_file, "model");
        groups
            .entry(domain)
            .or_default()
            .2
            .push(e.name.clone());
    }

    let vec_map: HashMap<&str, &[f32]> = embed_index
        .map(|idx| {
            idx.entries
                .iter()
                .map(|e| (e.id.as_str(), e.vector.as_slice()))
                .collect()
        })
        .unwrap_or_default();

    let mut clusters = Vec::new();
    let mut sorted_keys: Vec<String> = groups.keys().cloned().collect();
    sorted_keys.sort();

    for (i, domain) in sorted_keys.into_iter().enumerate() {
        let (mut slice_ids, route_ids, entity_names) = groups.remove(&domain).unwrap_or_default();
        if slice_ids.is_empty() && route_ids.is_empty() && entity_names.is_empty() {
            continue;
        }

        // If we have embeddings, also attach high-affinity cross-module slices (> 0.68 cosine)
        if !vec_map.is_empty() && !route_ids.is_empty() {
            for rid in &route_ids {
                if let Some(&rvec) = vec_map.get(rid.as_str()) {
                    for (entry_id, &svec) in &vec_map {
                        if entry_id.contains(':')
                            && !slice_ids.iter().any(|x| x == entry_id)
                            && cosine_similarity(rvec, svec) > 0.68
                        {
                            slice_ids.push((*entry_id).to_string());
                        }
                    }
                }
            }
        }

        let slug = format!("{:02}-{}", i + 1, crate::analyzer::sanitise_id(&domain));
        let title = humanize_domain(&domain);
        clusters.push(FeatureCluster {
            id: format!("cluster_{}", crate::analyzer::sanitise_id(&domain)),
            slug,
            title: title.clone(),
            description: format!(
                "Subsystem `{}` covering {} code slices, {} API routes, and {} data models.",
                title,
                slice_ids.len(),
                route_ids.len(),
                entity_names.len()
            ),
            slice_ids,
            route_ids,
            entity_names,
            keywords: vec![domain],
            markdown_spec: String::new(),
        });
    }

    clusters
}

fn infer_domain_key(source_file: &str, fallback: &str) -> String {
    let parts: Vec<&str> = source_file
        .split('/')
        .filter(|p| !p.is_empty() && *p != "src" && *p != "crates" && *p != "app")
        .collect();
    if parts.len() >= 2 {
        parts[0].to_string()
    } else if let Some(first) = parts.first() {
        first
            .split('.')
            .next()
            .unwrap_or(fallback)
            .to_string()
    } else {
        fallback.to_string()
    }
}

fn humanize_domain(raw: &str) -> String {
    raw.split(|c: char| c == '_' || c == '-' || c == '/')
        .filter(|s| !s.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
