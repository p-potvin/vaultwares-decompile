use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintManifest {
    pub project_name: String,
    pub target: String,
    pub extracted_at: String,
    pub version: String,
    pub detected_stack: Vec<String>,
    pub stats: BlueprintStats,
    pub services: Vec<DetectedService>,
    #[serde(default)]
    pub domain_context: DomainContext,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DomainContext {
    pub content_signals: Vec<String>,
    pub external_domains: Vec<String>,
    pub media_modalities: Vec<String>,
    pub ui_theme_tokens: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlueprintStats {
    pub files_scanned: usize,
    pub bundles_unpacked: usize,
    pub vendor_modules_stripped: usize,
    pub proprietary_slices_kept: usize,
    pub api_routes_discovered: usize,
    pub entities_extracted: usize,
    pub state_stores_extracted: usize,
    pub ui_views_extracted: usize,
    pub feature_clusters: usize,
    pub embedded_items: usize,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedService {
    pub name: String,
    pub kind: String,
    pub bind_or_url: Option<String>,
    pub entrypoint: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRoute {
    pub id: String,
    pub method: String,
    pub path: String,
    pub pattern: String,
    pub path_params: Vec<String>,
    pub query_params: Vec<String>,
    pub summary: String,
    pub source_file: String,
    pub source_line: usize,
    pub handler_name: Option<String>,
    pub request_fields: Vec<FieldSpec>,
    pub response_fields: Vec<FieldSpec>,
    pub sample_request: Option<Value>,
    pub sample_response: Option<Value>,
    pub cdp_initiator: Option<String>,
    pub related_slices: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpec {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityModel {
    pub id: String,
    pub name: String,
    pub kind: String, // "struct" | "interface" | "pydantic" | "dataclass" | "sql_table" | "json_payload"
    pub source_file: String,
    pub source_line: usize,
    pub fields: Vec<FieldSpec>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateStore {
    pub id: String,
    pub name: String,
    pub kind: String, // "zustand" | "redux" | "react_state" | "config_store" | "worker_queue" | "global_state"
    pub source_file: String,
    pub state_keys: Vec<String>,
    pub actions_or_mutators: Vec<String>,
    pub persists_to: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiViewSpec {
    pub id: String,
    pub name: String,
    pub kind: String, // "page_or_tab" | "component" | "dom_section"
    pub source_file: String,
    pub start_line: usize,
    pub rendered_elements: Vec<String>,
    pub visible_labels_and_copy: Vec<String>,
    pub child_components: Vec<String>,
    pub bound_routes: Vec<String>,
    pub bound_models: Vec<String>,
    pub interactive_handlers: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub id: String,
    pub from_state_or_view: String,
    pub trigger: String,
    pub action: String,
    pub target_api_or_state: String,
    pub source_file: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StateMachineSpec {
    pub views_and_workers: Vec<String>,
    pub transitions: Vec<StateTransition>,
    pub data_flows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSlice {
    pub id: String,
    pub name: String,
    pub category: String, // "api_handler" | "business_logic" | "state_store" | "ui_component" | "worker_pipeline" | "data_model"
    pub language: String,
    pub source_file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub exported_symbols: Vec<String>,
    pub called_apis: Vec<String>,
    pub referenced_models: Vec<String>,
    #[serde(default)]
    pub calls_slices: Vec<String>,
    #[serde(default)]
    pub called_by_slices: Vec<String>,
    pub proprietary_score: f32,
    pub summary: String,
    pub signature_or_pseudo: String,
    pub code_excerpt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureCluster {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub slice_ids: Vec<String>,
    pub route_ids: Vec<String>,
    pub entity_names: Vec<String>,
    pub keywords: Vec<String>,
    pub markdown_spec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedEntry {
    pub id: String,
    pub kind: String, // "slice" | "route" | "entity" | "store" | "ui_view"
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub text: String,
    pub vector: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingIndex {
    pub model: String,
    pub dim: usize,
    pub entries: Vec<EmbeddedEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullBlueprint {
    pub manifest: BlueprintManifest,
    pub openapi: Value,
    pub routes: Vec<ApiRoute>,
    pub entities: Vec<EntityModel>,
    pub stores: Vec<StateStore>,
    #[serde(default)]
    pub ui_views: Vec<UiViewSpec>,
    pub state_machine: StateMachineSpec,
    pub slices: Vec<CodeSlice>,
    pub clusters: Vec<FeatureCluster>,
    pub overview_markdown: String,
    pub embeddings: Option<EmbeddingIndex>,
}

pub fn build_openapi_spec(
    project_name: &str,
    target: &str,
    routes: &[ApiRoute],
    entities: &[EntityModel],
) -> Value {
    let mut paths = IndexMap::<String, Value>::new();

    for route in routes {
        let method_lower = route.method.to_lowercase();
        let path_key = if route.pattern.starts_with('/') {
            route.pattern.clone()
        } else {
            format!("/{}", route.pattern)
        };

        let mut parameters = Vec::new();
        for p in &route.path_params {
            let clean = p.trim_start_matches(':').trim_matches(|c| c == '{' || c == '}');
            parameters.push(serde_json::json!({
                "name": clean,
                "in": "path",
                "required": true,
                "schema": { "type": "string" }
            }));
        }
        for q in &route.query_params {
            parameters.push(serde_json::json!({
                "name": q,
                "in": "query",
                "required": false,
                "schema": { "type": "string" }
            }));
        }

        let mut req_props = serde_json::Map::new();
        let mut req_required = Vec::new();
        for f in &route.request_fields {
            req_props.insert(
                f.name.clone(),
                serde_json::json!({
                    "type": f.field_type,
                    "description": f.description.clone().unwrap_or_default()
                }),
            );
            if f.required {
                req_required.push(f.name.clone());
            }
        }

        let mut resp_props = serde_json::Map::new();
        for f in &route.response_fields {
            resp_props.insert(
                f.name.clone(),
                serde_json::json!({
                    "type": f.field_type,
                    "description": f.description.clone().unwrap_or_default()
                }),
            );
        }

        let mut op = serde_json::Map::new();
        op.insert("operationId".into(), Value::String(route.id.clone()));
        op.insert("summary".into(), Value::String(route.summary.clone()));
        op.insert(
            "x-vw-source".into(),
            serde_json::json!({
                "file": route.source_file,
                "line": route.source_line,
                "handler": route.handler_name,
                "cdpInitiator": route.cdp_initiator,
                "relatedSlices": route.related_slices,
            }),
        );

        if !parameters.is_empty() {
            op.insert("parameters".into(), Value::Array(parameters));
        }

        if !req_props.is_empty() || route.sample_request.is_some() {
            op.insert(
                "requestBody".into(),
                serde_json::json!({
                    "required": true,
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": req_props,
                                "required": req_required
                            },
                            "example": route.sample_request
                        }
                    }
                }),
            );
        }

        op.insert(
            "responses".into(),
            serde_json::json!({
                "200": {
                    "description": "Successful response",
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": resp_props
                            },
                            "example": route.sample_response
                        }
                    }
                }
            }),
        );

        let entry = paths
            .entry(path_key)
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        if let Value::Object(map) = entry {
            map.insert(method_lower, Value::Object(op));
        }
    }

    let mut schemas = serde_json::Map::new();
    for entity in entities {
        let mut props = serde_json::Map::new();
        let mut req = Vec::new();
        for f in &entity.fields {
            props.insert(
                f.name.clone(),
                serde_json::json!({
                    "type": f.field_type,
                    "description": f.description.clone().unwrap_or_default()
                }),
            );
            if f.required {
                req.push(f.name.clone());
            }
        }
        schemas.insert(
            entity.name.clone(),
            serde_json::json!({
                "type": "object",
                "description": format!("{} ({}:{})", entity.description, entity.source_file, entity.source_line),
                "properties": props,
                "required": req
            }),
        );
    }

    serde_json::json!({
        "openapi": "3.1.0",
        "info": {
            "title": format!("{} — Decompiled Blueprint API", project_name),
            "version": "0.2.0",
            "description": format!("Clean-room OpenAPI 3.1 contract extracted by VaultWares Decompile from {}", target)
        },
        "paths": paths,
        "components": {
            "schemas": schemas
        }
    })
}
