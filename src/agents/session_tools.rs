//! session_tools — Agent-accessible session management tools.
//! Ported from `openclaw/src/agents/tools/sessions-*.ts`.
//!
//! Provides tools for agents to spawn, send to, list, and view history of sessions.

use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::agents::tool::{Tool, ToolDefinition};
use crate::sessions::{SessionRegistry, TranscriptEntry};

// ─── Shared session registry ref ──────────────────────────────────────────────

pub type SharedSessionRegistry = Arc<RwLock<SessionRegistry>>;

// ─── Sessions List Tool ───────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SessionsListTool {
    registry: SharedSessionRegistry,
}

impl SessionsListTool {
    pub fn new(registry: SharedSessionRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for SessionsListTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "sessions_list".to_string(),
            description: "List all active sessions. Returns session IDs, labels, models, and activity timestamps.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "filter": {
                        "type": "string",
                        "description": "Optional filter string to match against session ID or label"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of sessions to return (default: 50)"
                    }
                }
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments).unwrap_or(Value::Null);
        let filter = args.get("filter").and_then(|v| v.as_str()).unwrap_or("");
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;

        let registry = self.registry.read().await;
        let sessions: Vec<SessionSummary> = registry
            .iter()
            .filter(|(id, session)| {
                if filter.is_empty() {
                    return true;
                }
                let filter_lower = filter.to_lowercase();
                id.to_lowercase().contains(&filter_lower)
                    || session
                        .label
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&filter_lower)
            })
            .take(limit)
            .map(|(id, session)| SessionSummary {
                id: id.clone(),
                label: session.label.clone(),
                model_override: session.model_override.clone(),
                message_count: session.transcript.len(),
                created_at: session.created_at.to_rfc3339(),
                last_active: session.last_active.to_rfc3339(),
                elevated: session.elevated,
            })
            .collect();

        Ok(serde_json::to_string_pretty(&sessions)?)
    }
}

#[derive(Serialize)]
struct SessionSummary {
    id: String,
    label: Option<String>,
    model_override: Option<String>,
    message_count: usize,
    created_at: String,
    last_active: String,
    elevated: bool,
}

// ─── Sessions History Tool ────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SessionsHistoryTool {
    registry: SharedSessionRegistry,
}

impl SessionsHistoryTool {
    pub fn new(registry: SharedSessionRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for SessionsHistoryTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "sessions_history".to_string(),
            description: "View the conversation history of a specific session.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "The session ID to retrieve history for"
                    },
                    "last_n": {
                        "type": "integer",
                        "description": "Number of recent messages to return (default: 20)"
                    }
                },
                "required": ["session_id"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let session_id = args
            .get("session_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("session_id is required"))?;
        let last_n = args.get("last_n").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

        let registry = self.registry.read().await;
        match registry.get(session_id) {
            Some(session) => {
                let messages: Vec<HistoryEntry> = session
                    .recent_messages(last_n)
                    .into_iter()
                    .map(|(role, text)| HistoryEntry {
                        role: role.to_string(),
                        text: if text.len() > 2000 {
                            format!("{}… (truncated)", &text[..2000])
                        } else {
                            text.to_string()
                        },
                    })
                    .collect();
                Ok(serde_json::to_string_pretty(&messages)?)
            }
            None => Ok(format!(
                "{{\"error\": \"Session '{}' not found\"}}",
                session_id
            )),
        }
    }
}

#[derive(Serialize)]
struct HistoryEntry {
    role: String,
    text: String,
}

// ─── Sessions Send Tool ───────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SessionsSendTool {
    registry: SharedSessionRegistry,
}

impl SessionsSendTool {
    pub fn new(registry: SharedSessionRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for SessionsSendTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "sessions_send".to_string(),
            description: "Send a message to an existing session. The message will be appended to the session's transcript.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "The target session ID"
                    },
                    "message": {
                        "type": "string",
                        "description": "The message text to send"
                    },
                    "role": {
                        "type": "string",
                        "description": "Message role: 'user', 'assistant', or 'system' (default: 'user')",
                        "enum": ["user", "assistant", "system"]
                    }
                },
                "required": ["session_id", "message"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let session_id = args
            .get("session_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("session_id is required"))?;
        let message = args
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("message is required"))?;
        let role = args
            .get("role")
            .and_then(|v| v.as_str())
            .unwrap_or("user");

        let mut registry = self.registry.write().await;
        let session = registry.get_or_create(session_id);

        let entry = match role {
            "assistant" => TranscriptEntry::assistant(message),
            "system" => TranscriptEntry::system(message),
            _ => TranscriptEntry::user(message),
        };

        session.append_transcript(entry);

        Ok(serde_json::json!({
            "success": true,
            "session_id": session_id,
            "message_count": session.transcript.len()
        })
        .to_string())
    }
}

// ─── Sessions Spawn Tool ──────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SessionsSpawnTool {
    registry: SharedSessionRegistry,
}

impl SessionsSpawnTool {
    pub fn new(registry: SharedSessionRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for SessionsSpawnTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "sessions_spawn".to_string(),
            description: "Spawn a new session (or reuse existing) with a specific configuration. Use this to create isolated conversation contexts.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "ID for the new session. If it already exists, returns the existing session."
                    },
                    "label": {
                        "type": "string",
                        "description": "Human-readable label for the session"
                    },
                    "model": {
                        "type": "string",
                        "description": "Model override for this session"
                    },
                    "system_message": {
                        "type": "string",
                        "description": "Initial system message to prepend"
                    },
                    "initial_message": {
                        "type": "string",
                        "description": "Initial user message to start the conversation"
                    }
                },
                "required": ["session_id"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let session_id = args
            .get("session_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("session_id is required"))?;

        let mut registry = self.registry.write().await;
        let is_new = registry.get(session_id).is_none();
        let session = registry.get_or_create(session_id);

        if is_new {
            if let Some(label) = args.get("label").and_then(|v| v.as_str()) {
                session.label = Some(label.to_string());
            }
            if let Some(model) = args.get("model").and_then(|v| v.as_str()) {
                session.model_override = Some(model.to_string());
            }
            if let Some(sys_msg) = args.get("system_message").and_then(|v| v.as_str()) {
                session.append_transcript(TranscriptEntry::system(sys_msg));
            }
            if let Some(init_msg) = args.get("initial_message").and_then(|v| v.as_str()) {
                session.append_transcript(TranscriptEntry::user(init_msg));
            }
        }

        Ok(serde_json::json!({
            "success": true,
            "session_id": session_id,
            "is_new": is_new,
            "label": session.label,
            "model_override": session.model_override,
            "message_count": session.transcript.len()
        })
        .to_string())
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    async fn make_registry() -> SharedSessionRegistry {
        let mut reg = SessionRegistry::new();
        {
            let s = reg.get_or_create("test-session");
            s.label = Some("Test Session".to_string());
            s.append_transcript(TranscriptEntry::user("Hello"));
            s.append_transcript(TranscriptEntry::assistant("Hi there!"));
        }
        Arc::new(RwLock::new(reg))
    }

    #[tokio::test]
    async fn list_sessions() {
        let registry = make_registry().await;
        let tool = SessionsListTool::new(registry);
        let result = tool.call("{}").await.unwrap();
        assert!(result.contains("test-session"));
        assert!(result.contains("Test Session"));
    }

    #[tokio::test]
    async fn list_sessions_with_filter() {
        let registry = make_registry().await;
        let tool = SessionsListTool::new(registry);
        let result = tool
            .call(r#"{"filter": "test"}"#)
            .await
            .unwrap();
        assert!(result.contains("test-session"));
    }

    #[tokio::test]
    async fn history_existing() {
        let registry = make_registry().await;
        let tool = SessionsHistoryTool::new(registry);
        let result = tool
            .call(r#"{"session_id": "test-session"}"#)
            .await
            .unwrap();
        assert!(result.contains("Hello"));
        assert!(result.contains("Hi there!"));
    }

    #[tokio::test]
    async fn history_missing() {
        let registry = make_registry().await;
        let tool = SessionsHistoryTool::new(registry);
        let result = tool
            .call(r#"{"session_id": "nonexistent"}"#)
            .await
            .unwrap();
        assert!(result.contains("not found"));
    }

    #[tokio::test]
    async fn send_to_session() {
        let registry = make_registry().await;
        let tool = SessionsSendTool::new(registry.clone());
        let result = tool
            .call(r#"{"session_id": "test-session", "message": "new message"}"#)
            .await
            .unwrap();
        assert!(result.contains("success"));

        let reg = registry.read().await;
        let session = reg.get("test-session").unwrap();
        assert_eq!(session.transcript.len(), 3);
    }

    #[tokio::test]
    async fn send_creates_session() {
        let registry = make_registry().await;
        let tool = SessionsSendTool::new(registry.clone());
        let result = tool
            .call(r#"{"session_id": "new-session", "message": "first!"}"#)
            .await
            .unwrap();
        assert!(result.contains("success"));

        let reg = registry.read().await;
        assert!(reg.get("new-session").is_some());
    }

    #[tokio::test]
    async fn spawn_new_session() {
        let registry = make_registry().await;
        let tool = SessionsSpawnTool::new(registry.clone());
        let result = tool
            .call(r#"{
                "session_id": "spawn-test",
                "label": "Spawned",
                "model": "gpt-4o",
                "system_message": "You are helpful.",
                "initial_message": "Hello!"
            }"#)
            .await
            .unwrap();
        assert!(result.contains("is_new"));
        assert!(result.contains("true"));

        let reg = registry.read().await;
        let session = reg.get("spawn-test").unwrap();
        assert_eq!(session.label, Some("Spawned".to_string()));
        assert_eq!(session.model_override, Some("gpt-4o".to_string()));
        assert_eq!(session.transcript.len(), 2);
    }

    #[tokio::test]
    async fn spawn_existing_no_overwrite() {
        let registry = make_registry().await;
        let tool = SessionsSpawnTool::new(registry.clone());
        let result = tool
            .call(r#"{"session_id": "test-session", "model": "gpt-4o-mini"}"#)
            .await
            .unwrap();
        assert!(result.contains("\"is_new\":false") || result.contains("\"is_new\": false"));

        // Should not overwrite existing session's model
        let reg = registry.read().await;
        let session = reg.get("test-session").unwrap();
        assert_eq!(session.model_override, None);
    }
}
