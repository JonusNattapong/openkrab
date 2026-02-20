//! tlon — Tlon/Urbit messaging connector.
//! Ported from `openkrab/extensions/tlon/` (Phase 13).
//!
//! Connects to a Urbit ship's Eyre HTTP interface to send/receive messages
//! via Landscape groups (channels) or DMs.
//! Reference: https://urbit.org/docs/userspace/threads/basics

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlonConfig {
    /// Urbit ship URL (e.g. "http://localhost:8080").
    pub ship_url: String,
    /// Ship name (e.g. "~sampel-palnet").
    pub ship_name: String,
    /// Landscape access code ("+code" from your ship).
    pub access_code: String,
    /// Group/channel path (e.g. "~sampel-palnet/my-group").
    pub group: Option<String>,
    /// Channel within the group.
    pub channel: Option<String>,
}

impl Default for TlonConfig {
    fn default() -> Self {
        Self {
            ship_url: std::env::var("TLON_SHIP_URL")
                .unwrap_or_else(|_| "http://localhost:8080".into()),
            ship_name: std::env::var("TLON_SHIP_NAME").unwrap_or_default(),
            access_code: std::env::var("TLON_ACCESS_CODE").unwrap_or_default(),
            group: std::env::var("TLON_GROUP").ok(),
            channel: std::env::var("TLON_CHANNEL").ok(),
        }
    }
}

impl TlonConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
    pub fn validate(&self) -> Result<()> {
        if self.ship_name.is_empty() {
            bail!("TLON_SHIP_NAME is required (e.g. ~sampel-palnet)");
        }
        if self.access_code.is_empty() {
            bail!("TLON_ACCESS_CODE is required");
        }
        Ok(())
    }
}

// ─── Urbit event types ────────────────────────────────────────────────────────

/// SSE event from Urbit's Eyre (simplified).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbitEvent {
    pub id: Option<String>,
    pub data: serde_json::Value,
}

/// A Urbit graph-store message (Landscape DM or group message).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbitGraphEntry {
    pub index: String,
    pub post: UrbitPost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbitPost {
    pub author: String,
    pub index: String,
    #[serde(rename = "time-sent")]
    pub time_sent: u64,
    pub contents: Vec<UrbitContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UrbitContent {
    Text { text: String },
    Url { url: String },
    Code { code: UrbitCode },
    Reference { reference: serde_json::Value },
    Other(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbitCode {
    pub expression: String,
    pub output: Option<serde_json::Value>,
}

/// Parsed message from a Urbit graph post.
#[derive(Debug, Clone)]
pub struct ParsedTlonMessage {
    pub author: String,
    pub index: String,
    pub time_sent: u64,
    pub text: String,
    pub has_url: bool,
    pub resource: Option<String>, // graph resource path
}

pub fn extract_text(contents: &[UrbitContent]) -> String {
    contents
        .iter()
        .filter_map(|c| match c {
            UrbitContent::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

pub fn parse_graph_entry(
    entry: &UrbitGraphEntry,
    resource: Option<&str>,
) -> Option<ParsedTlonMessage> {
    let text = extract_text(&entry.post.contents);
    if text.is_empty() {
        return None;
    }

    let has_url = entry
        .post
        .contents
        .iter()
        .any(|c| matches!(c, UrbitContent::Url { .. }));

    Some(ParsedTlonMessage {
        author: entry.post.author.clone(),
        index: entry.post.index.clone(),
        time_sent: entry.post.time_sent,
        text,
        has_url,
        resource: resource.map(|s| s.to_string()),
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

/// Build a graph-store poke to add a text post.
pub fn build_text_post(ship: &str, resource: &str, text: &str) -> serde_json::Value {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    serde_json::json!({
        "add-nodes": {
            "resource": { "ship": ship, "name": resource },
            "nodes": {
                format!("/{}", now): {
                    "post": {
                        "author": ship,
                        "index": format!("/{}", now),
                        "time-sent": now,
                        "contents": [{ "text": text }],
                        "hash": null,
                        "signatures": []
                    },
                    "children": null
                }
            }
        }
    })
}

/// Build the Eyre login payload.
pub fn build_login_payload(access_code: &str) -> Vec<(&str, &str)> {
    vec![("password", access_code)]
}

/// Send a text message via graph-store poke.
pub async fn send_message(
    client: &reqwest::Client,
    cfg: &TlonConfig,
    resource: &str,
    text: &str,
) -> Result<()> {
    // First we need to be logged in (session cookie handles auth)
    let poke_url = format!("{}/~/channel/0", cfg.ship_url);
    let payload = serde_json::json!([{
        "id": 1,
        "action": "poke",
        "ship": cfg.ship_name.trim_start_matches('~'),
        "app": "graph-store",
        "mark": "graph-update-3",
        "json": build_text_post(&cfg.ship_name, resource, text)
    }]);
    client
        .put(&poke_url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

pub fn normalize_inbound(msg: &ParsedTlonMessage) -> Message {
    Message {
        id: format!("tlon:{}", msg.index),
        text: msg.text.clone(),
        from: Some(UserId(format!("tlon:{}", msg.author))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[tlon] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(text: &str) -> UrbitGraphEntry {
        UrbitGraphEntry {
            index: "/1700000".into(),
            post: UrbitPost {
                author: "~sampel-palnet".into(),
                index: "/1700000".into(),
                time_sent: 1_700_000,
                contents: vec![UrbitContent::Text { text: text.into() }],
            },
        }
    }

    #[test]
    fn parse_text_entry() {
        let e = make_entry("hello urbit");
        let msg = parse_graph_entry(&e, Some("my-group/my-channel")).unwrap();
        assert_eq!(msg.text, "hello urbit");
        assert_eq!(msg.author, "~sampel-palnet");
        assert!(!msg.has_url);
    }

    #[test]
    fn extract_text_multi_content() {
        let contents = vec![
            UrbitContent::Text {
                text: "hello".into(),
            },
            UrbitContent::Url {
                url: "https://example.com".into(),
            },
            UrbitContent::Text {
                text: "world".into(),
            },
        ];
        assert_eq!(extract_text(&contents), "hello world");
    }

    #[test]
    fn parse_empty_entry_returns_none() {
        let mut e = make_entry("");
        e.post.contents.clear();
        assert!(parse_graph_entry(&e, None).is_none());
    }

    #[test]
    fn config_validate_missing_ship() {
        let cfg = TlonConfig {
            ship_name: "".into(),
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn normalize_inbound_test() {
        let msg = ParsedTlonMessage {
            author: "~zod".into(),
            index: "/1234".into(),
            time_sent: 0,
            text: "hi".into(),
            has_url: false,
            resource: None,
        };
        let m = normalize_inbound(&msg);
        assert!(m.id.starts_with("tlon:"));
    }
}
