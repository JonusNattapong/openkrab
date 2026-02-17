use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ChannelId, MessageId, SessionId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub name: String,
    pub channel_id: ChannelId,
    pub user_id: String,
    pub chat_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub config: SessionConfig,
    pub context: SessionContext,
    pub state: SessionState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub model: Option<String>,
    pub sandbox: SandboxConfig,
    pub reply_back: bool,
    pub queue_mode: QueueMode,
    pub auto_react: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub mode: SandboxMode,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SandboxMode {
    Host,
    NonMain,
    Docker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueMode {
    Parallel,
    Sequential,
    Batch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Active,
    Inactive,
    Closed,
}

impl Default for SessionState {
    fn default() -> Self {
        SessionState::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionContext {
    pub history: Vec<MessageSummary>,
    pub variables: HashMap<String, String>,
    pub state: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSummary {
    pub id: MessageId,
    pub sender_id: String,
    pub content_preview: String,
    pub timestamp: DateTime<Utc>,
    pub direction: crate::Direction,
}

impl Session {
    pub fn new(channel_id: ChannelId, name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: crate::SessionId::new(),
            name: name.unwrap_or_else(|| "default".to_string()),
            channel_id,
            user_id: String::new(),
            chat_id: String::new(),
            created_at: now,
            updated_at: now,
            last_activity_at: now,
            config: SessionConfig::default(),
            context: SessionContext::default(),
            state: SessionState::Active,
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity_at = Utc::now();
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            model: None,
            sandbox: SandboxConfig {
                mode: SandboxMode::Host,
                timeout_seconds: 30,
            },
            reply_back: true,
            queue_mode: QueueMode::Parallel,
            auto_react: false,
        }
    }
}
