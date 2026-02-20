//! BlueBubbles target parsing and normalization.
//! Ported from openkrab/extensions/bluebubbles/src/targets.ts

use crate::connectors::bluebubbles::types::{looks_like_target_id, BlueBubblesSendTarget};

pub fn normalize_target(raw: &str) -> String {
    let mut s = raw.trim();

    if let Some(rest) = s.strip_prefix("bluebubbles:") {
        s = rest.trim();
    }

    let lower = s.to_ascii_lowercase();
    for prefix in ["chat_guid:", "chat_id:", "chat_identifier:"] {
        if lower.starts_with(prefix) {
            return s[prefix.len()..].trim().to_string();
        }
    }

    s.to_string()
}

pub fn normalize_messaging_target(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if looks_like_target_id(trimmed) {
        return normalize_target(trimmed);
    }

    if trimmed.starts_with("+") || trimmed.contains('@') {
        return normalize_handle(trimmed);
    }

    normalize_target(trimmed)
}

pub fn normalize_handle(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if trimmed.starts_with('+') {
        let digits: String = trimmed.chars().filter(|c| c.is_numeric()).collect();
        return format!("+{}", digits);
    }

    if trimmed.contains('@') {
        return trimmed.to_lowercase();
    }

    trimmed.to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedTarget {
    ChatGuid {
        chat_guid: String,
    },
    ChatId {
        chat_id: i64,
    },
    ChatIdentifier {
        chat_identifier: String,
    },
    Handle {
        address: String,
        service: Option<String>,
    },
}

impl ParsedTarget {
    pub fn parse(raw: &str) -> Self {
        let trimmed = raw.trim();
        let lower = trimmed.to_lowercase();

        if lower.starts_with("chat_guid:") {
            return ParsedTarget::ChatGuid {
                chat_guid: trimmed[10..].trim().to_string(),
            };
        }

        if lower.starts_with("chat_id:") {
            if let Ok(id) = trimmed[8..].trim().parse::<i64>() {
                return ParsedTarget::ChatId { chat_id: id };
            }
        }

        if lower.starts_with("chat_identifier:") {
            return ParsedTarget::ChatIdentifier {
                chat_identifier: trimmed[16..].trim().to_string(),
            };
        }

        if trimmed.starts_with('+') || trimmed.contains('@') {
            return ParsedTarget::Handle {
                address: normalize_handle(trimmed),
                service: detect_service(trimmed),
            };
        }

        if trimmed.contains(";-;") || trimmed.contains(";+;") {
            return ParsedTarget::ChatGuid {
                chat_guid: trimmed.to_string(),
            };
        }

        ParsedTarget::Handle {
            address: normalize_handle(trimmed),
            service: None,
        }
    }

    pub fn to_send_target(&self) -> BlueBubblesSendTarget {
        match self {
            ParsedTarget::ChatGuid { chat_guid } => BlueBubblesSendTarget::ChatGuid {
                chat_guid: chat_guid.clone(),
            },
            ParsedTarget::ChatId { chat_id } => BlueBubblesSendTarget::ChatId { chat_id: *chat_id },
            ParsedTarget::ChatIdentifier { chat_identifier } => {
                BlueBubblesSendTarget::ChatIdentifier {
                    chat_identifier: chat_identifier.clone(),
                }
            }
            ParsedTarget::Handle { address, service } => BlueBubblesSendTarget::Handle {
                address: address.clone(),
                service: service.clone(),
            },
        }
    }
}

fn detect_service(address: &str) -> Option<String> {
    let lower = address.to_lowercase();

    if lower.starts_with("imessage") || lower.starts_with("+") {
        return Some("imessage".to_string());
    }

    if lower.contains('@') && !lower.contains("imessage") {
        return Some("sms".to_string());
    }

    None
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

pub fn is_group_chat_guid(chat_guid: &str) -> bool {
    chat_guid.contains(";+;")
}

pub fn is_dm_chat_guid(chat_guid: &str) -> bool {
    chat_guid.contains(";-;")
}

pub fn get_service_from_chat_guid(chat_guid: &str) -> Option<String> {
    let parts: Vec<&str> = chat_guid.split(';').collect();
    if parts.is_empty() {
        return None;
    }
    Some(parts[0].to_lowercase())
}

pub fn format_target_display(target: &str, display: Option<&str>) -> String {
    if let Some(d) = display {
        let trimmed = d.trim();
        if !looks_like_target_id(trimmed) {
            return trimmed.to_string();
        }
    }

    let parsed = ParsedTarget::parse(target);
    match &parsed {
        ParsedTarget::ChatGuid { chat_guid } => {
            if let Some(handle) = extract_handle_from_chat_guid(chat_guid) {
                return handle;
            }
            if is_group_chat_guid(chat_guid) {
                return "Group Chat".to_string();
            }
        }
        ParsedTarget::Handle { address, .. } => {
            return address.clone();
        }
        _ => {}
    }

    display.unwrap_or(target).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_target_strips_prefix() {
        assert_eq!(normalize_target("bluebubbles:+15551234567"), "+15551234567");
        assert_eq!(
            normalize_target("bluebubbles:CHAT_GUID:iMessage;-;+123"),
            "iMessage;-;+123"
        );
    }

    #[test]
    fn test_normalize_handle_phone() {
        assert_eq!(normalize_handle("+1 (555) 123-4567"), "+15551234567");
        assert_eq!(normalize_handle("+66812345678"), "+66812345678");
    }

    #[test]
    fn test_normalize_handle_email() {
        assert_eq!(normalize_handle("TEST@EXAMPLE.COM"), "test@example.com");
    }

    #[test]
    fn test_parse_target_chat_guid() {
        let parsed = ParsedTarget::parse("chat_guid:iMessage;-;+15551234567");
        match parsed {
            ParsedTarget::ChatGuid { chat_guid } => {
                assert_eq!(chat_guid, "iMessage;-;+15551234567");
            }
            _ => panic!("Expected ChatGuid"),
        }
    }

    #[test]
    fn test_parse_target_handle() {
        let parsed = ParsedTarget::parse("+15551234567");
        match parsed {
            ParsedTarget::Handle { address, service } => {
                assert_eq!(address, "+15551234567");
                assert_eq!(service, Some("imessage".to_string()));
            }
            _ => panic!("Expected Handle"),
        }
    }

    #[test]
    fn test_extract_handle_from_chat_guid() {
        let guid = "iMessage;-;+15551234567";
        assert_eq!(
            extract_handle_from_chat_guid(guid),
            Some("+15551234567".to_string())
        );

        let group_guid = "iMessage;+;+15551112222,+15553334444";
        assert_eq!(extract_handle_from_chat_guid(group_guid), None);
    }

    #[test]
    fn test_is_group_dm_chat() {
        assert!(is_dm_chat_guid("iMessage;-;+15551234567"));
        assert!(is_group_chat_guid("iMessage;+;+15551112222,+15553334444"));
        assert!(!is_group_chat_guid("iMessage;-;+15551234567"));
    }

    #[test]
    fn test_format_target_display() {
        assert_eq!(format_target_display("+15551234567", None), "+15551234567");
        assert_eq!(
            format_target_display("chat_guid:xxx", Some("Alice")),
            "Alice"
        );
    }
}
