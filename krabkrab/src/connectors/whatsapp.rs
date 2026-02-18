use crate::common::{Message, UserId};
use crate::gateway::GatewayState;
use std::sync::Arc;

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
                let msg_type = msg.get("type").and_then(|t| t.as_str()).unwrap_or("");
                if msg_type != "text" {
                    continue;
                }

                let text = match msg
                    .get("text")
                    .and_then(|t| t.get("body"))
                    .and_then(|b| b.as_str())
                {
                    Some(t) => t.to_string(),
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

            // Mark message as read
            let _ = crate::connectors::whatsapp_client::mark_as_read(
                &client,
                &access_token,
                &pid,
                &message_id,
            )
            .await;

            match state_clone.agent.answer(&text).await {
                Ok(answer) => {
                    if let Err(e) = crate::connectors::whatsapp_client::send_message(
                        &client,
                        &access_token,
                        &pid,
                        &from,
                        &answer,
                    )
                    .await
                    {
                        tracing::error!("[whatsapp] Failed to send reply: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("[whatsapp] Agent error: {}", e);
                    let _ = crate::connectors::whatsapp_client::send_message(
                        &client,
                        &access_token,
                        &pid,
                        &from,
                        &format!("⚠️ Agent unavailable: {e}"),
                    )
                    .await;
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
        assert_eq!(msgs.len(), 0);
    }
}
