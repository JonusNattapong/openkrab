use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
struct CanvasState {
    view: Value,
    revision: u64,
}

static CANVAS_STATES: Lazy<Mutex<HashMap<String, CanvasState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn run_action(action: &str, payload: &Value) -> Result<Value> {
    let workspace = payload
        .get("workspace")
        .and_then(Value::as_str)
        .unwrap_or("main")
        .trim();
    if workspace.is_empty() {
        bail!("canvas workspace must not be empty");
    }

    match action {
        "push" => push_view(workspace, payload),
        "snapshot" => snapshot(workspace),
        "reset" => reset(workspace),
        "status" => status(),
        other => bail!("unsupported canvas_host action: {other}"),
    }
}

fn push_view(workspace: &str, payload: &Value) -> Result<Value> {
    let view = payload
        .get("view")
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("canvas push requires payload.view"))?;

    let mut states = CANVAS_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("canvas state mutex poisoned"))?;
    let state = states
        .entry(workspace.to_string())
        .or_insert_with(|| CanvasState {
            view: json!({}),
            revision: 0,
        });

    state.view = view;
    state.revision += 1;

    Ok(json!({
        "ok": true,
        "workspace": workspace,
        "revision": state.revision,
        "message": "canvas view updated"
    }))
}

fn snapshot(workspace: &str) -> Result<Value> {
    let states = CANVAS_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("canvas state mutex poisoned"))?;
    let state = states.get(workspace).cloned().unwrap_or(CanvasState {
        view: json!({}),
        revision: 0,
    });

    Ok(json!({
        "ok": true,
        "workspace": workspace,
        "revision": state.revision,
        "view": state.view,
        "message": "canvas snapshot"
    }))
}

fn reset(workspace: &str) -> Result<Value> {
    let mut states = CANVAS_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("canvas state mutex poisoned"))?;
    states.remove(workspace);
    Ok(json!({
        "ok": true,
        "workspace": workspace,
        "message": "canvas reset"
    }))
}

fn status() -> Result<Value> {
    let states = CANVAS_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("canvas state mutex poisoned"))?;
    let workspaces = states.len();
    Ok(json!({
        "ok": true,
        "workspaces": workspaces,
        "message": "canvas host ready"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_snapshot_reset_roundtrip() {
        let push = run_action(
            "push",
            &json!({
                "workspace": "test",
                "view": { "type": "card", "text": "hello" }
            }),
        )
        .unwrap();
        assert_eq!(push["ok"], true);
        assert_eq!(push["revision"], 1);

        let snap = run_action("snapshot", &json!({ "workspace": "test" })).unwrap();
        assert_eq!(snap["view"]["text"], "hello");

        let reset = run_action("reset", &json!({ "workspace": "test" })).unwrap();
        assert_eq!(reset["ok"], true);

        let snap2 = run_action("snapshot", &json!({ "workspace": "test" })).unwrap();
        assert_eq!(snap2["revision"], 0);
    }
}
