//! BlueBubbles reaction handling.
//! Ported from openkrab/extensions/bluebubbles/src/reactions.ts

use serde::{Deserialize, Serialize};

use super::types::{build_api_url, BlueBubblesSendResult, DEFAULT_TIMEOUT_MS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionOptions {
    pub message_guid: String,
    pub chat_guid: String,
    pub reaction: String,
    pub action: String,
}

pub fn send_reaction(
    base_url: &str,
    password: &str,
    message_guid: &str,
    chat_guid: &str,
    reaction: &str,
    timeout_ms: Option<u64>,
) -> Result<BlueBubblesSendResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/message/react", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "chatGuid": chat_guid,
        "messageGuid": message_guid,
        "reaction": reaction,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().unwrap_or_default();
        return Err(format!("reaction failed ({}): {}", status, error_text));
    }

    let json: serde_json::Value = response.json().map_err(|e| format!("parse error: {}", e))?;

    let message_id = json
        .get("data")
        .and_then(|d| d.get("guid"))
        .and_then(|g| g.as_str())
        .unwrap_or("ok")
        .to_string();

    Ok(BlueBubblesSendResult { message_id })
}

pub fn remove_reaction(
    base_url: &str,
    password: &str,
    message_guid: &str,
    chat_guid: &str,
    timeout_ms: Option<u64>,
) -> Result<BlueBubblesSendResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/message/unreact", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "chatGuid": chat_guid,
        "messageGuid": message_guid,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().unwrap_or_default();
        return Err(format!("unreact failed ({}): {}", status, error_text));
    }

    Ok(BlueBubblesSendResult {
        message_id: "ok".to_string(),
    })
}

pub fn map_emoji_to_reaction(emoji: &str) -> Option<String> {
    let emoji_map: std::collections::HashMap<&str, &str> = [
        ("👍", "com.apple.MobileSMS.like"),
        ("👎", "com.apple.MobileSMS.dislike"),
        ("❤️", "com.apple.MobileSMS.love"),
        ("😊", "com.apple.MobileSMS.haha"),
        ("😲", "com.apple.MobileSMS.wwow"),
        ("😢", "com.apple.MobileSMS.sorry"),
        ("🔥", "com.apple.MobileSMS.emphasize"),
        ("🔼", "com.apple.MobileSMS.question"),
    ]
    .into_iter()
    .collect();

    emoji_map.get(emoji).map(|s| s.to_string())
}

/// Parse a reaction action string into (action_type, optional_emoji).
/// Returns owned Strings to avoid lifetime issues with the local `action` variable.
pub fn parse_reaction_action(action: &str) -> (String, Option<String>) {
    let action_lower = action.to_lowercase();

    if action_lower.contains("remove") || action_lower.contains("un") || action_lower.contains("delete") {
        return ("remove".to_string(), None);
    }

    if let Some(emoji) = action_lower.strip_prefix("react:") {
        return ("add".to_string(), Some(emoji.to_string()));
    }

    if action_lower.starts_with('[') {
        if let Some(end) = action_lower.find(']') {
            return ("add".to_string(), Some(action_lower[1..end].to_string()));
        }
    }

    ("add".to_string(), Some(action_lower))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_emoji_to_reaction() {
        assert_eq!(
            map_emoji_to_reaction("👍"),
            Some("com.apple.MobileSMS.like".to_string())
        );
        assert_eq!(
            map_emoji_to_reaction("❤️"),
            Some("com.apple.MobileSMS.love".to_string())
        );
        assert_eq!(map_emoji_to_reaction("unknown"), None);
    }

    #[test]
    fn test_parse_reaction_action() {
        let (action, emoji) = parse_reaction_action("react:👍");
        assert_eq!(action, "add");
        assert_eq!(emoji.as_deref(), Some("👍"));

        let (action2, _) = parse_reaction_action("remove");
        assert_eq!(action2, "remove");
    }
}
