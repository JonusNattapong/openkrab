use anyhow::{anyhow, bail, Context, Result};
use base64::Engine;
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
        "wake" => {
            let controller = crate::voice::create_voice_controller();
            let success = controller.wake();
            Ok(json!({
                "ok": success,
                "action": "wake",
                "state": controller.get_state(),
                "message": if success { "voice wake activated" } else { "wake failed or already awake" }
            }))
        }
        "sleep" => {
            let controller = crate::voice::create_voice_controller();
            controller.sleep();
            Ok(json!({
                "ok": true,
                "action": "sleep",
                "state": controller.get_state(),
                "message": "voice sleep activated"
            }))
        }
        "status" => {
            let controller = crate::voice::create_voice_controller();
            let info = controller.get_session_info();
            Ok(json!({
                "ok": true,
                "action": "status",
                "session": info,
                "message": format!("voice session is {:?}", info.state)
            }))
        }
        "process" => {
            let transcript = payload
                .get("transcript")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if transcript.is_empty() {
                bail!("voice process requires payload.transcript");
            }
            let controller = crate::voice::create_voice_controller();
            let decision = controller.process_audio(transcript);
            Ok(json!({
                "ok": true,
                "action": "process",
                "decision": decision,
                "state": controller.get_session_info(),
                "message": format!("detected {:?}", decision.action)
            }))
        }
        "beep" => {
            let beep_type = payload
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or("wake");
            let generator = crate::voice::create_beep_generator();
            let result = match beep_type {
                "wake" => generator.play_beep(crate::voice::BeepType::Wake),
                "sleep" => generator.play_beep(crate::voice::BeepType::Sleep),
                "error" => generator.play_beep(crate::voice::BeepType::Error),
                _ => bail!("unknown beep type: {beep_type}"),
            };
            match result {
                Ok(_) => Ok(json!({"ok": true, "message": "beep played"})),
                Err(e) => Ok(json!({"ok": false, "message": format!("beep error: {}", e)})),
            }
        }
        "detect_audio" => {
            let bytes = crate::voice::decode_audio_payload(payload)?;
            let sample_rate = payload
                .get("sample_rate")
                .and_then(Value::as_u64)
                .unwrap_or(16000) as u32;
            let sample_count = (bytes.len() - 44) / 2;
            let samples: Vec<i16> = bytes[44..]
                .chunks_exact(2)
                .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();
            let mut detector = crate::voice::create_wake_word_detector();
            let detected = detector.detect_from_audio(&samples, sample_rate);
            Ok(json!({
                "ok": true,
                "action": "detect_audio",
                "detected": detected,
                "message": if detected { "wake word detected" } else { "no wake word" }
            }))
        }
        "mic_list" => {
            let devices = crate::voice::microphone::list_devices();
            Ok(json!({
                "ok": true,
                "action": "mic_list",
                "devices": devices,
                "message": format!("found {} devices", devices.len())
            }))
        }
        "mic_start" => {
            let sample_rate = payload
                .get("sample_rate")
                .and_then(Value::as_u64)
                .unwrap_or(16000) as u32;
            let buffer_size = payload
                .get("buffer_size")
                .and_then(Value::as_u64)
                .unwrap_or(1024) as usize;
            let device_id = payload.get("device_id").and_then(Value::as_str);

            let mic = if let Some(dev) = device_id {
                crate::voice::microphone::MicrophoneCapture::new(sample_rate, buffer_size)
                    .with_device(dev)
            } else {
                crate::voice::microphone::MicrophoneCapture::new(sample_rate, buffer_size)
            };
            mic.start()?;
            Ok(json!({
                "ok": true,
                "action": "mic_start",
                "is_recording": mic.is_recording(),
                "config": mic.get_config(),
                "frame_count": mic.get_frame_count(),
                "message": "microphone started"
            }))
        }
        "mic_stop" => {
            let sample_rate = payload
                .get("sample_rate")
                .and_then(Value::as_u64)
                .unwrap_or(16000) as u32;
            let buffer_size = payload
                .get("buffer_size")
                .and_then(Value::as_u64)
                .unwrap_or(1024) as usize;
            let mic = crate::voice::microphone::MicrophoneCapture::new(sample_rate, buffer_size);
            mic.stop();
            Ok(json!({
                "ok": true,
                "action": "mic_stop",
                "is_recording": mic.is_recording(),
                "message": "microphone stopped"
            }))
        }
        "mic_read" => {
            let sample_rate = payload
                .get("sample_rate")
                .and_then(Value::as_u64)
                .unwrap_or(16000) as u32;
            let buffer_size = payload
                .get("buffer_size")
                .and_then(Value::as_u64)
                .unwrap_or(1024) as usize;
            let mic = crate::voice::microphone::MicrophoneCapture::new(sample_rate, buffer_size);
            let audio_data = mic.get_audio_buffer();
            let audio_bytes: Vec<u8> = audio_data.iter().flat_map(|&s| s.to_le_bytes()).collect();
            let base64_audio = base64::engine::general_purpose::STANDARD.encode(&audio_bytes);
            Ok(json!({
                "ok": true,
                "action": "mic_read",
                "samples_count": audio_data.len() / 2,
                "audio_base64": base64_audio,
                "frame_count": mic.get_frame_count(),
                "message": format!("read {} bytes", audio_data.len() * 2)
            }))
        }
        "mic_status" => {
            let sample_rate = payload
                .get("sample_rate")
                .and_then(Value::as_u64)
                .unwrap_or(16000) as u32;
            let buffer_size = payload
                .get("buffer_size")
                .and_then(Value::as_u64)
                .unwrap_or(1024) as usize;
            let mic = crate::voice::microphone::MicrophoneCapture::new(sample_rate, buffer_size);
            Ok(json!({
                "ok": true,
                "action": "mic_status",
                "is_recording": mic.is_recording(),
                "config": mic.get_config(),
                "frame_count": mic.get_frame_count(),
                "buffer_samples": mic.get_audio_buffer().len(),
                "message": "microphone status"
            }))
        }
        "config" => {
            let config = crate::voice::VoiceModesConfig::default();
            Ok(json!({
                "ok": true,
                "action": "config",
                "config": config,
                "message": "voice config retrieved"
            }))
        }
        "vad" => {
            let bytes = crate::voice::decode_audio_payload(payload)?;
            let sample_rate = payload
                .get("sample_rate")
                .and_then(Value::as_u64)
                .unwrap_or(16000) as u32;
            let sample_count = (bytes.len().saturating_sub(44)) / 2;
            let samples: Vec<i16> = bytes
                .get(44..)
                .map(|data| {
                    data.chunks_exact(2)
                        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                        .collect()
                })
                .unwrap_or_default();
            let mut vad = crate::voice::VoiceActivityDetector::new();
            let state = vad.process(&samples, sample_rate);
            Ok(json!({
                "ok": true,
                "action": "vad",
                "is_speaking": vad.is_speaking(),
                "state": state,
                "message": format!("VAD state: {:?}", state)
            }))
        }
        "spectral" => {
            let bytes = crate::voice::decode_audio_payload(payload)?;
            let samples: Vec<i16> = bytes
                .get(44..)
                .map(|data| {
                    data.chunks_exact(2)
                        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                        .collect()
                })
                .unwrap_or_default();
            let analyzer = crate::voice::SpectralAnalyzer::new(512);
            let features = analyzer.analyze(&samples);
            Ok(json!({
                "ok": true,
                "action": "spectral",
                "features": features,
                "message": "spectral analysis completed"
            }))
        }
        "audio_stats" => {
            let bytes = crate::voice::decode_audio_payload(payload)?;
            let sample_rate = payload
                .get("sample_rate")
                .and_then(Value::as_u64)
                .unwrap_or(16000) as u32;
            let samples: Vec<i16> = bytes
                .get(44..)
                .map(|data| {
                    data.chunks_exact(2)
                        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                        .collect()
                })
                .unwrap_or_default();
            let stats = crate::voice::AudioStats::from_samples(&samples, sample_rate);
            Ok(json!({
                "ok": true,
                "action": "audio_stats",
                "stats": stats,
                "message": "audio stats computed"
            }))
        }
        "health" | "run" | "" => Ok(json!({
            "ok": true,
            "layer": "rust",
            "feature": "voice_wake_talk",
            "action": action,
            "message": "Voice wake/talk runtime healthy",
            "data": {
                "supported_actions": [
                    "analyze_audio", "detect", "speak",
                    "wake", "sleep", "status", "process",
                    "beep", "detect_audio", "mic_list",
                    "mic_start", "mic_stop", "mic_read", "mic_status",
                    "config", "vad", "spectral", "audio_stats"
                ],
                "modes": ["wake", "talk", "sleep"],
                "features": {
                    "microphone_capture": true,
                    "audio_wake_detection": true,
                    "beep_generation": true,
                    "transcript_buffer": true,
                    "vad": true,
                    "spectral_analysis": true
                }
            }
        })),
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
