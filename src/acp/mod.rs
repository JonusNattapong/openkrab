//! acp — Agent Communication Protocol types, client and server stubs.
//! Ported from `openclaw/src/acp/` (Phase 9).
//!
//! ACP provides a structured JSON-over-HTTP/WS protocol so external processes
//! (e.g. desktop apps, mobile apps) can communicate with a running krabkrab
//! agent instance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Core message types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AcpRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpMessage {
    pub role: AcpRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl AcpMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self { role: AcpRole::User, content: content.into(), name: None, tool_call_id: None }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: AcpRole::Assistant, content: content.into(), name: None, tool_call_id: None }
    }
    pub fn system(content: impl Into<String>) -> Self {
        Self { role: AcpRole::System, content: content.into(), name: None, tool_call_id: None }
    }
}

// ─── Session ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpSession {
    pub id: String,
    pub messages: Vec<AcpMessage>,
    pub metadata: HashMap<String, String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl AcpSession {
    pub fn new(id: impl Into<String>) -> Self {
        let now = unix_now();
        Self { id: id.into(), messages: Vec::new(), metadata: HashMap::new(), created_at: now, updated_at: now }
    }

    pub fn push(&mut self, msg: AcpMessage) {
        self.messages.push(msg);
        self.updated_at = unix_now();
    }

    pub fn last_user_message(&self) -> Option<&str> {
        self.messages.iter().rev()
            .find(|m| m.role == AcpRole::User)
            .map(|m| m.content.as_str())
    }
}

fn unix_now() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64
}

// ─── Commands ─────────────────────────────────────────────────────────────────

/// Commands the client can send to the ACP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AcpCommand {
    Chat { session_id: String, message: String },
    NewSession { session_id: Option<String> },
    EndSession { session_id: String },
    GetStatus,
    ListSessions,
    ClearSession { session_id: String },
}

/// Events the ACP server emits to clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AcpEvent {
    SessionCreated { session_id: String },
    SessionEnded { session_id: String },
    ChatReply { session_id: String, content: String, done: bool },
    Error { message: String },
    Status { version: String, sessions: usize, uptime_secs: u64 },
}

// ─── Request / Response (HTTP transport) ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpRequest {
    pub command: AcpCommand,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<AcpEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl AcpResponse {
    pub fn ok(event: AcpEvent) -> Self {
        Self { ok: true, event: Some(event), error: None, request_id: None }
    }
    pub fn err(msg: impl Into<String>) -> Self {
        Self { ok: false, event: None, error: Some(msg.into()), request_id: None }
    }
    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = Some(id.into());
        self
    }
}

// ─── Session mapper ───────────────────────────────────────────────────────────

/// Maps connector channel IDs to ACP session IDs.
#[derive(Debug, Default)]
pub struct SessionMapper {
    channel_to_session: HashMap<String, String>,
    session_to_channel: HashMap<String, String>,
}

impl SessionMapper {
    pub fn new() -> Self { Self::default() }

    pub fn map(&mut self, channel_id: impl Into<String>, session_id: impl Into<String>) {
        let ch = channel_id.into();
        let sess = session_id.into();
        self.channel_to_session.insert(ch.clone(), sess.clone());
        self.session_to_channel.insert(sess, ch);
    }

    pub fn session_for_channel(&self, channel_id: &str) -> Option<&str> {
        self.channel_to_session.get(channel_id).map(|s| s.as_str())
    }

    pub fn channel_for_session(&self, session_id: &str) -> Option<&str> {
        self.session_to_channel.get(session_id).map(|s| s.as_str())
    }

    pub fn remove_session(&mut self, session_id: &str) {
        if let Some(ch) = self.session_to_channel.remove(session_id) {
            self.channel_to_session.remove(&ch);
        }
    }
}

// ─── Event mapper ─────────────────────────────────────────────────────────────

/// Convert a connector inbound message string into an ACP command.
pub fn inbound_to_command(session_id: &str, text: &str) -> AcpCommand {
    AcpCommand::Chat { session_id: session_id.to_string(), message: text.to_string() }
}

/// Convert an ACP event into a plain text reply string.
pub fn event_to_reply_text(event: &AcpEvent) -> Option<String> {
    match event {
        AcpEvent::ChatReply { content, .. } => Some(content.clone()),
        AcpEvent::Error { message } => Some(format!("⚠️ Error: {}", message)),
        _ => None,
    }
}

// ─── Meta ─────────────────────────────────────────────────────────────────────

pub const ACP_VERSION: &str = "1.0";
pub const ACP_DEFAULT_PORT: u16 = 3284;
pub const ACP_DEFAULT_PATH: &str = "/acp";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_push_and_last_user_message() {
        let mut s = AcpSession::new("sess-1");
        s.push(AcpMessage::user("hello"));
        s.push(AcpMessage::assistant("hi there"));
        assert_eq!(s.last_user_message(), Some("hello"));
        assert_eq!(s.messages.len(), 2);
    }

    #[test]
    fn acp_response_ok_err() {
        let r = AcpResponse::ok(AcpEvent::Status { version: "1.0".into(), sessions: 0, uptime_secs: 10 });
        assert!(r.ok);
        let e = AcpResponse::err("bad request");
        assert!(!e.ok);
        assert_eq!(e.error.as_deref(), Some("bad request"));
    }

    #[test]
    fn session_mapper_roundtrip() {
        let mut sm = SessionMapper::new();
        sm.map("ch-1", "sess-1");
        assert_eq!(sm.session_for_channel("ch-1"), Some("sess-1"));
        assert_eq!(sm.channel_for_session("sess-1"), Some("ch-1"));
        sm.remove_session("sess-1");
        assert!(sm.session_for_channel("ch-1").is_none());
    }

    #[test]
    fn inbound_to_command_test() {
        let cmd = inbound_to_command("s1", "What's the weather?");
        match cmd {
            AcpCommand::Chat { session_id, message } => {
                assert_eq!(session_id, "s1");
                assert!(message.contains("weather"));
            }
            _ => panic!("unexpected"),
        }
    }

    #[test]
    fn event_to_reply_text_test() {
        let e = AcpEvent::ChatReply { session_id: "s1".into(), content: "hello".into(), done: true };
        assert_eq!(event_to_reply_text(&e), Some("hello".to_string()));
        let err = AcpEvent::Error { message: "oops".into() };
        assert!(event_to_reply_text(&err).unwrap().contains("oops"));
    }

    #[test]
    fn acp_message_constructors() {
        let u = AcpMessage::user("hi");
        assert_eq!(u.role, AcpRole::User);
        let a = AcpMessage::assistant("hello");
        assert_eq!(a.role, AcpRole::Assistant);
    }
}
