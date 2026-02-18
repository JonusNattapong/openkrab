//! auto_reply — Auto-reply dispatch engine.
//! Ported from `openclaw/src/auto-reply/` (Phase 6).
//!
//! Implements the core inbound message → agent response pipeline:
//! command detection, activation gating, heartbeat handling,
//! and final reply dispatch.

use serde::{Deserialize, Serialize};

// ─── Inbound envelope ─────────────────────────────────────────────────────────

/// Normalised inbound message from any connector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundEnvelope {
    /// Connector name (e.g. "telegram", "slack").
    pub connector: String,
    /// Unique message ID (connector-specific).
    pub message_id: String,
    /// Sender identifier (user ID).
    pub sender_id: String,
    /// Sender display name.
    pub sender_name: String,
    /// Chat / conversation ID.
    pub chat_id: String,
    /// Chat type ("direct", "group", "supergroup", "channel").
    pub chat_type: String,
    /// The raw text of the message.
    pub text: String,
    /// Whether the bot was @-mentioned.
    pub mentioned: bool,
    /// Reply-to message ID, if this is a reply.
    pub reply_to_id: Option<String>,
    /// Attached media items (file IDs or URLs).
    pub media: Vec<String>,
    /// ISO 8601 timestamp.
    pub timestamp: String,
}

impl InboundEnvelope {
    pub fn new(
        connector: impl Into<String>,
        sender_id: impl Into<String>,
        chat_id: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            connector: connector.into(),
            message_id: uuid_v4(),
            sender_id: sender_id.into(),
            sender_name: String::new(),
            chat_id: chat_id.into(),
            chat_type: "direct".to_string(),
            text: text.into(),
            mentioned: false,
            reply_to_id: None,
            media: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn is_empty_text(&self) -> bool {
        self.text.trim().is_empty()
    }

    pub fn is_command(&self) -> bool {
        let t = self.text.trim();
        t.starts_with('/') || t.starts_with('!')
    }
}

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("{:x}", t)
}

// ─── Reply envelope ───────────────────────────────────────────────────────────

/// Outbound reply to be delivered back to the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyEnvelope {
    pub connector: String,
    pub chat_id: String,
    pub text: String,
    pub reply_to_id: Option<String>,
    pub parse_mode: Option<String>,
}

impl ReplyEnvelope {
    pub fn new(connector: impl Into<String>, chat_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            connector: connector.into(),
            chat_id: chat_id.into(),
            text: text.into(),
            reply_to_id: None,
            parse_mode: Some("Markdown".to_string()),
        }
    }

    pub fn in_reply_to(mut self, msg_id: impl Into<String>) -> Self {
        self.reply_to_id = Some(msg_id.into());
        self
    }

    pub fn plain(mut self) -> Self {
        self.parse_mode = None;
        self
    }
}

// ─── Command detection ────────────────────────────────────────────────────────

/// Parsed command from a message (e.g. `/help arg1 arg2`).
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
    pub raw: String,
}

impl ParsedCommand {
    pub fn parse(text: &str) -> Option<Self> {
        let t = text.trim();
        if !t.starts_with('/') && !t.starts_with('!') {
            return None;
        }
        let t = &t[1..]; // strip prefix
        let mut parts = t.split_whitespace();
        let name = parts.next()?.to_string();
        // Remove @bot_name suffix if present (e.g. /help@mybot)
        let name = name.split('@').next().unwrap_or(&name).to_lowercase();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();
        Some(ParsedCommand { name, args, raw: text.to_string() })
    }
}

// ─── Activation check ─────────────────────────────────────────────────────────

/// Determines whether the bot should respond to this envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivationResult {
    /// Bot should respond.
    Active,
    /// Not activated in this context.
    Inactive { reason: String },
}

/// Configuration for reply activation.
#[derive(Debug, Clone)]
pub struct ActivationConfig {
    /// Respond to direct messages regardless of mention.
    pub respond_to_dm: bool,
    /// Respond in groups only when mentioned.
    pub require_mention_in_groups: bool,
    /// Allowed sender IDs (* = any).
    pub allowed_senders: Vec<String>,
}

impl Default for ActivationConfig {
    fn default() -> Self {
        Self {
            respond_to_dm: true,
            require_mention_in_groups: true,
            allowed_senders: vec!["*".to_string()],
        }
    }
}

pub fn check_activation(env: &InboundEnvelope, cfg: &ActivationConfig) -> ActivationResult {
    // Sender allowlist check
    if !cfg.allowed_senders.iter().any(|s| s == "*" || s == &env.sender_id) {
        return ActivationResult::Inactive {
            reason: format!("sender {} not in allowlist", env.sender_id),
        };
    }

    // DM check
    if env.chat_type == "direct" {
        if cfg.respond_to_dm {
            return ActivationResult::Active;
        }
        return ActivationResult::Inactive { reason: "DM disabled".to_string() };
    }

    // Group check
    if cfg.require_mention_in_groups && !env.mentioned && !env.is_command() {
        return ActivationResult::Inactive {
            reason: "not mentioned in group".to_string(),
        };
    }

    ActivationResult::Active
}

// ─── Heartbeat ────────────────────────────────────────────────────────────────

/// A periodic heartbeat reply payload (status ping from the agent).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatPayload {
    pub agent_name: String,
    pub status: String,
    pub uptime_secs: u64,
    pub version: String,
}

impl HeartbeatPayload {
    pub fn new(agent_name: impl Into<String>, uptime_secs: u64) -> Self {
        Self {
            agent_name: agent_name.into(),
            status: "ok".to_string(),
            uptime_secs,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "💓 *{}* — `{}` | uptime: {}s | v{}",
            self.agent_name, self.status, self.uptime_secs, self.version
        )
    }
}

// ─── Dispatch result ──────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum DispatchResult {
    /// Reply to send back.
    Reply(ReplyEnvelope),
    /// Heartbeat ping to send.
    Heartbeat(HeartbeatPayload),
    /// Silently ignored.
    Ignored { reason: String },
    /// Error during processing.
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_parse() {
        let cmd = ParsedCommand::parse("/help arg1 arg2").unwrap();
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.args, vec!["arg1", "arg2"]);

        let cmd2 = ParsedCommand::parse("/status@mybot").unwrap();
        assert_eq!(cmd2.name, "status");

        assert!(ParsedCommand::parse("not a command").is_none());
    }

    #[test]
    fn activation_dm_allowed() {
        let mut env = InboundEnvelope::new("telegram", "u1", "c1", "hello");
        env.chat_type = "direct".to_string();
        let cfg = ActivationConfig::default();
        assert_eq!(check_activation(&env, &cfg), ActivationResult::Active);
    }

    #[test]
    fn activation_group_no_mention() {
        let mut env = InboundEnvelope::new("telegram", "u1", "g1", "hello there");
        env.chat_type = "group".to_string();
        env.mentioned = false;
        let cfg = ActivationConfig::default();
        match check_activation(&env, &cfg) {
            ActivationResult::Inactive { .. } => {}
            other => panic!("expected Inactive, got {:?}", other),
        }
    }

    #[test]
    fn activation_group_with_mention() {
        let mut env = InboundEnvelope::new("telegram", "u1", "g1", "@bot hello");
        env.chat_type = "group".to_string();
        env.mentioned = true;
        let cfg = ActivationConfig::default();
        assert_eq!(check_activation(&env, &cfg), ActivationResult::Active);
    }

    #[test]
    fn activation_blocked_sender() {
        let env = InboundEnvelope::new("telegram", "evil", "c1", "hack");
        let cfg = ActivationConfig {
            allowed_senders: vec!["alice".to_string()],
            ..Default::default()
        };
        match check_activation(&env, &cfg) {
            ActivationResult::Inactive { .. } => {}
            other => panic!("expected Inactive, got {:?}", other),
        }
    }

    #[test]
    fn heartbeat_format() {
        let hb = HeartbeatPayload::new("KrabBot", 300);
        let formatted = hb.format();
        assert!(formatted.contains("KrabBot"));
        assert!(formatted.contains("300s"));
    }

    #[test]
    fn inbound_is_command() {
        let env = InboundEnvelope::new("slack", "u", "c", "/status");
        assert!(env.is_command());
        let env2 = InboundEnvelope::new("slack", "u", "c", "hello");
        assert!(!env2.is_command());
    }
}
