//! acp — Agent Communication Protocol types and in-process runtime.
//! Ported from `openkrab/src/acp/` (Phase 9).
//!
//! ACP provides a structured JSON-over-HTTP/WS protocol so external processes
//! (e.g. desktop apps, mobile apps) can communicate with a running krabkrab
//! agent instance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

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
        Self {
            role: AcpRole::User,
            content: content.into(),
            name: None,
            tool_call_id: None,
        }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: AcpRole::Assistant,
            content: content.into(),
            name: None,
            tool_call_id: None,
        }
    }
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: AcpRole::System,
            content: content.into(),
            name: None,
            tool_call_id: None,
        }
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
        Self {
            id: id.into(),
            messages: Vec::new(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn push(&mut self, msg: AcpMessage) {
        self.messages.push(msg);
        self.updated_at = unix_now();
    }

    pub fn last_user_message(&self) -> Option<&str> {
        self.messages
            .iter()
            .rev()
            .find(|m| m.role == AcpRole::User)
            .map(|m| m.content.as_str())
    }
}

fn unix_now() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
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
    SessionCreated {
        session_id: String,
    },
    SessionEnded {
        session_id: String,
    },
    ChatReply {
        session_id: String,
        content: String,
        done: bool,
    },
    Error {
        message: String,
    },
    Status {
        version: String,
        sessions: usize,
        uptime_secs: u64,
    },
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
        Self {
            ok: true,
            event: Some(event),
            error: None,
            request_id: None,
        }
    }
    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            event: None,
            error: Some(msg.into()),
            request_id: None,
        }
    }
    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = Some(id.into());
        self
    }
}

// ─── Runtime ──────────────────────────────────────────────────────────────────

/// In-process ACP runtime handling request/response workflow.
pub struct AcpRuntime {
    sessions: RwLock<HashMap<String, AcpSession>>,
    started_at: Instant,
    auth_token: Option<String>,
    agent: Option<Arc<crate::agents::Agent>>,
}

impl Default for AcpRuntime {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl AcpRuntime {
    pub fn new(auth_token: Option<String>, agent: Option<Arc<crate::agents::Agent>>) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            started_at: Instant::now(),
            auth_token,
            agent,
        }
    }

    pub fn with_agent(agent: Arc<crate::agents::Agent>) -> Self {
        Self::new(None, Some(agent))
    }

    pub async fn handle_request(&self, req: AcpRequest) -> AcpResponse {
        if let Some(expected) = &self.auth_token {
            if req.auth_token.as_deref() != Some(expected.as_str()) {
                let err = AcpResponse::err("unauthorized");
                return if let Some(id) = req.request_id {
                    err.with_request_id(id)
                } else {
                    err
                };
            }
        }

        let rid = req.request_id.clone();
        let resp = match req.command {
            AcpCommand::NewSession { session_id } => {
                let sid = session_id.unwrap_or_else(|| format!("acp-{}", uuid::Uuid::new_v4()));
                let mut sessions = self.sessions.write().await;
                sessions
                    .entry(sid.clone())
                    .or_insert_with(|| AcpSession::new(sid.clone()));
                AcpResponse::ok(AcpEvent::SessionCreated { session_id: sid })
            }
            AcpCommand::EndSession { session_id } => {
                let mut sessions = self.sessions.write().await;
                if sessions.remove(&session_id).is_some() {
                    AcpResponse::ok(AcpEvent::SessionEnded { session_id })
                } else {
                    AcpResponse::err(format!("session not found: {session_id}"))
                }
            }
            AcpCommand::ClearSession { session_id } => {
                let mut sessions = self.sessions.write().await;
                match sessions.get_mut(&session_id) {
                    Some(s) => {
                        s.messages.clear();
                        s.updated_at = unix_now();
                        AcpResponse::ok(AcpEvent::Status {
                            version: ACP_VERSION.to_string(),
                            sessions: sessions.len(),
                            uptime_secs: self.started_at.elapsed().as_secs(),
                        })
                    }
                    None => AcpResponse::err(format!("session not found: {session_id}")),
                }
            }
            AcpCommand::ListSessions => {
                let sessions = self.sessions.read().await;
                let list = sessions.keys().cloned().collect::<Vec<_>>().join(",");
                AcpResponse::ok(AcpEvent::ChatReply {
                    session_id: "_system".to_string(),
                    content: list,
                    done: true,
                })
            }
            AcpCommand::GetStatus => {
                let sessions = self.sessions.read().await;
                AcpResponse::ok(AcpEvent::Status {
                    version: ACP_VERSION.to_string(),
                    sessions: sessions.len(),
                    uptime_secs: self.started_at.elapsed().as_secs(),
                })
            }
            AcpCommand::Chat {
                session_id,
                message,
            } => self.handle_chat(session_id, message).await,
        };

        if let Some(id) = rid {
            resp.with_request_id(id)
        } else {
            resp
        }
    }

    async fn handle_chat(&self, session_id: String, message: String) -> AcpResponse {
        {
            let mut sessions = self.sessions.write().await;
            let session = sessions
                .entry(session_id.clone())
                .or_insert_with(|| AcpSession::new(session_id.clone()));
            session.push(AcpMessage::user(message.clone()));
        }

        let reply = if let Some(agent) = &self.agent {
            match agent.answer(&message).await {
                Ok(r) => r,
                Err(e) => {
                    return AcpResponse::err(format!("agent error: {}", e));
                }
            }
        } else {
            format!("[acp] {}", message)
        };

        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(&session_id) {
                session.push(AcpMessage::assistant(reply.clone()));
            }
        }

        AcpResponse::ok(AcpEvent::ChatReply {
            session_id,
            content: reply,
            done: true,
        })
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
    pub fn new() -> Self {
        Self::default()
    }

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
    AcpCommand::Chat {
        session_id: session_id.to_string(),
        message: text.to_string(),
    }
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
        let r = AcpResponse::ok(AcpEvent::Status {
            version: "1.0".into(),
            sessions: 0,
            uptime_secs: 10,
        });
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
            AcpCommand::Chat {
                session_id,
                message,
            } => {
                assert_eq!(session_id, "s1");
                assert!(message.contains("weather"));
            }
            _ => panic!("unexpected"),
        }
    }

    #[test]
    fn event_to_reply_text_test() {
        let e = AcpEvent::ChatReply {
            session_id: "s1".into(),
            content: "hello".into(),
            done: true,
        };
        assert_eq!(event_to_reply_text(&e), Some("hello".to_string()));
        let err = AcpEvent::Error {
            message: "oops".into(),
        };
        assert!(event_to_reply_text(&err).unwrap().contains("oops"));
    }

    #[test]
    fn acp_message_constructors() {
        let u = AcpMessage::user("hi");
        assert_eq!(u.role, AcpRole::User);
        let a = AcpMessage::assistant("hello");
        assert_eq!(a.role, AcpRole::Assistant);
    }

    #[tokio::test]
    async fn runtime_new_chat_end_session() {
        let runtime = AcpRuntime::default();

        let created = runtime
            .handle_request(AcpRequest {
                command: AcpCommand::NewSession {
                    session_id: Some("s1".into()),
                },
                auth_token: None,
                request_id: Some("r1".into()),
            })
            .await;
        assert!(created.ok);
        assert_eq!(created.request_id.as_deref(), Some("r1"));

        let reply = runtime
            .handle_request(AcpRequest {
                command: AcpCommand::Chat {
                    session_id: "s1".into(),
                    message: "hello".into(),
                },
                auth_token: None,
                request_id: None,
            })
            .await;
        assert!(reply.ok);
        match reply.event {
            Some(AcpEvent::ChatReply { session_id, .. }) => assert_eq!(session_id, "s1"),
            _ => panic!("expected chat reply"),
        }

        let ended = runtime
            .handle_request(AcpRequest {
                command: AcpCommand::EndSession {
                    session_id: "s1".into(),
                },
                auth_token: None,
                request_id: None,
            })
            .await;
        assert!(ended.ok);
    }

    #[tokio::test]
    async fn runtime_auth_rejects_invalid_token() {
        let runtime = AcpRuntime::new(Some("secret".into()), None);
        let resp = runtime
            .handle_request(AcpRequest {
                command: AcpCommand::GetStatus,
                auth_token: Some("bad".into()),
                request_id: Some("req-1".into()),
            })
            .await;
        assert!(!resp.ok);
        assert_eq!(resp.error.as_deref(), Some("unauthorized"));
        assert_eq!(resp.request_id.as_deref(), Some("req-1"));
    }
}
