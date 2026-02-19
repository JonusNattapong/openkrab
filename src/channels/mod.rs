//! Channel subsystem (partial port from openclaw/src/channels)

pub mod account_summary;
pub mod ack_reactions;
pub mod allowlist_match;
pub mod channel_config;
pub mod chat_type;
pub mod command_gating;
pub mod conversation_label;
pub mod dock;
pub mod draft_stream_loop;
pub mod location;
pub mod logging;
pub mod mention_gating;
pub mod registry;
pub mod reply_prefix;
pub mod sender_identity;
pub mod sender_label;
pub mod session;
pub mod targets;
pub mod typing;

/// Common message handler trait for all connectors
#[async_trait::async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle_message(&self, message: serde_json::Value) -> Result<()>;
}

/// Monitor options for connector monitoring
#[derive(Debug, Clone, Default)]
pub struct MonitorOptions {
    pub account_id: Option<String>,
    pub verbose: bool,
    pub heartbeat_seconds: Option<u64>,
}

// Additional modules will be added incrementally as porting progresses.

pub use channel_config::ChannelConfig;
pub use registry::Registry;
pub use session::Session;
// mention_gating module is intentionally not glob-exported; use via `channels::mention_gating`.
// reply_prefix module available via `channels::reply_prefix`.
// sender_identity available via `channels::sender_identity`.
// chat_type available via `channels::chat_type`.
