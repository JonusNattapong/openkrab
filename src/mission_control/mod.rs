//! Mission Control - AI Agent Orchestration
//!
//! Provides work orchestration, agent management, approvals, and activity tracking.

pub mod activity;
pub mod agents;
pub mod approvals;
pub mod boards;
pub mod tasks;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardGroup {
    pub id: String,
    pub org_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub todo: String,
    pub in_progress: String,
    pub done: String,
    pub blocked: String,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self {
            todo: "todo".to_string(),
            in_progress: "in_progress".to_string(),
            done: "done".to_string(),
            blocked: "blocked".to_string(),
        }
    }
}
