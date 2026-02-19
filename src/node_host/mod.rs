use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use serde_json::{json, Value};

/// iOS native module (Swift)
pub mod ios;
/// Android native module (Kotlin)
pub mod android;

/// Mobile device capabilities for iOS/Android nodes
pub mod mobile {
    use serde::{Deserialize, Serialize};

    /// Camera capture options
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CameraOptions {
        pub camera: Option<String>, // "front" or "back"
        pub quality: Option<String>, // "low", "medium", "high"
        pub flash: Option<bool>,
    }

    /// Screen recording options
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ScreenRecordOptions {
        pub audio: Option<bool>,
        pub quality: Option<String>, // "low", "medium", "high"
        pub duration_secs: Option<u32>,
    }

    /// Location options
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LocationOptions {
        pub accuracy: Option<String>, // "high", "balanced", "low"
        pub timeout_ms: Option<u32>,
    }

    /// Notification options
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NotificationOptions {
        pub title: String,
        pub body: String,
        pub sound: Option<bool>,
        pub badge: Option<u32>,
    }
}

#[derive(Debug, Clone)]
struct NodeState {
    platform: String,
    capabilities: Vec<String>,
    notifications: u64,
    runs: u64,
    camera_snaps: u64,
    screen_recordings: u64,
    location_requests: u64,
}

static NODE_STATES: Lazy<Mutex<HashMap<String, NodeState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn run_action(action: &str, payload: &Value) -> Result<Value> {
    match action {
        "pair" => pair_node(payload),
        "notify" => notify_node(payload),
        "run" => run_on_node(payload),
        "list" => list_nodes(),
        "status" => status(),
        // Mobile-specific actions
        "camera.snap" => camera_snap(payload),
        "screen.record" => screen_record(payload),
        "location.get" => location_get(payload),
        other => bail!("unsupported node_host action: {other}"),
    }
}

fn pair_node(payload: &Value) -> Result<Value> {
    let node_id = payload
        .get("node_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if node_id.is_empty() {
        bail!("node pair requires payload.node_id");
    }

    let platform = payload
        .get("platform")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .trim()
        .to_string();

    let capabilities = payload
        .get("capabilities")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let mut nodes = NODE_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("node state mutex poisoned"))?;
    nodes.insert(
        node_id.clone(),
        NodeState {
            platform: platform.clone(),
            capabilities: capabilities.clone(),
            notifications: 0,
            runs: 0,
            camera_snaps: 0,
            screen_recordings: 0,
            location_requests: 0,
        },
    );

    Ok(json!({
        "ok": true,
        "node_id": node_id,
        "platform": platform,
        "capabilities": capabilities,
        "message": "node paired"
    }))
}

fn notify_node(payload: &Value) -> Result<Value> {
    let node_id = payload
        .get("node_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if node_id.is_empty() {
        bail!("node notify requires payload.node_id");
    }

    let title = payload
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or("notification")
        .to_string();

    let body = payload
        .get("body")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let mut nodes = NODE_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("node state mutex poisoned"))?;
    let node = nodes
        .get_mut(&node_id)
        .ok_or_else(|| anyhow::anyhow!("unknown node_id: {node_id}"))?;
    node.notifications += 1;

    Ok(json!({
        "ok": true,
        "node_id": node_id,
        "title": title,
        "body": body,
        "notifications": node.notifications,
        "message": "node notification accepted"
    }))
}

fn run_on_node(payload: &Value) -> Result<Value> {
    let node_id = payload
        .get("node_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if node_id.is_empty() {
        bail!("node run requires payload.node_id");
    }

    let command = payload
        .get("command")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if command.is_empty() {
        bail!("node run requires payload.command");
    }

    let mut nodes = NODE_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("node state mutex poisoned"))?;
    let node = nodes
        .get_mut(&node_id)
        .ok_or_else(|| anyhow::anyhow!("unknown node_id: {node_id}"))?;
    node.runs += 1;

    Ok(json!({
        "ok": true,
        "node_id": node_id,
        "command": command,
        "runs": node.runs,
        "exit_code": 0,
        "stdout": "",
        "stderr": "",
        "message": "node run queued"
    }))
}

fn list_nodes() -> Result<Value> {
    let nodes = NODE_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("node state mutex poisoned"))?;
    let items: Vec<Value> = nodes
        .iter()
        .map(|(id, state)| {
            json!({
                "node_id": id,
                "platform": state.platform,
                "capabilities": state.capabilities,
                "notifications": state.notifications,
                "runs": state.runs
            })
        })
        .collect();

    Ok(json!({
        "ok": true,
        "nodes": items,
        "count": items.len(),
        "message": "node list"
    }))
}

fn status() -> Result<Value> {
    let nodes = NODE_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("node state mutex poisoned"))?;
    Ok(json!({
        "ok": true,
        "nodes": nodes.len(),
        "message": "node host ready"
    }))
}

/// Capture a photo from the mobile device camera
fn camera_snap(payload: &Value) -> Result<Value> {
    let node_id = payload
        .get("node_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if node_id.is_empty() {
        bail!("camera snap requires payload.node_id");
    }

    let camera = payload
        .get("camera")
        .and_then(Value::as_str)
        .unwrap_or("back")
        .to_string();

    let quality = payload
        .get("quality")
        .and_then(Value::as_str)
        .unwrap_or("high")
        .to_string();

    let flash = payload
        .get("flash")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let mut nodes = NODE_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("node state mutex poisoned"))?;
    let node = nodes
        .get_mut(&node_id)
        .ok_or_else(|| anyhow::anyhow!("unknown node_id: {node_id}"))?;
    
    // Validate platform supports camera
    if node.platform != "ios" && node.platform != "android" {
        bail!("camera snap only supported on ios/android platforms");
    }
    
    node.camera_snaps += 1;

    Ok(json!({
        "ok": true,
        "node_id": node_id,
        "camera": camera,
        "quality": quality,
        "flash": flash,
        "snap_id": format!("snap_{}_{}", node_id, node.camera_snaps),
        "camera_snaps": node.camera_snaps,
        "message": "camera snap queued"
    }))
}

/// Start screen recording on the mobile device
fn screen_record(payload: &Value) -> Result<Value> {
    let node_id = payload
        .get("node_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if node_id.is_empty() {
        bail!("screen record requires payload.node_id");
    }

    let audio = payload
        .get("audio")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let quality = payload
        .get("quality")
        .and_then(Value::as_str)
        .unwrap_or("high")
        .to_string();

    let duration_secs = payload
        .get("duration_secs")
        .and_then(Value::as_u64)
        .unwrap_or(60) as u32;

    let mut nodes = NODE_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("node state mutex poisoned"))?;
    let node = nodes
        .get_mut(&node_id)
        .ok_or_else(|| anyhow::anyhow!("unknown node_id: {node_id}"))?;
    
    // Validate platform supports screen recording
    if node.platform != "ios" && node.platform != "android" {
        bail!("screen recording only supported on ios/android platforms");
    }
    
    node.screen_recordings += 1;

    Ok(json!({
        "ok": true,
        "node_id": node_id,
        "audio": audio,
        "quality": quality,
        "duration_secs": duration_secs,
        "recording_id": format!("rec_{}_{}", node_id, node.screen_recordings),
        "screen_recordings": node.screen_recordings,
        "message": "screen recording started"
    }))
}

/// Get current location from the mobile device
fn location_get(payload: &Value) -> Result<Value> {
    let node_id = payload
        .get("node_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if node_id.is_empty() {
        bail!("location get requires payload.node_id");
    }

    let accuracy = payload
        .get("accuracy")
        .and_then(Value::as_str)
        .unwrap_or("high")
        .to_string();

    let timeout_ms = payload
        .get("timeout_ms")
        .and_then(Value::as_u64)
        .unwrap_or(10000) as u32;

    let mut nodes = NODE_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("node state mutex poisoned"))?;
    let node = nodes
        .get_mut(&node_id)
        .ok_or_else(|| anyhow::anyhow!("unknown node_id: {node_id}"))?;
    
    // Validate platform supports location
    if node.platform != "ios" && node.platform != "android" {
        bail!("location only supported on ios/android platforms");
    }
    
    node.location_requests += 1;

    // Return mock location data for demonstration
    Ok(json!({
        "ok": true,
        "node_id": node_id,
        "accuracy": accuracy,
        "timeout_ms": timeout_ms,
        "latitude": 13.7563,
        "longitude": 100.5018,
        "altitude": 12.0,
        "accuracy_meters": 10.0,
        "location_requests": node.location_requests,
        "message": "location acquired"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_notify_run_list() {
        let pair = run_action(
            "pair",
            &json!({
                "node_id": "node-a",
                "platform": "ios",
                "capabilities": ["notify", "system.run"]
            }),
        )
        .unwrap();
        assert_eq!(pair["ok"], true);

        let notify = run_action(
            "notify",
            &json!({"node_id":"node-a", "title":"t", "body":"b"}),
        )
        .unwrap();
        assert_eq!(notify["notifications"], 1);

        let run = run_action("run", &json!({"node_id":"node-a", "command":"uptime"})).unwrap();
        assert_eq!(run["runs"], 1);

        let list = run_action("list", &json!({})).unwrap();
        assert!(list["count"].as_u64().unwrap() >= 1);
    }
}
