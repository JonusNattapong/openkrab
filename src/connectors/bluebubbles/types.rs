//! BlueBubbles type definitions.
//! Ported from openclaw/extensions/bluebubbles/src/types.ts

use serde::{Deserialize, Serialize};

pub const DEFAULT_TIMEOUT_MS: u64 = 10_000;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlueBubblesGroupConfig {
    pub require_mention: Option<bool>,
    pub tools: Option<BlueBubblesToolPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlueBubblesToolPolicy {
    pub allow: Option<Vec<String>>,
    pub deny: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlueBubblesAccountConfig {
    pub name: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub config_writes: Option<bool>,
    pub enabled: Option<bool>,
    pub server_url: Option<String>,
    pub password: Option<String>,
    pub webhook_path: Option<String>,
    pub dm_policy: Option<String>,
    pub allow_from: Option<Vec<String>>,
    pub group_allow_from: Option<Vec<String>>,
    pub group_policy: Option<String>,
    pub history_limit: Option<usize>,
    pub dm_history_limit: Option<usize>,
    pub text_chunk_limit: Option<usize>,
    pub chunk_mode: Option<String>,
    pub block_streaming: Option<bool>,
    pub media_max_mb: Option<usize>,
    pub media_local_roots: Option<Vec<String>>,
    pub send_read_receipts: Option<bool>,
    pub groups: Option<std::collections::HashMap<String, BlueBubblesGroupConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlueBubblesActionConfig {
    pub reactions: Option<bool>,
    pub edit: Option<bool>,
    pub unsend: Option<bool>,
    pub reply: Option<bool>,
    pub send_with_effect: Option<bool>,
    pub rename_group: Option<bool>,
    pub add_participant: Option<bool>,
    pub remove_participant: Option<bool>,
    pub leave_group: Option<bool>,
    pub send_attachment: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlueBubblesConfig {
    #[serde(flatten)]
    pub account: BlueBubblesAccountConfig,
    pub accounts: Option<std::collections::HashMap<String, BlueBubblesAccountConfig>>,
    pub actions: Option<BlueBubblesActionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BlueBubblesSendTarget {
    ChatId {
        chat_id: i64,
    },
    ChatGuid {
        chat_guid: String,
    },
    ChatIdentifier {
        chat_identifier: String,
    },
    Handle {
        address: String,
        service: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlueBubblesAttachment {
    pub guid: Option<String>,
    pub uti: Option<String>,
    pub mime_type: Option<String>,
    pub transfer_name: Option<String>,
    pub total_bytes: Option<i64>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub original_rowid: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueBubblesSendResult {
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueBubblesServerInfo {
    pub os_version: Option<String>,
    pub private_api: Option<bool>,
    pub bundle_id: Option<String>,
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueBubblesProbeResult {
    pub ok: bool,
    pub server_info: Option<BlueBubblesServerInfo>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedWebhookMessage {
    pub message_id: Option<String>,
    pub chat_guid: Option<String>,
    pub chat_id: Option<i64>,
    pub chat_identifier: Option<String>,
    pub sender_id: String,
    pub text: String,
    pub is_group: bool,
    pub from_me: bool,
    pub timestamp: Option<i64>,
    pub attachments: Option<Vec<BlueBubblesAttachment>>,
    pub reply_to_id: Option<String>,
    pub reply_to_body: Option<String>,
    pub reply_to_sender: Option<String>,
    pub balloon_bundle_id: Option<String>,
    pub associated_message_guid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedWebhookReaction {
    pub message_id: String,
    pub sender_id: String,
    pub action: String,
    pub reaction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub data: Option<serde_json::Value>,
}

pub fn normalize_server_url(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("BlueBubbles serverUrl is required".to_string());
    }
    let with_scheme = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("http://{}", trimmed)
    };
    Ok(with_scheme.trim_end_matches('/').to_string())
}

pub fn build_api_url(base_url: &str, path: &str, password: Option<&str>) -> String {
    let normalized = normalize_server_url(base_url).unwrap_or_else(|_| base_url.to_string());
    let mut url = format!("{}{}", normalized.trim_end_matches('/'), path);
    if let Some(pwd) = password {
        url = format!("{}?password={}", url, urlencoding::encode(pwd));
    }
    url
}

pub fn extract_message_id(response: &serde_json::Value) -> String {
    response
        .get("data")
        .and_then(|d| d.get("guid"))
        .and_then(|g| g.as_str())
        .unwrap_or("ok")
        .to_string()
}

pub fn extract_handle_from_chat_guid(chat_guid: &str) -> Option<String> {
    let parts: Vec<&str> = chat_guid.split(';').collect();
    if parts.len() < 3 {
        return None;
    }
    let identifier = parts.get(2)?.trim();
    if identifier.is_empty() {
        return None;
    }
    Some(normalize_handle(identifier))
}

pub fn normalize_handle(raw: &str) -> String {
    let trimmed = raw.trim();
    let without_prefix = trimmed
        .strip_prefix("+")
        .unwrap_or(trimmed)
        .replace(|c: char| !c.is_alphanumeric(), "");
    if let Some(stripped) = trimmed.strip_prefix("+") {
        format!("+{}", stripped.replace(|c: char| !c.is_numeric(), ""))
    } else {
        trimmed.to_lowercase()
    }
}

pub fn looks_like_target_id(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return false;
    }
    trimmed.starts_with("chat_guid:")
        || trimmed.starts_with("chat_id:")
        || trimmed.starts_with("chat_identifier:")
        || trimmed.starts_with("bluebubbles:")
        || trimmed.starts_with("+")
        || trimmed.starts_with("+1")
        || trimmed.contains('@')
        || trimmed.contains(";-;")
        || trimmed.contains(";+;")
}

pub fn resolve_effect_id(raw: Option<&str>) -> Option<String> {
    let input = raw?;
    let trimmed = input.trim().to_lowercase();

    let effect_map: std::collections::HashMap<&str, &str> = [
        ("slam", "com.apple.MobileSMS.expressivesend.impact"),
        ("loud", "com.apple.MobileSMS.expressivesend.loud"),
        ("gentle", "com.apple.MobileSMS.expressivesend.gentle"),
        (
            "invisible",
            "com.apple.MobileSMS.expressivesend.invisibleink",
        ),
        (
            "invisible-ink",
            "com.apple.MobileSMS.expressivesend.invisibleink",
        ),
        (
            "invisible ink",
            "com.apple.MobileSMS.expressivesend.invisibleink",
        ),
        (
            "invisibleink",
            "com.apple.MobileSMS.expressivesend.invisibleink",
        ),
        ("echo", "com.apple.messages.effect.CKEchoEffect"),
        ("spotlight", "com.apple.messages.effect.CKSpotlightEffect"),
        (
            "balloons",
            "com.apple.messages.effect.CKHappyBirthdayEffect",
        ),
        ("confetti", "com.apple.messages.effect.CKConfettiEffect"),
        ("love", "com.apple.messages.effect.CKHeartEffect"),
        ("heart", "com.apple.messages.effect.CKHeartEffect"),
        ("hearts", "com.apple.messages.effect.CKHeartEffect"),
        ("lasers", "com.apple.messages.effect.CKLasersEffect"),
        ("fireworks", "com.apple.messages.effect.CKFireworksEffect"),
        ("celebration", "com.apple.messages.effect.CKSparklesEffect"),
    ]
    .into_iter()
    .collect();

    effect_map.get(trimmed.as_str()).map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_server_url() {
        assert_eq!(
            normalize_server_url("localhost:8080").unwrap(),
            "http://localhost:8080"
        );
        assert_eq!(
            normalize_server_url("http://localhost:8080/").unwrap(),
            "http://localhost:8080"
        );
        assert!(normalize_server_url("").is_err());
    }

    #[test]
    fn test_build_api_url() {
        let url = build_api_url("http://localhost:8080", "/api/v1/test", Some("secret"));
        assert!(url.contains("password="));
        assert!(url.contains("/api/v1/test"));
    }

    #[test]
    fn test_extract_handle_from_chat_guid() {
        let guid = "iMessage;-;+15551234567";
        assert_eq!(
            extract_handle_from_chat_guid(guid),
            Some("+15551234567".to_string())
        );

        let invalid = "invalid";
        assert!(extract_handle_from_chat_guid(invalid).is_none());
    }

    #[test]
    fn test_looks_like_target_id() {
        assert!(looks_like_target_id("+15551234567"));
        assert!(looks_like_target_id("chat_guid:iMessage;-;+123"));
        assert!(looks_like_target_id("test@example.com"));
        assert!(!looks_like_target_id("hello world"));
    }
}
