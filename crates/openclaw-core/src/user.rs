use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User information across channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub channel_id: String,
    pub channel_user_id: String,
    pub global_user_id: Option<String>,
    pub display_name: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        channel_id: impl Into<String>,
        channel_user_id: impl Into<String>,
        display_name: impl Into<String>,
    ) -> Self {
        Self {
            channel_id: channel_id.into(),
            channel_user_id: channel_user_id.into(),
            global_user_id: None,
            display_name: display_name.into(),
            metadata: serde_json::Value::Null,
            created_at: Utc::now(),
        }
    }
}
