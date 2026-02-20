//! irc — IRC messaging connector.
//! Ported from `openkrab/extensions/irc/` (Phase 11).

use crate::common::{Message, UserId};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrcConfig {
    /// IRC server hostname.
    pub server: String,
    /// IRC server port (default 6667, TLS 6697).
    pub port: u16,
    /// Bot nickname.
    pub nick: String,
    /// Channels to join (e.g. ["#krabkrab", "#general"]).
    pub channels: Vec<String>,
    /// Optional password for NickServ or PASS.
    pub password: Option<String>,
    /// Whether to use TLS.
    pub tls: bool,
}

impl Default for IrcConfig {
    fn default() -> Self {
        Self {
            server: std::env::var("IRC_SERVER").unwrap_or_else(|_| "irc.libera.chat".to_string()),
            port: std::env::var("IRC_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(6667),
            nick: std::env::var("IRC_NICK").unwrap_or_else(|_| "krabkrab".to_string()),
            channels: std::env::var("IRC_CHANNELS")
                .unwrap_or_default()
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
            password: std::env::var("IRC_PASSWORD").ok(),
            tls: std::env::var("IRC_TLS")
                .map(|v| v == "1" || v == "true")
                .unwrap_or(false),
        }
    }
}

impl IrcConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.server.is_empty() {
            anyhow::bail!("IRC_SERVER is required");
        }
        if self.nick.is_empty() {
            anyhow::bail!("IRC_NICK is required");
        }
        Ok(())
    }
}

// ─── IRC message ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IrcMessage {
    pub from_nick: String,
    pub channel: String,
    pub text: String,
    pub is_private: bool,
}

/// Parse a raw IRC PRIVMSG line.
/// Format: `:nick!user@host PRIVMSG #channel :message text`
pub fn parse_privmsg(raw: &str, bot_nick: &str) -> Option<IrcMessage> {
    if !raw.contains("PRIVMSG") {
        return None;
    }

    let from_nick = raw.strip_prefix(':')?.split('!').next()?.to_string();
    let parts: Vec<&str> = raw.splitn(4, ' ').collect();
    if parts.len() < 4 {
        return None;
    }

    let target = parts[2];
    let text = parts[3].strip_prefix(':').unwrap_or(parts[3]).to_string();
    let is_private = target == bot_nick;
    let channel = if is_private {
        from_nick.clone()
    } else {
        target.to_string()
    };

    Some(IrcMessage {
        from_nick,
        channel,
        text,
        is_private,
    })
}

/// Build a raw IRC PRIVMSG command.
pub fn build_privmsg(target: &str, text: &str) -> String {
    format!("PRIVMSG {} :{}\r\n", target, text)
}

/// Build a raw IRC JOIN command.
pub fn build_join(channel: &str) -> String {
    format!("JOIN {}\r\n", channel)
}

/// Build a raw IRC NICK command.
pub fn build_nick(nick: &str) -> String {
    format!("NICK {}\r\n", nick)
}

/// Build IRC USER command.
pub fn build_user(nick: &str) -> String {
    format!("USER {} 0 * :{}\r\n", nick, nick)
}

pub fn normalize_inbound(msg: &IrcMessage) -> Message {
    Message {
        id: format!("irc:{}:{}", msg.channel, msg.from_nick),
        text: msg.text.clone(),
        from: Some(UserId(format!("irc:{}", msg.from_nick))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[irc] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_channel_message() {
        let raw = ":alice!alice@host.net PRIVMSG #krabkrab :hello bot";
        let msg = parse_privmsg(raw, "krabbot").unwrap();
        assert_eq!(msg.from_nick, "alice");
        assert_eq!(msg.channel, "#krabkrab");
        assert_eq!(msg.text, "hello bot");
        assert!(!msg.is_private);
    }

    #[test]
    fn parse_private_message() {
        let raw = ":bob!bob@irc.net PRIVMSG krabbot :hey there";
        let msg = parse_privmsg(raw, "krabbot").unwrap();
        assert!(msg.is_private);
        assert_eq!(msg.channel, "bob");
        assert_eq!(msg.text, "hey there");
    }

    #[test]
    fn parse_non_privmsg_returns_none() {
        let raw = ":server NOTICE krabbot :*** Looking up your hostname";
        assert!(parse_privmsg(raw, "krabbot").is_none());
    }

    #[test]
    fn build_commands() {
        assert!(build_privmsg("#ch", "hi").contains("PRIVMSG #ch :hi"));
        assert!(build_join("#general").contains("JOIN #general"));
        assert!(build_nick("mybot").contains("NICK mybot"));
    }

    #[test]
    fn config_validate_missing_server() {
        let cfg = IrcConfig {
            server: "".into(),
            ..IrcConfig::default()
        };
        assert!(cfg.validate().is_err());
    }
}
