//! Agent management for Mission Control

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedAgent {
    pub id: String,
    pub name: String,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub status: AgentStatus,
    pub channels: Vec<String>,
    pub created_at: i64,
    pub last_active: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Active,
    Idle,
    Error,
    Stopped,
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self::Idle
    }
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStatus::Active => write!(f, "active"),
            AgentStatus::Idle => write!(f, "idle"),
            AgentStatus::Error => write!(f, "error"),
            AgentStatus::Stopped => write!(f, "stopped"),
        }
    }
}
