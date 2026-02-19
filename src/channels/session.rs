/// Minimal session representation for channel sessions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub id: String,
}

impl Session {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

use serde_json::Value;
use std::fs;
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone)]
pub struct InboundLastRouteUpdate {
    pub session_key: String,
    pub channel: String,
    pub to: String,
    pub account_id: Option<String>,
    pub thread_id: Option<String>,
}

pub struct RecordInboundSessionParams {
    pub store_path: String,
    pub session_key: String,
    pub ctx: Value,
    pub create_if_missing: bool,
    pub update_last_route: Option<InboundLastRouteUpdate>,
}

pub fn record_inbound_session(params: RecordInboundSessionParams) {
    // Best-effort: write a small JSON file representing session metadata.
    let dir = Path::new(&params.store_path);
    if params.create_if_missing {
        let _ = fs::create_dir_all(dir);
    }
    let path = dir.join(format!("session-{}.json", params.session_key));
    let mut obj = serde_json::Map::new();
    obj.insert(
        "sessionKey".to_string(),
        Value::String(params.session_key.clone()),
    );
    obj.insert("ctx".to_string(), params.ctx.clone());
    if let Some(route) = params.update_last_route {
        let mut route_map = serde_json::Map::new();
        route_map.insert("channel".to_string(), Value::String(route.channel));
        route_map.insert("to".to_string(), Value::String(route.to));
        if let Some(a) = route.account_id {
            route_map.insert("accountId".to_string(), Value::String(a));
        }
        if let Some(t) = route.thread_id {
            route_map.insert("threadId".to_string(), Value::String(t));
        }
        obj.insert("lastRoute".to_string(), Value::Object(route_map));
    }
    let _ = fs::write(
        path,
        serde_json::to_string_pretty(&Value::Object(obj)).unwrap_or_default(),
    );
    info!("recorded inbound session {}", params.session_key);
}
