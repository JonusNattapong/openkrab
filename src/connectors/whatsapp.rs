use crate::common::{Message, UserId};
use crate::gateway::GatewayState;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

pub mod signature {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    pub fn validate_whatsapp_signature(body: &str, signature_header: &str, app_secret: &str) -> bool {
        type HmacSha256 = Hmac<Sha256>;

        if app_secret.trim().is_empty() {
            return false;
        }

        let received = match signature_header.strip_prefix("sha256=") {
            Some(value) if !value.trim().is_empty() => value.trim().to_ascii_lowercase(),
            _ => return false,
        };

        let mut mac = match HmacSha256::new_from_slice(app_secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };
        mac.update(body.as_bytes());
        let expected = hex::encode(mac.finalize().into_bytes());

        constant_time_compare(expected.as_bytes(), received.as_bytes())
    }

    fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut diff = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            diff |= x ^ y;
        }

        diff == 0
    }
}

const WHATSAPP_MAX_TEXT_CHARS: usize = 4096;

pub fn format_outbound(text: &str) -> String {
    format!("[whatsapp] {text}")
}

pub fn normalize_inbound(text: &str, from: &str) -> Message {
    Message {
        id: format!("wa:{from}"),
        text: text.to_string(),
        from: Some(UserId(format!("wa:{from}"))),
    }
}

/// Parsed WhatsApp inbound text message.
#[derive(Debug, Clone)]
pub struct WhatsAppMessage {
    pub from: String,
    pub message_id: String,
    pub text: String,
    pub phone_number_id: String,
}

/// Inbound event coming from a WhatsApp Web bridge (e.g. Baileys sidecar).
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppWebInbound {
    pub from: String,
    pub text: String,
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub message_id: Option<String>,
}

/// Outbound event to be consumed by a WhatsApp Web bridge.
#[derive(Debug, Clone, Serialize)]
pub struct WhatsAppWebOutbound {
    #[serde(rename = "type")]
    pub event_type: String,
    pub to: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
}

fn parse_web_bridge_inbound(payload: &serde_json::Value) -> Option<WhatsAppWebInbound> {
    let event_type = payload
        .get("type")
        .and_then(|v| v.as_str())
        .or_else(|| payload.get("event").and_then(|v| v.as_str()))
        .unwrap_or("");
    if event_type != "message" {
        return None;
    }

    let from = payload.get("from").and_then(|v| v.as_str())?.trim().to_string();
    let text = payload.get("text").and_then(|v| v.as_str())?.trim().to_string();
    if from.is_empty() || text.is_empty() {
        return None;
    }

    Some(WhatsAppWebInbound {
        from,
        text,
        chat_id: payload.get("chat_id").and_then(|v| v.as_str()).map(ToString::to_string),
        message_id: payload
            .get("message_id")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
    })
}

/// Handle a single WhatsApp Web bridge JSON event and return the outbound bridge message.
pub async fn handle_web_bridge_event(
    state: Arc<GatewayState>,
    payload: serde_json::Value,
) -> serde_json::Value {
    let inbound = match parse_web_bridge_inbound(&payload) {
        Some(v) => v,
        None => {
            return json!({
                "type": "error",
                "error": "invalid-or-unsupported-whatsapp-web-event"
            })
        }
    };

    match state.agent.answer(&inbound.text).await {
        Ok(answer) => {
            let outbound = WhatsAppWebOutbound {
                event_type: "send".to_string(),
                to: inbound.from,
                text: answer,
                reply_to: inbound.message_id,
            };
            serde_json::to_value(outbound).unwrap_or_else(|_| {
                json!({
                    "type": "error",
                    "error": "failed-to-serialize-whatsapp-web-outbound"
                })
            })
        }
        Err(e) => json!({
            "type": "error",
            "error": format!("agent-error: {e}")
        }),
    }
}

fn extract_message_text(msg: &serde_json::Value) -> Option<String> {
    let msg_type = msg.get("type").and_then(|t| t.as_str()).unwrap_or("");
    match msg_type {
        "text" => msg
            .get("text")
            .and_then(|t| t.get("body"))
            .and_then(|b| b.as_str())
            .map(ToString::to_string),
        "button" => msg
            .get("button")
            .and_then(|b| b.get("text"))
            .and_then(|t| t.as_str())
            .map(|t| format!("[button] {t}")),
        "interactive" => {
            let interactive = msg.get("interactive")?;
            let interactive_type = interactive.get("type").and_then(|t| t.as_str()).unwrap_or("");
            match interactive_type {
                "button_reply" => {
                    let id = interactive
                        .get("button_reply")
                        .and_then(|b| b.get("id"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let title = interactive
                        .get("button_reply")
                        .and_then(|b| b.get("title"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    Some(format!("[interactive:button_reply] {title} ({id})"))
                }
                "list_reply" => {
                    let id = interactive
                        .get("list_reply")
                        .and_then(|b| b.get("id"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let title = interactive
                        .get("list_reply")
                        .and_then(|b| b.get("title"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    Some(format!("[interactive:list_reply] {title} ({id})"))
                }
                _ => Some("[interactive]".to_string()),
            }
        }
        "image" => {
            let caption = msg
                .get("image")
                .and_then(|i| i.get("caption"))
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            Some(if caption.is_empty() {
                "[image]".to_string()
            } else {
                format!("[image] {caption}")
            })
        }
        "video" => {
            let caption = msg
                .get("video")
                .and_then(|i| i.get("caption"))
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            Some(if caption.is_empty() {
                "[video]".to_string()
            } else {
                format!("[video] {caption}")
            })
        }
        "audio" => Some("[audio]".to_string()),
        "sticker" => Some("[sticker]".to_string()),
        "document" => {
            let filename = msg
                .get("document")
                .and_then(|d| d.get("filename"))
                .and_then(|f| f.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            Some(if filename.is_empty() {
                "[document]".to_string()
            } else {
                format!("[document] {filename}")
            })
        }
        "reaction" => {
            let emoji = msg
                .get("reaction")
                .and_then(|r| r.get("emoji"))
                .and_then(|e| e.as_str())
                .unwrap_or("?");
            Some(format!("[reaction] {emoji}"))
        }
        "location" => {
            let lat = msg
                .get("location")
                .and_then(|l| l.get("latitude"))
                .and_then(|v| v.as_f64())
                .unwrap_or_default();
            let lon = msg
                .get("location")
                .and_then(|l| l.get("longitude"))
                .and_then(|v| v.as_f64())
                .unwrap_or_default();
            let name = msg
                .get("location")
                .and_then(|l| l.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("location");
            Some(format!("[location] {name} ({lat}, {lon})"))
        }
        "contacts" => {
            let count = msg
                .get("contacts")
                .and_then(|c| c.as_array())
                .map(|c| c.len())
                .unwrap_or(0);
            Some(format!("[contacts] {count}"))
        }
        _ => None,
    }
}

fn split_outbound_chunks(text: &str, max_chars: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }

    if text.chars().count() <= max_chars {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut current_len = 0usize;

    for ch in text.chars() {
        if current_len >= max_chars {
            chunks.push(current);
            current = String::new();
            current_len = 0;
        }

        current.push(ch);
        current_len += 1;
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

/// Parse WhatsApp Cloud API webhook payload and extract text messages.
/// Reference: https://developers.facebook.com/docs/whatsapp/cloud-api/webhooks/payload-examples
pub fn parse_messages(payload: &serde_json::Value) -> Vec<WhatsAppMessage> {
    let mut out = Vec::new();

    let entries = match payload.get("entry").and_then(|e| e.as_array()) {
        Some(e) => e,
        None => return out,
    };

    for entry in entries {
        let changes = match entry.get("changes").and_then(|c| c.as_array()) {
            Some(c) => c,
            None => continue,
        };

        for change in changes {
            let value = match change.get("value") {
                Some(v) => v,
                None => continue,
            };

            // Extract the phone_number_id from metadata
            let phone_number_id = value
                .get("metadata")
                .and_then(|m| m.get("phone_number_id"))
                .and_then(|p| p.as_str())
                .unwrap_or("")
                .to_string();

            let messages = match value.get("messages").and_then(|m| m.as_array()) {
                Some(m) => m,
                None => continue,
            };

            for msg in messages {
                let text = match extract_message_text(msg) {
                    Some(t) => t,
                    None => continue,
                };

                let from = msg
                    .get("from")
                    .and_then(|f| f.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let message_id = msg
                    .get("id")
                    .and_then(|i| i.as_str())
                    .unwrap_or("")
                    .to_string();

                out.push(WhatsAppMessage {
                    from,
                    message_id,
                    text,
                    phone_number_id: phone_number_id.clone(),
                });
            }
        }
    }

    out
}

/// Handle incoming WhatsApp webhook events: parse → agent → reply.
pub async fn handle_events(state: Arc<GatewayState>, payload: serde_json::Value) {
    let messages = parse_messages(&payload);

    for msg in messages {
        let state_clone = state.clone();
        let from = msg.from.clone();
        let text = msg.text.clone();
        let message_id = msg.message_id.clone();
        let phone_number_id = msg.phone_number_id.clone();

        let normalized = normalize_inbound(&text, &from);
        tracing::info!("[whatsapp] Received: {:?}", normalized);

        tokio::spawn(async move {
            let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN").unwrap_or_default();
            let pid = if phone_number_id.is_empty() {
                std::env::var("WHATSAPP_PHONE_NUMBER_ID").unwrap_or_default()
            } else {
                phone_number_id
            };

            if access_token.is_empty() || pid.is_empty() {
                tracing::warn!("[whatsapp] WHATSAPP_ACCESS_TOKEN or WHATSAPP_PHONE_NUMBER_ID not set.");
                return;
            }

            let client = reqwest::Client::new();

            if !message_id.is_empty() {
                let _ = crate::connectors::whatsapp_client::mark_as_read(
                    &client,
                    &access_token,
                    &pid,
                    &message_id,
                )
                .await;
            }

            match state_clone.agent.answer(&text).await {
                Ok(answer) => {
                    for chunk in split_outbound_chunks(&answer, WHATSAPP_MAX_TEXT_CHARS) {
                        if let Err(e) = crate::connectors::whatsapp_client::send_message(
                            &client,
                            &access_token,
                            &pid,
                            &from,
                            &chunk,
                        )
                        .await
                        {
                            tracing::error!("[whatsapp] Failed to send reply: {}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("[whatsapp] Agent error: {}", e);
                    for chunk in split_outbound_chunks(
                        &format!("⚠️ Agent unavailable: {e}"),
                        WHATSAPP_MAX_TEXT_CHARS,
                    ) {
                        let _ = crate::connectors::whatsapp_client::send_message(
                            &client,
                            &access_token,
                            &pid,
                            &from,
                            &chunk,
                        )
                        .await;
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_format_outbound() {
        assert_eq!(format_outbound("hi"), "[whatsapp] hi");
    }

    #[test]
    fn test_normalize_inbound() {
        let msg = normalize_inbound("hello", "+66812345678");
        assert_eq!(msg.id, "wa:+66812345678");
        assert_eq!(msg.text, "hello");
    }

    #[test]
    fn test_parse_messages_text() {
        let payload = json!({
            "object": "whatsapp_business_account",
            "entry": [{
                "changes": [{
                    "value": {
                        "messaging_product": "whatsapp",
                        "metadata": {
                            "display_phone_number": "15550123456",
                            "phone_number_id": "123456789"
                        },
                        "messages": [{
                            "from": "+66812345678",
                            "id": "wamid.abc123",
                            "type": "text",
                            "text": { "body": "Hello Bot!" }
                        }]
                    },
                    "field": "messages"
                }]
            }]
        });

        let msgs = parse_messages(&payload);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].from, "+66812345678");
        assert_eq!(msgs[0].text, "Hello Bot!");
        assert_eq!(msgs[0].message_id, "wamid.abc123");
        assert_eq!(msgs[0].phone_number_id, "123456789");
    }

    #[test]
    fn test_parse_messages_non_text_skipped() {
        let payload = json!({
            "entry": [{
                "changes": [{
                    "value": {
                        "metadata": { "phone_number_id": "pid" },
                        "messages": [{
                            "from": "+1",
                            "id": "abc",
                            "type": "image",
                            "image": {}
                        }]
                    }
                }]
            }]
        });
        let msgs = parse_messages(&payload);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].text, "[image]");
    }

    #[test]
    fn test_parse_messages_interactive_button_reply() {
        let payload = json!({
            "entry": [{
                "changes": [{
                    "value": {
                        "metadata": { "phone_number_id": "pid" },
                        "messages": [{
                            "from": "+1",
                            "id": "abc",
                            "type": "interactive",
                            "interactive": {
                                "type": "button_reply",
                                "button_reply": { "id": "yes", "title": "Yes" }
                            }
                        }]
                    }
                }]
            }]
        });

        let msgs = parse_messages(&payload);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].text, "[interactive:button_reply] Yes (yes)");
    }

    #[test]
    fn test_split_outbound_chunks_respects_limit() {
        let chunks = split_outbound_chunks("abcdef", 2);
        assert_eq!(chunks, vec!["ab", "cd", "ef"]);
    }

    #[test]
    fn test_validate_whatsapp_signature() {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let body = r#"{"object":"whatsapp_business_account"}"#;
        let secret = "secret";
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        let sig = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

        assert!(signature::validate_whatsapp_signature(body, &sig, secret));
        assert!(!signature::validate_whatsapp_signature(body, "sha256=deadbeef", secret));
        assert!(!signature::validate_whatsapp_signature(body, &sig, "wrong-secret"));
    }

    #[test]
    fn test_parse_web_bridge_inbound_message() {
        let payload = json!({
            "type": "message",
            "from": "12025550123",
            "text": "hello",
            "message_id": "mid-1"
        });
        let parsed = parse_web_bridge_inbound(&payload).expect("inbound should parse");
        assert_eq!(parsed.from, "12025550123");
        assert_eq!(parsed.text, "hello");
        assert_eq!(parsed.message_id.as_deref(), Some("mid-1"));
    }

    #[test]
    fn test_parse_web_bridge_inbound_rejects_non_message() {
        let payload = json!({ "type": "presence", "from": "x", "text": "y" });
        assert!(parse_web_bridge_inbound(&payload).is_none());
    }
}
