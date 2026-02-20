use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2UISurface {
    pub surface_id: String,
    pub catalog_id: Option<String>,
    pub theme: Option<A2UITheme>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2UITheme {
    pub colors: Option<A2UIColors>,
    pub typography: Option<A2UITypography>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2UIColors {
    pub background: Option<String>,
    pub surface: Option<String>,
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub text: Option<String>,
    pub muted: Option<String>,
    pub border: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2UITypography {
    pub font_family: Option<String>,
    pub font_size_base: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2UIComponent {
    pub id: String,
    pub component: String,
    #[serde(default)]
    pub props: HashMap<String, Value>,
    #[serde(default)]
    pub children: Vec<String>,
    pub child: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2UIUpdateComponents {
    pub surface_id: String,
    pub components: Vec<A2UIComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2UIUpdateDataModel {
    pub surface_id: String,
    pub path: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum A2UIMessage {
    #[serde(rename_all = "camelCase")]
    CreateSurface {
        version: String,
        create_surface: A2UISurface,
    },
    #[serde(rename_all = "camelCase")]
    UpdateDataModel {
        version: String,
        update_data_model: A2UIUpdateDataModel,
    },
    #[serde(rename_all = "camelCase")]
    UpdateComponents {
        version: String,
        update_components: A2UIUpdateComponents,
    },
    #[serde(rename_all = "camelCase")]
    DeleteSurface {
        version: String,
        delete_surface: DeleteSurface,
    },
    #[serde(rename_all = "camelCase")]
    BeginRendering {
        version: String,
        begin_rendering: BeginRendering,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSurface {
    pub surface_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginRendering {
    pub root: String,
    pub surface_id: String,
}

#[derive(Debug, Clone)]
struct CanvasState {
    surface: Option<A2UISurface>,
    data_model: Value,
    components: Vec<A2UIComponent>,
    revision: u64,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            surface: None,
            data_model: json!({}),
            components: Vec::new(),
            revision: 0,
        }
    }
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
        "create_surface" => create_surface(workspace, payload),
        "update_data_model" => update_data_model(workspace, payload),
        "update_components" => update_components(workspace, payload),
        "delete_surface" => delete_surface(workspace, payload),
        other => bail!("unsupported canvas_host action: {other}"),
    }
}

fn create_surface(workspace: &str, payload: &Value) -> Result<Value> {
    let surface: A2UISurface = serde_json::from_value(
        payload
            .get("surface")
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("create_surface requires payload.surface"))?,
    )
    .map_err(|e| anyhow::anyhow!("invalid surface: {}", e))?;

    let mut states = CANVAS_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("canvas state mutex poisoned"))?;
    let state = states
        .entry(workspace.to_string())
        .or_insert_with(CanvasState::default);

    state.surface = Some(surface);
    state.revision += 1;

    Ok(json!({
        "ok": true,
        "workspace": workspace,
        "revision": state.revision,
        "message": "surface created"
    }))
}

fn update_data_model(workspace: &str, payload: &Value) -> Result<Value> {
    let path = payload
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("update_data_model requires payload.path"))?;
    let value = payload
        .get("value")
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("update_data_model requires payload.value"))?;

    let mut states = CANVAS_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("canvas state mutex poisoned"))?;
    let state = states
        .entry(workspace.to_string())
        .or_insert_with(CanvasState::default);

    update_value_at_path(&mut state.data_model, path, value);
    state.revision += 1;

    Ok(json!({
        "ok": true,
        "workspace": workspace,
        "revision": state.revision,
        "path": path,
        "message": "data model updated"
    }))
}

fn update_value_at_path(root: &mut Value, path: &str, value: Value) {
    let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    if parts.is_empty() || (parts.len() == 1 && parts[0].is_empty()) {
        *root = value;
        return;
    }

    let mut current = root;
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            if let Some(obj) = current.as_object_mut() {
                obj.insert(part.to_string(), value);
            }
            return;
        }

        if current.get(*part).is_none() {
            if let Some(obj) = current.as_object_mut() {
                obj.insert(part.to_string(), json!({}));
            }
        }
        if let Some(v) = current.get_mut(*part) {
            current = v;
        } else {
            return;
        }
    }
}

fn update_components(workspace: &str, payload: &Value) -> Result<Value> {
    let components: Vec<A2UIComponent> = serde_json::from_value(
        payload
            .get("components")
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("update_components requires payload.components"))?,
    )
    .map_err(|e| anyhow::anyhow!("invalid components: {}", e))?;

    let mut states = CANVAS_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("canvas state mutex poisoned"))?;
    let state = states
        .entry(workspace.to_string())
        .or_insert_with(CanvasState::default);

    state.components = components;
    state.revision += 1;

    Ok(json!({
        "ok": true,
        "workspace": workspace,
        "revision": state.revision,
        "message": "components updated"
    }))
}

fn delete_surface(workspace: &str, _payload: &Value) -> Result<Value> {
    let mut states = CANVAS_STATES
        .lock()
        .map_err(|_| anyhow::anyhow!("canvas state mutex poisoned"))?;
    states.remove(workspace);
    Ok(json!({
        "ok": true,
        "workspace": workspace,
        "message": "surface deleted"
    }))
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
        .or_insert_with(CanvasState::default);

    if let Some(obj) = view.as_object() {
        if let Some(surface) = obj.get("surface") {
            if let Ok(s) = serde_json::from_value(surface.clone()) {
                state.surface = Some(s);
            }
        }
        if let Some(data_model) = obj.get("dataModel") {
            state.data_model = data_model.clone();
        }
        if let Some(components) = obj.get("components") {
            if let Ok(c) = serde_json::from_value(components.clone()) {
                state.components = c;
            }
        }
    }

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
    let state = states.get(workspace).cloned().unwrap_or_default();

    Ok(json!({
        "ok": true,
        "workspace": workspace,
        "revision": state.revision,
        "surface": state.surface,
        "dataModel": state.data_model,
        "components": state.components,
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
        assert_eq!(snap["ok"], true);

        let reset = run_action("reset", &json!({ "workspace": "test" })).unwrap();
        assert_eq!(reset["ok"], true);

        let snap2 = run_action("snapshot", &json!({ "workspace": "test" })).unwrap();
        assert_eq!(snap2["revision"], 0);
    }

    #[test]
    fn a2ui_create_surface() {
        let result = run_action(
            "create_surface",
            &json!({
                "workspace": "test",
                "surface": {
                    "surfaceId": "main",
                    "title": "Test Surface",
                    "catalogId": "https://a2ui.org/catalog/standard"
                }
            }),
        )
        .unwrap();
        assert_eq!(result["ok"], true);
        assert_eq!(result["revision"], 1);
    }

    #[test]
    fn a2ui_update_data_model() {
        run_action(
            "create_surface",
            &json!({
                "workspace": "test",
                "surface": { "surfaceId": "main" }
            }),
        )
        .unwrap();

        let result = run_action(
            "update_data_model",
            &json!({
                "workspace": "test",
                "path": "/user/name",
                "value": "Alice"
            }),
        )
        .unwrap();
        assert_eq!(result["ok"], true);

        let snap = run_action("snapshot", &json!({ "workspace": "test" })).unwrap();
        assert_eq!(snap["dataModel"]["user"]["name"], "Alice");
    }

    #[test]
    fn a2ui_update_components() {
        let result = run_action(
            "update_components",
            &json!({
                "workspace": "test",
                "components": [
                    { "id": "root", "component": "Column", "children": ["card1"] },
                    { "id": "card1", "component": "Card", "props": { "title": "Hello" } }
                ]
            }),
        )
        .unwrap();
        assert_eq!(result["ok"], true);

        let snap = run_action("snapshot", &json!({ "workspace": "test" })).unwrap();
        assert_eq!(snap["components"].as_array().unwrap().len(), 2);
    }
}
