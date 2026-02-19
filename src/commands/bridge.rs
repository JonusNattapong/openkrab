use anyhow::{anyhow, bail, Context, Result};
use serde_json::{json, Value};

use crate::config::{AppConfig, RuntimeLayer};
use crate::process::{exec, ExecOptions};

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
        Err(primary_err) => {
            let fallback = match layer_override {
                Some(_) => None,
                None => route.fallback,
            };
            if let Some(fallback_layer) = fallback {
                let fallback_out = run_layer(fallback_layer, feature, action, &payload).with_context(
                    || {
                        format!(
                            "primary layer failed ({primary:?}): {primary_err}; fallback failed ({fallback_layer:?})"
                        )
                    },
                )?;
                return Ok(format_success(
                    feature,
                    action,
                    fallback_layer,
                    &fallback_out,
                ));
            }
            Err(primary_err)
        }
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
        "js" | "javascript" => RuntimeLayer::Js,
        other => bail!("unsupported layer override: {other}"),
    };
    Ok(Some(parsed))
}

fn run_layer(layer: RuntimeLayer, feature: &str, action: &str, payload: &Value) -> Result<Value> {
    match layer {
        RuntimeLayer::Rust => run_rust_feature(feature, action, payload),
        RuntimeLayer::Js => run_js_bridge(feature, action, payload),
    }
}

fn run_rust_feature(feature: &str, action: &str, payload: &Value) -> Result<Value> {
    match feature {
        "canvas_host" => crate::canvas_host::run_action(action, payload),
        "voice_wake_talk" => run_voice_action(action, payload),
        "node_host" => crate::node_host::run_action(action, payload),
        "whatsapp_full" | "line_full" if action == "send_text" => {
            let text = payload
                .get("text")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            if text.is_empty() {
                bail!("rust layer requires payload.text for send_text");
            }
            Ok(json!({ "ok": true, "layer": "rust", "message": format!("sent text: {text}") }))
        }
        "whatsapp_full" | "line_full" => {
            bail!("rust runtime has partial support for {feature}; action '{action}' requires JS bridge")
        }
        _ => bail!("feature '{feature}' is JS-backed in this build"),
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

fn run_js_bridge(feature: &str, action: &str, payload: &Value) -> Result<Value> {
    let cmd = std::env::var("KRABKRAB_JS_BRIDGE_CMD").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "node .claude/js-bridge.mjs".to_string()
        } else {
            "node .claude/js-bridge.mjs".to_string()
        }
    });

    let request = json!({
        "feature": feature,
        "action": action,
        "payload": payload,
    });

    let mut opts = ExecOptions::default();
    opts.env
        .push(("KRABKRAB_BRIDGE_REQUEST".to_string(), request.to_string()));

    let out = exec(&cmd, &opts)?;
    if !out.success {
        bail!(
            "js bridge command failed (exit {}): {}",
            out.exit_code,
            out.stderr.trim()
        );
    }
    let stdout = out.stdout.trim();
    if stdout.is_empty() {
        bail!("js bridge command returned empty stdout");
    }
    let val = parse_bridge_json(stdout)
        .with_context(|| format!("js bridge stdout is not valid JSON: {stdout}"))?;
    Ok(val)
}

fn parse_bridge_json(stdout: &str) -> Result<Value> {
    if let Ok(v) = serde_json::from_str::<Value>(stdout) {
        return Ok(v);
    }

    let mut candidates = Vec::new();
    candidates.push(stdout.to_string());
    candidates.push(stdout.replace("\\\"", "\""));

    if let (Some(start), Some(end)) = (stdout.find('{'), stdout.rfind('}')) {
        if end > start {
            candidates.push(stdout[start..=end].to_string());
            candidates.push(stdout[start..=end].replace("\\\"", "\""));
        }
    }

    for candidate in candidates {
        if let Ok(v) = serde_json::from_str::<Value>(candidate.trim()) {
            return Ok(v);
        }
    }

    bail!("unable to parse bridge json")
}

fn format_success(feature: &str, action: &str, layer: RuntimeLayer, out: &Value) -> String {
    let layer_text = match layer {
        RuntimeLayer::Rust => "rust",
        RuntimeLayer::Js => "js",
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
    fn auto_mode_falls_back_to_js() {
        let key = "KRABKRAB_JS_BRIDGE_CMD";
        let previous = std::env::var(key).ok();
        #[cfg(target_os = "windows")]
        let cmd = "echo {\"ok\":true,\"layer\":\"js\",\"message\":\"fallback\"}";
        #[cfg(not(target_os = "windows"))]
        let cmd = "printf '{\"ok\":true,\"layer\":\"js\",\"message\":\"fallback\"}'";
        std::env::set_var(key, cmd);

        let out = bridge_command("whatsapp_full", Some("sync_full"), None, None).unwrap();
        assert!(out.contains("layer=js"));
        assert!(out.contains("feature=whatsapp_full"));

        if let Some(value) = previous {
            std::env::set_var(key, value);
        } else {
            std::env::remove_var(key);
        }
    }
}
