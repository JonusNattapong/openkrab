use serde_json::json;
use anyhow::Result;
use reqwest::Client;
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

/// Async send shim for Telegram Bot API.
/// `token` should be the bot token (no `bot` prefix needed here).
pub async fn send_message(client: &Client, token: &str, chat_id: &str, text: &str, reply_to_message_id: Option<i64>) -> Result<serde_json::Value> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let payload = build_telegram_http_payload(chat_id, text, reply_to_message_id);
    let resp = client.post(&url).json(&payload).send().await?;
    let v: serde_json::Value = resp.json().await?;
    Ok(v)
}

/// Async update fetch shim for Telegram Bot API.
pub async fn get_updates(client: &Client, token: &str, offset: Option<i64>, timeout: Option<u64>) -> Result<serde_json::Value> {
    let url = format!("https://api.telegram.org/bot{}/getUpdates", token);
    let mut payload = json!({});
    if let Some(o) = offset {
        payload["offset"] = json!(o);
    }
    if let Some(t) = timeout {
        payload["timeout"] = json!(t);
    }
    let resp = client.post(&url).json(&payload).send().await?;
    let v: serde_json::Value = resp.json().await?;
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
