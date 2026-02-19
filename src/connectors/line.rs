use crate::common::{Message, UserId};
use crate::gateway::GatewayState;
use std::sync::Arc;

const LINE_MAX_TEXT_CHARS: usize = 5000;

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
    pub reply_token: Option<String>,
    pub user_id: String,
    pub text: String,
}

fn extract_event_text(event: &serde_json::Value) -> Option<String> {
    let event_type = event.get("type").and_then(|t| t.as_str()).unwrap_or("");
    match event_type {
        "message" => {
            let message = event.get("message")?;
            let msg_type = message.get("type").and_then(|t| t.as_str()).unwrap_or("");
            match msg_type {
                "text" => message
                    .get("text")
                    .and_then(|t| t.as_str())
                    .map(ToString::to_string),
                "sticker" => {
                    let package_id = message
                        .get("packageId")
                        .and_then(|v| v.as_str())
                        .unwrap_or("?");
                    let sticker_id = message
                        .get("stickerId")
                        .and_then(|v| v.as_str())
                        .unwrap_or("?");
                    Some(format!("[sticker] package={package_id} id={sticker_id}"))
                }
                "image" => Some("[image]".to_string()),
                "video" => Some("[video]".to_string()),
                "audio" => Some("[audio]".to_string()),
                "file" => {
                    let filename = message
                        .get("fileName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("file")
                        .to_string();
                    Some(format!("[file] {filename}"))
                }
                "location" => {
                    let title = message
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("location");
                    let lat = message
                        .get("latitude")
                        .and_then(|v| v.as_f64())
                        .unwrap_or_default();
                    let lon = message
                        .get("longitude")
                        .and_then(|v| v.as_f64())
                        .unwrap_or_default();
                    Some(format!("[location] {title} ({lat}, {lon})"))
                }
                _ => None,
            }
        }
        "postback" => {
            let data = event
                .get("postback")
                .and_then(|v| v.get("data"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let display_text = event
                .get("postback")
                .and_then(|v| v.get("params"))
                .map(|p| p.to_string())
                .unwrap_or_default();
            if display_text.is_empty() {
                Some(format!("[postback] {data}"))
            } else {
                Some(format!("[postback] {data} {display_text}"))
            }
        }
        "follow" => Some("[event] follow".to_string()),
        "join" => Some("[event] join".to_string()),
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

/// Extract text message events from a Line webhook payload.
/// Reference: https://developers.line.biz/en/reference/messaging-api/#webhook-event-objects
pub fn parse_events(payload: &serde_json::Value) -> Vec<LineEvent> {
    let mut out = Vec::new();
    let events = match payload.get("events").and_then(|e| e.as_array()) {
        Some(e) => e,
        None => return out,
    };

    for event in events {
        let text = match extract_event_text(event) {
            Some(t) => t,
            None => continue,
        };

        let reply_token = event
            .get("replyToken")
            .and_then(|r| r.as_str())
            .map(ToString::to_string);

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
                        if let Some(reply_token) = reply_token.as_ref() {
                            let client = reqwest::Client::new();
                            for chunk in split_outbound_chunks(&answer, LINE_MAX_TEXT_CHARS) {
                                if let Err(e) = crate::connectors::line_client::reply_message(
                                    &client,
                                    &token,
                                    reply_token,
                                    &chunk,
                                )
                                .await
                                {
                                    tracing::error!("[line] Failed to reply: {}", e);
                                    break;
                                }
                            }
                        } else {
                            tracing::warn!("[line] Event has no reply token; skipping reply.");
                        }
                    } else {
                        tracing::warn!("[line] LINE_CHANNEL_ACCESS_TOKEN not set; skipping reply.");
                    }
                }
                Err(e) => {
                    tracing::error!("[line] Agent error: {}", e);
                    if let (Ok(token), Some(reply_token)) =
                        (std::env::var("LINE_CHANNEL_ACCESS_TOKEN"), reply_token.as_ref())
                    {
                        let client = reqwest::Client::new();
                        for chunk in split_outbound_chunks(
                            &format!("⚠️ Agent unavailable: {e}"),
                            LINE_MAX_TEXT_CHARS,
                        ) {
                            let _ = crate::connectors::line_client::reply_message(
                                &client,
                                &token,
                                reply_token,
                                &chunk,
                            )
                            .await;
                        }
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
        assert_eq!(events[0].reply_token.as_deref(), Some("abc-reply"));
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
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].text, "[image]");
    }

    #[test]
    fn test_parse_events_follow_skipped() {
        let payload = json!({
            "events": [
                { "type": "follow", "replyToken": "reply", "source": { "userId": "U1" } }
            ]
        });
        let events = parse_events(&payload);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].text, "[event] follow");
        assert_eq!(events[0].reply_token.as_deref(), Some("reply"));
    }

    #[test]
    fn test_parse_events_postback() {
        let payload = json!({
            "events": [
                {
                    "type": "postback",
                    "replyToken": "reply-2",
                    "source": { "userId": "U2" },
                    "postback": { "data": "action=confirm" }
                }
            ]
        });
        let events = parse_events(&payload);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].text, "[postback] action=confirm");
    }

    #[test]
    fn test_split_outbound_chunks_respects_limit() {
        let chunks = split_outbound_chunks("abcdef", 3);
        assert_eq!(chunks, vec!["abc", "def"]);
    }
}
