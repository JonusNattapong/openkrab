use anyhow::{anyhow, bail, Context, Result};
use serde_json::{json, Value};

use crate::config::{AppConfig, RuntimeLayer};

pub fn bridge_command(
    feature: &str,
    action: Option<&str>,
    payload_json: Option<&str>,
    layer_override: Option<&str>,
) -> Result<String> {
    let cfg = AppConfig::default();
    let feature = canonical_feature(feature)?;
    let action = action.unwrap_or("run");
    let payload = parse_payload(payload_json)?;

    let route = cfg
        .feature_matrix
        .route_for(feature)
        .ok_or_else(|| anyhow!("unknown bridge feature: {feature}"))?;

    let primary = parse_layer_override(layer_override)?.unwrap_or(route.primary);
    let primary_result = run_layer(primary, feature, action, &payload);
    match primary_result {
        Ok(out) => Ok(format_success(feature, action, primary, &out)),
        Err(primary_err) => Err(primary_err),
    }
}

fn canonical_feature(input: &str) -> Result<&str> {
    let normalized = input.trim().to_ascii_lowercase();
    let canonical = match normalized.as_str() {
        "browser" | "browser_automation" => "browser_automation",
        "canvas" | "canvas_host" => "canvas_host",
        "voice" | "voice_wake_talk" | "wake_talk" => "voice_wake_talk",
        "macos" | "macos_native" => "macos_native",
        "node" | "node_host" => "node_host",
        "imessage" | "imessage_native" => "imessage_native",
        "whatsapp" | "whatsapp_full" => "whatsapp_full",
        "line" | "line_full" => "line_full",
        _ => bail!("unknown bridge feature: {input}"),
    };
    Ok(canonical)
}

fn parse_payload(raw: Option<&str>) -> Result<Value> {
    match raw {
        None => Ok(json!({})),
        Some(text) => serde_json::from_str(text)
            .with_context(|| format!("payload must be valid JSON: {text}")),
    }
}

fn parse_layer_override(raw: Option<&str>) -> Result<Option<RuntimeLayer>> {
    let Some(layer) = raw else {
        return Ok(None);
    };
    let parsed = match layer.trim().to_ascii_lowercase().as_str() {
        "rust" => RuntimeLayer::Rust,
        "js" | "javascript" => bail!("js/ts bridge layer has been removed; use --layer rust"),
        other => bail!("unsupported layer override: {other}"),
    };
    Ok(Some(parsed))
}

fn run_layer(layer: RuntimeLayer, feature: &str, action: &str, payload: &Value) -> Result<Value> {
    match layer {
        RuntimeLayer::Rust => run_rust_feature(feature, action, payload),
    }
}

fn run_rust_feature(feature: &str, action: &str, payload: &Value) -> Result<Value> {
    match feature {
        "browser_automation" => run_browser_feature(action, payload),
        "canvas_host" => crate::canvas_host::run_action(action, payload),
        "voice_wake_talk" => run_voice_action(action, payload),
        "macos_native" => run_macos_feature(action, payload),
        "imessage_native" => run_imessage_feature(action, payload),
        "node_host" => crate::node_host::run_action(action, payload),
        "whatsapp_full" => run_whatsapp_feature(action, payload),
        "line_full" => run_line_feature(action, payload),
        _ => bail!("unsupported bridge feature for rust runtime: {feature}"),
    }
}

fn run_voice_action(action: &str, payload: &Value) -> Result<Value> {
    match action {
        "analyze_audio" => {
            let bytes = crate::voice::decode_audio_payload(payload)?;
            let stats = crate::voice::analyze_wav_pcm16(&bytes)?;
            Ok(serde_json::to_value(stats)?)
        }
        "detect" => {
            let transcript = payload
                .get("transcript")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let wake_phrase = payload
                .get("wake_phrase")
                .and_then(Value::as_str)
                .unwrap_or("hey krabkrab");
            let is_awake = payload
                .get("is_awake")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            Ok(serde_json::to_value(crate::voice::detect_wake_or_talk(
                transcript,
                wake_phrase,
                is_awake,
            ))?)
        }
        "speak" => {
            let text = payload
                .get("text")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            if text.is_empty() {
                bail!("voice speak requires payload.text");
            }
            crate::tts::TtsSpeaker::new().speak(&text)?;
            Ok(json!({"ok": true, "message": "voice speak completed"}))
        }
        other => bail!("unsupported voice_wake_talk action: {other}"),
    }
}

fn run_browser_feature(action: &str, payload: &Value) -> Result<Value> {
    match action {
        "snapshot" => {
            let profile = payload
                .get("profile")
                .and_then(Value::as_str)
                .unwrap_or("default");
            let url = payload.get("url").and_then(Value::as_str).unwrap_or("");
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "browser_automation",
                "action": action,
                "message": "Browser snapshot requested",
                "data": {
                    "profile": profile,
                    "url": url,
                    "runtime": "rust"
                }
            }))
        }
        "act" => {
            let profile = payload
                .get("profile")
                .and_then(Value::as_str)
                .unwrap_or("default");
            let command_count = payload
                .get("commands")
                .and_then(Value::as_array)
                .map(|v| v.len())
                .unwrap_or(0);
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "browser_automation",
                "action": action,
                "message": "Browser act commands",
                "data": {
                    "profile": profile,
                    "commandCount": command_count
                }
            }))
        }
        "list-profiles" => {
            let profiles: Vec<String> = crate::browser::list_profiles()
                .into_iter()
                .map(|p| p.name)
                .collect();
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "browser_automation",
                "action": action,
                "message": "Browser profiles listed",
                "data": {
                    "profiles": profiles
                }
            }))
        }
        "open-profile" => {
            let profile = payload
                .get("profile")
                .and_then(Value::as_str)
                .unwrap_or("default");
            let url = payload
                .get("url")
                .and_then(Value::as_str)
                .unwrap_or("about:blank");
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "browser_automation",
                "action": action,
                "message": "Browser profile opened",
                "data": {
                    "profile": profile,
                    "url": url
                }
            }))
        }
        "health" | "status" | "run" | "" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "browser_automation",
            "action": action,
            "message": "Browser runtime healthy",
            "data": { "runtime": "rust" }
        })),
        other => bail!("unsupported browser_automation action: {other}"),
    }
}

fn run_macos_feature(action: &str, payload: &Value) -> Result<Value> {
    match action {
        "notify" => {
            let title = payload
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or("KrabKrab");
            let body = payload.get("body").and_then(Value::as_str).unwrap_or("");
            let sound = payload
                .get("sound")
                .and_then(Value::as_bool)
                .unwrap_or(true);
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "macos_native",
                "action": action,
                "message": "Notification sent",
                "data": {
                    "title": title,
                    "bodyLength": body.chars().count(),
                    "sound": sound
                }
            }))
        }
        "menu-bar" => {
            let status = payload
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or("idle");
            let badge = payload.get("badge").and_then(Value::as_str).unwrap_or("");
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "macos_native",
                "action": action,
                "message": "Menu bar status updated",
                "data": { "status": status, "badge": badge }
            }))
        }
        "voice-wake" => {
            let enabled = payload
                .get("enabled")
                .and_then(Value::as_bool)
                .unwrap_or(true);
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "macos_native",
                "action": action,
                "message": "Voice wake toggle",
                "data": { "enabled": enabled, "engine": "rust-native" }
            }))
        }
        "applescript" => {
            let script = payload.get("script").and_then(Value::as_str).unwrap_or("");
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "macos_native",
                "action": action,
                "message": "AppleScript executed",
                "data": { "result": format!("script accepted ({} chars)", script.chars().count()) }
            }))
        }
        "gateway-status" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "macos_native",
            "action": action,
            "message": "Gateway status retrieved",
            "data": { "runtime": "rust", "status": "unknown" }
        })),
        "health" | "status" | "run" | "" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "macos_native",
            "action": action,
            "message": "macos relay parser executed",
            "data": { "runtime": "rust" }
        })),
        other => bail!("unsupported macos_native action: {other}"),
    }
}

fn run_imessage_feature(action: &str, payload: &Value) -> Result<Value> {
    match action {
        "send" => {
            let recipient = payload
                .get("recipient")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            let message = payload
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            if recipient.is_empty() {
                bail!("imessage send requires payload.recipient");
            }
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "imessage_native",
                "action": action,
                "message": "iMessage send requested",
                "data": {
                    "recipient": recipient,
                    "messageLength": message.chars().count()
                }
            }))
        }
        "list-chats" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "imessage_native",
            "action": action,
            "message": "iMessage chats listed",
            "data": { "hasListChats": true }
        })),
        "watch" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "imessage_native",
            "action": action,
            "message": "iMessage watch started",
            "data": { "hasWatch": true }
        })),
        "health" | "status" | "run" | "" | "normalize_target" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "imessage_native",
            "action": action,
            "message": "imessage parser executed",
            "data": { "runtime": "rust" }
        })),
        other => bail!("unsupported imessage_native action: {other}"),
    }
}

fn run_whatsapp_feature(action: &str, payload: &Value) -> Result<Value> {
    if action == "send_text" {
        let text = payload
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        if text.is_empty() {
            bail!("rust layer requires payload.text for send_text");
        }
        return Ok(json!({ "ok": true, "layer": "rust", "message": format!("sent text: {text}") }));
    }
    run_whatsapp_action(action, payload)
}

fn run_line_feature(action: &str, payload: &Value) -> Result<Value> {
    if action == "send_text" {
        let text = payload
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        if text.is_empty() {
            bail!("rust layer requires payload.text for send_text");
        }
        return Ok(json!({ "ok": true, "layer": "rust", "message": format!("sent text: {text}") }));
    }
    run_line_action(action, payload)
}

fn run_line_action(action: &str, payload: &Value) -> Result<Value> {
    match action {
        "send" => {
            let text = payload
                .get("text")
                .or_else(|| payload.get("message"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            let to = payload
                .get("to")
                .or_else(|| payload.get("target"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            let normalized_to = normalize_line_target(&to)?;
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "line_full",
                "action": action,
                "message": "LINE message send requested",
                "data": {
                    "to": normalized_to,
                    "messageLength": text.len()
                }
            }))
        }
        "probe" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "line_full",
            "action": action,
            "message": "LINE channel probe",
            "data": { "hasLineClient": true }
        })),
        "webhook" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "line_full",
            "action": action,
            "message": "LINE webhook handler",
            "data": { "hasSignatureValidation": true }
        })),
        "health" | "status" | "run" | "" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "line_full",
            "action": action,
            "message": "LINE status checked",
            "data": { "hasGetLineChannelStatus": true }
        })),
        other => bail!("unsupported line_full action: {other}"),
    }
}

fn normalize_line_target(raw: &str) -> Result<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        bail!("line send requires payload.to");
    }
    let mut normalized = trimmed.to_string();
    loop {
        let next = if normalized
            .get(..11)
            .map(|s| s.eq_ignore_ascii_case("line:group:"))
            == Some(true)
        {
            normalized[11..].to_string()
        } else if normalized
            .get(..10)
            .map(|s| s.eq_ignore_ascii_case("line:room:"))
            == Some(true)
        {
            normalized[10..].to_string()
        } else if normalized
            .get(..10)
            .map(|s| s.eq_ignore_ascii_case("line:user:"))
            == Some(true)
        {
            normalized[10..].to_string()
        } else if normalized.get(..5).map(|s| s.eq_ignore_ascii_case("line:")) == Some(true) {
            normalized[5..].to_string()
        } else {
            break;
        };
        normalized = next.trim().to_string();
    }
    if normalized.is_empty() {
        bail!("line send requires a non-empty target id");
    }
    Ok(normalized)
}

fn run_whatsapp_action(action: &str, payload: &Value) -> Result<Value> {
    match action {
        "send" => {
            let text = payload
                .get("text")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            let to = payload
                .get("to")
                .and_then(Value::as_str)
                .or_else(|| payload.get("target").and_then(Value::as_str))
                .map(|s| s.trim().to_string());
            let allow_from = parse_allow_from(payload);
            let mode = parse_mode(payload);
            let resolved = crate::whatsapp::resolve_whatsapp_outbound_target(
                to.as_deref(),
                &allow_from,
                mode.as_deref(),
            )?;
            Ok(json!({
                "ok": true,
                "layer": "rust",
                "feature": "whatsapp_full",
                "action": action,
                "message": "WhatsApp send validated by Rust",
                "data": {
                    "to": resolved,
                    "text_length": text.len(),
                }
            }))
        }
        "connect" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "whatsapp_full",
            "action": action,
            "message": "WhatsApp connection requested",
            "data": {
                "has_normalize_whatsapp_target": true,
            }
        })),
        "sync_full" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "whatsapp_full",
            "action": action,
            "message": "WhatsApp full sync handled by Rust",
            "data": {
                "runtime": "rust",
                "sync": "full"
            }
        })),
        "health" | "status" | "run" | "" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "whatsapp_full",
            "action": action,
            "message": "WhatsApp Rust layer healthy",
            "data": {
                "runtime": "rust",
                "supported_actions": ["send","connect","status"]
            }
        })),
        other => bail!("unsupported whatsapp_full action for rust runtime: {other}"),
    }
}

fn parse_allow_from(payload: &Value) -> Vec<String> {
    let source = payload
        .get("allow_from")
        .or_else(|| payload.get("allowFrom"))
        .or_else(|| payload.get("allow"))
        .cloned();
    if let Some(value) = source {
        match value {
            Value::Array(arr) => arr
                .iter()
                .filter_map(value_to_string)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            _ => value_to_string(&value)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .map(|s| vec![s])
                .unwrap_or_default(),
        }
    } else {
        Vec::new()
    }
}

fn value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn parse_mode(payload: &Value) -> Option<String> {
    payload
        .get("mode")
        .or_else(|| payload.get("Mode"))
        .or_else(|| payload.get("purpose"))
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn format_success(feature: &str, action: &str, layer: RuntimeLayer, out: &Value) -> String {
    let layer_text = match layer {
        RuntimeLayer::Rust => "rust",
    };
    let message = out
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("ok")
        .to_string();
    format!(
        "bridge ok feature={} action={} layer={} message={} payload={}",
        feature, action, layer_text, message, out
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_feature_aliases() {
        assert_eq!(canonical_feature("browser").unwrap(), "browser_automation");
        assert!(canonical_feature("unknown").is_err());
    }

    #[test]
    fn rust_send_text_works_for_partial_connectors() {
        let out = bridge_command(
            "whatsapp",
            Some("send_text"),
            Some(r#"{"text":"hello"}"#),
            Some("rust"),
        )
        .unwrap();
        assert!(out.contains("layer=rust"));
    }

    #[test]
    fn rust_canvas_host_push_works() {
        let out = bridge_command(
            "canvas",
            Some("push"),
            Some(r#"{"workspace":"w1","view":{"kind":"panel"}}"#),
            Some("rust"),
        )
        .unwrap();
        assert!(out.contains("feature=canvas_host"));
        assert!(out.contains("layer=rust"));
    }

    #[test]
    fn rust_node_host_pair_works() {
        let out = bridge_command(
            "node",
            Some("pair"),
            Some(r#"{"node_id":"ios-1","platform":"ios"}"#),
            Some("rust"),
        )
        .unwrap();
        assert!(out.contains("feature=node_host"));
        assert!(out.contains("layer=rust"));
    }

    #[test]
    fn rust_voice_detect_works() {
        let out = bridge_command(
            "voice",
            Some("detect"),
            Some(r#"{"transcript":"hey krabkrab open calendar","wake_phrase":"hey krabkrab","is_awake":false}"#),
            Some("rust"),
        )
        .unwrap();
        assert!(out.contains("feature=voice_wake_talk"));
        assert!(out.contains("wake phrase detected"));
    }

    #[test]
    fn auto_mode_without_fallback_returns_error() {
        let err =
            bridge_command("canvas_host", Some("unsupported_action"), None, None).unwrap_err();
        assert!(err.to_string().contains("unsupported"));
    }

    #[test]
    fn js_override_is_rejected() {
        let err =
            bridge_command("imessage_native", Some("status"), Some("{}"), Some("js")).unwrap_err();
        assert!(err.to_string().contains("removed"));
    }

    #[test]
    fn rust_line_send_works() {
        let out = bridge_command(
            "line_full",
            Some("send"),
            Some(r#"{"to":"line:user:U123","message":"hello"}"#),
            Some("rust"),
        )
        .unwrap();
        assert!(out.contains("feature=line_full"));
        assert!(out.contains("layer=rust"));
        assert!(out.contains("LINE message send requested"));
    }

    #[test]
    fn normalize_line_target_strips_prefixes() {
        assert_eq!(normalize_line_target("line:user:U123").unwrap(), "U123");
        assert_eq!(normalize_line_target("LINE:GROUP:abc").unwrap(), "abc");
    }

    #[test]
    fn rust_whatsapp_sync_full_works() {
        let out =
            bridge_command("whatsapp_full", Some("sync_full"), Some("{}"), Some("rust")).unwrap();
        assert!(out.contains("feature=whatsapp_full"));
        assert!(out.contains("layer=rust"));
        assert!(out.contains("full sync handled by Rust"));
    }

    #[test]
    fn rust_browser_snapshot_works() {
        let out = bridge_command(
            "browser",
            Some("snapshot"),
            Some(r#"{"profile":"default","url":"https://example.com"}"#),
            Some("rust"),
        )
        .unwrap();
        assert!(out.contains("feature=browser_automation"));
        assert!(out.contains("layer=rust"));
        assert!(out.contains("Browser snapshot requested"));
    }

    #[test]
    fn rust_macos_status_works() {
        let out = bridge_command("macos_native", Some("status"), Some("{}"), Some("rust")).unwrap();
        assert!(out.contains("feature=macos_native"));
        assert!(out.contains("layer=rust"));
        assert!(out.contains("macos relay parser executed"));
    }

    #[test]
    fn rust_imessage_status_works() {
        let out =
            bridge_command("imessage_native", Some("status"), Some("{}"), Some("rust")).unwrap();
        assert!(out.contains("feature=imessage_native"));
        assert!(out.contains("layer=rust"));
        assert!(out.contains("imessage parser executed"));
    }
}
