use serde_json::json;
use anyhow::Result;
use reqwest::blocking::Client;
use std::time::Duration;

/// Build a JSON payload for Telegram `sendMessage` API
pub fn build_telegram_http_payload(chat_id: &str, text: &str, reply_to_message_id: Option<i64>) -> serde_json::Value {
    let mut payload = json!({
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "MarkdownV2",
    });
    if let Some(reply_id) = reply_to_message_id {
        payload["reply_to_message_id"] = json!(reply_id);
    }
    payload
}

/// Blocking send shim for Telegram Bot API.
/// `token` should be the bot token (no `bot` prefix needed here).
pub fn send_message(token: &str, chat_id: &str, text: &str, reply_to_message_id: Option<i64>) -> Result<serde_json::Value> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let payload = build_telegram_http_payload(chat_id, text, reply_to_message_id);
    let resp = client.post(&url).json(&payload).send()?;
    let v: serde_json::Value = resp.json()?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telegram_payload_has_chat_and_text() {
        let p = build_telegram_http_payload("-12345", "hello", None);
        assert_eq!(p["chat_id"], "-12345");
        assert_eq!(p["text"], "hello");
        assert_eq!(p["parse_mode"], "MarkdownV2");
        assert!(p.get("reply_to_message_id").is_none());
    }

    #[test]
    fn telegram_payload_includes_reply_id() {
        let p = build_telegram_http_payload("-12345", "reply", Some(42));
        assert_eq!(p["reply_to_message_id"], 42);
    }
}
