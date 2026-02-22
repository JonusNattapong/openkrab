//! Activity timeline for Mission Control

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub actor: String,
    pub action: String,
    pub target_type: String,
    pub target_id: String,
    pub details: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLog {
    pub activities: Vec<Activity>,
}

impl Default for ActivityLog {
    fn default() -> Self {
        Self {
            activities: Vec::new(),
        }
    }
}
