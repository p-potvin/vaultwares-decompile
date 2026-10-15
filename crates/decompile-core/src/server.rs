use crate::cdp::probe_cdp_targets;
use crate::embeddings::{check_embedding_service, semantic_search, DEFAULT_EMBED_URL};
use crate::model::FullBlueprint;
use crate::synthesizer::{check_local_llm, DEFAULT_LOCAL_LLM_KEY, DEFAULT_LOCAL_LLM_URL};
use anyhow::Result;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct StudioState {
    pub blueprint: Arc<FullBlueprint>,
    pub embed_url: String,
}

#[derive(Deserialize)]
pub struct SearchQueryPayload {
    pub query: String,
    pub top_k: Option<usize>,
}

pub async fn run_studio_server(blueprint: FullBlueprint, port: u16) -> Result<()> {
    let state = StudioState {
        blueprint: Arc::new(blueprint),
        embed_url: DEFAULT_EMBED_URL.to_string(),
    };

    let app = Router::new()
        .route("/", get(index_html))
        .route("/api/blueprint", get(get_blueprint))
        .route("/api/health", get(get_health))
        .route("/api/search", post(post_search))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("VaultWares Decompile Studio (vaultsqware) listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn get_blueprint(State(state): State<StudioState>) -> impl IntoResponse {
    Json((*state.blueprint).clone())
}

async fn get_health(State(state): State<StudioState>) -> impl IntoResponse {
    let eg2 = check_embedding_service(&state.embed_url).await;
    let gemma4 = check_local_llm(DEFAULT_LOCAL_LLM_URL, DEFAULT_LOCAL_LLM_KEY).await;
    let cdp = probe_cdp_targets(&[9122, 9222]).await;
    Json(serde_json::json!({
        "status": "online",
        "version": state.blueprint.manifest.version,
        "project": state.blueprint.manifest.project_name,
        "services": {
            "embedding_gemma2_8791": eg2,
            "gemma4_e2b_8794": gemma4,
            "cdp_ports": cdp.iter().map(|(p, t)| serde_json::json!({"port": p, "targets": t.len()})).collect::<Vec<_>>(),
        }
    }))
}

async fn post_search(
    State(state): State<StudioState>,
    Json(payload): Json<SearchQueryPayload>,
) -> impl IntoResponse {
    let top_k = payload.top_k.unwrap_or(12);
    if let Some(idx) = &state.blueprint.embeddings {
        match semantic_search(&state.embed_url, idx, &payload.query, top_k).await {
            Ok(hits) => Json(serde_json::json!({ "ok": true, "mode": "embeddinggemma-2", "hits": hits })),
            Err(e) => Json(serde_json::json!({ "ok": false, "error": e.to_string(), "hits": [] })),
        }
    } else {
        Json(serde_json::json!({ "ok": false, "error": "No embedding index loaded", "hits": [] }))
    }
}

async fn index_html() -> Html<&'static str> {
    Html(STUDIO_HTML)
}

const STUDIO_HTML: &str = r##"<!DOCTYPE html>
<!-- v0.2.0 -->
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>vaultwares-decompile · Blueprint Studio</title>
  <style>
    :root {
      --vwsq-console-bg: #0A0C11;
      --vwsq-console-surface: #11141B;
      --vwsq-console-sunken: #070810;
      --vwsq-console-raised: #191D27;
      --vwsq-console-elevated: #242935;
      --vwsq-console-active: #323847;
      --vwsq-console-hover: #1F2430;
      --vwsq-console-text: #BAC2D2;
      --vwsq-console-text-secondary: rgba(186,194,210,.72);
      --vwsq-console-text-dim: rgba(186,194,210,.46);
      --vwsq-console-text-bright: #E6EAF2;
      --vwsq-console-border: rgba(255,255,255,.06);
      --vwsq-console-border-strong: rgba(255,255,255,.12);

      --vwsq-warm-bg: #EDECE8;
      --vwsq-warm-raised: #F8F7F4;
      --vwsq-warm-muted: #DEDCD6;
      --vwsq-warm-sunken: #E4E2DD;
      --vwsq-warm-ink: #0F1116;
      --vwsq-warm-ink-secondary: rgba(15,17,22,.66);
      --vwsq-warm-ink-dim: rgba(15,17,22,.42);
      --vwsq-warm-border: rgba(15,17,22,.10);
      --vwsq-warm-border-strong: rgba(15,17,22,.18);

      --vwsq-iris-400: #8189F5;
      --vwsq-iris-500: #6E7BF2;
      --vwsq-iris-600: #5866DC;
      --vwsq-iris-soft: rgba(110,123,242,.14);
      --vwsq-coral-400: #FF9679;
      --vwsq-coral-500: #FF8A6B;
      --vwsq-coral-soft: rgba(255,138,107,.14);

      --vwsq-signal-online: #56D98D;
      --vwsq-signal-online-soft: rgba(86,217,141,.14);
      --vwsq-signal-relay: #5AB8F0;
      --vwsq-signal-relay-soft: rgba(90,184,240,.14);
      --vwsq-signal-warning: #E9B054;
      --vwsq-signal-alert: #F45D6B;
      --vwsq-signal-sync: #A585F5;

      --vwsq-font-sans: "Inter", "Segoe UI", ui-sans-serif, system-ui, sans-serif;
      --vwsq-font-mono: "JetBrains Mono", "Victor Mono", ui-monospace, monospace;
      --vwsq-radius-panel: 16px;
      --vwsq-radius-pill: 20px;
      --vwsq-radius-control: 4px;
    }

    * { box-sizing: border-box; }
    html, body {
      margin: 0;
      height: 100%;
      background: var(--vwsq-console-bg);
      color: var(--vwsq-console-text);
      font-family: var(--vwsq-font-sans);
      overflow: hidden;
    }

    .shell {
      display: grid;
      grid-template-columns: 290px 1fr 340px;
      grid-template-rows: 38px 1fr 28px;
      height: 100vh;
    }

    .titlebar {
      grid-column: 1 / -1;
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 16px;
      background: var(--vwsq-console-sunken);
      border-bottom: 1px solid var(--vwsq-console-border);
      font-family: var(--vwsq-font-mono);
      font-size: 12px;
    }

    .brand {
      display: flex;
      align-items: center;
      gap: 10px;
      color: var(--vwsq-console-text-bright);
      font-weight: 700;
    }

    .nav-history {
      display: flex;
      align-items: center;
      gap: 6px;
    }

    .hist-btn {
      background: var(--vwsq-console-raised);
      color: var(--vwsq-console-text);
      border: 1px solid var(--vwsq-console-border-strong);
      border-radius: var(--vwsq-radius-control);
      padding: 3px 10px;
      font-family: var(--vwsq-font-mono);
      font-size: 11px;
      cursor: pointer;
    }
    .hist-btn:hover { background: var(--vwsq-console-hover); color: var(--vwsq-console-text-bright); }

    .pill {
      padding: 2px 10px;
      border-radius: var(--vwsq-radius-pill);
      background: var(--vwsq-iris-soft);
      color: var(--vwsq-iris-400);
      font-size: 11px;
      font-family: var(--vwsq-font-mono);
    }

    .warm-rail {
      background: var(--vwsq-warm-bg);
      color: var(--vwsq-warm-ink);
      border-right: 1px solid var(--vwsq-warm-border-strong);
      display: flex;
      flex-direction: column;
      padding: 12px;
      gap: 6px;
      overflow-y: auto;
    }

    .rail-section-label {
      font-family: var(--vwsq-font-mono);
      font-size: 10px;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      color: var(--vwsq-warm-ink-dim);
      margin: 10px 4px 4px;
    }

    .nav-btn {
      width: 100%;
      text-align: left;
      border: 1px solid transparent;
      background: transparent;
      color: var(--vwsq-warm-ink-secondary);
      padding: 7px 10px;
      border-radius: var(--vwsq-radius-control);
      font-family: var(--vwsq-font-mono);
      font-size: 12px;
      cursor: pointer;
      display: flex;
      justify-content: space-between;
      align-items: center;
      text-decoration: none;
    }

    .nav-btn:hover {
      background: var(--vwsq-warm-sunken);
      color: var(--vwsq-warm-ink);
    }

    .nav-btn.active {
      background: var(--vwsq-warm-raised);
      color: var(--vwsq-warm-ink);
      font-weight: 600;
      box-shadow: inset 3px 0 0 var(--vwsq-iris-500);
      border-color: var(--vwsq-warm-border);
    }

    .console-main {
      background: var(--vwsq-console-bg);
      padding: 18px 22px;
      overflow-y: auto;
      display: flex;
      flex-direction: column;
      gap: 14px;
    }

    .codemap-aside {
      background: var(--vwsq-console-surface);
      border-left: 1px solid var(--vwsq-console-border-strong);
      padding: 14px 16px;
      overflow-y: auto;
      display: flex;
      flex-direction: column;
      gap: 12px;
    }

    .codemap-title {
      font-family: var(--vwsq-font-mono);
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      color: var(--vwsq-coral-400);
      border-bottom: 1px solid var(--vwsq-console-border);
      padding-bottom: 6px;
    }

    .search-bar {
      display: flex;
      gap: 8px;
      background: var(--vwsq-console-raised);
      border: 1px solid var(--vwsq-console-border-strong);
      padding: 8px 12px;
      border-radius: var(--vwsq-radius-control);
      align-items: center;
    }

    .search-bar input {
      flex: 1;
      background: transparent;
      border: 0;
      color: var(--vwsq-console-text-bright);
      font-family: var(--vwsq-font-mono);
      font-size: 13px;
      outline: none;
    }

    .btn-primary {
      background: var(--vwsq-iris-500);
      color: var(--vwsq-console-text-bright);
      border: 0;
      border-radius: var(--vwsq-radius-control);
      padding: 6px 12px;
      font-family: var(--vwsq-font-mono);
      font-size: 11px;
      font-weight: 600;
      cursor: pointer;
    }
    .btn-primary:hover { background: var(--vwsq-iris-600); }

    .btn-ghost {
      background: var(--vwsq-console-elevated);
      color: var(--vwsq-console-text);
      border: 1px solid var(--vwsq-console-border-strong);
      border-radius: var(--vwsq-radius-control);
      padding: 5px 10px;
      font-family: var(--vwsq-font-mono);
      font-size: 11px;
      cursor: pointer;
    }
    .btn-ghost:hover { background: var(--vwsq-console-hover); color: var(--vwsq-console-text-bright); }

    .stats-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
      gap: 10px;
    }

    .stat-card {
      background: var(--vwsq-console-raised);
      border: 1px solid var(--vwsq-console-border);
      border-radius: var(--vwsq-radius-panel);
      padding: 10px 14px;
      cursor: pointer;
      transition: border-color 90ms;
    }
    .stat-card:hover { border-color: var(--vwsq-iris-500); }

    .stat-val {
      font-family: var(--vwsq-font-mono);
      font-size: 20px;
      font-weight: 700;
      color: var(--vwsq-console-text-bright);
    }

    .stat-lbl {
      font-family: var(--vwsq-font-mono);
      font-size: 10px;
      color: var(--vwsq-console-text-dim);
      margin-top: 3px;
    }

    details.vw-collapse {
      background: var(--vwsq-console-surface);
      border: 1px solid var(--vwsq-console-border);
      border-radius: var(--vwsq-radius-panel);
      margin-bottom: 10px;
      overflow: hidden;
    }

    details.vw-collapse > summary {
      padding: 12px 16px;
      font-family: var(--vwsq-font-mono);
      font-size: 13px;
      font-weight: 600;
      color: var(--vwsq-console-text-bright);
      background: var(--vwsq-console-raised);
      cursor: pointer;
      user-select: none;
      display: flex;
      align-items: center;
      justify-content: space-between;
      list-style: none;
    }
    details.vw-collapse > summary::-webkit-details-marker { display: none; }
    details.vw-collapse > summary::before {
      content: "▸";
      margin-right: 8px;
      color: var(--vwsq-iris-400);
      display: inline-block;
      transition: transform 120ms;
    }
    details.vw-collapse[open] > summary::before {
      transform: rotate(90deg);
    }

    .collapse-body {
      padding: 14px 16px;
      border-top: 1px solid var(--vwsq-console-border);
      overflow-wrap: anywhere;
      word-break: break-word;
    }

    pre, .code-block {
      background: var(--vwsq-console-sunken);
      border: 1px solid var(--vwsq-console-border);
      border-radius: var(--vwsq-radius-control);
      padding: 12px;
      font-family: var(--vwsq-font-mono);
      font-size: 12px;
      line-height: 1.5;
      color: var(--vwsq-console-text);
      margin: 8px 0 0;
      white-space: pre-wrap;
      overflow-wrap: anywhere;
      word-break: break-word;
    }

    .row-card {
      background: var(--vwsq-console-raised);
      border: 1px solid var(--vwsq-console-border);
      border-radius: var(--vwsq-radius-control);
      padding: 11px 14px;
      margin-bottom: 8px;
      overflow-wrap: anywhere;
      word-break: break-word;
      transition: border-color 90ms;
    }
    .row-card:hover, .row-card.focused {
      border-color: var(--vwsq-coral-500);
    }

    .chip {
      display: inline-flex;
      align-items: center;
      gap: 4px;
      padding: 2px 8px;
      margin: 2px 4px 2px 0;
      border-radius: var(--vwsq-radius-control);
      font-family: var(--vwsq-font-mono);
      font-size: 11px;
      background: var(--vwsq-console-elevated);
      color: var(--vwsq-iris-400);
      border: 1px solid var(--vwsq-console-border-strong);
      text-decoration: none;
      cursor: pointer;
    }
    .chip:hover {
      background: var(--vwsq-iris-soft);
      border-color: var(--vwsq-iris-500);
      color: var(--vwsq-console-text-bright);
    }
    .chip.coral {
      color: var(--vwsq-coral-400);
    }
    .chip.coral:hover {
      background: var(--vwsq-coral-soft);
      border-color: var(--vwsq-coral-500);
    }
    .chip.online {
      color: var(--vwsq-signal-online);
      background: var(--vwsq-signal-online-soft);
    }

    .badge-method {
      font-family: var(--vwsq-font-mono);
      font-size: 11px;
      font-weight: 700;
      padding: 2px 6px;
      border-radius: var(--vwsq-radius-control);
      background: var(--vwsq-iris-soft);
      color: var(--vwsq-iris-400);
      margin-right: 8px;
    }

    /* Markdown rendered output */
    .md-body {
      font-size: 13.5px;
      line-height: 1.65;
      color: var(--vwsq-console-text);
      overflow-wrap: anywhere;
    }
    .md-body h1, .md-body h2, .md-body h3 {
      font-family: var(--vwsq-font-mono);
      color: var(--vwsq-console-text-bright);
      margin: 18px 0 8px;
    }
    .md-body h1 { font-size: 19px; border-bottom: 1px solid var(--vwsq-console-border-strong); padding-bottom: 8px; }
    .md-body h2 { font-size: 15px; color: var(--vwsq-iris-400); }
    .md-body h3 { font-size: 13px; color: var(--vwsq-coral-400); }
    .md-body p { margin: 8px 0; }
    .md-body ul { margin: 6px 0; padding-left: 20px; }
    .md-body li { margin: 4px 0; }
    .md-body blockquote {
      margin: 8px 0;
      padding: 8px 12px;
      border-left: 3px solid var(--vwsq-iris-500);
      background: var(--vwsq-console-raised);
      color: var(--vwsq-console-text-secondary);
    }
    .md-body code {
      font-family: var(--vwsq-font-mono);
      font-size: 12px;
      background: var(--vwsq-console-sunken);
      padding: 1px 5px;
      border-radius: var(--vwsq-radius-control);
      color: var(--vwsq-console-text-bright);
    }
    .md-body table {
      width: 100%;
      border-collapse: collapse;
      margin: 10px 0;
      font-size: 12px;
      table-layout: fixed;
    }
    .md-body th, .md-body td {
      border: 1px solid var(--vwsq-console-border-strong);
      padding: 7px 10px;
      text-align: left;
      overflow-wrap: anywhere;
      word-break: break-word;
      vertical-align: top;
    }
    .md-body th {
      background: var(--vwsq-console-raised);
      color: var(--vwsq-console-text-bright);
      font-family: var(--vwsq-font-mono);
      font-size: 11px;
    }

    /* Code syntax & Intellisense tokens */
    .tok-kw { color: var(--vwsq-signal-sync); font-weight: 600; }
    .tok-str { color: var(--vwsq-signal-online); }
    .tok-com { color: var(--vwsq-console-text-dim); font-style: italic; }
    .tok-sym {
      color: var(--vwsq-coral-400);
      text-decoration: underline dotted var(--vwsq-coral-500);
      cursor: pointer;
    }
    .tok-sym:hover {
      background: var(--vwsq-coral-soft);
      color: var(--vwsq-console-text-bright);
    }

    .statusbar {
      grid-column: 1 / -1;
      background: var(--vwsq-console-sunken);
      border-top: 1px solid var(--vwsq-console-border);
      display: flex;
      align-items: center;
      gap: 16px;
      padding: 0 16px;
      font-family: var(--vwsq-font-mono);
      font-size: 11px;
      color: var(--vwsq-console-text-secondary);
    }

    .led {
      display: inline-block;
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: var(--vwsq-signal-online);
      margin-right: 6px;
    }
  </style>
</head>
<body>
  <div class="shell">
    <header class="titlebar">
      <div class="brand">
        <span>⬡ vaultwares-decompile</span>
        <span class="pill" id="proj-title">loading…</span>
        <div class="nav-history">
          <button class="hist-btn" onclick="history.back()" title="Navigate Back">← Back</button>
          <button class="hist-btn" onclick="history.forward()" title="Navigate Forward">Forward →</button>
          <button class="hist-btn" onclick="toggleAllDetails(true)">Expand All</button>
          <button class="hist-btn" onclick="toggleAllDetails(false)">Collapse All</button>
        </div>
      </div>
      <div id="stack-pills" style="color:var(--vwsq-console-text-secondary)"></div>
    </header>

    <nav class="warm-rail">
      <div class="rail-section-label">Blueprint Views</div>
      <a class="nav-btn" id="nav-overview" href="#overview"><span>Overview & Context</span><span id="cnt-svc">0</span></a>
      <a class="nav-btn" id="nav-ui-views" href="#ui-views"><span>UI Pages & Anatomy</span><span id="cnt-ui">0</span></a>
      <a class="nav-btn" id="nav-openapi" href="#openapi"><span>OpenAPI 3.1 Routes</span><span id="cnt-routes">0</span></a>
      <a class="nav-btn" id="nav-models" href="#models"><span>JSON Models & Stores</span><span id="cnt-models">0</span></a>
      <a class="nav-btn" id="nav-clusters" href="#clusters"><span>Feature Clusters</span><span id="cnt-clusters">0</span></a>
      <a class="nav-btn" id="nav-slices" href="#slices"><span>Codemap & AST Slices</span><span id="cnt-slices">0</span></a>

      <div class="rail-section-label">Feature Subsystems</div>
      <div id="rail-clusters"></div>
    </nav>

    <main class="console-main" id="main-scroll">
      <div class="search-bar">
        <input id="q" placeholder="Semantic Search via EmbeddingGemma2 (:8791) — try 'live cam stream hls', 'face identity merge', 'alerts rules'" onkeydown="if(event.key==='Enter')runSearch()" />
        <button class="btn-primary" onclick="runSearch()">Semantic Search</button>
        <button class="btn-ghost" onclick="document.getElementById('search-results').innerHTML=''">Clear</button>
      </div>
      <div id="search-results"></div>
      <div class="stats-grid" id="stats"></div>
      <div id="view-content"></div>
    </main>

    <aside class="codemap-aside" id="codemap-panel">
      <div class="codemap-title">Intellisense · Codemap Inspector</div>
      <div id="codemap-body" style="font-size:12px;color:var(--vwsq-console-text-secondary);line-height:1.5">
        Click any <strong>function slice</strong>, <strong>API route</strong>, <strong>data model</strong>, <strong>UI view</strong>, or highlighted token in code to inspect its Callers, Callees, Models, and Bound Routes.
      </div>
    </aside>

    <footer class="statusbar">
      <span><span class="led"></span>EmbeddingGemma2 (:8791)</span>
      <span><span class="led"></span>Gemma 4 E2B (:8794)</span>
      <span id="status-route" style="color:var(--vwsq-iris-400)">#overview</span>
      <span id="status-target" style="margin-left:auto"></span>
    </footer>
  </div>

  <script>
    let BP = null;
    const SYM_INDEX = new Map(); // symbol name or path -> hash target

    async function init() {
      const res = await fetch('/api/blueprint');
      BP = await res.json();
      buildSymbolIndex();

      document.getElementById('proj-title').textContent = BP.manifest.project_name;
      document.getElementById('status-target').textContent = BP.manifest.target;
      document.getElementById('stack-pills').textContent = BP.manifest.detected_stack.join(' · ');
      document.getElementById('cnt-svc').textContent = BP.manifest.services.length;
      document.getElementById('cnt-ui').textContent = (BP.ui_views || []).length;
      document.getElementById('cnt-routes').textContent = BP.routes.length;
      document.getElementById('cnt-models').textContent = BP.entities.length + BP.stores.length;
      document.getElementById('cnt-clusters').textContent = BP.clusters.length;
      document.getElementById('cnt-slices').textContent = BP.slices.length;

      document.getElementById('rail-clusters').innerHTML = BP.clusters.map(c =>
        `<a class="nav-btn" href="#cluster/${encodeURIComponent(c.slug)}"><span>${esc(c.title)}</span><span>${c.slice_ids.length}</span></a>`
      ).join('');

      const s = BP.manifest.stats;
      document.getElementById('stats').innerHTML = [
        ['Overview', s.files_scanned + ' files', '#overview'],
        ['UI Views & Pages', (BP.ui_views || []).length, '#ui-views'],
        ['API Endpoints', s.api_routes_discovered, '#openapi'],
        ['Models & Stores', s.entities_extracted + s.state_stores_extracted, '#models'],
        ['Feature Clusters', s.feature_clusters, '#clusters'],
        ['Code Slices', s.proprietary_slices_kept, '#slices'],
      ].map(([l, v, h]) => `<div class="stat-card" onclick="location.hash='${h}'"><div class="stat-val">${v}</div><div class="stat-lbl">${l}</div></div>`).join('');

      window.addEventListener('hashchange', handleHashRoute);
      handleHashRoute();
    }

    function buildSymbolIndex() {
      SYM_INDEX.clear();
      for (const r of BP.routes) {
        SYM_INDEX.set(r.pattern, `#route/${encodeURIComponent(r.id)}`);
        SYM_INDEX.set(r.path, `#route/${encodeURIComponent(r.id)}`);
        SYM_INDEX.set(r.id, `#route/${encodeURIComponent(r.id)}`);
      }
      for (const e of BP.entities) {
        SYM_INDEX.set(e.name, `#model/${encodeURIComponent(e.name)}`);
        SYM_INDEX.set(e.id, `#model/${encodeURIComponent(e.name)}`);
      }
      for (const st of BP.stores) {
        SYM_INDEX.set(st.name, `#store/${encodeURIComponent(st.id)}`);
        SYM_INDEX.set(st.id, `#store/${encodeURIComponent(st.id)}`);
      }
      for (const uv of (BP.ui_views || [])) {
        SYM_INDEX.set(uv.name, `#view/${encodeURIComponent(uv.id)}`);
        SYM_INDEX.set(uv.id, `#view/${encodeURIComponent(uv.id)}`);
      }
      for (const s of BP.slices) {
        SYM_INDEX.set(s.id, `#slice/${encodeURIComponent(s.id)}`);
        if (s.name.length >= 3 && !SYM_INDEX.has(s.name)) {
          SYM_INDEX.set(s.name, `#slice/${encodeURIComponent(s.id)}`);
        }
      }
    }

    function handleHashRoute() {
      if (!BP) return;
      const rawHash = decodeURIComponent(location.hash || '#overview');
      document.getElementById('status-route').textContent = rawHash;

      document.querySelectorAll('.warm-rail .nav-btn').forEach(b => {
        b.classList.toggle('active', b.getAttribute('href') === location.hash || (rawHash === '#overview' && b.id === 'nav-overview'));
      });

      if (rawHash.startsWith('#slice/')) {
        const sid = rawHash.slice(7);
        renderSlicesView(sid);
        const s = BP.slices.find(x => x.id === sid || x.name === sid);
        if (s) inspectSlice(s);
      } else if (rawHash.startsWith('#route/')) {
        const rid = rawHash.slice(7);
        renderOpenApiView(rid);
        const r = BP.routes.find(x => x.id === rid || x.pattern === rid);
        if (r) inspectRoute(r);
      } else if (rawHash.startsWith('#model/')) {
        const mname = rawHash.slice(7);
        renderModelsView(mname);
        const e = BP.entities.find(x => x.name === mname || x.id === mname);
        if (e) inspectEntity(e);
      } else if (rawHash.startsWith('#store/')) {
        const stid = rawHash.slice(7);
        renderModelsView(stid);
        const st = BP.stores.find(x => x.id === stid || x.name === stid);
        if (st) inspectStore(st);
      } else if (rawHash.startsWith('#view/')) {
        const vid = rawHash.slice(6);
        renderUiViews(vid);
        const uv = (BP.ui_views || []).find(x => x.id === vid || x.name === vid);
        if (uv) inspectUiView(uv);
      } else if (rawHash.startsWith('#cluster/')) {
        const slug = rawHash.slice(9);
        renderClustersView(slug);
      } else if (rawHash === '#ui-views') {
        renderUiViews(null);
      } else if (rawHash === '#openapi') {
        renderOpenApiView(null);
      } else if (rawHash === '#models') {
        renderModelsView(null);
      } else if (rawHash === '#clusters') {
        renderClustersView(null);
      } else if (rawHash === '#slices') {
        renderSlicesView(null);
      } else {
        renderOverview();
      }
    }

    function renderOverview() {
      const dc = BP.manifest.domain_context || {};
      const el = document.getElementById('view-content');
      el.innerHTML = `
        <details class="vw-collapse" open>
          <summary><span>Domain & Content Context (What the App Parses & Shows)</span><span class="pill">${(dc.content_signals || []).length} signals</span></summary>
          <div class="collapse-body">
            <div style="margin-bottom:8px"><strong>Media & Inference Modalities:</strong> ${(dc.media_modalities || []).map(m => `<span class="chip online">${esc(m)}</span>`).join(' ') || '–'}</div>
            <div style="margin-bottom:8px"><strong>Content Signals & Domain Vocabulary:</strong> ${(dc.content_signals || []).map(c => `<span class="chip">${esc(c)}</span>`).join(' ')}</div>
            <div><strong>External Domains:</strong> ${(dc.external_domains || []).map(d => `<span class="chip coral">${esc(d)}</span>`).join(' ') || 'None'}</div>
          </div>
        </details>
        <details class="vw-collapse" open>
          <summary><span>OVERVIEW.md (Rendered Blueprint)</span><span class="pill">Markdown</span></summary>
          <div class="collapse-body md-body">${renderMarkdown(BP.overview_markdown)}</div>
        </details>
      `;
    }

    function renderUiViews(focusId) {
      const views = BP.ui_views || [];
      const el = document.getElementById('view-content');
      el.innerHTML = `<details class="vw-collapse" open>
        <summary><span>UI Pages, Tabs & Component Anatomy (${views.length})</span><span class="pill">Visual Surface</span></summary>
        <div class="collapse-body">
          ${views.map(v => `
            <div class="row-card ${focusId === v.id || focusId === v.name ? 'focused' : ''}" id="card-${esc(v.id)}" onclick="location.hash='#view/${encodeURIComponent(v.id)}'">
              <div>
                <span class="badge-method">${esc(v.kind)}</span>
                <strong>${esc(v.name)}</strong>
                <span style="float:right;font-family:var(--vwsq-font-mono);font-size:11px;opacity:.65">${esc(v.source_file)}</span>
              </div>
              <div style="margin-top:6px;font-size:12px;color:var(--vwsq-console-text-secondary)">${esc(v.summary)}</div>
              ${v.rendered_elements.length ? `<div style="margin-top:6px"><strong style="font-size:11px">DOM/JSX Elements:</strong> ${v.rendered_elements.map(e => `<span class="chip">${esc(e)}</span>`).join('')}</div>` : ''}
              ${v.visible_labels_and_copy.length ? `<div style="margin-top:6px"><strong style="font-size:11px">Visible Copy / Controls:</strong> ${v.visible_labels_and_copy.map(c => `<span class="chip online">${esc(c)}</span>`).join('')}</div>` : ''}
              ${v.child_components.length ? `<div style="margin-top:6px"><strong style="font-size:11px">Child Components:</strong> ${v.child_components.map(c => symChip(c)).join('')}</div>` : ''}
              ${v.bound_routes.length ? `<div style="margin-top:6px"><strong style="font-size:11px">Bound API Routes:</strong> ${v.bound_routes.map(r => symChip(r, 'coral')).join('')}</div>` : ''}
            </div>
          `).join('')}
        </div>
      </details>`;
      scrollToFocused(focusId);
    }

    function renderOpenApiView(focusId) {
      const el = document.getElementById('view-content');
      el.innerHTML = `
        <details class="vw-collapse" open>
          <summary><span>Interactive API Route Map (${BP.routes.length} endpoints)</span><span class="pill">OpenAPI 3.1</span></summary>
          <div class="collapse-body">
            ${BP.routes.map(r => `
              <div class="row-card ${focusId === r.id || focusId === r.pattern ? 'focused' : ''}" id="card-${esc(r.id)}" onclick="location.hash='#route/${encodeURIComponent(r.id)}'">
                <div>
                  <span class="badge-method">${esc(r.method)}</span>
                  <strong>${esc(r.pattern)}</strong>
                  <span style="float:right;font-family:var(--vwsq-font-mono);font-size:11px;opacity:.65">${esc(r.source_file)}:${r.source_line}</span>
                </div>
                <div style="margin-top:4px;font-size:12px;opacity:.85">${esc(r.summary)}</div>
                ${r.request_fields.length ? `<div style="margin-top:6px"><strong style="font-size:11px">Request Fields:</strong> ${r.request_fields.map(f => `<span class="chip">${esc(f.name)}: ${esc(f.field_type)}</span>`).join('')}</div>` : ''}
                ${r.response_fields.length ? `<div style="margin-top:6px"><strong style="font-size:11px">Response Keys:</strong> ${r.response_fields.map(f => `<span class="chip online">${esc(f.name)}</span>`).join('')}</div>` : ''}
                ${r.related_slices.length ? `<div style="margin-top:6px"><strong style="font-size:11px">Linked AST Slices:</strong> ${r.related_slices.map(s => `<a class="chip coral" href="#slice/${encodeURIComponent(s)}" onclick="event.stopPropagation()">${esc(s.split(':').pop())} (${esc(s.split(':')[0])})</a>`).join('')}</div>` : ''}
              </div>
            `).join('')}
          </div>
        </details>
        <details class="vw-collapse">
          <summary><span>Raw openapi.json Specification</span><span class="pill">JSON</span></summary>
          <div class="collapse-body"><pre>${esc(JSON.stringify(BP.openapi, null, 2))}</pre></div>
        </details>
      `;
      scrollToFocused(focusId);
    }

    function renderModelsView(focusId) {
      const el = document.getElementById('view-content');
      el.innerHTML = `
        <details class="vw-collapse" open>
          <summary><span>Entities, Interfaces & SQL Tables (${BP.entities.length})</span><span class="pill">models/entities.json</span></summary>
          <div class="collapse-body">
            ${BP.entities.map(e => `
              <div class="row-card ${focusId === e.name || focusId === e.id ? 'focused' : ''}" id="card-${esc(e.name)}" onclick="location.hash='#model/${encodeURIComponent(e.name)}'">
                <div>
                  <span class="badge-method">${esc(e.kind)}</span>
                  <strong>${esc(e.name)}</strong>
                  <span style="float:right;font-family:var(--vwsq-font-mono);font-size:11px;opacity:.65">${esc(e.source_file)}:${e.source_line}</span>
                </div>
                <div style="margin-top:6px">
                  ${e.fields.map(f => `<span class="chip">${esc(f.name)}: <span style="opacity:.7">${esc(f.field_type)}</span></span>`).join('')}
                </div>
              </div>
            `).join('')}
          </div>
        </details>
        <details class="vw-collapse" open>
          <summary><span>Stateful Workers, Queues & React Stores (${BP.stores.length})</span><span class="pill">models/stores.json</span></summary>
          <div class="collapse-body">
            ${BP.stores.map(st => `
              <div class="row-card ${focusId === st.id || focusId === st.name ? 'focused' : ''}" id="card-${esc(st.id)}" onclick="location.hash='#store/${encodeURIComponent(st.id)}'">
                <div>
                  <span class="badge-method">${esc(st.kind)}</span>
                  <strong>${esc(st.name)}</strong>
                  <span style="float:right;font-family:var(--vwsq-font-mono);font-size:11px;opacity:.65">${esc(st.source_file)}</span>
                </div>
                <div style="margin-top:6px"><strong style="font-size:11px">State Keys:</strong> ${st.state_keys.map(k => `<span class="chip">${esc(k)}</span>`).join('') || '–'}</div>
                ${st.actions_or_mutators.length ? `<div style="margin-top:6px"><strong style="font-size:11px">Methods / Actions:</strong> ${st.actions_or_mutators.map(m => symChip(m, 'coral')).join('')}</div>` : ''}
              </div>
            `).join('')}
          </div>
        </details>
      `;
      scrollToFocused(focusId);
    }

    function renderClustersView(focusSlug) {
      const el = document.getElementById('view-content');
      el.innerHTML = BP.clusters.map(c => `
        <details class="vw-collapse" ${!focusSlug || focusSlug === c.slug ? 'open' : ''} id="card-${esc(c.slug)}">
          <summary>
            <span>${esc(c.title)} (${esc(c.slug)})</span>
            <span class="pill">${c.slice_ids.length} slices · ${c.route_ids.length} routes · ${c.entity_names.length} models</span>
          </summary>
          <div class="collapse-body md-body">${renderMarkdown(c.markdown_spec)}</div>
        </details>
      `).join('');
      scrollToFocused(focusSlug);
    }

    function renderSlicesView(focusId) {
      const el = document.getElementById('view-content');
      let list = BP.slices.slice(0, 120);
      if (focusId) {
        const target = BP.slices.find(s => s.id === focusId || s.name === focusId);
        if (target && !list.includes(target)) list.unshift(target);
      }
      el.innerHTML = `
        <details class="vw-collapse" open>
          <summary><span>Proprietary AST Slices & Codemap (${BP.slices.length} total, showing ${list.length})</span><span class="pill">Click any symbol to jump</span></summary>
          <div class="collapse-body">
            ${list.map(s => `
              <div class="row-card ${focusId === s.id || focusId === s.name ? 'focused' : ''}" id="card-${esc(s.id)}" onclick="location.hash='#slice/${encodeURIComponent(s.id)}'">
                <div>
                  <span class="badge-method">${esc(s.category)}</span>
                  <strong>${esc(s.name)}</strong>
                  <span style="float:right;font-family:var(--vwsq-font-mono);font-size:11px;opacity:.65">${esc(s.source_file)}:${s.start_line}-${s.end_line}</span>
                </div>
                <div style="margin-top:4px;font-family:var(--vwsq-font-mono);font-size:12px;color:var(--vwsq-iris-400)">${esc(s.signature_or_pseudo)}</div>
                <div style="margin-top:4px;font-size:12px;opacity:.85">${esc(s.summary)}</div>
                <div style="margin-top:6px">
                  ${(s.called_by_slices || []).map(c => `<a class="chip" href="#slice/${encodeURIComponent(c)}" onclick="event.stopPropagation()" title="Caller">← ${esc(c.split(':').pop())}</a>`).join('')}
                  ${(s.calls_slices || []).map(c => `<a class="chip coral" href="#slice/${encodeURIComponent(c)}" onclick="event.stopPropagation()" title="Callee">→ ${esc(c.split(':').pop())}</a>`).join('')}
                  ${(s.referenced_models || []).map(m => `<a class="chip online" href="#model/${encodeURIComponent(m)}" onclick="event.stopPropagation()" title="Model">${esc(m)}</a>`).join('')}
                </div>
                <pre class="code-block">${highlightAndLinkCode(s.code_excerpt)}</pre>
              </div>
            `).join('')}
          </div>
        </details>
      `;
      scrollToFocused(focusId);
    }

    function inspectSlice(s) {
      const box = document.getElementById('codemap-body');
      box.innerHTML = `
        <div style="font-family:var(--vwsq-font-mono);font-size:14px;font-weight:700;color:var(--vwsq-console-text-bright)">${esc(s.name)}</div>
        <div style="font-family:var(--vwsq-font-mono);font-size:11px;color:var(--vwsq-iris-400);margin:4px 0">${esc(s.source_file)}:${s.start_line}-${s.end_line}</div>
        <div style="margin:8px 0">${esc(s.summary)}</div>
        <div style="margin-top:10px"><strong>Signature:</strong><pre>${esc(s.signature_or_pseudo)}</pre></div>
        <div style="margin-top:10px"><strong>Called By (Callers):</strong><br/>${(s.called_by_slices || []).map(c => `<a class="chip" href="#slice/${encodeURIComponent(c)}">← ${esc(c)}</a>`).join('') || '<span style="opacity:.5">None detected</span>'}</div>
        <div style="margin-top:10px"><strong>Calls (Callees):</strong><br/>${(s.calls_slices || []).map(c => `<a class="chip coral" href="#slice/${encodeURIComponent(c)}">→ ${esc(c)}</a>`).join('') || '<span style="opacity:.5">None detected</span>'}</div>
        <div style="margin-top:10px"><strong>Models & Tables Touched:</strong><br/>${(s.referenced_models || []).map(m => `<a class="chip online" href="#model/${encodeURIComponent(m)}">${esc(m)}</a>`).join('') || '<span style="opacity:.5">None</span>'}</div>
        <div style="margin-top:10px"><strong>API Routes Referenced:</strong><br/>${(s.called_apis || []).map(a => symChip(a, 'coral')).join('') || '<span style="opacity:.5">None</span>'}</div>
        <div style="margin-top:12px"><strong>Code (Intellisense Active):</strong><pre>${highlightAndLinkCode(s.code_excerpt)}</pre></div>
      `;
    }

    function inspectRoute(r) {
      const box = document.getElementById('codemap-body');
      const boundViews = (BP.ui_views || []).filter(v => (v.bound_routes || []).includes(r.pattern));
      box.innerHTML = `
        <div style="font-family:var(--vwsq-font-mono);font-size:14px;font-weight:700;color:var(--vwsq-console-text-bright)"><span class="badge-method">${esc(r.method)}</span>${esc(r.pattern)}</div>
        <div style="font-family:var(--vwsq-font-mono);font-size:11px;color:var(--vwsq-iris-400);margin:4px 0">${esc(r.source_file)}:${r.source_line}</div>
        <div style="margin:8px 0">${esc(r.summary)}</div>
        <div style="margin-top:10px"><strong>Linked AST Slices:</strong><br/>${(r.related_slices || []).map(s => `<a class="chip coral" href="#slice/${encodeURIComponent(s)}">${esc(s)}</a>`).join('') || 'None'}</div>
        <div style="margin-top:10px"><strong>UI Views Calling Route:</strong><br/>${boundViews.map(v => `<a class="chip online" href="#view/${encodeURIComponent(v.id)}">${esc(v.name)}</a>`).join('') || 'None'}</div>
        <div style="margin-top:10px"><strong>Request Fields:</strong><br/>${r.request_fields.map(f => `<span class="chip">${esc(f.name)}: ${esc(f.field_type)}</span>`).join('') || 'None'}</div>
        <div style="margin-top:10px"><strong>Response Keys:</strong><br/>${r.response_fields.map(f => `<span class="chip online">${esc(f.name)}</span>`).join('') || 'None'}</div>
      `;
    }

    function inspectEntity(e) {
      const box = document.getElementById('codemap-body');
      const referencingSlices = BP.slices.filter(s => (s.referenced_models || []).includes(e.name));
      box.innerHTML = `
        <div style="font-family:var(--vwsq-font-mono);font-size:14px;font-weight:700;color:var(--vwsq-console-text-bright)"><span class="badge-method">${esc(e.kind)}</span>${esc(e.name)}</div>
        <div style="font-family:var(--vwsq-font-mono);font-size:11px;color:var(--vwsq-iris-400);margin:4px 0">${esc(e.source_file)}:${e.source_line}</div>
        <div style="margin-top:10px"><strong>Fields (${e.fields.length}):</strong><br/>${e.fields.map(f => `<span class="chip">${esc(f.name)}: ${esc(f.field_type)}</span>`).join('')}</div>
        <div style="margin-top:12px"><strong>Referenced By Slices (${referencingSlices.length}):</strong><br/>${referencingSlices.slice(0, 20).map(s => `<a class="chip coral" href="#slice/${encodeURIComponent(s.id)}">${esc(s.name)} (${esc(s.source_file)})</a>`).join('') || 'None'}</div>
      `;
    }

    function inspectStore(st) {
      const box = document.getElementById('codemap-body');
      box.innerHTML = `
        <div style="font-family:var(--vwsq-font-mono);font-size:14px;font-weight:700;color:var(--vwsq-console-text-bright)"><span class="badge-method">${esc(st.kind)}</span>${esc(st.name)}</div>
        <div style="font-family:var(--vwsq-font-mono);font-size:11px;color:var(--vwsq-iris-400);margin:4px 0">${esc(st.source_file)}</div>
        <div style="margin-top:10px"><strong>State Keys:</strong><br/>${st.state_keys.map(k => `<span class="chip">${esc(k)}</span>`).join('')}</div>
        <div style="margin-top:10px"><strong>Methods / Mutators:</strong><br/>${st.actions_or_mutators.map(m => symChip(m, 'coral')).join('')}</div>
      `;
    }

    function inspectUiView(uv) {
      const box = document.getElementById('codemap-body');
      box.innerHTML = `
        <div style="font-family:var(--vwsq-font-mono);font-size:14px;font-weight:700;color:var(--vwsq-console-text-bright)"><span class="badge-method">${esc(uv.kind)}</span>${esc(uv.name)}</div>
        <div style="font-family:var(--vwsq-font-mono);font-size:11px;color:var(--vwsq-iris-400);margin:4px 0">${esc(uv.source_file)}</div>
        <div style="margin-top:10px"><strong>Rendered Elements:</strong><br/>${uv.rendered_elements.map(e => `<span class="chip">${esc(e)}</span>`).join('') || 'JSX'}</div>
        <div style="margin-top:10px"><strong>Visible Labels & Copy:</strong><br/>${uv.visible_labels_and_copy.map(c => `<span class="chip online">${esc(c)}</span>`).join('') || '–'}</div>
        <div style="margin-top:10px"><strong>Child Components:</strong><br/>${uv.child_components.map(c => symChip(c)).join('') || '–'}</div>
        <div style="margin-top:10px"><strong>Bound API Routes:</strong><br/>${uv.bound_routes.map(r => symChip(r, 'coral')).join('') || '–'}</div>
      `;
    }

    function symChip(name, extraClass = '') {
      const target = SYM_INDEX.get(name);
      if (target) {
        return `<a class="chip ${extraClass}" href="${target}" onclick="event.stopPropagation()">${esc(name)}</a>`;
      }
      return `<span class="chip ${extraClass}">${esc(name)}</span>`;
    }

    function highlightAndLinkCode(code) {
      const escaped = esc(code);
      return escaped.replace(/\b([A-Za-z_][A-Za-z0-9_]{2,})\b/g, (m) => {
        if (['def','async','await','return','class','from','import','const','let','var','function','export','interface','type','pub','struct','if','else','for','while','try','except','in','None','True','False','null','true','false'].includes(m)) {
          return `<span class="tok-kw">${m}</span>`;
        }
        const target = SYM_INDEX.get(m);
        if (target) {
          return `<a class="tok-sym" href="${target}" onclick="event.stopPropagation()" title="Jump to ${m}">${m}</a>`;
        }
        return m;
      });
    }

    function renderMarkdown(md) {
      if (!md) return '';
      const lines = md.split('\n');
      let html = '';
      let inTable = false;
      let inList = false;
      let inCode = false;

      const formatInline = (text) => {
        let s = esc(text);
        s = s.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
        s = s.replace(/`([^`]+)`/g, (_, codeTxt) => {
          const target = SYM_INDEX.get(codeTxt) || SYM_INDEX.get(codeTxt.split('(')[0]);
          if (target) {
            return `<a class="chip coral" href="${target}">${esc(codeTxt)}</a>`;
          }
          return `<code>${esc(codeTxt)}</code>`;
        });
        return s;
      };

      for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmed = line.trim();

        if (trimmed.startsWith('```')) {
          if (inCode) { html += '</pre>'; inCode = false; }
          else { html += '<pre>'; inCode = true; }
          continue;
        }
        if (inCode) {
          html += esc(line) + '\n';
          continue;
        }

        if (trimmed.startsWith('|') && trimmed.endsWith('|')) {
          if (inList) { html += '</ul>'; inList = false; }
          const cells = trimmed.slice(1, -1).split('|').map(c => c.trim());
          if (cells.every(c => /^[-:]+$/.test(c))) continue;
          if (!inTable) {
            html += '<table><thead><tr>' + cells.map(c => `<th>${formatInline(c)}</th>`).join('') + '</tr></thead><tbody>';
            inTable = true;
          } else {
            html += '<tr>' + cells.map(c => `<td>${formatInline(c)}</td>`).join('') + '</tr>';
          }
          continue;
        } else if (inTable) {
          html += '</tbody></table>';
          inTable = false;
        }

        if (trimmed.startsWith('- ')) {
          if (!inList) { html += '<ul>'; inList = true; }
          html += `<li>${formatInline(trimmed.slice(2))}</li>`;
          continue;
        } else if (inList) {
          html += '</ul>';
          inList = false;
        }

        if (trimmed.startsWith('# ')) html += `<h1>${formatInline(trimmed.slice(2))}</h1>`;
        else if (trimmed.startsWith('## ')) html += `<h2>${formatInline(trimmed.slice(3))}</h2>`;
        else if (trimmed.startsWith('### ')) html += `<h3>${formatInline(trimmed.slice(4))}</h3>`;
        else if (trimmed.startsWith('> ')) html += `<blockquote>${formatInline(trimmed.slice(2))}</blockquote>`;
        else if (trimmed.length > 0) html += `<p>${formatInline(trimmed)}</p>`;
      }
      if (inTable) html += '</tbody></table>';
      if (inList) html += '</ul>';
      if (inCode) html += '</pre>';
      return html;
    }

    async function runSearch() {
      const q = document.getElementById('q').value.trim();
      if (!q) return;
      const box = document.getElementById('search-results');
      box.innerHTML = `<div class="row-card">Searching EmbeddingGemma2 (:8791)…</div>`;
      const res = await fetch('/api/search', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({ query: q, top_k: 10 })
      });
      const data = await res.json();
      if (!data.ok) {
        box.innerHTML = `<div class="row-card">Search error: ${esc(data.error || 'unknown')}</div>`;
        return;
      }
      box.innerHTML = `<details class="vw-collapse" open><summary><span>Semantic Hits for "${esc(q)}" (EmbeddingGemma2 256-d)</span><span class="pill">${data.hits.length} results</span></summary><div class="collapse-body">` +
        data.hits.map(h => {
          const hash = SYM_INDEX.get(h.id) || SYM_INDEX.get(h.title) || '#slices';
          return `<div class="row-card" onclick="location.hash='${hash}'"><span class="badge-method">${esc(h.kind)} · ${(h.score * 100).toFixed(1)}%</span><strong>${esc(h.title)}</strong> <span style="float:right;font-family:var(--vwsq-font-mono);font-size:11px;opacity:.65">${esc(h.source)}</span><div style="margin-top:4px;font-size:12px;opacity:.85">${esc(h.subtitle)}</div></div>`;
        }).join('') +
        `</div></details>`;
    }

    function toggleAllDetails(open) {
      document.querySelectorAll('details.vw-collapse').forEach(d => d.open = open);
    }

    function scrollToFocused(focusId) {
      if (!focusId) return;
      setTimeout(() => {
        const card = document.getElementById('card-' + focusId);
        if (card) card.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }, 40);
    }

    function esc(s) {
      return String(s ?? '').replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
    }
    init();
  </script>
</body>
</html>
"##;
