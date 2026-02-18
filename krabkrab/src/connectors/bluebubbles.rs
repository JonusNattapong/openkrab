//! connectors::bluebubbles — BlueBubbles (iMessage via macOS server) primitives.
//! Phase 19 initial port slice: inbound/outbound normalization + target canonicalization.

use crate::common::{Message, UserId};

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
        let msg = normalize_inbound("hello", "iMessage;-;+15551234567", "+15551234567", "abc-123");
        assert_eq!(msg.id, "bluebubbles:iMessage;-;+15551234567:abc-123");
        assert_eq!(msg.text, "hello");
        assert_eq!(
            msg.from.as_ref().map(|u| u.0.as_str()),
            Some("bluebubbles:+15551234567")
        );
    }
}

