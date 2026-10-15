use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpackedModule {
    pub id: String,
    pub bundler: String, // "webpack5" | "webpack4" | "nextjs_rsc" | "esm_module"
    pub code: String,
    pub start_line: usize,
    pub end_line: usize,
    pub dependencies: Vec<String>,
    pub is_vendor: bool,
    pub proprietary_score: f32,
}

static WEBPACK5_HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:self|window|globalThis)\.webpackChunk[a-zA-Z0-9_$]*\s*=").unwrap()
});

static WEBPACK_MODULE_KEY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)^\s*(?:"([a-zA-Z0-9_./+-]+)"|'([a-zA-Z0-9_./+-]+)'|(\d+))\s*:\s*(?:function\s*\([^)]*\)|\([^)]*\)\s*=>)\s*\{"#).unwrap()
});

static NEXT_RSC_PUSH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"self\.__next_f\.push\(\[\s*\d+\s*,\s*"((?:[^"\\]|\\.)*)"\s*\]\)"#).unwrap()
});

static WEBPACK_REQUIRE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:__webpack_require__|n|r)\(\s*([0-9]+|'[^']+'|\x22[^\x22]+\x22)\s*\)").unwrap()
});

/// Scores a JS/TS/Python code block on how likely it is proprietary domain logic (0.0 = pure vendor, 1.0 = pure domain).
pub fn score_proprietary(code: &str, filename: &str) -> (bool, f32) {
    let lower_path = filename.to_lowercase();
    if lower_path.contains("node_modules")
        || lower_path.contains("vendor")
        || lower_path.contains("polyfills")
        || lower_path.contains("react-dom.")
        || lower_path.contains("chunk-vendors")
    {
        return (true, 0.05);
    }

    let vendor_markers = [
        "__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED",
        "react.element",
        "react.portal",
        "react.fragment",
        "scheduler.development",
        "/@babel/runtime",
        "core-js",
        "regeneratorRuntime",
        "Copyright (c) Facebook",
        "Licensed under the MIT License",
    ];
    let mut vendor_hits = 0usize;
    for m in vendor_markers {
        if code.contains(m) {
            vendor_hits += 1;
        }
    }
    if vendor_hits >= 2 {
        return (true, 0.08);
    }

    let domain_markers = [
        "/api/",
        "fetch(",
        "XMLHttpRequest",
        "WebSocket",
        "EventSource",
        "localStorage",
        "sessionStorage",
        "FastAPI",
        "APIRouter",
        "@app.",
        "sqlite3",
        "CREATE TABLE",
        "Embeddings",
        "tokio::",
        "axum::",
        "useState",
        "useEffect",
        "useReducer",
        "dispatch(",
        "addEventListener",
        "querySelector",
    ];
    let mut domain_hits = 0usize;
    for m in domain_markers {
        if code.contains(m) {
            domain_hits += 1;
        }
    }

    let base: f32 = if vendor_hits == 1 { 0.35 } else { 0.65 };
    let bonus: f32 = (domain_hits as f32 * 0.07).min(0.34);
    let score = (base + bonus).clamp(0.0, 1.0);
    (score < 0.25, score)
}

/// Unpacks a bundled JS file (Webpack 4/5, Next.js RSC, or raw module) into logical sub-modules.
pub fn unpack_js_bundle(code: &str, filename: &str) -> Vec<UnpackedModule> {
    let mut modules = Vec::new();

    // 1. Check for Next.js RSC payloads
    if code.contains("self.__next_f.push") {
        for (idx, cap) in NEXT_RSC_PUSH_RE.captures_iter(code).enumerate() {
            if let Some(m) = cap.get(1) {
                let decoded = m.as_str().replace("\\\"", "\"").replace("\\n", "\n");
                if decoded.len() > 80 {
                    let (is_vendor, proprietary_score) = score_proprietary(&decoded, filename);
                    modules.push(UnpackedModule {
                        id: format!("{}:rsc_{}", filename, idx),
                        bundler: "nextjs_rsc".into(),
                        code: decoded,
                        start_line: 1,
                        end_line: 1,
                        dependencies: Vec::new(),
                        is_vendor,
                        proprietary_score,
                    });
                }
            }
        }
    }

    // 2. Check for Webpack 4 / 5 module maps
    let is_wp5 = WEBPACK5_HEADER_RE.is_match(code);
    let is_wp4 = code.contains("__webpack_require__") || code.contains("webpackJsonp");
    if is_wp5 || is_wp4 {
        for cap in WEBPACK_MODULE_KEY_RE.captures_iter(code) {
            let mod_id = cap
                .get(1)
                .or_else(|| cap.get(2))
                .or_else(|| cap.get(3))
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| "anon".into());
            let full_match = cap.get(0).unwrap();
            let brace_start = full_match.end() - 1;
            if let Some(body) = extract_balanced_braces(code, brace_start) {
                let start_line = 1 + code[..full_match.start()].bytes().filter(|&b| b == b'\n').count();
                let end_line = start_line + body.bytes().filter(|&b| b == b'\n').count();
                let deps = WEBPACK_REQUIRE_RE
                    .captures_iter(&body)
                    .filter_map(|c| c.get(1).map(|m| m.as_str().trim_matches(|ch| ch == '\'' || ch == '"').to_string()))
                    .collect();
                let (is_vendor, proprietary_score) = score_proprietary(&body, filename);
                modules.push(UnpackedModule {
                    id: mod_id,
                    bundler: if is_wp5 { "webpack5".into() } else { "webpack4".into() },
                    code: body,
                    start_line,
                    end_line,
                    dependencies: deps,
                    is_vendor,
                    proprietary_score,
                });
            }
        }
    }

    // 3. If not a multi-module bundle, return the whole file as a single ESM/script module
    if modules.is_empty() {
        let total_lines = code.lines().count().max(1);
        let (is_vendor, proprietary_score) = score_proprietary(code, filename);
        modules.push(UnpackedModule {
            id: filename.to_string(),
            bundler: "esm_module".into(),
            code: code.to_string(),
            start_line: 1,
            end_line: total_lines,
            dependencies: Vec::new(),
            is_vendor,
            proprietary_score,
        });
    }

    modules
}

/// Extracts a `{ ... }` block starting at `open_brace_idx`, respecting strings, template literals, and comments.
pub fn extract_balanced_braces(src: &str, open_brace_idx: usize) -> Option<String> {
    let bytes = src.as_bytes();
    if open_brace_idx >= bytes.len() || bytes[open_brace_idx] != b'{' {
        return None;
    }

    let mut depth = 0i32;
    let mut i = open_brace_idx;
    let len = bytes.len();
    let mut in_single = false;
    let mut in_double = false;
    let mut in_template = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while i < len {
        let b = bytes[i];
        let next = if i + 1 < len { bytes[i + 1] } else { 0 };

        if in_line_comment {
            if b == b'\n' {
                in_line_comment = false;
            }
            i += 1;
            continue;
        }
        if in_block_comment {
            if b == b'*' && next == b'/' {
                in_block_comment = false;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if in_single {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == b'\'' {
                in_single = false;
            }
            i += 1;
            continue;
        }
        if in_double {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == b'"' {
                in_double = false;
            }
            i += 1;
            continue;
        }
        if in_template {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == b'`' {
                in_template = false;
            }
            i += 1;
            continue;
        }

        if b == b'/' && next == b'/' {
            in_line_comment = true;
            i += 2;
            continue;
        }
        if b == b'/' && next == b'*' {
            in_block_comment = true;
            i += 2;
            continue;
        }
        if b == b'\'' {
            in_single = true;
            i += 1;
            continue;
        }
        if b == b'"' {
            in_double = true;
            i += 1;
            continue;
        }
        if b == b'`' {
            in_template = true;
            i += 1;
            continue;
        }

        if b == b'{' {
            depth += 1;
        } else if b == b'}' {
            depth -= 1;
            if depth == 0 {
                return src.get(open_brace_idx..=i).map(|s| s.to_string());
            }
        }
        i += 1;
    }

    None
}
