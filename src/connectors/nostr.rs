//! nostr — Nostr decentralized messaging connector.
//! Ported from `openclaw/extensions/nostr/` (Phase 12).
//!
//! Uses NIP-01 (basic protocol) and NIP-04 (encrypted DMs).
//! Reference: https://github.com/nostr-protocol/nostr

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NostrConfig {
    /// Bot private key (hex-encoded 32-byte secp256k1 key).
    pub private_key_hex: String,
    /// Bot public key (hex-encoded, derived from private key).
    pub public_key_hex: Option<String>,
    /// Relay URLs to connect to.
    pub relays: Vec<String>,
}

impl Default for NostrConfig {
    fn default() -> Self {
        Self {
            private_key_hex: std::env::var("NOSTR_PRIVATE_KEY").unwrap_or_default(),
            public_key_hex: std::env::var("NOSTR_PUBLIC_KEY").ok(),
            relays: std::env::var("NOSTR_RELAYS")
                .unwrap_or_else(|_| "wss://relay.damus.io,wss://relay.nostr.info".into())
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}

impl NostrConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
    pub fn validate(&self) -> Result<()> {
        if self.private_key_hex.is_empty() {
            bail!("NOSTR_PRIVATE_KEY is required");
        }
        if self.private_key_hex.len() != 64 {
            bail!("NOSTR_PRIVATE_KEY must be a 64-character hex string (32 bytes)");
        }
        if self.relays.is_empty() {
            bail!("NOSTR_RELAYS must not be empty");
        }
        Ok(())
    }
}

// ─── Nostr event types ────────────────────────────────────────────────────────

/// Nostr event kinds (NIP-01).
pub mod kind {
    pub const METADATA: u64 = 0;
    pub const TEXT_NOTE: u64 = 1;
    pub const RECOMMEND_RELAY: u64 = 2;
    pub const CONTACT_LIST: u64 = 3;
    pub const ENCRYPTED_DM: u64 = 4;
    pub const DELETE: u64 = 5;
    pub const REPOST: u64 = 6;
    pub const REACTION: u64 = 7;
    pub const CHANNEL_MSG: u64 = 42;
}

/// A Nostr event (NIP-01 format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NostrEvent {
    pub id: String,
    pub pubkey: String,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    pub kind: u64,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

/// Relay message types received from WebSocket.
#[derive(Debug, Clone)]
pub enum RelayMessage {
    Event {
        subscription_id: String,
        event: NostrEvent,
    },
    Ok {
        event_id: String,
        success: bool,
        message: String,
    },
    Notice(String),
    Eose(String), // End of stored events
}

/// Parse a JSON relay message (from WebSocket frame).
pub fn parse_relay_message(raw: &str) -> Option<RelayMessage> {
    let v: serde_json::Value = serde_json::from_str(raw).ok()?;
    let arr = v.as_array()?;
    match arr.first()?.as_str()? {
        "EVENT" => {
            let sub_id = arr.get(1)?.as_str()?.to_string();
            let event: NostrEvent = serde_json::from_value(arr.get(2)?.clone()).ok()?;
            Some(RelayMessage::Event {
                subscription_id: sub_id,
                event,
            })
        }
        "OK" => {
            let event_id = arr.get(1)?.as_str()?.to_string();
            let success = arr.get(2)?.as_bool().unwrap_or(false);
            let message = arr.get(3)?.as_str().unwrap_or("").to_string();
            Some(RelayMessage::Ok {
                event_id,
                success,
                message,
            })
        }
        "NOTICE" => {
            let msg = arr.get(1)?.as_str()?.to_string();
            Some(RelayMessage::Notice(msg))
        }
        "EOSE" => {
            let sub_id = arr.get(1)?.as_str()?.to_string();
            Some(RelayMessage::Eose(sub_id))
        }
        _ => None,
    }
}

/// Parsed message from a Nostr event.
#[derive(Debug, Clone)]
pub struct ParsedNostrMessage {
    pub event_id: String,
    pub pubkey: String,
    pub text: String,
    pub kind: u64,
    pub is_dm: bool,
    pub reply_to: Option<String>,
    pub created_at: i64,
}

/// Parse a Nostr event into a message (kind 1 text note or kind 4 DM).
pub fn parse_event(event: &NostrEvent) -> Option<ParsedNostrMessage> {
    if event.kind != kind::TEXT_NOTE && event.kind != kind::ENCRYPTED_DM {
        return None;
    }
    let text = event.content.trim().to_string();
    if text.is_empty() {
        return None;
    }

    // Extract reply reference from tags
    let reply_to = event
        .tags
        .iter()
        .find(|t| t.first().map(|s| s == "e") == Some(true))
        .and_then(|t| t.get(1))
        .cloned();

    Some(ParsedNostrMessage {
        event_id: event.id.clone(),
        pubkey: event.pubkey.clone(),
        text,
        kind: event.kind,
        is_dm: event.kind == kind::ENCRYPTED_DM,
        reply_to,
        created_at: event.created_at,
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

/// Build a REQ (subscription) message.
pub fn build_req(subscription_id: &str, filters: &serde_json::Value) -> String {
    serde_json::json!(["REQ", subscription_id, filters]).to_string()
}

/// Build a CLOSE (unsubscribe) message.
pub fn build_close(subscription_id: &str) -> String {
    serde_json::json!(["CLOSE", subscription_id]).to_string()
}

/// Build an EVENT publish message (unsigned — caller must fill id/sig).
pub fn build_event_payload(
    pubkey: &str,
    kind: u64,
    content: &str,
    tags: Vec<Vec<String>>,
) -> serde_json::Value {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    serde_json::json!({
        "pubkey": pubkey,
        "created_at": now,
        "kind": kind,
        "tags": tags,
        "content": content
    })
}

/// Build a DM event shell (content must be NIP-04 encrypted by caller).
pub fn build_dm_event(
    from_pubkey: &str,
    to_pubkey: &str,
    encrypted_content: &str,
) -> serde_json::Value {
    build_event_payload(
        from_pubkey,
        kind::ENCRYPTED_DM,
        encrypted_content,
        vec![vec!["p".to_string(), to_pubkey.to_string()]],
    )
}

pub fn normalize_inbound(msg: &ParsedNostrMessage) -> Message {
    Message {
        id: format!("nostr:{}", msg.event_id),
        text: msg.text.clone(),
        from: Some(UserId(format!("nostr:{}", msg.pubkey))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[nostr] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(k: u64, content: &str) -> NostrEvent {
        NostrEvent {
            id: "ev1abc".into(),
            pubkey: "pub1".into(),
            created_at: 1_700_000,
            kind: k,
            tags: vec![],
            content: content.into(),
            sig: "sig1".into(),
        }
    }

    #[test]
    fn parse_text_note() {
        let ev = make_event(kind::TEXT_NOTE, "hello nostr");
        let msg = parse_event(&ev).unwrap();
        assert_eq!(msg.text, "hello nostr");
        assert!(!msg.is_dm);
    }

    #[test]
    fn parse_dm_event() {
        let ev = make_event(kind::ENCRYPTED_DM, "encrypted_payload");
        let msg = parse_event(&ev).unwrap();
        assert!(msg.is_dm);
    }

    #[test]
    fn parse_other_kind_returns_none() {
        let ev = make_event(kind::METADATA, "{}");
        assert!(parse_event(&ev).is_none());
    }

    #[test]
    fn parse_relay_event_message() {
        let raw = r#"["EVENT","sub1",{"id":"abc","pubkey":"pub1","created_at":1700000,"kind":1,"tags":[],"content":"hi","sig":"s1"}]"#;
        let msg = parse_relay_message(raw).unwrap();
        assert!(matches!(msg, RelayMessage::Event { .. }));
    }

    #[test]
    fn parse_relay_notice() {
        let raw = r#"["NOTICE","hello from relay"]"#;
        let msg = parse_relay_message(raw).unwrap();
        assert!(matches!(msg, RelayMessage::Notice(_)));
    }

    #[test]
    fn config_validate_invalid_key_length() {
        let cfg = NostrConfig {
            private_key_hex: "abc".into(), // too short
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }
}
