//! Browser automation via Chrome DevTools Protocol (CDP).

use anyhow::{anyhow, bail, Context, Result};
use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrowserProfile {
    pub name: String,
    pub cdp_http_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTab {
    pub id: String,
    pub title: String,
    pub url: String,
    pub websocket_debugger_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSnapshot {
    pub url: Option<String>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub screenshot_base64: Option<String>,
}

pub fn register_profile(name: &str, cdp_http_url: &str) -> Result<()> {
    let key = name.trim();
    if key.is_empty() {
        bail!("profile name is required");
    }
    let endpoint = normalize_http_endpoint(cdp_http_url)?;

    let mut store = load_profiles()?;
    store.insert(
        key.to_string(),
        BrowserProfile {
            name: key.to_string(),
            cdp_http_url: endpoint,
        },
    );
    save_profiles(&store)?;
    Ok(())
}

pub fn remove_profile(name: &str) -> bool {
    let key = name.trim();
    if key.is_empty() {
        return false;
    }
    match load_profiles() {
        Ok(mut s) => {
            let removed = s.remove(key).is_some();
            if removed {
                let _ = save_profiles(&s);
            }
            removed
        }
        Err(_) => false,
    }
}

pub fn list_profiles() -> Vec<BrowserProfile> {
    match load_profiles() {
        Ok(store) => {
            let mut v: Vec<BrowserProfile> = store.values().cloned().collect();
            v.sort_by(|a, b| a.name.cmp(&b.name));
            v
        }
        Err(_) => Vec::new(),
    }
}

fn resolve_profile(name: &str) -> Result<BrowserProfile> {
    let key = name.trim();
    let store = load_profiles()?;
    if let Some(p) = store.get(key).cloned() {
        return Ok(p);
    }

    let env_key = format!(
        "BROWSER_CDP_URL_{}",
        key.to_ascii_uppercase().replace('-', "_")
    );
    let fallback = std::env::var(&env_key)
        .ok()
        .or_else(|| std::env::var("BROWSER_CDP_URL").ok())
        .or_else(|| {
            if key == "default" {
                Some("http://127.0.0.1:9222".to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow!("unknown browser profile: {key}"))?;

    Ok(BrowserProfile {
        name: key.to_string(),
        cdp_http_url: normalize_http_endpoint(&fallback)?,
    })
}

fn profiles_path() -> PathBuf {
    if let Ok(custom) = std::env::var("KRABKRAB_BROWSER_PROFILES_PATH") {
        let p = custom.trim();
        if !p.is_empty() {
            return PathBuf::from(p);
        }
    }
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("krabkrab").join("browser-profiles.json")
}

fn load_profiles() -> Result<HashMap<String, BrowserProfile>> {
    let path = profiles_path();
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read browser profiles: {}", path.display()))?;
    let parsed: HashMap<String, BrowserProfile> = serde_json::from_str(&raw)
        .with_context(|| format!("invalid browser profiles JSON: {}", path.display()))?;
    Ok(parsed)
}

fn save_profiles(profiles: &HashMap<String, BrowserProfile>) -> Result<()> {
    let path = profiles_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create profile dir: {}", parent.display()))?;
    }
    let data = serde_json::to_string_pretty(profiles)?;
    fs::write(&path, data)
        .with_context(|| format!("failed to write browser profiles: {}", path.display()))?;
    Ok(())
}

pub async fn list_tabs(profile: &str) -> Result<Vec<BrowserTab>> {
    let p = resolve_profile(profile)?;
    let url = format!("{}/json/list", p.cdp_http_url.trim_end_matches('/'));
    let raw: Vec<Value> = Client::new()
        .get(&url)
        .send()
        .await
        .with_context(|| format!("failed to query tabs: {url}"))?
        .error_for_status()?
        .json()
        .await?;

    Ok(raw
        .into_iter()
        .map(|t| BrowserTab {
            id: t
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            title: t
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            url: t
                .get("url")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            websocket_debugger_url: t
                .get("webSocketDebuggerUrl")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        })
        .collect())
}

pub async fn open_tab(profile: &str, url: &str) -> Result<BrowserTab> {
    let p = resolve_profile(profile)?;
    let encoded = urlencoding::encode(url);
    let endpoint = format!(
        "{}/json/new?{}",
        p.cdp_http_url.trim_end_matches('/'),
        encoded
    );
    let v: Value = Client::new()
        .put(&endpoint)
        .send()
        .await
        .with_context(|| format!("failed to open new tab: {endpoint}"))?
        .error_for_status()?
        .json()
        .await?;

    Ok(BrowserTab {
        id: v
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        title: v
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        url: v
            .get("url")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        websocket_debugger_url: v
            .get("webSocketDebuggerUrl")
            .and_then(Value::as_str)
            .map(ToString::to_string),
    })
}

pub async fn navigate(profile: &str, url: &str) -> Result<()> {
    let ws = resolve_primary_tab_ws_url(profile).await?;
    let _ = cdp_call(&ws, "Page.enable", json!({})).await?;
    let _ = cdp_call(&ws, "Page.navigate", json!({ "url": url })).await?;
    Ok(())
}

pub async fn click(profile: &str, selector: &str) -> Result<()> {
    let ws = resolve_primary_tab_ws_url(profile).await?;
    let expr = format!(
        "(() => {{ const el = document.querySelector({}); if (!el) return false; el.click(); return true; }})()",
        serde_json::to_string(selector)?
    );
    let out = cdp_call(
        &ws,
        "Runtime.evaluate",
        json!({ "expression": expr, "returnByValue": true }),
    )
    .await?;
    let ok = out
        .get("result")
        .and_then(|r| r.get("result"))
        .and_then(|r| r.get("value"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if !ok {
        bail!("click failed; selector not found: {selector}");
    }
    Ok(())
}

pub async fn type_text(profile: &str, selector: &str, text: &str) -> Result<()> {
    let ws = resolve_primary_tab_ws_url(profile).await?;
    let expr = format!(
        "(() => {{ const el = document.querySelector({}); if (!el) return false; el.focus(); el.value = {}; el.dispatchEvent(new Event('input', {{bubbles:true}})); return true; }})()",
        serde_json::to_string(selector)?,
        serde_json::to_string(text)?
    );
    let out = cdp_call(
        &ws,
        "Runtime.evaluate",
        json!({ "expression": expr, "returnByValue": true }),
    )
    .await?;
    let ok = out
        .get("result")
        .and_then(|r| r.get("result"))
        .and_then(|r| r.get("value"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if !ok {
        bail!("type failed; selector not found: {selector}");
    }
    Ok(())
}

pub async fn upload_files(profile: &str, selector: &str, files: &[String]) -> Result<()> {
    if files.is_empty() {
        bail!("at least one file is required");
    }
    let ws = resolve_primary_tab_ws_url(profile).await?;

    let _ = cdp_call(&ws, "DOM.enable", json!({})).await?;
    let eval = cdp_call(
        &ws,
        "Runtime.evaluate",
        json!({
            "expression": format!("document.querySelector({})", serde_json::to_string(selector)?),
        }),
    )
    .await?;

    let object_id = eval
        .get("result")
        .and_then(|r| r.get("result"))
        .and_then(|r| r.get("objectId"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("upload failed; selector not found: {selector}"))?;

    let node = cdp_call(&ws, "DOM.requestNode", json!({ "objectId": object_id })).await?;
    let node_id = node
        .get("result")
        .and_then(|r| r.get("nodeId"))
        .and_then(Value::as_i64)
        .ok_or_else(|| anyhow!("upload failed; unable to resolve nodeId"))?;

    let _ = cdp_call(
        &ws,
        "DOM.setFileInputFiles",
        json!({ "files": files, "nodeId": node_id }),
    )
    .await?;
    Ok(())
}

pub async fn snapshot(profile: &str) -> Result<BrowserSnapshot> {
    let ws = resolve_primary_tab_ws_url(profile).await?;
    let _ = cdp_call(&ws, "Page.enable", json!({})).await?;

    let text_out = cdp_call(
        &ws,
        "Runtime.evaluate",
        json!({
            "expression": "({ title: document.title, url: location.href, text: document.body ? document.body.innerText : '' })",
            "returnByValue": true
        }),
    )
    .await?;

    let shot = cdp_call(
        &ws,
        "Page.captureScreenshot",
        json!({ "format": "png", "fromSurface": true }),
    )
    .await?;

    let value = text_out
        .get("result")
        .and_then(|r| r.get("result"))
        .and_then(|r| r.get("value"))
        .cloned()
        .unwrap_or_else(|| json!({}));

    Ok(BrowserSnapshot {
        url: value
            .get("url")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        title: value
            .get("title")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        text: value
            .get("text")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        screenshot_base64: shot
            .get("result")
            .and_then(|r| r.get("data"))
            .and_then(Value::as_str)
            .map(ToString::to_string),
    })
}

async fn resolve_primary_tab_ws_url(profile: &str) -> Result<String> {
    let tabs = list_tabs(profile).await?;
    let ws = tabs
        .into_iter()
        .find_map(|t| t.websocket_debugger_url)
        .ok_or_else(|| anyhow!("no debuggable tabs found"))?;
    Ok(ws)
}

async fn cdp_call(ws_url: &str, method: &str, params: Value) -> Result<Value> {
    let (mut ws, _) = connect_async(ws_url)
        .await
        .with_context(|| format!("failed to connect CDP websocket: {ws_url}"))?;

    let id = 1_i64;
    let req = json!({ "id": id, "method": method, "params": params });
    ws.send(Message::Text(req.to_string())).await?;

    while let Some(msg) = ws.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let v: Value = serde_json::from_str(&text)?;
                if v.get("id").and_then(Value::as_i64) == Some(id) {
                    if let Some(err) = v.get("error") {
                        bail!("cdp {} failed: {}", method, err);
                    }
                    return Ok(v);
                }
            }
            Ok(_) => continue,
            Err(e) => return Err(anyhow!("cdp websocket error: {e}")),
        }
    }

    Err(anyhow!("cdp {} failed: websocket closed", method))
}

fn normalize_http_endpoint(input: &str) -> Result<String> {
    let raw = input.trim();
    if raw.is_empty() {
        bail!("cdp endpoint is required");
    }
    let with_scheme = if raw.starts_with("http://") || raw.starts_with("https://") {
        raw.to_string()
    } else {
        format!("http://{raw}")
    };
    let parsed = reqwest::Url::parse(&with_scheme)
        .with_context(|| format!("invalid cdp endpoint: {input}"))?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        bail!("unsupported cdp endpoint scheme: {}", parsed.scheme());
    }
    Ok(parsed.to_string().trim_end_matches('/').to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn profile_registry_roundtrip() {
        let tmp = tempdir().expect("tmp");
        let path = tmp.path().join("profiles.json");
        std::env::set_var(
            "KRABKRAB_BROWSER_PROFILES_PATH",
            path.to_string_lossy().to_string(),
        );
        register_profile("default", "127.0.0.1:9222").expect("register");
        let profiles = list_profiles();
        assert!(profiles.iter().any(|p| p.name == "default"));
        assert!(remove_profile("default"));
        std::env::remove_var("KRABKRAB_BROWSER_PROFILES_PATH");
    }

    #[test]
    fn endpoint_normalization_adds_scheme() {
        let v = normalize_http_endpoint("127.0.0.1:9222").expect("endpoint");
        assert_eq!(v, "http://127.0.0.1:9222");
    }

    #[test]
    fn endpoint_rejects_empty() {
        assert!(normalize_http_endpoint(" ").is_err());
    }
}
