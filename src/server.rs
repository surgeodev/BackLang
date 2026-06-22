use axum::{
    extract::{Path, Json},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use crate::interpreter::{Env, execute_stmts, value_to_json, Value};
use crate::parser::Program;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

// État partagé de l'application
pub struct AppState {
    pub program: Arc<Program>,
    pub env: Arc<Mutex<Env>>,
}

impl AppState {
    pub fn new(program: Program, env: Env) -> Self {
        AppState {
            program: Arc::new(program),
            env: Arc::new(Mutex::new(env)),
        }
    }
}

// Gestionnaire de requête dynamique pour tous les endpoints
async fn handle_dynamic(
    method: Method,
    path: String,
    headers: HeaderMap,
    body: Option<Json<JsonValue>>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    let program = state.program.clone();
    let cors_enabled = program.servers.first().map(|s| s.cors).unwrap_or(false);
    let base_env = state.env.lock().await.clone();
    drop(state);
    
    // CORS preflight
    if cors_enabled && method == Method::OPTIONS {
        return Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "*")
            .header("Access-Control-Allow-Headers", "*")
            .body(String::new())
            .unwrap();
    }
    
    let method_str = method.as_str();
    let path_parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
    
    for ep in &program.endpoints {
        if ep.method != method_str {
            continue;
        }
        
        let ep_parts: Vec<&str> = ep.path.split('/').filter(|p| !p.is_empty()).collect();
        
        if ep_parts.len() != path_parts.len() {
            continue;
        }
        
        let mut params = HashMap::new();
        let mut matched = true;
        
        for (ep_part, path_part) in ep_parts.iter().zip(path_parts.iter()) {
            if ep_part.starts_with(':') {
                let param_name = ep_part.trim_start_matches(':');
                params.insert(param_name.to_string(), path_part.to_string());
            } else if ep_part != path_part {
                matched = false;
                break;
            }
        }
        
        if matched {
            let mut env_with_params = Env::child(&base_env);
            for (k, v) in &params {
                env_with_params.define(k.clone(), Value::Str(v.clone()), false);
            }
            
            for mw_name in &ep.middlewares {
                let mw_key = format!("_middleware_{}", mw_name);
                if let Some(Value::Func(fd)) = base_env.get(&mw_key) {
                    let mut mw_env = Env::child(&base_env);
                    let mut req_obj = HashMap::new();
                    req_obj.insert("method".to_string(), Value::Str(method_str.to_string()));
                    req_obj.insert("path".to_string(), Value::Str(path.clone()));
                    let mut headers_obj = HashMap::new();
                    for (key, value) in headers.iter() {
                        if let Ok(v) = value.to_str() {
                            headers_obj.insert(key.to_string(), Value::Str(v.to_string()));
                        }
                    }
                    req_obj.insert("headers".to_string(), Value::Obj(headers_obj));
                    if let Some(Json(json_body)) = &body {
                        req_obj.insert("body".to_string(), json_to_value(json_body));
                    }
                    mw_env.define("req".to_string(), Value::Obj(req_obj), false);
                    
                    let _ = execute_stmts(&fd.body, &mut mw_env);
                }
            }
            
            let mut ep_req_obj = HashMap::new();
            ep_req_obj.insert("method".to_string(), Value::Str(method_str.to_string()));
            ep_req_obj.insert("path".to_string(), Value::Str(path.clone()));
            let mut ep_headers_obj = HashMap::new();
            for (key, value) in headers.iter() {
                if let Ok(v) = value.to_str() {
                    ep_headers_obj.insert(key.to_string(), Value::Str(v.to_string()));
                }
            }
            ep_req_obj.insert("headers".to_string(), Value::Obj(ep_headers_obj));
            if let Some(Json(json_body)) = &body {
                ep_req_obj.insert("body".to_string(), json_to_value(json_body));
            }
            env_with_params.define("req".to_string(), Value::Obj(ep_req_obj), false);

            let result = execute_stmts(&ep.body, &mut env_with_params);
            let (status, response_str) = match result {
                Ok(crate::interpreter::ExecResult::Return(v)) => {
                    if let Value::Obj(ref map) = v {
                        let code = map.get("status")
                            .and_then(|s| if let Value::Num(n) = s { Some(*n as u16) } else { None })
                            .unwrap_or(200);
                        let body = map.get("body").unwrap_or(&v).clone();
                        (code, value_to_json(&body))
                    } else {
                        (200, value_to_json(&v))
                    }
                }
                Ok(_) => (200, "{\"ok\": true}".to_string()),
                Err(e) => (500, format!("{{\"error\": \"{}\"}}", e.replace('"', "\\\""))),
            };
            let mut builder = Response::builder()
                .status(StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
                .header("Content-Type", "application/json");
            if cors_enabled {
                builder = builder
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "*")
                    .header("Access-Control-Allow-Headers", "*");
            }
            return builder.body(response_str).unwrap();
        }
    }
    
    let mut builder = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "application/json");
    if cors_enabled {
        builder = builder
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "*")
            .header("Access-Control-Allow-Headers", "*");
    }
    builder.body("{\"error\": \"endpoint not found\"}".to_string()).unwrap()
}

// Convertir serde_json::Value en Value de l'interpréteur
fn json_to_value(j: &JsonValue) -> Value {
    match j {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Number(n) => Value::Num(n.as_f64().unwrap_or(0.0)),
        JsonValue::String(s) => Value::Str(s.clone()),
        JsonValue::Array(a) => Value::Arr(a.iter().map(json_to_value).collect()),
        JsonValue::Object(o) => {
            let mut map = HashMap::new();
            for (k, v) in o.iter() {
                map.insert(k.clone(), json_to_value(v));
            }
            Value::Obj(map)
        }
    }
}

pub async fn start_server(host: &str, port: u16, program: Program, env: Env) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState::new(program, env));
    
    let router = Router::new()
        .route("/*path", any(move |method: Method, Path(path): Path<String>, headers: HeaderMap, body: Option<Json<JsonValue>>| {
            let state_clone = state.clone();
            async move {
                handle_dynamic(method, path, headers, body, state_clone).await
            }
        }))
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", host, port).parse::<std::net::SocketAddr>()?;
    println!("BackLang v1.0.0 - Axum Server on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    
    Ok(())
}
