//! Channel subsystem (partial port from openclaw/src/channels)

pub mod registry;
pub mod session;
pub mod channel_config;
pub mod command_gating;
pub mod allowlist_match;
pub mod mention_gating;
pub mod reply_prefix;
pub mod sender_identity;
pub mod chat_type;
pub mod sender_label;
pub mod conversation_label;
pub mod logging;
pub mod location;
pub mod typing;
pub mod draft_stream_loop;
pub mod ack_reactions;
pub mod targets;

// Additional modules will be added incrementally as porting progresses.

pub use registry::Registry;
pub use session::Session;
pub use channel_config::ChannelConfig;
// mention_gating module is intentionally not glob-exported; use via `channels::mention_gating`.
// reply_prefix module available via `channels::reply_prefix`.
// sender_identity available via `channels::sender_identity`.
// chat_type available via `channels::chat_type`.
