//! connectors::bluebubbles — BlueBubbles (iMessage via macOS server) primitives.
//! Phase 19 initial port slice: inbound/outbound normalization + target canonicalization.
//! Phase 21: Added send functionality

use crate::common::{Message, UserId};
use serde::{Deserialize, Serialize};

/// Normalize inbound BlueBubbles webhook payload fields into core [`Message`].
pub fn normalize_inbound(text: &str, chat_guid: &str, sender: &str, message_id: &str) -> Message {
    Message {
        id: format!("bluebubbles:{chat_guid}:{message_id}"),
        text: text.to_string(),
        from: Some(UserId(format!("bluebubbles:{}", normalize_sender(sender)))),
    }
}

/// Format outbound text for connector-level debugging paths.
pub fn format_outbound(text: &str) -> String {
    format!("[bluebubbles] {text}")
}

/// Canonicalize BlueBubbles target:
/// - strips `bluebubbles:` prefix
/// - strips known chat prefixes (`chat_guid:`, `chat_id:`, `chat_identifier:`)
/// - preserves handle targets (phone/email) as-is
pub fn normalize_target(raw: &str) -> String {
    let mut s = raw.trim();

    if let Some(rest) = s.strip_prefix("bluebubbles:") {
        s = rest.trim();
    }

    // Keep parity with legacy behavior: accept case-insensitive chat_* prefixes.
    let lower = s.to_ascii_lowercase();
    for prefix in ["chat_guid:", "chat_id:", "chat_identifier:"] {
        if lower.starts_with(prefix) {
            return s[prefix.len()..].trim().to_string();
        }
    }

    s.to_string()
}

fn normalize_sender(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return "unknown".to_string();
    }
    normalize_target(trimmed)
}

// ─── BlueBubbles Send ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueBubblesSendResult {
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueBubblesAccount {
    pub server_url: String,
    pub password: String,
    pub name: Option<String>,
}

impl BlueBubblesAccount {
    pub fn new(server_url: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            server_url: server_url.into(),
            password: password.into(),
            name: None,
        }
    }
}

/// Send a message via BlueBubbles REST API
pub fn send_message(
    account: &BlueBubblesAccount,
    target: &str,
    text: &str,
) -> Result<BlueBubblesSendResult, String> {
    use reqwest::blocking::Client;
    use serde_json::json;

    let client = Client::new();
    let url = format!("{}/api/v1/chat/{}/message", account.server_url, target);

    let payload = json!({
        "text": text
    });

    let response = client
        .post(&url)
        .basic_auth("BlueBubbles", Some(&account.password))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("parse response: {}", e))?;

    let message_id = json
        .get("data")
        .and_then(|d| d.get("guid"))
        .and_then(|g| g.as_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(BlueBubblesSendResult { message_id })
}

/// Resolve effect ID from short name to Apple effect identifier
pub fn resolve_effect_id(raw: Option<&str>) -> Option<String> {
    let input = raw?.trim().to_lowercase();

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

    input.and_then(|k| effect_map.get(k).map(|s| s.to_string()))
}

/// Resolve effect ID from short name to Apple effect identifier
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
    fn normalize_target_strips_channel_prefix() {
        assert_eq!(normalize_target("bluebubbles:+15551234567"), "+15551234567");
    }

    #[test]
    fn normalize_target_strips_chat_guid_case_insensitive() {
        assert_eq!(
            normalize_target("bluebubbles:CHAT_GUID:iMessage;-;+15551234567"),
            "iMessage;-;+15551234567"
        );
    }

    #[test]
    fn normalize_inbound_builds_message_shape() {
        let msg = normalize_inbound(
            "hello",
            "iMessage;-;+15551234567",
            "+15551234567",
            "abc-123",
        );
        assert_eq!(msg.id, "bluebubbles:iMessage;-;+15551234567:abc-123");
        assert_eq!(msg.text, "hello");
        assert_eq!(
            msg.from.as_ref().map(|u| u.0.as_str()),
            Some("bluebubbles:+15551234567")
        );
    }
}
