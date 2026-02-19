use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gateway session defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewaySessionsDefaults {
    pub model_provider: Option<String>,
    pub model: Option<String>,
    pub context_tokens: Option<u32>,
}

/// Gateway session row (simplified from TypeScript)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewaySessionRow {
    pub key: String,
    pub kind: SessionKind,
    pub label: Option<String>,
    pub display_name: Option<String>,
    pub derived_title: Option<String>,
    pub last_message_preview: Option<String>,
    pub channel: Option<String>,
    pub subject: Option<String>,
    pub group_channel: Option<String>,
    pub space: Option<String>,
    pub chat_type: Option<String>,
    pub updated_at: Option<i64>,
    pub session_id: Option<String>,
    pub system_sent: Option<bool>,
    pub aborted_last_run: Option<bool>,
    pub thinking_level: Option<String>,
    pub verbose_level: Option<String>,
    pub reasoning_level: Option<String>,
    pub elevated_level: Option<String>,
    pub send_policy: Option<SendPolicy>,
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub total_tokens_fresh: Option<bool>,
    pub response_usage: Option<ResponseUsage>,
    pub model_provider: Option<String>,
    pub model: Option<String>,
    pub context_tokens: Option<u32>,
    pub last_channel: Option<String>,
    pub last_to: Option<String>,
    pub last_account_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionKind {
    Direct,
    Group,
    Global,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SendPolicy {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseUsage {
    On,
    Off,
    Tokens,
    Full,
}

/// Gateway agent row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayAgentRow {
    pub id: String,
    pub name: Option<String>,
    pub identity: Option<AgentIdentity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub name: Option<String>,
    pub theme: Option<String>,
    pub emoji: Option<String>,
    pub avatar: Option<String>,
    pub avatar_url: Option<String>,
}

/// Session preview types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPreviewItem {
    pub role: MessageRole,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    Tool,
    System,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionsPreviewEntry {
    pub key: String,
    pub status: PreviewStatus,
    pub items: Vec<SessionPreviewItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreviewStatus {
    Ok,
    Empty,
    Missing,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionsPreviewResult {
    pub ts: i64,
    pub previews: Vec<SessionsPreviewEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionsListResult {
    pub ts: i64,
    pub path: String,
    pub count: usize,
    pub defaults: GatewaySessionsDefaults,
    pub sessions: Vec<GatewaySessionRow>,
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum GatewayMessage {
    #[serde(rename = "hello")]
    Hello {
        client_id: String,
        version: String,
        capabilities: Vec<String>,
    },

    #[serde(rename = "chat")]
    Chat {
        session_key: String,
        message: String,
        attachments: Option<Vec<Attachment>>,
    },

    #[serde(rename = "status")]
    Status {
        sessions: Vec<GatewaySessionRow>,
        agents: Vec<GatewayAgentRow>,
    },

    #[serde(rename = "error")]
    Error { code: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub kind: String,
    pub url: Option<String>,
    pub data: Option<String>,
    pub mime_type: Option<String>,
    pub filename: Option<String>,
}

/// Authentication types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: Option<i64>,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
}

/// Rate limiting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub burst_limit: u32,
}

/// Health monitoring types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelHealth {
    pub channel: String,
    pub status: ChannelStatus,
    pub last_check: i64,
    pub error_message: Option<String>,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Plugin types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub config: HashMap<String, serde_json::Value>,
}

/// Node event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEvent {
    pub node_id: String,
    pub event_type: NodeEventType,
    pub timestamp: i64,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeEventType {
    Connected,
    Disconnected,
    MessageReceived,
    CommandExecuted,
    Error,
}
