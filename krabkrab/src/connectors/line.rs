use crate::common::{Message, UserId};
use crate::gateway::GatewayState;
use std::sync::Arc;

pub mod signature {
    use base64::Engine;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    pub fn validate_line_signature(body: &str, signature: &str, channel_secret: &str) -> bool {
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes()).expect("HMAC can take key of any size");
        mac.update(body.as_bytes());
        let result = mac.finalize();

        let expected = base64::engine::general_purpose::STANDARD.encode(result.into_bytes());

        constant_time_compare(&expected, signature)
    }

    fn constant_time_compare(a: &str, b: &str) -> bool {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        if a_bytes.len() != b_bytes.len() {
            return false;
        }

        let mut result = 0u8;
        for (x, y) in a_bytes.iter().zip(b_bytes.iter()) {
            result |= x ^ y;
        }

        result == 0
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn compute_signature(body: &str, channel_secret: &str) -> String {
            use hmac::{Hmac, Mac};
            use sha2::Sha256;
            type HmacSha256 = Hmac<Sha256>;
            let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes()).unwrap();
            mac.update(body.as_bytes());
            let result = mac.finalize();
            base64::engine::general_purpose::STANDARD.encode(result.into_bytes())
        }

        #[test]
        fn test_valid_signature() {
            let body = r#"{"events":[]}"#;
            let secret = "test_secret";
            let signature = compute_signature(body, secret);
            assert!(validate_line_signature(body, &signature, secret));
        }

        #[test]
        fn test_invalid_signature() {
            let body = r#"{"events":[]}"#;
            let secret = "test_secret";
            assert!(!validate_line_signature(body, "invalid_signature", secret));
        }

        #[test]
        fn test_wrong_secret() {
            let body = r#"{"events":[]}"#;
            let signature = compute_signature(body, "correct_secret");
            assert!(!validate_line_signature(body, &signature, "wrong_secret"));
        }

        #[test]
        fn test_empty_secret() {
            let body = r#"{"events":[]}"#;
            assert!(!validate_line_signature(body, "any_signature", ""));
        }

        #[test]
        fn test_tampered_body() {
            let body = r#"{"events":[]}"#;
            let secret = "test_secret";
            let signature = compute_signature(body, secret);
            let tampered = r#"{"events":[{"type":"message"}]}"#;
            assert!(!validate_line_signature(tampered, &signature, secret));
        }
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[line] {text}")
}

pub fn normalize_inbound(text: &str, user_id: &str) -> Message {
    Message {
        id: format!("line:{user_id}"),
        text: text.to_string(),
        from: Some(UserId(format!("line:{user_id}"))),
    }
}

/// Parsed Line webhook event (text messages only).
#[derive(Debug, Clone)]
pub struct LineEvent {
    pub reply_token: String,
    pub user_id: String,
    pub text: String,
}

/// Extract text message events from a Line webhook payload.
/// Reference: https://developers.line.biz/en/reference/messaging-api/#webhook-event-objects
pub fn parse_events(payload: &serde_json::Value) -> Vec<LineEvent> {
    let mut out = Vec::new();
    let events = match payload.get("events").and_then(|e| e.as_array()) {
        Some(e) => e,
        None => return out,
    };

    for event in events {
        let event_type = match event.get("type").and_then(|t| t.as_str()) {
            Some(t) => t,
            None => continue,
        };

        if event_type != "message" {
            continue;
        }

        let message = match event.get("message") {
            Some(m) => m,
            None => continue,
        };

        let msg_type = message.get("type").and_then(|t| t.as_str()).unwrap_or("");
        if msg_type != "text" {
            continue;
        }

        let text = match message.get("text").and_then(|t| t.as_str()) {
            Some(t) => t.to_string(),
            None => continue,
        };

        let reply_token = match event.get("replyToken").and_then(|r| r.as_str()) {
            Some(r) => r.to_string(),
            None => continue,
        };

        let user_id = event
            .get("source")
            .and_then(|s| s.get("userId"))
            .and_then(|u| u.as_str())
            .unwrap_or("unknown")
            .to_string();

        out.push(LineEvent {
            reply_token,
            user_id,
            text,
        });
    }

    out
}

/// Handle incoming Line webhook events: parse → agent → reply.
pub async fn handle_events(state: Arc<GatewayState>, payload: serde_json::Value) {
    let events = parse_events(&payload);

    for event in events {
        let state_clone = state.clone();
        let reply_token = event.reply_token.clone();
        let user_id = event.user_id.clone();
        let text = event.text.clone();

        let normalized = normalize_inbound(&text, &user_id);
        tracing::info!("[line] Received: {:?}", normalized);

        tokio::spawn(async move {
            match state_clone.agent.answer(&text).await {
                Ok(answer) => {
                    if let Ok(token) = std::env::var("LINE_CHANNEL_ACCESS_TOKEN") {
                        let client = reqwest::Client::new();
                        if let Err(e) = crate::connectors::line_client::reply_message(
                            &client,
                            &token,
                            &reply_token,
                            &answer,
                        )
                        .await
                        {
                            tracing::error!("[line] Failed to reply: {}", e);
                        }
                    } else {
                        tracing::warn!("[line] LINE_CHANNEL_ACCESS_TOKEN not set; skipping reply.");
                    }
                }
                Err(e) => {
                    tracing::error!("[line] Agent error: {}", e);
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
        assert_eq!(format_outbound("hi"), "[line] hi");
    }

    #[test]
    fn test_normalize_inbound() {
        let msg = normalize_inbound("hello", "U12345");
        assert_eq!(msg.id, "line:U12345");
        assert_eq!(msg.text, "hello");
    }

    #[test]
    fn test_parse_events_text() {
        let payload = json!({
            "destination": "U123",
            "events": [
                {
                    "type": "message",
                    "replyToken": "abc-reply",
                    "source": { "type": "user", "userId": "U456" },
                    "message": { "type": "text", "id": "1", "text": "Hello Bot" }
                }
            ]
        });
        let events = parse_events(&payload);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].text, "Hello Bot");
        assert_eq!(events[0].reply_token, "abc-reply");
        assert_eq!(events[0].user_id, "U456");
    }

    #[test]
    fn test_parse_events_non_text_skipped() {
        let payload = json!({
            "events": [
                {
                    "type": "message",
                    "replyToken": "abc",
                    "source": { "userId": "U1" },
                    "message": { "type": "image", "id": "2" }
                }
            ]
        });
        let events = parse_events(&payload);
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_parse_events_follow_skipped() {
        let payload = json!({
            "events": [
                { "type": "follow", "source": { "userId": "U1" } }
            ]
        });
        let events = parse_events(&payload);
        assert_eq!(events.len(), 0);
    }
}
