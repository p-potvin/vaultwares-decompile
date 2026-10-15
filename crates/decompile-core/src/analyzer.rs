use crate::cdp::HarvestedWebBundle;
use crate::model::{
    ApiRoute, BlueprintStats, CodeSlice, DetectedService, DomainContext, EntityModel, FieldSpec,
    StateMachineSpec, StateStore, StateTransition, UiViewSpec,
};
use crate::unpacker::{score_proprietary, unpack_js_bundle};
use anyhow::Result;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(Debug, Default)]
pub struct AnalysisOutput {
    pub detected_stack: Vec<String>,
    pub domain_context: DomainContext,
    pub services: Vec<DetectedService>,
    pub routes: Vec<ApiRoute>,
    pub entities: Vec<EntityModel>,
    pub stores: Vec<StateStore>,
    pub ui_views: Vec<UiViewSpec>,
    pub state_machine: StateMachineSpec,
    pub slices: Vec<CodeSlice>,
    pub stats: BlueprintStats,
}

static DECORATOR_ROUTE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)@(?:app|router|api|bp)\.(get|post|put|delete|patch|websocket|route)\(\s*["']([^"']+)["']"#).unwrap()
});

static EXPRESS_AXUM_ROUTE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)\b(?:app|router|server)\.(get|post|put|delete|patch)\(\s*["'](/[^"']*)["']"#).unwrap()
});

static AXUM_METHOD_ROUTE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\.route\(\s*"(/[^"]*)"\s*,\s*(get|post|put|delete|patch)\(([a-zA-Z0-9_:]+)\)"#).unwrap()
});

static PATH_BRANCH_ROUTE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)(?:path|pathname|route|url|p)\s*(?:==|===|in\s*\(|\.startswith\()\s*["'](/[a-zA-Z0-9_./:-]+)["']"#).unwrap()
});

static GENERIC_API_LITERAL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"["'`](/(?:(?:api|ws|webservices|ajax|graphql|v1|v2)/[a-zA-Z0-9_./?=&${}:-]+|[a-zA-Z0-9_./-]+\.(?:php|json|m3u8)(?:\?[a-zA-Z0-9_=&${}:-]+)?|health|embed|status|metrics|ws|stream))["'`]"#).unwrap()
});

static WINDOW_STATE_ASSIGN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"window\.(__[a-zA-Z0-9_]+|[A-Z][a-zA-Z0-9_]{2,})\s*=\s*\{").unwrap()
});

static NAMED_ARRAY_KEY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"["']([a-zA-Z_][a-zA-Z0-9_]{2,28})["']\s*:\s*\[\s*\{"#).unwrap()
});

static FETCH_CALL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"fetch\(\s*[`"']((?:https?://[^`"']+|/[a-zA-Z0-9_./${}:-]+))[`"'](?:\s*,\s*\{([^}]{0,350})\})?"#).unwrap()
});

static PYTHON_HTTP_CALL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:requests|httpx|session|client)\.(get|post|put|delete)\(\s*f?["']([^"']+)["']"#).unwrap()
});

static PY_DICT_GET_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:body|data|payload|req|item|params|query|args)\.get\(\s*["']([a-zA-Z0-9_]+)["']"#).unwrap()
});

static JSON_KEY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"["']([a-zA-Z_][a-zA-Z0-9_]{1,32})["']\s*:"#).unwrap()
});

static PY_CLASS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^class\s+([A-Z][a-zA-Z0-9_]*)\s*(?:\(([^)]*)\))?\s*:").unwrap()
});

static PY_DEF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^([ \t]*)(?:async\s+)?def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?\s*:").unwrap()
});

static JS_FUNC_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)(?:export\s+(?:default\s+)?)?(?:async\s+)?function\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(([^)]*)\)").unwrap()
});

static JS_CONST_FN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)(?:export\s+)?const\s+([a-zA-Z_$][a-zA-Z0-9_$]{2,})\s*=\s*(?:async\s*)?\(([^)]*)\)\s*=>").unwrap()
});

static TS_INTERFACE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)(?:export\s+)?(?:interface|type)\s+([A-Z][a-zA-Z0-9_]*)\s*=?\s*\{").unwrap()
});

static TS_UNION_TYPE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)(?:export\s+)?type\s+([A-Z][a-zA-Z0-9_]*)\s*=\s*(['"][^;\n]+)"#).unwrap()
});

static RUST_STRUCT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)pub\s+struct\s+([A-Z][a-zA-Z0-9_]*)\s*\{").unwrap()
});

static SQL_TABLE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)CREATE\s+TABLE\s+(?:IF\s+NOT\s+EXISTS\s+)?([a-zA-Z0-9_]+)\s*\(([^;]+)\)").unwrap()
});

static PORT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:port|--port|PORT)\s*[:=]\s*(\d{4,5})").unwrap()
});

static USE_STATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"const\s+\[\s*([a-zA-Z0-9_]+)\s*,\s*set[A-Za-z0-9_]+\s*\]\s*=\s*useState").unwrap()
});

static URL_HOST_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"https?://([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})(?:/[^\s"'`)]*)?"#).unwrap()
});

static JSX_COMPONENT_TAG_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<([A-Z][a-zA-Z0-9_]*)\b").unwrap()
});

static JSX_HTML_TAG_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<(video|audio|canvas|img|table|form|dialog|select|input|textarea|button|iframe|svg)\b").unwrap()
});

static JSX_TEXT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r">\s*([A-Z][a-zA-Z0-9 /·:&+#-]{2,48})\s*<").unwrap()
});

static JSX_ATTR_COPY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:placeholder|title|label|aria-label)\s*=\s*["']([^"']{3,60})["']"#).unwrap()
});

static JSX_HANDLER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(on(?:Click|Submit|Change|KeyDown|MouseDown|DoubleClick|DragStart|Drop))\s*=").unwrap()
});

static CSS_VAR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(--[a-zA-Z0-9-]+)\s*:").unwrap()
});

static IDENT_CALL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]{2,})\s*\(").unwrap()
});

static IDENT_WORD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b([A-Z][a-zA-Z0-9_]{2,})\b").unwrap()
});

pub fn analyze_directory(root: &Path) -> Result<AnalysisOutput> {
    let mut out = AnalysisOutput::default();
    let mut stack_set = BTreeSet::new();
    let mut ext_domains = BTreeSet::new();
    let mut media_mods = BTreeSet::new();
    let mut content_signals = BTreeSet::new();
    let mut theme_tokens = BTreeSet::new();

    let mut files = Vec::new();
    collect_files(root, root, &mut files);
    out.stats.files_scanned = files.len();

    for file_path in &files {
        let rel = file_path
            .strip_prefix(root)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/");

        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let Ok(content) = fs::read_to_string(file_path) else {
            continue;
        };

        detect_stack_from_file(&rel, &ext, &content, &mut stack_set);
        detect_service_from_file(&rel, &content, &mut out.services);
        harvest_domain_context(
            &rel,
            &content,
            &mut ext_domains,
            &mut media_mods,
            &mut content_signals,
            &mut theme_tokens,
        );

        match ext.as_str() {
            "js" | "jsx" | "ts" | "tsx" | "mjs" => {
                analyze_js_ts_file(&rel, &content, &mut out, &mut content_signals);
            }
            "py" => {
                analyze_python_file(&rel, &content, &mut out);
            }
            "rs" => {
                analyze_rust_file(&rel, &content, &mut out);
            }
            "sql" => {
                extract_sql_tables(&rel, &content, &mut out.entities);
            }
            "html" | "css" => {
                analyze_html_file(&rel, &content, &mut out);
            }
            _ => {}
        }
    }

    finalize_analysis(
        &mut out,
        stack_set,
        ext_domains,
        media_mods,
        content_signals,
        theme_tokens,
    );

    Ok(out)
}

pub fn analyze_harvested_web_bundle(bundle: &HarvestedWebBundle) -> Result<AnalysisOutput> {
    let mut out = AnalysisOutput::default();
    let mut stack_set = BTreeSet::new();
    let mut ext_domains = BTreeSet::new();
    let mut media_mods = BTreeSet::new();
    let mut content_signals = BTreeSet::new();
    let mut theme_tokens = BTreeSet::new();

    stack_set.insert("Web SPA / HTTP".into());
    out.stats.files_scanned = 1 + bundle.scripts.len();

    if !bundle.html.is_empty() {
        harvest_domain_context(
            "index.html",
            &bundle.html,
            &mut ext_domains,
            &mut media_mods,
            &mut content_signals,
            &mut theme_tokens,
        );
        analyze_html_file("index.html", &bundle.html, &mut out);
    }

    for probed in &bundle.live_endpoints_probed {
        let mut resp_fields = Vec::new();
        if let Some(serde_json::Value::Object(map)) = &probed.sample_body {
            for (k, v) in map {
                resp_fields.push(FieldSpec {
                    name: k.clone(),
                    field_type: json_value_type(v),
                    required: true,
                    description: Some("Observed from live HTTP response".into()),
                });
            }
        }
        out.routes.push(ApiRoute {
            id: format!("live_{}_{}", probed.method.to_lowercase(), sanitise_id(&probed.path)),
            method: probed.method.clone(),
            path: probed.path.clone(),
            pattern: normalise_route_pattern(&probed.path),
            path_params: extract_path_params(&probed.path),
            query_params: Vec::new(),
            summary: format!("Live probed endpoint {} {}", probed.method, probed.path),
            source_file: bundle.target_url.clone(),
            source_line: 1,
            handler_name: None,
            request_fields: Vec::new(),
            response_fields: resp_fields.clone(),
            sample_request: None,
            sample_response: probed.sample_body.clone(),
            cdp_initiator: None,
            related_slices: Vec::new(),
        });

        // If the live endpoint is an OpenAPI 3.x / Swagger spec (`/openapi.json`), ingest its paths & schemas
        if probed.path.ends_with("openapi.json") {
            if let Some(serde_json::Value::Object(root_obj)) = &probed.sample_body {
                if let Some(serde_json::Value::Object(paths_obj)) = root_obj.get("paths") {
                    for (path_key, methods_val) in paths_obj {
                        if let serde_json::Value::Object(methods_map) = methods_val {
                            for (method_key, op_val) in methods_map {
                                let m_upper = method_key.to_uppercase();
                                if !matches!(m_upper.as_str(), "GET" | "POST" | "PUT" | "DELETE" | "PATCH") {
                                    continue;
                                }
                                let summary = op_val
                                    .get("summary")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Live OpenAPI route")
                                    .to_string();
                                let pattern = normalise_route_pattern(path_key);
                                out.routes.push(ApiRoute {
                                    id: format!("openapi_{}_{}", method_key.to_lowercase(), sanitise_id(&pattern)),
                                    method: m_upper,
                                    path: path_key.clone(),
                                    pattern: pattern.clone(),
                                    path_params: extract_path_params(&pattern),
                                    query_params: Vec::new(),
                                    summary,
                                    source_file: "openapi.json".into(),
                                    source_line: 1,
                                    handler_name: op_val.get("operationId").and_then(|v| v.as_str()).map(|s| s.to_string()),
                                    request_fields: Vec::new(),
                                    response_fields: Vec::new(),
                                    sample_request: None,
                                    sample_response: None,
                                    cdp_initiator: None,
                                    related_slices: Vec::new(),
                                });
                            }
                        }
                    }
                }
                if let Some(serde_json::Value::Object(schemas_obj)) = root_obj
                    .get("components")
                    .and_then(|c| c.get("schemas"))
                {
                    for (schema_name, schema_val) in schemas_obj {
                        let mut fields = Vec::new();
                        if let Some(serde_json::Value::Object(props)) = schema_val.get("properties") {
                            for (prop_name, prop_val) in props {
                                let ftype = prop_val
                                    .get("type")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("any")
                                    .to_string();
                                fields.push(FieldSpec {
                                    name: prop_name.clone(),
                                    field_type: ftype,
                                    required: true,
                                    description: prop_val.get("title").and_then(|v| v.as_str()).map(|s| s.to_string()),
                                });
                            }
                        }
                        if !fields.is_empty() {
                            out.entities.push(EntityModel {
                                id: format!("openapi_schema_{}", sanitise_id(schema_name)),
                                name: schema_name.clone(),
                                kind: "openapi_schema".into(),
                                source_file: "openapi.json".into(),
                                source_line: 1,
                                fields,
                                description: format!("Live OpenAPI component schema `{}`", schema_name),
                            });
                        }
                    }
                }
            }
        } else if !resp_fields.is_empty() {
            out.entities.push(EntityModel {
                id: format!("live_response_{}", sanitise_id(&probed.path)),
                name: format!("LiveResponse_{}", sanitise_id(&probed.path)),
                kind: "json_response".into(),
                source_file: probed.path.clone(),
                source_line: 1,
                fields: resp_fields,
                description: format!("Live JSON response contract from `{} {}`", probed.method, probed.path),
            });
        }
    }

    for script in &bundle.scripts {
        detect_stack_from_file(&script.filename, "js", &script.content, &mut stack_set);
        harvest_domain_context(
            &script.filename,
            &script.content,
            &mut ext_domains,
            &mut media_mods,
            &mut content_signals,
            &mut theme_tokens,
        );
        analyze_js_ts_file(&script.filename, &script.content, &mut out, &mut content_signals);
    }

    finalize_analysis(
        &mut out,
        stack_set,
        ext_domains,
        media_mods,
        content_signals,
        theme_tokens,
    );

    Ok(out)
}

fn finalize_analysis(
    out: &mut AnalysisOutput,
    stack_set: BTreeSet<String>,
    ext_domains: BTreeSet<String>,
    media_mods: BTreeSet<String>,
    content_signals: BTreeSet<String>,
    theme_tokens: BTreeSet<String>,
) {
    deduplicate_routes(&mut out.routes);
    build_codemap_links(
        &mut out.slices,
        &mut out.routes,
        &out.entities,
        &out.stores,
    );

    out.detected_stack = stack_set.into_iter().collect();
    out.domain_context = DomainContext {
        content_signals: content_signals.into_iter().take(48).collect(),
        external_domains: ext_domains.into_iter().take(24).collect(),
        media_modalities: media_mods.into_iter().collect(),
        ui_theme_tokens: theme_tokens.into_iter().take(24).collect(),
    };

    out.stats.proprietary_slices_kept = out.slices.len();
    out.stats.api_routes_discovered = out.routes.len();
    out.stats.entities_extracted = out.entities.len();
    out.stats.state_stores_extracted = out.stores.len();
    out.stats.ui_views_extracted = out.ui_views.len();
}

/// Builds deterministic Codemap / Intellisense links between slices (Callers <-> Callees),
/// models (`referenced_models`), and API routes (`related_slices`).
fn build_codemap_links(
    slices: &mut [CodeSlice],
    routes: &mut [ApiRoute],
    entities: &[EntityModel],
    stores: &[StateStore],
) {
    let model_names: HashSet<&str> = entities
        .iter()
        .map(|e| e.name.as_str())
        .chain(stores.iter().map(|s| s.name.as_str()))
        .collect();

    // Map function name -> slice IDs
    let mut name_to_slice_id: HashMap<String, String> = HashMap::new();
    for s in slices.iter() {
        if s.name.len() >= 3 && s.name != "main" && s.name != "__init__" {
            name_to_slice_id
                .entry(s.name.clone())
                .or_insert_with(|| s.id.clone());
        }
    }

    let mut reverse_calls: HashMap<String, BTreeSet<String>> = HashMap::new();

    for s in slices.iter_mut() {
        let mut callees = BTreeSet::new();
        for cap in IDENT_CALL_RE.captures_iter(&s.code_excerpt) {
            let called_name = cap.get(1).unwrap().as_str();
            if called_name != s.name {
                if let Some(target_id) = name_to_slice_id.get(called_name) {
                    callees.insert(target_id.clone());
                    reverse_calls
                        .entry(target_id.clone())
                        .or_default()
                        .insert(s.id.clone());
                }
            }
        }
        s.calls_slices = callees.into_iter().take(12).collect();

        let mut refs = BTreeSet::new();
        for cap in IDENT_WORD_RE.captures_iter(&s.code_excerpt) {
            let word = cap.get(1).unwrap().as_str();
            if model_names.contains(word) {
                refs.insert(word.to_string());
            }
        }
        for cap in IDENT_WORD_RE.captures_iter(&s.signature_or_pseudo) {
            let word = cap.get(1).unwrap().as_str();
            if model_names.contains(word) {
                refs.insert(word.to_string());
            }
        }
        s.referenced_models = refs.into_iter().take(10).collect();
    }

    for s in slices.iter_mut() {
        if let Some(callers) = reverse_calls.get(&s.id) {
            s.called_by_slices = callers.iter().take(12).cloned().collect();
        }
    }

    // Deterministically link routes to slices that either live on the immediately following lines
    // of the same file OR explicitly call that route path
    let slice_lookup: Vec<(String, String, usize, Vec<String>)> = slices
        .iter()
        .map(|s| {
            (
                s.id.clone(),
                s.source_file.clone(),
                s.start_line,
                s.called_apis.clone(),
            )
        })
        .collect();

    for route in routes.iter_mut() {
        let mut exact_matches = Vec::new();
        for (sid, sfile, sline, called_apis) in &slice_lookup {
            if sfile == &route.source_file && *sline >= route.source_line && *sline <= route.source_line + 6 {
                exact_matches.push(sid.clone());
            } else if called_apis.iter().any(|api| api == &route.pattern || api == &route.path) {
                exact_matches.push(sid.clone());
            }
        }
        for m in exact_matches.into_iter().rev() {
            if !route.related_slices.contains(&m) {
                route.related_slices.insert(0, m);
            }
        }
    }
}

fn harvest_domain_context(
    rel: &str,
    content: &str,
    ext_domains: &mut BTreeSet<String>,
    media_mods: &mut BTreeSet<String>,
    content_signals: &mut BTreeSet<String>,
    theme_tokens: &mut BTreeSet<String>,
) {
    for cap in URL_HOST_RE.captures_iter(content) {
        let host = cap.get(1).unwrap().as_str().to_lowercase();
        if !host.starts_with("127.")
            && !host.starts_with("localhost")
            && !host.contains("w3.org")
            && !host.contains("schema.org")
            && !host.contains("mozilla.org")
            && !host.contains("github.com")
        {
            ext_domains.insert(host);
        }
    }

    for cap in CSS_VAR_RE.captures_iter(content) {
        theme_tokens.insert(cap.get(1).unwrap().as_str().to_string());
    }

    let lower = content.to_lowercase();
    let modality_checks = [
        ("m3u8", "HLS Live Video Streams (.m3u8)"),
        ("hls.js", "HLS Live Video Playback"),
        ("ffmpeg", "FFmpeg Video/Audio Transcoding & Frame Extraction"),
        ("audio_wave", "Waveform Audio Sampling & Speech Processing"),
        ("clap", "CLAP Audio/Acoustic Liveliness Scoring"),
        ("whisper", "Speech-to-Text Transcription & Language ID"),
        ("nudenet", "NudeNet Visual Nudity / State Classification"),
        ("insightface", "Face Detection & ArcFace Identity Embedding"),
        ("face_crops", "Face Crop Extraction & Identity Gallery Clustering"),
        ("embeddinggemma", "Multimodal Vector Embeddings (EmbeddingGemma 2)"),
    ];
    for (needle, label) in modality_checks {
        if lower.contains(needle) {
            media_mods.insert(label.into());
        }
    }

    // Domain content keywords & platforms
    let domain_vocab = [
        "chaturbate",
        "stripchat",
        "camsoda",
        "bongacams",
        "myfreecams",
        "onlyfans",
        "fansly",
        "cam4",
        "flirt4free",
        "livejasmin",
        "performer",
        "tippers",
        "tokens",
        "toy_runs",
        "lush",
        "lovense",
        "nude",
        "topless",
        "viewers",
        "private_show",
        "personas",
        "sightings",
        "twins",
        "director",
        "taste_head",
        "moments",
        "stills",
        "recordings",
    ];
    for kw in domain_vocab {
        if lower.contains(kw) {
            content_signals.insert(kw.into());
        }
    }

    let _ = rel;
}

fn collect_files(root: &Path, dir: &Path, acc: &mut Vec<PathBuf>) {
    let skip_dirs = [
        ".git",
        "node_modules",
        "target",
        ".venv",
        "venv",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        "dist",
        "build",
        "vaultwares-themes",
        "vaultwares-adk",
        "blueprints",
    ];

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            if skip_dirs.contains(&name.as_str()) || name.starts_with('.') {
                continue;
            }
            collect_files(root, &path, acc);
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if matches!(
                ext.as_str(),
                "js" | "jsx" | "ts" | "tsx" | "mjs" | "py" | "rs" | "sql" | "html" | "css" | "toml" | "json" | "yaml" | "yml"
            ) {
                if let Ok(meta) = path.metadata() {
                    if meta.len() <= 2_500_000 && !name.ends_with("lock.json") && name != "Cargo.lock" {
                        acc.push(path);
                    }
                }
            }
        }
    }
}

fn detect_stack_from_file(rel: &str, ext: &str, content: &str, stack: &mut BTreeSet<String>) {
    match ext {
        "py" => {
            stack.insert("Python".into());
            if content.contains("FastAPI") || content.contains("fastapi") {
                stack.insert("FastAPI".into());
            }
            if content.contains("http.server") || content.contains("BaseHTTPRequestHandler") {
                stack.insert("Python http.server".into());
            }
            if content.contains("torch") {
                stack.insert("PyTorch / CUDA".into());
            }
            if content.contains("cv2") || content.contains("opencv") {
                stack.insert("OpenCV / Vision".into());
            }
            if content.contains("sqlite3") {
                stack.insert("SQLite".into());
            }
            if content.contains("sentence_transformers") || content.contains("AutoModel") || content.contains("Embedding") {
                stack.insert("Vector Embeddings".into());
            }
        }
        "js" | "jsx" | "ts" | "tsx" | "mjs" => {
            stack.insert("JavaScript/TypeScript".into());
            if content.contains("from 'react'") || content.contains("from \"react\"") || content.contains("react.element") || content.contains("useState") {
                stack.insert("React".into());
            }
            if content.contains("@tauri-apps") {
                stack.insert("Tauri v2".into());
            }
            if content.contains("vwsq-") || content.contains("@vwsq") {
                stack.insert("vaultsqware Design System".into());
            }
        }
        "rs" => {
            stack.insert("Rust".into());
            if content.contains("axum::") {
                stack.insert("Axum".into());
            }
        }
        _ => {}
    }
    let _ = rel;
}

fn detect_service_from_file(rel: &str, content: &str, services: &mut Vec<DetectedService>) {
    let lower = rel.to_lowercase();
    if lower.ends_with("server.py")
        || lower.ends_with("worker.py")
        || lower.ends_with("main.py")
        || lower.ends_with("app.py")
        || lower.ends_with("server.js")
        || lower.ends_with("main.rs")
    {
        let port = PORT_RE
            .captures(content)
            .and_then(|c| c.get(1))
            .map(|m| format!("http://127.0.0.1:{}", m.as_str()));

        let kind = if lower.contains("worker") {
            "background-worker"
        } else if lower.contains("embed") {
            "embedding-service"
        } else {
            "http-service"
        };

        let summary = extract_module_docstring_or_header(content)
            .unwrap_or_else(|| format!("Service entrypoint in {}", rel));

        services.push(DetectedService {
            name: rel
                .trim_end_matches(".py")
                .trim_end_matches(".js")
                .trim_end_matches(".rs")
                .replace('/', "-"),
            kind: kind.into(),
            bind_or_url: port,
            entrypoint: rel.to_string(),
            summary,
        });
    }
}

fn analyze_js_ts_file(
    rel: &str,
    content: &str,
    out: &mut AnalysisOutput,
    content_signals: &mut BTreeSet<String>,
) {
    let unpacked = unpack_js_bundle(content, rel);
    if unpacked.len() > 1 {
        out.stats.bundles_unpacked += 1;
    }

    // Capture TypeScript union enums (e.g. `type SiteId = 'chaturbate' | ...`, `type RuleEvent = ...`)
    for cap in TS_UNION_TYPE_RE.captures_iter(content) {
        let type_name = cap.get(1).unwrap().as_str();
        let union_vals = cap.get(2).unwrap().as_str().trim();
        content_signals.insert(format!("type {} = {}", type_name, union_vals));
    }

    for module in unpacked {
        if module.is_vendor {
            out.stats.vendor_modules_stripped += 1;
            continue;
        }
        let code = &module.code;

        // 1. Extract client fetch() calls
        for cap in FETCH_CALL_RE.captures_iter(code) {
            let raw_url = cap.get(1).map(|m| m.as_str()).unwrap_or("/");
            let opts = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let method = if opts.to_uppercase().contains("POST") {
                "POST"
            } else if opts.to_uppercase().contains("PUT") {
                "PUT"
            } else if opts.to_uppercase().contains("DELETE") {
                "DELETE"
            } else {
                "GET"
            };
            let pattern = normalise_route_pattern(raw_url);
            out.routes.push(ApiRoute {
                id: format!("client_{}_{}", method.to_lowercase(), sanitise_id(&pattern)),
                method: method.into(),
                path: raw_url.into(),
                pattern: pattern.clone(),
                path_params: extract_path_params(&pattern),
                query_params: extract_query_params(raw_url),
                summary: format!("Client fetch({}) call in {}", method, rel),
                source_file: rel.to_string(),
                source_line: module.start_line,
                handler_name: Some(module.id.clone()),
                request_fields: Vec::new(),
                response_fields: Vec::new(),
                sample_request: None,
                sample_response: None,
                cdp_initiator: Some(format!("{}:{}", rel, module.start_line)),
                related_slices: Vec::new(),
            });
        }

        // 2. Extract Express/server routes
        for cap in EXPRESS_AXUM_ROUTE_RE.captures_iter(code) {
            let method = cap.get(1).unwrap().as_str().to_uppercase();
            let path = cap.get(2).unwrap().as_str();
            let pattern = normalise_route_pattern(path);
            out.routes.push(ApiRoute {
                id: format!("route_{}_{}", method.to_lowercase(), sanitise_id(&pattern)),
                method,
                path: path.into(),
                pattern: pattern.clone(),
                path_params: extract_path_params(&pattern),
                query_params: Vec::new(),
                summary: format!("HTTP route {} in {}", path, rel),
                source_file: rel.to_string(),
                source_line: module.start_line,
                handler_name: None,
                request_fields: Vec::new(),
                response_fields: Vec::new(),
                sample_request: None,
                sample_response: None,
                cdp_initiator: None,
                related_slices: Vec::new(),
            });
        }

        // 2b. Extract client XHR / AJAX / API endpoint literals in JS bundles
        for cap in GENERIC_API_LITERAL_RE.captures_iter(code) {
            let full_m = cap.get(0).unwrap();
            let raw_url = cap.get(1).unwrap().as_str();
            if raw_url.len() < 4 || raw_url.ends_with(".js") || raw_url.ends_with(".css") || raw_url.contains(".tpl.php") {
                continue;
            }
            let line = module.start_line
                + code[..full_m.start()].bytes().filter(|&b| b == b'\n').count();
            let pattern = normalise_route_pattern(raw_url);
            out.routes.push(ApiRoute {
                id: format!("js_api_get_{}", sanitise_id(&pattern)),
                method: "GET".into(),
                path: raw_url.into(),
                pattern: pattern.clone(),
                path_params: extract_path_params(&pattern),
                query_params: extract_query_params(raw_url),
                summary: format!("Client API/endpoint literal in {}", rel),
                source_file: rel.to_string(),
                source_line: line,
                handler_name: Some(module.id.clone()),
                request_fields: Vec::new(),
                response_fields: Vec::new(),
                sample_request: None,
                sample_response: None,
                cdp_initiator: Some(format!("{}:{}", rel, line)),
                related_slices: Vec::new(),
            });
        }

        // 2c. Extract embedded window.__STATE__ stores and JSON entity schemas
        for cap in WINDOW_STATE_ASSIGN_RE.captures_iter(code) {
            let var_name = cap.get(1).unwrap().as_str().to_string();
            let full = cap.get(0).unwrap();
            let start_line = 1 + code[..full.start()].bytes().filter(|&b| b == b'\n').count();
            let window_end = (full.end() + 45_000).min(code.len());
            let snippet = &code[full.end() - 1..window_end];

            let mut top_keys = Vec::new();
            for kcap in JSON_KEY_RE.captures_iter(&snippet[..snippet.len().min(3000)]) {
                let k = kcap.get(1).unwrap().as_str().to_string();
                if !top_keys.contains(&k) && top_keys.len() < 16 {
                    top_keys.push(k);
                }
            }

            out.stores.push(StateStore {
                id: format!("window_store_{}", sanitise_id(&var_name)),
                name: format!("window.{}", var_name),
                kind: "window_global_store".into(),
                source_file: rel.to_string(),
                state_keys: top_keys,
                actions_or_mutators: Vec::new(),
                persists_to: None,
                description: format!("Embedded global state object `window.{}` in {}", var_name, rel),
            });

            for acap in NAMED_ARRAY_KEY_RE.captures_iter(snippet) {
                let arr_name = acap.get(1).unwrap().as_str().to_string();
                let brace_pos = acap.get(0).unwrap().end() - 1;
                if let Some(first_obj) = crate::unpacker::extract_balanced_braces(snippet, brace_pos) {
                    if let Ok(serde_json::Value::Object(map)) = serde_json::from_str::<serde_json::Value>(&first_obj) {
                        let fields: Vec<FieldSpec> = map
                            .into_iter()
                            .take(64)
                            .map(|(k, v)| FieldSpec {
                                name: k,
                                field_type: json_value_type(&v),
                                required: true,
                                description: Some(format!("Observed in window.{}.{}[]", var_name, arr_name)),
                            })
                            .collect();
                        if !fields.is_empty() {
                            out.entities.push(EntityModel {
                                id: format!("json_entity_{}_{}", sanitise_id(&var_name), sanitise_id(&arr_name)),
                                name: format!("{}_{}", var_name.trim_matches('_'), arr_name),
                                kind: "json_schema".into(),
                                source_file: rel.to_string(),
                                source_line: start_line,
                                fields,
                                description: format!("Hydrated JSON collection `window.{}.{}` in {}", var_name, arr_name, rel),
                            });
                        }
                    }
                }
            }
        }

        // 3. Extract TypeScript interfaces
        for cap in TS_INTERFACE_RE.captures_iter(code) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let full = cap.get(0).unwrap();
            if let Some(body) = crate::unpacker::extract_balanced_braces(code, full.end() - 1) {
                let fields = parse_ts_fields(&body);
                if !fields.is_empty() {
                    out.entities.push(EntityModel {
                        id: format!("ts_{}", sanitise_id(&name)),
                        name,
                        kind: "interface".into(),
                        source_file: rel.to_string(),
                        source_line: 1 + code[..full.start()].bytes().filter(|&b| b == b'\n').count(),
                        fields,
                        description: format!("TypeScript model in {}", rel),
                    });
                }
            }
        }

        // 4. Extract React state stores
        let state_keys: Vec<String> = USE_STATE_RE
            .captures_iter(code)
            .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
            .collect();
        if !state_keys.is_empty() {
            out.stores.push(StateStore {
                id: format!("react_state_{}", sanitise_id(rel)),
                name: format!("ComponentState({})", rel),
                kind: "react_state".into(),
                source_file: rel.to_string(),
                state_keys: state_keys.clone(),
                actions_or_mutators: Vec::new(),
                persists_to: if code.contains("localStorage") {
                    Some("localStorage".into())
                } else {
                    None
                },
                description: format!("UI component state hooks in {}", rel),
            });
        }

        // 5. Extract UI View / Page / Component Anatomy from JSX/TSX or bundled UI modules
        extract_ui_view_from_js(rel, code, &state_keys, out);

        // 6. Extract proprietary JS/TS functions as slices
        extract_js_slices(rel, code, module.proprietary_score, out);
    }
}

fn extract_ui_view_from_js(
    rel: &str,
    code: &str,
    state_keys: &[String],
    out: &mut AnalysisOutput,
) {
    let is_ui_file = rel.ends_with(".tsx")
        || rel.ends_with(".jsx")
        || code.contains("jsx(")
        || code.contains("jsxs(")
        || code.contains("createElement(")
        || JSX_HTML_TAG_RE.is_match(code);

    if !is_ui_file {
        return;
    }

    let rendered_elements: Vec<String> = JSX_HTML_TAG_RE
        .captures_iter(code)
        .filter_map(|c| c.get(1).map(|m| format!("<{}>", m.as_str())))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let child_components: Vec<String> = JSX_COMPONENT_TAG_RE
        .captures_iter(code)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .take(20)
        .collect();

    let mut visible_copy = BTreeSet::new();
    for cap in JSX_TEXT_RE.captures_iter(code) {
        let txt = cap.get(1).unwrap().as_str().trim();
        if txt.len() >= 3 && !txt.contains('{') && !txt.contains('}') {
            visible_copy.insert(txt.to_string());
        }
    }
    for cap in JSX_ATTR_COPY_RE.captures_iter(code) {
        visible_copy.insert(cap.get(1).unwrap().as_str().trim().to_string());
    }

    let interactive_handlers: Vec<String> = JSX_HANDLER_RE
        .captures_iter(code)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let bound_routes: Vec<String> = GENERIC_API_LITERAL_RE
        .captures_iter(code)
        .filter_map(|c| c.get(1).map(|m| normalise_route_pattern(m.as_str())))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let view_name = Path::new(rel)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(rel)
        .to_string();

    let kind = if rel.to_lowercase().contains("app.") || rel.contains("views/") || rel.contains("pages/") {
        "page_or_tab"
    } else {
        "component"
    };

    let summary = format!(
        "UI {} `{}` ({}) — state keys: [{}], renders: [{}], labels/copy: [{}]",
        kind,
        view_name,
        rel,
        if state_keys.is_empty() {
            "stateless".into()
        } else {
            state_keys.iter().take(8).cloned().collect::<Vec<_>>().join(", ")
        },
        if rendered_elements.is_empty() {
            "custom JSX".into()
        } else {
            rendered_elements.join(", ")
        },
        visible_copy.iter().take(8).cloned().collect::<Vec<_>>().join(" | ")
    );

    out.state_machine
        .views_and_workers
        .push(format!("UI View `{}` ({})", view_name, rel));

    out.ui_views.push(UiViewSpec {
        id: format!("ui_{}", sanitise_id(rel)),
        name: view_name,
        kind: kind.into(),
        source_file: rel.to_string(),
        start_line: 1,
        rendered_elements,
        visible_labels_and_copy: visible_copy.into_iter().take(24).collect(),
        child_components,
        bound_routes,
        bound_models: state_keys.to_vec(),
        interactive_handlers,
        summary,
    });
}

fn analyze_python_file(rel: &str, content: &str, out: &mut AnalysisOutput) {
    let (is_vendor, prop_score) = score_proprietary(content, rel);
    if is_vendor {
        out.stats.vendor_modules_stripped += 1;
        return;
    }

    extract_sql_tables(rel, content, &mut out.entities);

    // 1. Decorator routes (@app.get, @app.post, etc.)
    for cap in DECORATOR_ROUTE_RE.captures_iter(content) {
        let method = cap.get(1).unwrap().as_str().to_uppercase();
        let path = cap.get(2).unwrap().as_str();
        let line = 1 + content[..cap.get(0).unwrap().start()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count();
        let window_end = (cap.get(0).unwrap().end() + 1200).min(content.len());
        let snippet = &content[cap.get(0).unwrap().end()..window_end];
        let req_fields = extract_python_dict_fields(snippet);
        let resp_fields = extract_json_response_fields(snippet);
        let pattern = normalise_route_pattern(path);

        out.routes.push(ApiRoute {
            id: format!("py_{}_{}", method.to_lowercase(), sanitise_id(&pattern)),
            method,
            path: path.into(),
            pattern: pattern.clone(),
            path_params: extract_path_params(&pattern),
            query_params: Vec::new(),
            summary: format!("Python route handler at {}:{}", rel, line),
            source_file: rel.to_string(),
            source_line: line,
            handler_name: None,
            request_fields: req_fields,
            response_fields: resp_fields,
            sample_request: None,
            sample_response: None,
            cdp_initiator: None,
            related_slices: Vec::new(),
        });
    }

    // 2. Path branch routes (e.g., BaseHTTPRequestHandler `if path == "/embed":`)
    let inferred_method = if content.contains("def do_POST") && !content.contains("def do_GET") {
        "POST"
    } else {
        "GET"
    };
    for cap in PATH_BRANCH_ROUTE_RE.captures_iter(content) {
        let path = cap.get(1).unwrap().as_str();
        let match_start = cap.get(0).unwrap().start();
        let line = 1 + content[..match_start].bytes().filter(|&b| b == b'\n').count();
        let before = &content[..match_start];
        let method = if before.rfind("do_POST") > before.rfind("do_GET") {
            "POST"
        } else if before.rfind("do_DELETE") > before.rfind("do_GET") {
            "DELETE"
        } else {
            inferred_method
        };
        let window_end = (cap.get(0).unwrap().end() + 1500).min(content.len());
        let snippet = &content[cap.get(0).unwrap().end()..window_end];
        let req_fields = extract_python_dict_fields(snippet);
        let resp_fields = extract_json_response_fields(snippet);
        let pattern = normalise_route_pattern(path);

        out.routes.push(ApiRoute {
            id: format!("branch_{}_{}", method.to_lowercase(), sanitise_id(&pattern)),
            method: method.into(),
            path: path.into(),
            pattern: pattern.clone(),
            path_params: extract_path_params(&pattern),
            query_params: Vec::new(),
            summary: format!("HTTP handler branch {} {} in {}", method, path, rel),
            source_file: rel.to_string(),
            source_line: line,
            handler_name: None,
            request_fields: req_fields,
            response_fields: resp_fields,
            sample_request: None,
            sample_response: None,
            cdp_initiator: None,
            related_slices: Vec::new(),
        });
    }

    // 3. Outgoing HTTP calls from Python workers/clients
    for cap in PYTHON_HTTP_CALL_RE.captures_iter(content) {
        let method = cap.get(1).unwrap().as_str().to_uppercase();
        let raw_url = cap.get(2).unwrap().as_str();
        if raw_url.starts_with('/') || raw_url.contains("http") {
            let line = 1 + content[..cap.get(0).unwrap().start()]
                .bytes()
                .filter(|&b| b == b'\n')
                .count();
            let pattern = normalise_route_pattern(raw_url);
            out.routes.push(ApiRoute {
                id: format!("pyclient_{}_{}", method.to_lowercase(), sanitise_id(&pattern)),
                method: method.clone(),
                path: raw_url.into(),
                pattern: pattern.clone(),
                path_params: extract_path_params(&pattern),
                query_params: Vec::new(),
                summary: format!("Outgoing {} request from {}", method, rel),
                source_file: rel.to_string(),
                source_line: line,
                handler_name: None,
                request_fields: Vec::new(),
                response_fields: Vec::new(),
                sample_request: None,
                sample_response: None,
                cdp_initiator: Some(format!("{}:{}", rel, line)),
                related_slices: Vec::new(),
            });
        }
    }

    // 4. Classes / Entities / Workers
    let lines: Vec<&str> = content.lines().collect();
    for cap in PY_CLASS_RE.captures_iter(content) {
        let name = cap.get(1).unwrap().as_str().to_string();
        let bases = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        let start_line = 1 + content[..cap.get(0).unwrap().start()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count();
        let block = extract_indented_block(&lines, start_line - 1, 60);
        let fields = parse_python_class_fields(&block);

        if bases.contains("BaseHTTPRequestHandler")
            || name.ends_with("Handler")
            || name.ends_with("Worker")
            || name.ends_with("Spooler")
        {
            out.state_machine
                .views_and_workers
                .push(format!("Worker/Handler `{}` ({})", name, rel));
            out.stores.push(StateStore {
                id: format!("py_worker_{}", sanitise_id(&name)),
                name: name.clone(),
                kind: "worker_queue".into(),
                source_file: rel.to_string(),
                state_keys: fields.iter().map(|f| f.name.clone()).collect(),
                actions_or_mutators: extract_python_methods(&block),
                persists_to: if block.contains("sqlite") || block.contains("spool") {
                    Some("disk/sqlite".into())
                } else {
                    None
                },
                description: format!("Stateful Python service/worker class `{}` in {}", name, rel),
            });
        } else if !fields.is_empty() {
            out.entities.push(EntityModel {
                id: format!("py_class_{}", sanitise_id(&name)),
                name,
                kind: if bases.contains("BaseModel") {
                    "pydantic".into()
                } else {
                    "dataclass".into()
                },
                source_file: rel.to_string(),
                source_line: start_line,
                fields,
                description: format!("Python data model in {}", rel),
            });
        }
    }

    // 5. Python top-level functions & class methods as slices
    for cap in PY_DEF_RE.captures_iter(content) {
        let indent = cap.get(1).map(|m| m.as_str().len()).unwrap_or(0);
        let fn_name = cap.get(2).unwrap().as_str().to_string();
        if (fn_name.starts_with("__") && fn_name != "__init__") || indent > 8 {
            continue;
        }
        let args = cap.get(3).map(|m| m.as_str()).unwrap_or("");
        let ret = cap.get(4).map(|m| m.as_str().trim()).unwrap_or("Any");
        let start_line = 1 + content[..cap.get(0).unwrap().start()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count();
        let block = extract_indented_block(&lines, start_line - 1, 45);
        let end_line = start_line + block.lines().count();

        let called_apis: Vec<String> = PATH_BRANCH_ROUTE_RE
            .captures_iter(&block)
            .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
            .chain(
                GENERIC_API_LITERAL_RE
                    .captures_iter(&block)
                    .filter_map(|c| c.get(1).map(|m| normalise_route_pattern(m.as_str()))),
            )
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let doc = extract_first_docstring(&block)
            .unwrap_or_else(|| summarize_code_heuristically(&fn_name, &block));

        let category = classify_slice_category(&fn_name, &block, rel);

        if !called_apis.is_empty() {
            for api in &called_apis {
                out.state_machine.transitions.push(StateTransition {
                    id: format!("tr_{}_{}", sanitise_id(&fn_name), sanitise_id(api)),
                    from_state_or_view: rel.to_string(),
                    trigger: format!("{}({})", fn_name, args),
                    action: doc.clone(),
                    target_api_or_state: api.clone(),
                    source_file: rel.to_string(),
                });
            }
        }

        out.slices.push(CodeSlice {
            id: format!("{}:{}", rel, fn_name),
            name: fn_name.clone(),
            category,
            language: "python".into(),
            source_file: rel.to_string(),
            start_line,
            end_line,
            exported_symbols: vec![fn_name.clone()],
            called_apis,
            referenced_models: Vec::new(),
            calls_slices: Vec::new(),
            called_by_slices: Vec::new(),
            proprietary_score: prop_score,
            summary: doc,
            signature_or_pseudo: format!("def {}({}) -> {}", fn_name, args, ret),
            code_excerpt: truncate_lines(&block, 32),
            cluster_id: None,
        });
    }
}

fn analyze_rust_file(rel: &str, content: &str, out: &mut AnalysisOutput) {
    for cap in AXUM_METHOD_ROUTE_RE.captures_iter(content) {
        let path = cap.get(1).unwrap().as_str();
        let method = cap.get(2).unwrap().as_str().to_uppercase();
        let handler = cap.get(3).unwrap().as_str().to_string();
        let line = 1 + content[..cap.get(0).unwrap().start()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count();
        let pattern = normalise_route_pattern(path);
        out.routes.push(ApiRoute {
            id: format!("axum_{}_{}", method.to_lowercase(), sanitise_id(&pattern)),
            method,
            path: path.into(),
            pattern: pattern.clone(),
            path_params: extract_path_params(&pattern),
            query_params: Vec::new(),
            summary: format!("Axum route -> {} ({})", handler, rel),
            source_file: rel.to_string(),
            source_line: line,
            handler_name: Some(handler),
            request_fields: Vec::new(),
            response_fields: Vec::new(),
            sample_request: None,
            sample_response: None,
            cdp_initiator: None,
            related_slices: Vec::new(),
        });
    }

    for cap in RUST_STRUCT_RE.captures_iter(content) {
        let name = cap.get(1).unwrap().as_str().to_string();
        let full = cap.get(0).unwrap();
        if let Some(body) = crate::unpacker::extract_balanced_braces(content, full.end() - 1) {
            let fields = parse_ts_fields(&body);
            if !fields.is_empty() {
                out.entities.push(EntityModel {
                    id: format!("rs_{}", sanitise_id(&name)),
                    name,
                    kind: "struct".into(),
                    source_file: rel.to_string(),
                    source_line: 1 + content[..full.start()].bytes().filter(|&b| b == b'\n').count(),
                    fields,
                    description: format!("Rust struct in {}", rel),
                });
            }
        }
    }
}

fn analyze_html_file(rel: &str, content: &str, out: &mut AnalysisOutput) {
    if rel.ends_with(".css") {
        return;
    }
    let doc = dom_query::Document::from(content);
    let title = doc.select("title").text().to_string();
    let headings: Vec<String> = doc
        .select("h1, h2, h3, nav a, button")
        .iter()
        .map(|n| n.text().trim().to_string())
        .filter(|s| s.len() >= 2 && s.len() <= 60)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .take(24)
        .collect();

    let mut rendered_elements = Vec::new();
    for tag in ["header", "nav", "main", "section", "form", "video", "canvas", "table", "dialog"] {
        let count = doc.select(tag).length();
        if count > 0 {
            rendered_elements.push(format!("<{}> (×{})", tag, count));
        }
    }

    if !title.trim().is_empty() || !headings.is_empty() || !rendered_elements.is_empty() {
        let vname = if !title.trim().is_empty() {
            title.trim().to_string()
        } else {
            rel.to_string()
        };
        out.state_machine
            .views_and_workers
            .push(format!("HTML Document: {} ({})", vname, rel));
        out.ui_views.push(UiViewSpec {
            id: format!("html_{}", sanitise_id(rel)),
            name: vname.clone(),
            kind: "dom_section".into(),
            source_file: rel.to_string(),
            start_line: 1,
            rendered_elements,
            visible_labels_and_copy: headings,
            child_components: Vec::new(),
            bound_routes: Vec::new(),
            bound_models: Vec::new(),
            interactive_handlers: Vec::new(),
            summary: format!("HTML DOM entry `{}` ({})", vname, rel),
        });
    }

    for form in doc.select("form").iter() {
        let action = form
            .attr("action")
            .map(|a| a.to_string())
            .unwrap_or_else(|| "/".to_string());
        let method = form
            .attr("method")
            .map(|m| m.to_string())
            .unwrap_or_else(|| "POST".to_string())
            .to_uppercase();
        out.state_machine.transitions.push(StateTransition {
            id: format!("form_{}", sanitise_id(&action)),
            from_state_or_view: rel.to_string(),
            trigger: "form.submit".into(),
            action: format!("Submit HTML form to {}", action),
            target_api_or_state: action,
            source_file: rel.to_string(),
        });
        let _ = method;
    }
}

fn extract_js_slices(rel: &str, code: &str, prop_score: f32, out: &mut AnalysisOutput) {
    let mut found_named = 0usize;
    for cap in JS_FUNC_RE
        .captures_iter(code)
        .chain(JS_CONST_FN_RE.captures_iter(code))
    {
        let fn_name = cap.get(1).unwrap().as_str().to_string();
        if fn_name.len() <= 2 {
            continue;
        }
        found_named += 1;
        let args = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        let match_end = cap.get(0).unwrap().end();
        let start_line = 1 + code[..cap.get(0).unwrap().start()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count();

        let window_end = (match_end + 1400).min(code.len());
        let excerpt = &code[cap.get(0).unwrap().start()..window_end];
        let body = if let Some(brace_pos) = excerpt.find('{') {
            crate::unpacker::extract_balanced_braces(excerpt, brace_pos)
                .unwrap_or_else(|| excerpt.to_string())
        } else {
            excerpt.to_string()
        };

        let called_apis: Vec<String> = FETCH_CALL_RE
            .captures_iter(&body)
            .filter_map(|c| c.get(1).map(|m| normalise_route_pattern(m.as_str())))
            .chain(
                GENERIC_API_LITERAL_RE
                    .captures_iter(&body)
                    .filter_map(|c| c.get(1).map(|m| normalise_route_pattern(m.as_str()))),
            )
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let category = classify_slice_category(&fn_name, &body, rel);
        let summary = summarize_code_heuristically(&fn_name, &body);

        for api in &called_apis {
            out.state_machine.transitions.push(StateTransition {
                id: format!("jstr_{}_{}", sanitise_id(&fn_name), sanitise_id(api)),
                from_state_or_view: rel.to_string(),
                trigger: format!("{}({})", fn_name, args),
                action: summary.clone(),
                target_api_or_state: api.clone(),
                source_file: rel.to_string(),
            });
        }

        out.slices.push(CodeSlice {
            id: format!("{}:{}", rel, fn_name),
            name: fn_name.clone(),
            category,
            language: "javascript".into(),
            source_file: rel.to_string(),
            start_line,
            end_line: start_line + body.lines().count(),
            exported_symbols: vec![fn_name.clone()],
            called_apis,
            referenced_models: Vec::new(),
            calls_slices: Vec::new(),
            called_by_slices: Vec::new(),
            proprietary_score: prop_score,
            summary,
            signature_or_pseudo: format!("function {}({})", fn_name, args),
            code_excerpt: truncate_lines(&body, 30),
            cluster_id: None,
        });
    }

    // Fallback for heavily minified SPA bundles where functions are 1-2 chars (`function a(e,t)`):
    // Slice around any API literal or state dispatch in the bundle!
    if found_named <= 3 && code.len() > 1500 {
        for (idx, cap) in GENERIC_API_LITERAL_RE.captures_iter(code).take(32).enumerate() {
            let m = cap.get(0).unwrap();
            let api_path = cap.get(1).unwrap().as_str();
            let start = m.start().saturating_sub(220);
            let end = (m.end() + 380).min(code.len());
            let snippet = &code[start..end];
            let slice_name = format!("minified_api_caller_{}", idx + 1);
            out.slices.push(CodeSlice {
                id: format!("{}:{}", rel, slice_name),
                name: slice_name.clone(),
                category: "api_handler".into(),
                language: "javascript".into(),
                source_file: rel.to_string(),
                start_line: 1 + code[..start].bytes().filter(|&b| b == b'\n').count(),
                end_line: 1 + code[..end].bytes().filter(|&b| b == b'\n').count(),
                exported_symbols: Vec::new(),
                called_apis: vec![normalise_route_pattern(api_path)],
                referenced_models: Vec::new(),
                calls_slices: Vec::new(),
                called_by_slices: Vec::new(),
                proprietary_score: prop_score,
                summary: format!("Minified bundle slice invoking `{}`", api_path),
                signature_or_pseudo: format!("bundle_slice({}) -> fetch('{}')", idx + 1, api_path),
                code_excerpt: snippet.to_string(),
                cluster_id: None,
            });
        }
    }
}

fn extract_sql_tables(rel: &str, content: &str, entities: &mut Vec<EntityModel>) {
    for cap in SQL_TABLE_RE.captures_iter(content) {
        let table_name = cap.get(1).unwrap().as_str().to_string();
        let cols_raw = cap.get(2).unwrap().as_str();
        let start_line = 1 + content[..cap.get(0).unwrap().start()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count();

        let mut fields = Vec::new();
        for line in cols_raw.lines() {
            let trimmed = line.trim().trim_end_matches(',');
            if trimmed.is_empty()
                || trimmed.to_uppercase().starts_with("PRIMARY KEY")
                || trimmed.to_uppercase().starts_with("FOREIGN KEY")
                || trimmed.to_uppercase().starts_with("UNIQUE")
                || trimmed.to_uppercase().starts_with("CONSTRAINT")
            {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let col_name = parts[0].trim_matches(|c| c == '"' || c == '`' || c == '[' || c == ']');
                let col_type = parts[1].trim_end_matches(',').to_lowercase();
                if col_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    fields.push(FieldSpec {
                        name: col_name.to_string(),
                        field_type: col_type,
                        required: trimmed.to_uppercase().contains("NOT NULL")
                            || trimmed.to_uppercase().contains("PRIMARY KEY"),
                        description: Some(format!("SQL column in table `{}`", table_name)),
                    });
                }
            }
        }

        if !fields.is_empty() {
            entities.push(EntityModel {
                id: format!("sql_{}_{}", sanitise_id(rel), sanitise_id(&table_name)),
                name: table_name.clone(),
                kind: "sql_table".into(),
                source_file: rel.to_string(),
                source_line: start_line,
                fields,
                description: format!("SQLite/SQL table `{}` defined in {}", table_name, rel),
            });
        }
    }
}

fn extract_python_dict_fields(snippet: &str) -> Vec<FieldSpec> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for cap in PY_DICT_GET_RE.captures_iter(snippet) {
        let name = cap.get(1).unwrap().as_str().to_string();
        if seen.insert(name.clone()) {
            out.push(FieldSpec {
                name,
                field_type: "any".into(),
                required: false,
                description: Some("Extracted from payload .get()".into()),
            });
        }
    }
    out
}

fn extract_json_response_fields(snippet: &str) -> Vec<FieldSpec> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for cap in JSON_KEY_RE.captures_iter(snippet) {
        let name = cap.get(1).unwrap().as_str().to_string();
        if seen.insert(name.clone()) && out.len() < 16 {
            out.push(FieldSpec {
                name,
                field_type: "any".into(),
                required: true,
                description: Some("Extracted from handler response dict".into()),
            });
        }
    }
    out
}

fn parse_ts_fields(body: &str) -> Vec<FieldSpec> {
    let mut fields = Vec::new();
    for line in body.lines() {
        let t = line.trim().trim_end_matches(',').trim_end_matches(';');
        if t.starts_with("//") || t.starts_with('#') || t.starts_with('{') || t.starts_with('}') {
            continue;
        }
        let cleaned = t.strip_prefix("pub ").unwrap_or(t);
        if let Some((k, v)) = cleaned.split_once(':') {
            let name = k.trim().trim_end_matches('?').to_string();
            let required = !k.trim().ends_with('?') && !v.contains("Option<");
            let ftype = v.trim().to_string();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                fields.push(FieldSpec {
                    name,
                    field_type: ftype,
                    required,
                    description: None,
                });
            }
        }
    }
    fields
}

fn parse_python_class_fields(block: &str) -> Vec<FieldSpec> {
    const PY_KEYWORDS: &[&str] = &[
        "try", "else", "finally", "except", "for", "while", "if", "elif", "with", "match", "case",
        "def", "class", "return", "yield", "pass", "break", "continue", "lambda",
    ];
    let mut fields = Vec::new();
    let mut seen = HashSet::new();
    let mut in_other_method = false;

    for line in block.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.starts_with("def ") || trimmed.starts_with("async def ") {
            in_other_method = !trimmed.starts_with("def __init__");
            continue;
        }
        if in_other_method {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("self.") {
            if let Some((attr, _)) = rest.split_once('=') {
                let name = attr.trim().split(':').next().unwrap_or("").trim().to_string();
                if !name.is_empty()
                    && !name.starts_with('_')
                    && !PY_KEYWORDS.contains(&name.as_str())
                    && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                    && seen.insert(name.clone())
                {
                    fields.push(FieldSpec {
                        name,
                        field_type: "any".into(),
                        required: true,
                        description: Some("Instance attribute set in __init__".into()),
                    });
                }
            }
        } else if let Some((k, v)) = trimmed.split_once(':') {
            let name = k.trim().to_string();
            let ftype_raw = v.split('#').next().unwrap_or(v).trim();
            if !name.is_empty()
                && !ftype_raw.is_empty()
                && !name.contains(' ')
                && !name.starts_with('#')
                && !PY_KEYWORDS.contains(&name.as_str())
                && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                && seen.insert(name.clone())
            {
                let ftype = ftype_raw.split('=').next().unwrap_or("Any").trim().to_string();
                fields.push(FieldSpec {
                    name,
                    field_type: ftype,
                    required: !v.contains('='),
                    description: None,
                });
            }
        }
    }
    fields
}

fn extract_python_methods(block: &str) -> Vec<String> {
    PY_DEF_RE
        .captures_iter(block)
        .filter_map(|c| c.get(2).map(|m| m.as_str().to_string()))
        .collect()
}

fn extract_indented_block(lines: &[&str], start_idx: usize, max_lines: usize) -> String {
    if start_idx >= lines.len() {
        return String::new();
    }
    let header = lines[start_idx];
    let base_indent = header.len() - header.trim_start().len();
    let mut out = vec![header];

    for &line in lines.iter().skip(start_idx + 1).take(max_lines) {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            let indent = line.len() - line.trim_start().len();
            if indent <= base_indent {
                break;
            }
        }
        out.push(line);
    }
    out.join("\n")
}

fn extract_module_docstring_or_header(content: &str) -> Option<String> {
    let trimmed = content.trim_start();
    if let Some(rest) = trimmed.strip_prefix("\"\"\"").or_else(|| trimmed.strip_prefix("'''")) {
        if let Some(end) = rest.find("\"\"\"").or_else(|| rest.find("'''")) {
            let doc = rest[..end].lines().next().unwrap_or("").trim();
            if !doc.is_empty() {
                return Some(doc.to_string());
            }
        }
    }
    for line in content.lines().take(6) {
        let l = line.trim();
        if let Some(comment) = l.strip_prefix("//").or_else(|| l.strip_prefix('#')) {
            let c = comment.trim();
            if c.len() > 8 && !c.starts_with('!') {
                return Some(c.to_string());
            }
        }
    }
    None
}

fn extract_first_docstring(block: &str) -> Option<String> {
    if let Some(start) = block.find("\"\"\"") {
        let after = &block[start + 3..];
        if let Some(end) = after.find("\"\"\"") {
            let text = after[..end]
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

fn classify_slice_category(name: &str, body: &str, rel: &str) -> String {
    let lower_name = name.to_lowercase();
    if lower_name.starts_with("do_")
        || lower_name.contains("route")
        || lower_name.contains("handler")
        || body.contains("send_response")
    {
        "api_handler".into()
    } else if rel.ends_with(".jsx")
        || rel.ends_with(".tsx")
        || (name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) && body.contains("return"))
    {
        "ui_component".into()
    } else if lower_name.contains("worker")
        || lower_name.contains("loop")
        || lower_name.contains("drain")
        || lower_name.contains("spool")
        || lower_name.contains("batch")
    {
        "worker_pipeline".into()
    } else if lower_name.contains("store") || lower_name.contains("state") || lower_name.starts_with("use") {
        "state_store".into()
    } else {
        "business_logic".into()
    }
}

fn summarize_code_heuristically(name: &str, body: &str) -> String {
    let mut traits = Vec::new();
    if body.contains("fetch(") || body.contains("requests.") || body.contains("urllib") {
        traits.push("executes HTTP network I/O");
    }
    if body.contains("sqlite") || body.contains("SELECT ") || body.contains("INSERT ") {
        traits.push("queries/mutates SQLite storage");
    }
    if body.contains("torch.")
        || body.contains("onnx")
        || body.contains("embeddings")
        || body.contains("inference_mode")
        || body.contains("no_grad")
    {
        traits.push("runs model inference / tensor embedding");
    }
    if body.contains("queue") || body.contains("spool") {
        traits.push("manages asynchronous queue/spool buffering");
    }
    if JSX_HTML_TAG_RE.is_match(body) {
        traits.push("renders interactive DOM/JSX view");
    }
    if traits.is_empty() {
        format!("Proprietary routine `{}` ({} lines)", name, body.lines().count())
    } else {
        format!("`{}` — {}", name, traits.join(", "))
    }
}

pub fn normalise_route_pattern(raw_url: &str) -> String {
    let path_part = if let Ok(u) = url::Url::parse(raw_url) {
        u.path().to_string()
    } else {
        raw_url.split('?').next().unwrap_or(raw_url).to_string()
    };

    let segments: Vec<String> = path_part
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|seg| {
            if seg.starts_with("${") && seg.ends_with('}') {
                let inner = &seg[2..seg.len() - 1];
                format!("{{{}}}", inner)
            } else if seg.starts_with(':') {
                format!("{{{}}}", &seg[1..])
            } else if seg.chars().all(|c| c.is_ascii_digit()) {
                "{id}".into()
            } else {
                seg.to_string()
            }
        })
        .collect();

    let _ = BTreeMap::<u8, u8>::new();
    format!("/{}", segments.join("/"))
}

fn extract_path_params(pattern: &str) -> Vec<String> {
    pattern
        .split('/')
        .filter(|s| (s.starts_with('{') && s.ends_with('}')) || s.starts_with(':'))
        .map(|s| s.to_string())
        .collect()
}

fn extract_query_params(raw_url: &str) -> Vec<String> {
    if let Some((_, q)) = raw_url.split_once('?') {
        q.split('&')
            .filter_map(|pair| pair.split('=').next())
            .filter(|k| !k.is_empty())
            .map(|k| k.to_string())
            .collect()
    } else {
        Vec::new()
    }
}

fn deduplicate_routes(routes: &mut Vec<ApiRoute>) {
    let mut seen = HashSet::new();
    routes.retain(|r| {
        let key = format!("{}:{}", r.method.to_uppercase(), r.pattern);
        seen.insert(key)
    });
}

fn json_value_type(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::Null => "null".into(),
        serde_json::Value::Bool(_) => "boolean".into(),
        serde_json::Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                "integer".into()
            } else {
                "number".into()
            }
        }
        serde_json::Value::String(_) => "string".into(),
        serde_json::Value::Array(_) => "array".into(),
        serde_json::Value::Object(_) => "object".into(),
    }
}

fn truncate_lines(s: &str, max_lines: usize) -> String {
    let mut lines: Vec<&str> = s.lines().take(max_lines).collect();
    if s.lines().count() > max_lines {
        lines.push("// ... [truncated]");
    }
    lines.join("\n")
}

pub fn sanitise_id(s: &str) -> String {
    let out: String = s
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        "root".into()
    } else {
        trimmed.into()
    }
}
