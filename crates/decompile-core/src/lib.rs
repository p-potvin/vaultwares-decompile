pub mod analyzer;
pub mod cdp;
pub mod embeddings;
pub mod model;
pub mod server;
pub mod synthesizer;
pub mod unpacker;

use crate::analyzer::{analyze_directory, analyze_harvested_web_bundle};
use crate::cdp::harvest_live_url;
use crate::embeddings::build_semantic_index_and_clusters;
use crate::model::{build_openapi_spec, BlueprintManifest, FullBlueprint};
use crate::synthesizer::{synthesize_markdown_specs, write_blueprint_to_disk, LlmMode};
use anyhow::{Context, Result};
use chrono::Local;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ExtractOptions {
    pub target: String,
    pub output_dir: std::path::PathBuf,
    pub embed_url: String,
    pub llm_mode: LlmMode,
    pub cdp_port: Option<u16>,
}

/// Runs the complete VaultWares Decompile pipeline against either a live URL (`http://...` / `https://...`)
/// or a local application/bundle directory, producing a portable Clean-Room Blueprint.
pub async fn extract_blueprint(opts: &ExtractOptions) -> Result<FullBlueprint> {
    let started = Instant::now();

    let (project_name, mut analysis) = if opts.target.starts_with("http://")
        || opts.target.starts_with("https://")
    {
        let harvested = harvest_live_url(&opts.target, opts.cdp_port).await?;
        let name = harvested.page_title.clone();
        let out = analyze_harvested_web_bundle(&harvested)?;
        (name, out)
    } else {
        let path = Path::new(&opts.target);
        if !path.exists() {
            anyhow::bail!("Target path does not exist: {}", opts.target);
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("decompiled-app")
            .to_string();
        let out = analyze_directory(path)?;
        (name, out)
    };

    // Semantic embedding & clustering via EmbeddingGemma2 (http://127.0.0.1:8791)
    let (embeddings, mut clusters) = build_semantic_index_and_clusters(
        &opts.embed_url,
        &mut analysis.routes,
        &mut analysis.slices,
        &analysis.entities,
        &analysis.stores,
        &analysis.ui_views,
    )
    .await?;

    analysis.stats.feature_clusters = clusters.len();
    analysis.stats.embedded_items = embeddings.as_ref().map(|e| e.entries.len()).unwrap_or(0);
    analysis.stats.duration_ms = started.elapsed().as_millis() as u64;

    let manifest = BlueprintManifest {
        project_name: project_name.clone(),
        target: opts.target.clone(),
        extracted_at: Local::now().format("%a, %d %b %Y %H:%M").to_string(),
        version: "0.2.0".into(),
        detected_stack: analysis.detected_stack.clone(),
        stats: analysis.stats.clone(),
        services: analysis.services.clone(),
        domain_context: analysis.domain_context.clone(),
    };

    let openapi = build_openapi_spec(
        &project_name,
        &opts.target,
        &analysis.routes,
        &analysis.entities,
    );

    let overview_markdown = synthesize_markdown_specs(
        &manifest,
        &analysis.routes,
        &analysis.entities,
        &analysis.stores,
        &analysis.ui_views,
        &analysis.state_machine,
        &analysis.slices,
        &mut clusters,
        opts.llm_mode,
    )
    .await;

    let blueprint = FullBlueprint {
        manifest,
        openapi,
        routes: analysis.routes,
        entities: analysis.entities,
        stores: analysis.stores,
        ui_views: analysis.ui_views,
        state_machine: analysis.state_machine,
        slices: analysis.slices,
        clusters,
        overview_markdown,
        embeddings,
    };

    write_blueprint_to_disk(&blueprint, &opts.output_dir)
        .context("write blueprint artifacts to disk")?;

    Ok(blueprint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::normalise_route_pattern;
    use crate::embeddings::cosine_similarity;
    use crate::unpacker::{extract_balanced_braces, unpack_js_bundle};

    #[test]
    fn test_route_normalisation() {
        assert_eq!(
            normalise_route_pattern("/api/rooms/${site}/${room}?live=1"),
            "/api/rooms/{site}/{room}"
        );
        assert_eq!(
            normalise_route_pattern("https://example.com/api/v1/users/42"),
            "/api/v1/users/{id}"
        );
        assert_eq!(
            normalise_route_pattern("/api/moments/:mid/keep"),
            "/api/moments/{mid}/keep"
        );
    }

    #[test]
    fn test_webpack5_bundle_unpacker() {
        let sample = r#"
        (self.webpackChunk_app = self.webpackChunk_app || []).push([[123], {
          "mod_a": (e, t, n) => {
            const x = n("mod_b");
            fetch("/api/v1/items", { method: "POST" });
          },
          "mod_b": function(e, t, n) {
            const msg = "brace inside string } should not break";
            return msg;
          }
        }]);
        "#;
        let mods = unpack_js_bundle(sample, "app.bundle.js");
        assert_eq!(mods.len(), 2);
        assert_eq!(mods[0].id, "mod_a");
        assert_eq!(mods[0].bundler, "webpack5");
        assert_eq!(mods[0].dependencies, vec!["mod_b".to_string()]);
        assert_eq!(mods[1].id, "mod_b");
    }

    #[test]
    fn test_balanced_braces_ignores_strings_and_comments() {
        let code = r#"{ const a = "{ not a brace }"; /* } */ const b = 1; }"#;
        let extracted = extract_balanced_braces(code, 0);
        assert_eq!(extracted.as_deref(), Some(code));
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 1.0];
        let b = vec![1.0, 0.0, 1.0];
        let c = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-5);
        assert!(cosine_similarity(&a, &c).abs() < 1e-5);
    }
}

