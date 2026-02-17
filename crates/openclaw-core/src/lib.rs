//! OpenClaw Core Library
//!
//! Core types, traits, and utilities for the OpenClaw multi-channel AI gateway.

pub mod channel;
pub mod chat;
pub mod config;
pub mod error;
pub mod media;
pub mod message;
pub mod routing;
pub mod session;
pub mod tools;
pub mod user;

// Re-export commonly used core modules. Avoid broad `pub use ...::*` for modules that
// define overlapping type names (eg. `ChannelConfig` exists in both `channel` and
// `config`) to prevent ambiguous glob re-export warnings. Import modules directly
// via their paths (eg. `openclaw_core::channel::ChannelConfig`) where needed.
pub use error::*;
pub use message::*;
pub use session::*;
pub use user::*;

// Suppress ambiguous glob re-export warnings for other modules by keeping their
// exports explicit in-place where required. This reduces accidental symbol
// collisions across the workspace.

// ID types are declared below; avoid re-exporting crate::... which causes duplicate definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId(pub Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for sessions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChannelId(pub Uuid);

impl ChannelId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ChannelId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for users
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Allow constructing IDs from stable string forms (used by storage layer)
impl From<String> for MessageId {
    fn from(s: String) -> Self {
        match Uuid::parse_str(&s) {
            Ok(u) => MessageId(u),
            Err(_) => MessageId::new(),
        }
    }
}

impl From<String> for SessionId {
    fn from(s: String) -> Self {
        match Uuid::parse_str(&s) {
            Ok(u) => SessionId(u),
            Err(_) => SessionId::new(),
        }
    }
}

impl From<String> for ChannelId {
    fn from(s: String) -> Self {
        match Uuid::parse_str(&s) {
            Ok(u) => ChannelId(u),
            Err(_) => ChannelId::new(),
        }
    }
}

impl From<String> for UserId {
    fn from(s: String) -> Self {
        match Uuid::parse_str(&s) {
            Ok(u) => UserId(u),
            Err(_) => UserId::new(),
        }
    }
}

/// Direction of message flow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Inbound,
    Outbound,
    Incoming,
    Outgoing,
}

/// Channel types supported by OpenClaw
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    Telegram,
    Discord,
    Slack,
    WhatsApp,
    Signal,
    Web,
    Matrix,
}

/// Timestamp with timezone
pub type Timestamp = DateTime<Utc>;

/// Metadata attached to entities
pub type Metadata = std::collections::HashMap<String, serde_json::Value>;

/// Generic result type
pub type Result<T> = std::result::Result<T, OpenClawError>;
