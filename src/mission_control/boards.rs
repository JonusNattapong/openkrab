//! Board management for Mission Control

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub description: Option<String>,
    pub status_columns: Vec<String>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            id: generate_id(),
            name: "Default Board".to_string(),
            group_id: String::new(),
            description: None,
            status_columns: vec![
                "todo".to_string(),
                "in_progress".to_string(),
                "done".to_string(),
                "blocked".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{:x}", timestamp)
}
