use crate::model::{
    ApiRoute, BlueprintManifest, CodeSlice, EntityModel, FeatureCluster, FullBlueprint,
    StateMachineSpec, StateStore, UiViewSpec,
};
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;

pub const DEFAULT_LOCAL_LLM_URL: &str = "http://127.0.0.1:8794";
pub const DEFAULT_LOCAL_LLM_KEY: &str = "sk-whatever";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmMode {
    LocalGemma,
    Deterministic,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    messages: Vec<ChatMessage>,
    max_tokens: usize,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

/// Checks if local Gemma 4 E2B (`llama-server` on `127.0.0.1:8794`) is online.
pub async fn check_local_llm(base_url: &str, api_key: &str) -> bool {
    let client = Client::builder()
        .timeout(Duration::from_millis(1500))
        .build()
        .unwrap_or_default();
    let url = format!("{}/v1/models", base_url.trim_end_matches('/'));
    client
        .get(&url)
        .bearer_auth(api_key)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

/// Calls local Gemma 4 E2B (`127.0.0.1:8794/v1/chat/completions`) for a concise architectural synthesis.
pub async fn call_local_llm(
    base_url: &str,
    api_key: &str,
    system_prompt: &str,
    user_prompt: &str,
    max_tokens: usize,
) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("build local LLM client")?;

    let url = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));
    let req = ChatCompletionRequest {
        messages: vec![
            ChatMessage {
                role: "system".into(),
                content: system_prompt.into(),
            },
            ChatMessage {
                role: "user".into(),
                content: user_prompt.into(),
            },
        ],
        max_tokens: max_tokens.max(1800),
        temperature: 0.2,
    };

    let resp = client
        .post(&url)
        .bearer_auth(api_key)
        .json(&req)
        .send()
        .await
        .context("POST to local Gemma 4 E2B")?;

    if !resp.status().is_success() {
        anyhow::bail!("Local LLM returned HTTP {}", resp.status());
    }

    let parsed: ChatCompletionResponse = resp.json().await.context("parse local LLM response")?;
    let content = parsed
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .unwrap_or_default();
    Ok(strip_thinking_tags(&content))
}

fn strip_thinking_tags(raw: &str) -> String {
    if let Some(idx) = raw.find("</think>") {
        raw[idx + 8..].trim().to_string()
    } else if let Some(idx) = raw.find("<|channel|>thought") {
        if let Some(end) = raw[idx..].find("<|channel|>final") {
            raw[idx + end + 16..].trim().to_string()
        } else {
            raw.trim().to_string()
        }
    } else {
        raw.trim().to_string()
    }
}

/// Synthesizes `OVERVIEW.md` and per-cluster `features/*.md` specs.
#[allow(clippy::too_many_arguments)]
pub async fn synthesize_markdown_specs(
    manifest: &BlueprintManifest,
    routes: &[ApiRoute],
    entities: &[EntityModel],
    stores: &[StateStore],
    ui_views: &[UiViewSpec],
    state_machine: &StateMachineSpec,
    slices: &[CodeSlice],
    clusters: &mut [FeatureCluster],
    llm_mode: LlmMode,
) -> String {
    let use_llm = llm_mode == LlmMode::LocalGemma
        && check_local_llm(DEFAULT_LOCAL_LLM_URL, DEFAULT_LOCAL_LLM_KEY).await;

    let domain_summary = format!(
        "Content signals: [{}]\nExternal domains: [{}]\nMedia modalities: [{}]",
        manifest.domain_context.content_signals.join(", "),
        manifest.domain_context.external_domains.join(", "),
        manifest.domain_context.media_modalities.join(", ")
    );

    // 1. Populate each FeatureCluster's markdown_spec
    for cluster in clusters.iter_mut() {
        let cluster_slices: Vec<&CodeSlice> = slices
            .iter()
            .filter(|s| cluster.slice_ids.contains(&s.id))
            .collect();
        let cluster_routes: Vec<&ApiRoute> = routes
            .iter()
            .filter(|r| cluster.route_ids.contains(&r.id))
            .collect();
        let cluster_entities: Vec<&EntityModel> = entities
            .iter()
            .filter(|e| cluster.entity_names.contains(&e.name))
            .collect();
        let cluster_views: Vec<&UiViewSpec> = ui_views
            .iter()
            .filter(|v| v.source_file.starts_with(&cluster.keywords[0]))
            .collect();

        let mut md = format!(
            "# Feature Blueprint: {}\n\n> {}\n\n",
            cluster.title, cluster.description
        );

        if use_llm && (!cluster_slices.is_empty() || !cluster_routes.is_empty()) {
            let mut prompt_ctx = format!(
                "Subsystem: {}\n{}\n",
                cluster.title, domain_summary
            );
            if !cluster_views.is_empty() {
                prompt_ctx.push_str("UI Screens/Components:\n");
                for v in cluster_views.iter().take(10) {
                    prompt_ctx.push_str(&format!("- {}\n", v.summary));
                }
            }
            if !cluster_routes.is_empty() {
                prompt_ctx.push_str("Endpoints:\n");
                for r in cluster_routes.iter().take(10) {
                    prompt_ctx.push_str(&format!("- {} {} ({})\n", r.method, r.pattern, r.summary));
                }
            }
            if !cluster_entities.is_empty() {
                prompt_ctx.push_str("Data Models:\n");
                for e in cluster_entities.iter().take(8) {
                    let fnames: Vec<&str> = e.fields.iter().map(|f| f.name.as_str()).collect();
                    prompt_ctx.push_str(&format!("- {} ({}): [{}]\n", e.name, e.kind, fnames.join(", ")));
                }
            }
            prompt_ctx.push_str("Key Functions/Slices:\n");
            for s in cluster_slices.iter().take(12) {
                prompt_ctx.push_str(&format!(
                    "- `{}` ({}): {}\n",
                    s.signature_or_pseudo, s.source_file, s.summary
                ));
            }

            if let Ok(ai_summary) = call_local_llm(
                DEFAULT_LOCAL_LLM_URL,
                DEFAULT_LOCAL_LLM_KEY,
                "You are a clean-room software architect. Given extracted domain signals, UI screens, API routes, models, and function signatures from a subsystem, write a concise 2-paragraph behavioral specification explaining concretely what content/data the subsystem operates on, what it shows or computes, and how a developer can rebuild it from scratch. Be specific and concrete about the real domain subject matter; do not abstract it into generic enterprise prose.",
                &prompt_ctx,
                400,
            )
            .await
            {
                if !ai_summary.is_empty() {
                    md.push_str("## Clean-Room Behavioral Summary (Gemma 4 E2B)\n\n");
                    md.push_str(&ai_summary);
                    md.push_str("\n\n");
                }
            }
        }

        if !cluster_views.is_empty() {
            md.push_str("## UI Screens & Components\n\n| View / Component | Kind | Source | Rendered Elements | Visible Copy / Controls | Bound Routes |\n|---|---|---|---|---|---|\n");
            for v in &cluster_views {
                md.push_str(&format!(
                    "| **`{}`** | `{}` | `{}` | {} | {} | {} |\n",
                    v.name,
                    v.kind,
                    v.source_file,
                    if v.rendered_elements.is_empty() { "JSX".into() } else { v.rendered_elements.join(", ") },
                    if v.visible_labels_and_copy.is_empty() { "–".into() } else { v.visible_labels_and_copy.iter().take(6).cloned().collect::<Vec<_>>().join(" · ") },
                    if v.bound_routes.is_empty() { "–".into() } else { v.bound_routes.join(", ") }
                ));
            }
            md.push('\n');
        }

        if !cluster_routes.is_empty() {
            md.push_str("## API Surface\n\n| Method | Pattern | Handler / Source | Request Fields | Response Fields |\n|---|---|---|---|---|\n");
            for r in &cluster_routes {
                let req_f: Vec<&str> = r.request_fields.iter().map(|f| f.name.as_str()).collect();
                let resp_f: Vec<&str> = r.response_fields.iter().map(|f| f.name.as_str()).collect();
                md.push_str(&format!(
                    "| `{}` | `{}` | `{}:{}` | {} | {} |\n",
                    r.method,
                    r.pattern,
                    r.source_file,
                    r.source_line,
                    if req_f.is_empty() { "–".into() } else { req_f.join(", ") },
                    if resp_f.is_empty() { "–".into() } else { resp_f.join(", ") },
                ));
            }
            md.push('\n');
        }

        if !cluster_entities.is_empty() {
            md.push_str("## Data Models & Tables\n\n");
            for e in &cluster_entities {
                let f_list: Vec<String> = e
                    .fields
                    .iter()
                    .map(|f| format!("`{}: {}`", f.name, f.field_type))
                    .collect();
                md.push_str(&format!(
                    "- **`{}`** ({}, `{}:{}`): {}\n",
                    e.name,
                    e.kind,
                    e.source_file,
                    e.source_line,
                    f_list.join(", ")
                ));
            }
            md.push('\n');
        }

        if !cluster_slices.is_empty() {
            md.push_str("## Proprietary Logic Slices\n\n");
            for s in cluster_slices.iter().take(30) {
                md.push_str(&format!(
                    "### `{}` (`{}:{}-{}`)\n- **Category:** `{}` · **Proprietary Score:** `{:.2}`\n- **Signature:** `{}`\n- **Behavior:** {}\n",
                    s.name,
                    s.source_file,
                    s.start_line,
                    s.end_line,
                    s.category,
                    s.proprietary_score,
                    s.signature_or_pseudo,
                    s.summary
                ));
                if !s.called_apis.is_empty() {
                    md.push_str(&format!("- **Triggers / Routes:** `{}`\n", s.called_apis.join("`, `")));
                }
                if !s.calls_slices.is_empty() {
                    md.push_str(&format!("- **Calls:** `{}`\n", s.calls_slices.join("`, `")));
                }
                if !s.called_by_slices.is_empty() {
                    md.push_str(&format!("- **Called By:** `{}`\n", s.called_by_slices.join("`, `")));
                }
                if !s.referenced_models.is_empty() {
                    md.push_str(&format!("- **Models Referenced:** `{}`\n", s.referenced_models.join("`, `")));
                }
                md.push('\n');
            }
        }

        cluster.markdown_spec = md;
    }

    // 2. Build OVERVIEW.md
    let mut overview = format!(
        "# {} — Clean-Room Architecture Blueprint\n\n",
        manifest.project_name
    );
    overview.push_str(&format!(
        "- **Target:** `{}`\n- **Extracted:** `{}`\n- **Detected Stack:** {}\n- **Metrics:** {} files scanned · {} proprietary slices · {} API routes · {} data models · {} state stores · {} UI views/components · {} feature clusters\n\n",
        manifest.target,
        manifest.extracted_at,
        manifest.detected_stack.join(" · "),
        manifest.stats.files_scanned,
        manifest.stats.proprietary_slices_kept,
        manifest.stats.api_routes_discovered,
        manifest.stats.entities_extracted,
        manifest.stats.state_stores_extracted,
        manifest.stats.ui_views_extracted,
        manifest.stats.feature_clusters,
    ));

    // Domain & Content Context section
    overview.push_str("## Domain & Content Context (What the App Parses & Shows)\n\n");
    if !manifest.domain_context.content_signals.is_empty() {
        overview.push_str(&format!(
            "- **Content & Domain Signals:** `{}`\n",
            manifest.domain_context.content_signals.join("`, `")
        ));
    }
    if !manifest.domain_context.media_modalities.is_empty() {
        overview.push_str(&format!(
            "- **Media & Inference Modalities:** {}\n",
            manifest.domain_context.media_modalities.join(" · ")
        ));
    }
    if !manifest.domain_context.external_domains.is_empty() {
        overview.push_str(&format!(
            "- **Referenced External Domains / Providers:** `{}`\n",
            manifest.domain_context.external_domains.join("`, `")
        ));
    }
    overview.push('\n');

    if use_llm {
        let mut high_level_ctx = format!(
            "Project: {}\nStack: {}\n{}\nServices:\n",
            manifest.project_name,
            manifest.detected_stack.join(", "),
            domain_summary
        );
        for svc in &manifest.services {
            high_level_ctx.push_str(&format!(
                "- {} ({}, {}): {}\n",
                svc.name,
                svc.kind,
                svc.bind_or_url.as_deref().unwrap_or("local"),
                svc.summary
            ));
        }
        if !ui_views.is_empty() {
            high_level_ctx.push_str("UI Views/Pages:\n");
            for v in ui_views.iter().take(12) {
                high_level_ctx.push_str(&format!("- {}: {}\n", v.name, v.summary));
            }
        }
        high_level_ctx.push_str("Subsystems:\n");
        for c in clusters.iter() {
            high_level_ctx.push_str(&format!("- {}: {}\n", c.title, c.description));
        }

        if let Ok(exec_summary) = call_local_llm(
            DEFAULT_LOCAL_LLM_URL,
            DEFAULT_LOCAL_LLM_KEY,
            "You are a principal systems architect. Write a crisp 2-paragraph Executive Architecture Summary of this decompiled system. In paragraph 1, state concretely what domain content, platforms, media streams, and user-facing screens the application actually parses and displays (do NOT sanitize or speak only in abstract terms). In paragraph 2, explain how the backend, GPU vision worker, embedding server, and web UI interact.",
            &high_level_ctx,
            450,
        )
        .await
        {
            if !exec_summary.is_empty() {
                overview.push_str("## Executive Summary (Gemma 4 E2B Synthesis)\n\n");
                overview.push_str(&exec_summary);
                overview.push_str("\n\n");
            }
        }
    }

    if !manifest.services.is_empty() {
        overview.push_str("## Discovered Services & Entrypoints\n\n| Service | Kind | Bind / Endpoint | Entrypoint | Purpose |\n|---|---|---|---|---|\n");
        for svc in &manifest.services {
            overview.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} |\n",
                svc.name,
                svc.kind,
                svc.bind_or_url.as_deref().unwrap_or("–"),
                svc.entrypoint,
                svc.summary
            ));
        }
        overview.push('\n');
    }

    if !ui_views.is_empty() {
        overview.push_str("## UI Pages, Views & Visual Anatomy (`models/ui-views.json`)\n\n| View / Screen | Kind | Source File | Rendered Elements | Visible Copy / Controls | Bound API Routes |\n|---|---|---|---|---|---|\n");
        for v in ui_views {
            overview.push_str(&format!(
                "| **`{}`** | `{}` | `{}` | {} | {} | {} |\n",
                v.name,
                v.kind,
                v.source_file,
                if v.rendered_elements.is_empty() { "JSX".into() } else { v.rendered_elements.join(", ") },
                if v.visible_labels_and_copy.is_empty() { "–".into() } else { v.visible_labels_and_copy.iter().take(6).cloned().collect::<Vec<_>>().join(" · ") },
                if v.bound_routes.is_empty() { "–".into() } else { v.bound_routes.join(", ") }
            ));
        }
        overview.push('\n');
    }

    overview.push_str("## Feature Clusters (Semantic Domains)\n\n| Cluster | File | Slices | Routes | Models |\n|---|---|---|---|---|\n");
    for c in clusters.iter() {
        overview.push_str(&format!(
            "| **{}** | `features/{}.md` | {} | {} | {} |\n",
            c.title,
            c.slug,
            c.slice_ids.len(),
            c.route_ids.len(),
            c.entity_names.len()
        ));
    }
    overview.push('\n');

    if !routes.is_empty() {
        overview.push_str("## API Route Summary (`openapi.json`)\n\n| Method | Pattern | Source | Linked Slices |\n|---|---|---|---|\n");
        for r in routes.iter().take(45) {
            overview.push_str(&format!(
                "| `{}` | `{}` | `{}:{}` | {} |\n",
                r.method,
                r.pattern,
                r.source_file,
                r.source_line,
                if r.related_slices.is_empty() {
                    "–".into()
                } else {
                    r.related_slices.join(", ")
                }
            ));
        }
        overview.push('\n');
    }

    if !stores.is_empty() {
        overview.push_str("## Stateful Workers & Stores (`models/stores.json`)\n\n");
        for st in stores {
            overview.push_str(&format!(
                "- **`{}`** (`{}`, `{}`): keys `[{}]` · methods `[{}]`\n",
                st.name,
                st.kind,
                st.source_file,
                st.state_keys.iter().take(8).cloned().collect::<Vec<_>>().join(", "),
                st.actions_or_mutators.iter().take(8).cloned().collect::<Vec<_>>().join(", ")
            ));
        }
        overview.push('\n');
    }

    let _ = state_machine;
    overview
}

/// Writes the complete Blueprint package to `out_dir`.
pub fn write_blueprint_to_disk(bp: &FullBlueprint, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)?;
    fs::create_dir_all(out_dir.join("models"))?;
    fs::create_dir_all(out_dir.join("features"))?;
    fs::create_dir_all(out_dir.join("slices"))?;

    fs::write(out_dir.join("OVERVIEW.md"), &bp.overview_markdown)?;
    fs::write(
        out_dir.join("openapi.json"),
        serde_json::to_string_pretty(&bp.openapi)?,
    )?;
    fs::write(
        out_dir.join("state-machine.json"),
        serde_json::to_string_pretty(&bp.state_machine)?,
    )?;
    fs::write(
        out_dir.join("models").join("entities.json"),
        serde_json::to_string_pretty(&bp.entities)?,
    )?;
    fs::write(
        out_dir.join("models").join("stores.json"),
        serde_json::to_string_pretty(&bp.stores)?,
    )?;
    fs::write(
        out_dir.join("models").join("ui-views.json"),
        serde_json::to_string_pretty(&bp.ui_views)?,
    )?;
    fs::write(
        out_dir.join("slices").join("module-map.json"),
        serde_json::to_string_pretty(&bp.slices)?,
    )?;

    for cluster in &bp.clusters {
        fs::write(
            out_dir.join("features").join(format!("{}.md", cluster.slug)),
            &cluster.markdown_spec,
        )?;
    }

    if let Some(emb) = &bp.embeddings {
        fs::write(
            out_dir.join("embeddings.json"),
            serde_json::to_string(emb)?,
        )?;
    }

    fs::write(
        out_dir.join("blueprint.json"),
        serde_json::to_string_pretty(bp)?,
    )?;

    Ok(())
}
