use anyhow::{Context, Result};
use decompile_core::{
    cdp::probe_cdp_targets,
    embeddings::{check_embedding_service, semantic_search, DEFAULT_EMBED_URL},
    extract_blueprint,
    model::{EmbeddingIndex, FullBlueprint},
    server::run_studio_server,
    synthesizer::{check_local_llm, LlmMode, DEFAULT_LOCAL_LLM_KEY, DEFAULT_LOCAL_LLM_URL},
    ExtractOptions,
};
use std::env;
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn".into()),
        )
        .init();

    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("help");

    match cmd {
        "status" => cmd_status().await?,
        "extract" => cmd_extract(&args[1..]).await?,
        "search" => cmd_search(&args[1..]).await?,
        "serve" => cmd_serve(&args[1..]).await?,
        _ => print_usage(),
    }

    Ok(())
}

fn print_usage() {
    println!(
        "vw-decompile v0.2.0 — VaultWares Clean-Room SPA & App Blueprint Compiler\n\
         \n\
         USAGE:\n\
           vw-decompile status\n\
           vw-decompile extract <url-or-path> [--output <dir>] [--no-llm]\n\
           vw-decompile search <blueprint-dir> \"<query>\"\n\
           vw-decompile serve <blueprint-dir> [--port 7740]\n"
    );
}

async fn cmd_status() -> Result<()> {
    println!("=== VaultWares Decompile · Local Infrastructure Status ===");
    let eg2 = check_embedding_service(DEFAULT_EMBED_URL).await;
    println!(
        "  EmbeddingGemma2 ({}): {}",
        DEFAULT_EMBED_URL,
        if eg2 { "ONLINE (256-d CodeRetrieval/Clustering/SearchQuery)" } else { "OFFLINE" }
    );

    let gemma4 = check_local_llm(DEFAULT_LOCAL_LLM_URL, DEFAULT_LOCAL_LLM_KEY).await;
    println!(
        "  Gemma 4 E2B     ({}): {}",
        DEFAULT_LOCAL_LLM_URL,
        if gemma4 { "ONLINE (llama-server RTX 2060)" } else { "OFFLINE" }
    );

    let cdp = probe_cdp_targets(&[9122, 9222]).await;
    if cdp.is_empty() {
        println!("  Patchright/CDP  (9122/9222):       NO ACTIVE TARGETS");
    } else {
        for (port, targets) in cdp {
            println!(
                "  Patchright/CDP  (127.0.0.1:{}):  ONLINE ({} target tabs)",
                port,
                targets.len()
            );
        }
    }
    Ok(())
}

async fn cmd_extract(args: &[String]) -> Result<()> {
    let Some(target) = args.first() else {
        anyhow::bail!("Missing <url-or-path> for `extract`");
    };

    let mut output_dir: Option<PathBuf> = None;
    let mut llm_mode = LlmMode::LocalGemma;
    let mut cdp_port = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" | "-o" if i + 1 < args.len() => {
                output_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--cdp" if i + 1 < args.len() => {
                cdp_port = args[i + 1].parse::<u16>().ok();
                i += 2;
            }
            "--no-llm" => {
                llm_mode = LlmMode::Deterministic;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    let out_dir = output_dir.unwrap_or_else(|| {
        let slug = decompile_core::analyzer::sanitise_id(
            PathBuf::from(target)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(target),
        );
        PathBuf::from("blueprints").join(slug)
    });

    println!("[1/4] Extracting AST slices, API routes & data contracts from `{}`...", target);
    let opts = ExtractOptions {
        target: target.clone(),
        output_dir: out_dir.clone(),
        embed_url: DEFAULT_EMBED_URL.to_string(),
        llm_mode,
        cdp_port,
    };

    let bp = extract_blueprint(&opts).await?;
    let s = &bp.manifest.stats;
    println!(
        "[2/4] Scan complete in {} ms:\n      - Files scanned:         {}\n      - Vendor modules stripped: {}\n      - Proprietary slices:    {}\n      - API routes (OpenAPI):  {}\n      - Entities & tables:     {}\n      - State stores/workers:  {}\n      - Feature clusters:      {}\n      - Embedded vectors:      {}",
        s.duration_ms,
        s.files_scanned,
        s.vendor_modules_stripped,
        s.proprietary_slices_kept,
        s.api_routes_discovered,
        s.entities_extracted,
        s.state_stores_extracted,
        s.feature_clusters,
        s.embedded_items
    );
    println!("[3/4] Detected stack: {}", bp.manifest.detected_stack.join(" · "));
    println!("[4/4] Blueprint written to: {}", out_dir.display());
    println!("      - {}/OVERVIEW.md", out_dir.display());
    println!("      - {}/openapi.json", out_dir.display());
    println!("      - {}/state-machine.json", out_dir.display());
    println!("      - {}/models/entities.json", out_dir.display());
    println!("      - {}/models/stores.json", out_dir.display());
    println!("      - {}/features/*.md ({} files)", out_dir.display(), bp.clusters.len());
    println!("      - {}/slices/module-map.json", out_dir.display());

    Ok(())
}

async fn cmd_search(args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("Usage: vw-decompile search <blueprint-dir> \"<query>\"");
    }
    let dir = PathBuf::from(&args[0]);
    let query = args[1..].join(" ");
    let emb_path = dir.join("embeddings.json");
    let raw = fs::read_to_string(&emb_path)
        .with_context(|| format!("Failed to read {}", emb_path.display()))?;
    let index: EmbeddingIndex = serde_json::from_str(&raw)?;

    let hits = semantic_search(DEFAULT_EMBED_URL, &index, &query, 10).await?;
    println!("=== Semantic Search Results for \"{}\" (EmbeddingGemma2) ===", query);
    for (rank, h) in hits.iter().enumerate() {
        println!(
            "  #{:02} [{:.3}] ({}) {} — {}\n       Source: {}",
            rank + 1,
            h.score,
            h.kind,
            h.title,
            h.subtitle,
            h.source
        );
    }
    Ok(())
}

async fn cmd_serve(args: &[String]) -> Result<()> {
    let Some(dir_str) = args.first() else {
        anyhow::bail!("Usage: vw-decompile serve <blueprint-dir> [--port 7740]");
    };
    let dir = PathBuf::from(dir_str);
    let bp_path = if dir.ends_with("blueprint.json") {
        dir
    } else {
        dir.join("blueprint.json")
    };

    let mut port = 7740u16;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--port" && i + 1 < args.len() {
            port = args[i + 1].parse().unwrap_or(7740);
            i += 2;
        } else {
            i += 1;
        }
    }

    let raw = fs::read_to_string(&bp_path)
        .with_context(|| format!("Failed to read {}", bp_path.display()))?;
    let bp: FullBlueprint = serde_json::from_str(&raw)?;
    run_studio_server(bp, port).await
}
