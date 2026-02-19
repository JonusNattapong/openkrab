//! signal::identity — Signal sender identity and allowlist helpers.
//! Ported from `openclaw/src/signal/identity.ts` (Phase 13).

use std::collections::HashSet;

/// Signal sender identity (phone or UUID).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignalSender {
    Phone { raw: String, e164: String },
    Uuid { raw: String },
}

impl SignalSender {
    /// Parse sender from source number and UUID.
    pub fn resolve(source_number: Option<&str>, source_uuid: Option<&str>) -> Option<Self> {
        if let Some(num) = source_number.filter(|s| !s.trim().is_empty()) {
            let e164 = normalize_e164(num);
            return Some(Self::Phone {
                raw: num.to_string(),
                e164,
            });
        }
        if let Some(uuid) = source_uuid.filter(|s| !s.trim().is_empty()) {
            return Some(Self::Uuid {
                raw: uuid.to_string(),
            });
        }
        None
    }

    /// Get unique ID for this sender.
    pub fn id(&self) -> String {
        match self {
            Self::Phone { e164, .. } => e164.clone(),
            Self::Uuid { raw } => format!("uuid:{}", raw),
        }
    }

    /// Get display string.
    pub fn display(&self) -> String {
        self.id()
    }

    /// Get recipient string for API calls.
    pub fn recipient(&self) -> String {
        match self {
            Self::Phone { e164, .. } => e164.clone(),
            Self::Uuid { raw } => raw.clone(),
        }
    }

    /// Get peer ID for pairing.
    pub fn peer_id(&self) -> String {
        match self {
            Self::Phone { e164, .. } => e164.clone(),
            Self::Uuid { raw } => format!("uuid:{}", raw),
        }
    }

    /// Get pairing ID line for display.
    pub fn pairing_id_line(&self) -> String {
        match self {
            Self::Phone { e164, .. } => format!("Your Signal number: {}", e164),
            Self::Uuid { raw } => format!("Your Signal sender id: uuid:{}", raw),
        }
    }

    /// Check if this sender is allowed by allowlist.
    pub fn is_allowed(&self, allowlist: &[String]) -> bool {
        if allowlist.is_empty() {
            return false;
        }

        for entry in allowlist {
            let trimmed = entry.trim();
            if trimmed == "*" {
                return true;
            }

            // Check exact match
            if trimmed == self.id() {
                return true;
            }

            // Check phone number match (allowlist might have different formats)
            if let Self::Phone { e164, .. } = self {
                if trimmed == e164 || normalize_e164(trimmed) == *e164 {
                    return true;
                }
            }
        }

        false
    }
}

/// Normalize phone number to E164 format.
pub fn normalize_e164(phone: &str) -> String {
    let mut cleaned = phone
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect::<String>();

    // Ensure starts with +
    if !cleaned.starts_with('+') {
        cleaned.insert(0, '+');
    }

    // Remove any duplicate + signs
    if cleaned.starts_with("++") {
        cleaned = cleaned.trim_start_matches('+').to_string();
        cleaned.insert(0, '+');
    }

    cleaned
}

/// Strip "signal:" prefix from strings.
pub fn strip_signal_prefix(value: &str) -> String {
    value.trim_start_matches("signal:").trim().to_string()
}

/// Check if a string looks like a UUID.
pub fn looks_like_uuid(value: &str) -> bool {
    let compact = value.replace('-', "");
    if compact.len() == 32 {
        compact.chars().all(|c| c.is_ascii_hexdigit())
    } else if compact.len() == 36 {
        // Check hyphenated format
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() == 5
            && parts[0].len() == 8
            && parts[1].len() == 4
            && parts[2].len() == 4
            && parts[3].len() == 4
            && parts[4].len() == 12
        {
            value.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_e164_basic() {
        assert_eq!(normalize_e164("66812345678"), "+66812345678");
        assert_eq!(normalize_e164("+66812345678"), "+66812345678");
        assert_eq!(normalize_e164("++66812345678"), "+66812345678");
    }

    #[test]
    fn resolve_sender_phone() {
        let sender = SignalSender::resolve(Some("+66812345678"), None).unwrap();
        match sender {
            SignalSender::Phone { raw, e164 } => {
                assert_eq!(raw, "+66812345678");
                assert_eq!(e164, "+66812345678");
            }
            _ => panic!("Expected phone sender"),
        }
    }

    #[test]
    fn resolve_sender_uuid() {
        let sender = SignalSender::resolve(None, Some("abc123")).unwrap();
        match sender {
            SignalSender::Uuid { raw } => {
                assert_eq!(raw, "abc123");
            }
            _ => panic!("Expected UUID sender"),
        }
    }

    #[test]
    fn sender_id_phone() {
        let sender = SignalSender::Phone {
            raw: "+66812345678".into(),
            e164: "+66812345678".into(),
        };
        assert_eq!(sender.id(), "+66812345678");
    }

    #[test]
    fn sender_id_uuid() {
        let sender = SignalSender::Uuid {
            raw: "abc123".into(),
        };
        assert_eq!(sender.id(), "uuid:abc123");
    }

    #[test]
    fn is_allowed_wildcard() {
        let sender = SignalSender::Phone {
            raw: "+66812345678".into(),
            e164: "+66812345678".into(),
        };
        assert!(sender.is_allowed(&["*".into()]));
    }

    #[test]
    fn is_allowed_exact_match() {
        let sender = SignalSender::Phone {
            raw: "+66812345678".into(),
            e164: "+66812345678".into(),
        };
        assert!(sender.is_allowed(&["+66812345678".into()]));
    }

    #[test]
    fn is_allowed_no_match() {
        let sender = SignalSender::Phone {
            raw: "+66812345678".into(),
            e164: "+66812345678".into(),
        };
        assert!(!sender.is_allowed(&["+66987654321".into()]));
    }

    #[test]
    fn looks_like_uuid_hyphenated() {
        assert!(looks_like_uuid("12345678-1234-5678-9abc-def012345678"));
    }

    #[test]
    fn looks_like_uuid_compact() {
        assert!(looks_like_uuid("12345678123456789abcdef012345678"));
    }

    #[test]
    fn looks_like_uuid_invalid() {
        assert!(!looks_like_uuid("not-a-uuid"));
        assert!(!looks_like_uuid("123"));
    }
}
