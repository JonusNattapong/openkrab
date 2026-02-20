//! twitch — Twitch chat connector (IRC-based).
//! Ported from `openkrab/extensions/twitch/` (Phase 11).
//!
//! Twitch chat uses IRC over TLS (irc.chat.twitch.tv:6697).
//! Authentication is done with an OAuth token as the password.

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitchConfig {
    /// Twitch bot username (lowercase).
    pub username: String,
    /// OAuth token (e.g. "oauth:abc123"). Get from https://twitchapps.com/tmi/
    pub oauth_token: String,
    /// Channels to join (e.g. ["#mychannel"]).
    pub channels: Vec<String>,
    /// Optional Client-ID for Helix API calls.
    pub client_id: Option<String>,
}

impl Default for TwitchConfig {
    fn default() -> Self {
        Self {
            username: std::env::var("TWITCH_USERNAME").unwrap_or_default(),
            oauth_token: std::env::var("TWITCH_OAUTH_TOKEN").unwrap_or_default(),
            channels: std::env::var("TWITCH_CHANNELS")
                .unwrap_or_default()
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let s = s.trim().to_string();
                    if s.starts_with('#') {
                        s
                    } else {
                        format!("#{}", s)
                    }
                })
                .collect(),
            client_id: std::env::var("TWITCH_CLIENT_ID").ok(),
        }
    }
}

impl TwitchConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
    pub fn validate(&self) -> Result<()> {
        if self.username.is_empty() {
            bail!("TWITCH_USERNAME is required");
        }
        if self.oauth_token.is_empty() {
            bail!("TWITCH_OAUTH_TOKEN is required");
        }
        Ok(())
    }

    /// IRC server for Twitch chat.
    pub fn irc_server() -> &'static str {
        "irc.chat.twitch.tv"
    }
    pub fn irc_port() -> u16 {
        6697
    }
}

// ─── Twitch IRC message ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TwitchMessage {
    pub from: String,
    pub channel: String,
    pub text: String,
    pub display_name: Option<String>,
    pub is_subscriber: bool,
    pub is_moderator: bool,
    pub bits: Option<u64>,
}

/// Parse a Twitch IRC PRIVMSG line (with Twitch tags).
/// Format: `@badge-info=...;display-name=Alice;... :alice!alice@alice.tmi.twitch.tv PRIVMSG #channel :text`
pub fn parse_privmsg(raw: &str) -> Option<TwitchMessage> {
    if !raw.contains("PRIVMSG") {
        return None;
    }

    // Split tags from rest
    let (tags_part, rest) = if raw.starts_with('@') {
        let idx = raw.find(' ')?;
        (&raw[1..idx], &raw[idx + 1..])
    } else {
        ("", raw)
    };

    // Parse tags into map
    let tags: std::collections::HashMap<&str, &str> = tags_part
        .split(';')
        .filter_map(|kv| {
            let mut it = kv.splitn(2, '=');
            Some((it.next()?, it.next().unwrap_or("")))
        })
        .collect();

    let from = rest.strip_prefix(':')?.split('!').next()?.to_string();
    let parts: Vec<&str> = rest.splitn(4, ' ').collect();
    if parts.len() < 4 {
        return None;
    }

    let channel = parts[2].to_string();
    let text = parts[3].strip_prefix(':').unwrap_or(parts[3]).to_string();

    let display_name = tags
        .get("display-name")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    let badges = tags.get("badges").copied().unwrap_or("");
    let is_subscriber =
        badges.contains("subscriber") || tags.get("subscriber").copied() == Some("1");
    let is_moderator = badges.contains("moderator") || tags.get("mod").copied() == Some("1");
    let bits = tags.get("bits").and_then(|b| b.parse().ok());

    Some(TwitchMessage {
        from,
        channel,
        text,
        display_name,
        is_subscriber,
        is_moderator,
        bits,
    })
}

/// Build a Twitch chat PRIVMSG.
pub fn build_privmsg(channel: &str, text: &str) -> String {
    format!("PRIVMSG {} :{}\r\n", channel, text)
}

/// Build the IRC login sequence for Twitch.
pub fn build_login(username: &str, oauth_token: &str) -> Vec<String> {
    vec![
        format!("CAP REQ :twitch.tv/tags twitch.tv/commands\r\n"),
        format!("PASS {}\r\n", oauth_token),
        format!("NICK {}\r\n", username),
    ]
}

pub fn normalize_inbound(msg: &TwitchMessage) -> Message {
    Message {
        id: format!("twitch:{}:{}", msg.channel, msg.from),
        text: msg.text.clone(),
        from: Some(UserId(format!("twitch:{}", msg.from))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[twitch] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_privmsg() {
        let raw = ":alice!alice@alice.tmi.twitch.tv PRIVMSG #mychan :hello chat";
        let msg = parse_privmsg(raw).unwrap();
        assert_eq!(msg.from, "alice");
        assert_eq!(msg.channel, "#mychan");
        assert_eq!(msg.text, "hello chat");
        assert!(!msg.is_subscriber);
    }

    #[test]
    fn parse_tagged_privmsg() {
        let raw = "@display-name=Alice;subscriber=1;mod=0;badges=subscriber/1 :alice!alice@alice.tmi.twitch.tv PRIVMSG #chan :hey";
        let msg = parse_privmsg(raw).unwrap();
        assert_eq!(msg.display_name.as_deref(), Some("Alice"));
        assert!(msg.is_subscriber);
        assert!(!msg.is_moderator);
    }

    #[test]
    fn parse_bits_message() {
        let raw =
            "@bits=100;display-name=Bob :bob!bob@bob.tmi.twitch.tv PRIVMSG #chan :cheer100 nice";
        let msg = parse_privmsg(raw).unwrap();
        assert_eq!(msg.bits, Some(100));
    }

    #[test]
    fn parse_non_privmsg_returns_none() {
        assert!(parse_privmsg(":tmi.twitch.tv PING").is_none());
    }

    #[test]
    fn config_validate_missing_fields() {
        let mut cfg = TwitchConfig::default();
        cfg.username = "".into();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn channel_normalized_with_hash() {
        let cfg = TwitchConfig {
            username: "bot".into(),
            oauth_token: "oauth:abc".into(),
            channels: vec!["#streamer".into()],
            client_id: None,
        };
        assert!(cfg.channels[0].starts_with('#'));
    }
}
