use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{ChannelId, Direction, MessageId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub session_id: Option<crate::SessionId>,
    pub channel_id: ChannelId,
    pub chat_id: String,
    pub user_id: Option<String>,
    pub sender: UserId,
    pub recipient: UserId,
    pub content: MessageContent,
    pub direction: Direction,
    pub created_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    Text {
        text: String,
    },
    Image {
        url: Option<String>,
        data: Option<Vec<u8>>,
        caption: Option<String>,
    },
    Video {
        url: Option<String>,
        data: Option<Vec<u8>>,
        caption: Option<String>,
    },
    Audio {
        url: Option<String>,
        data: Option<Vec<u8>>,
        duration: Option<u32>,
    },
    File {
        name: String,
        data: Vec<u8>,
        mime_type: String,
    },
    Sticker {
        data: Vec<u8>,
    },
    Location {
        latitude: f64,
        longitude: f64,
    },
    Reaction {
        emoji: String,
        target_message_id: MessageId,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageMetadata {
    pub reply_to: Option<MessageId>,
    pub mentions: Vec<UserId>,
    pub is_forwarded: bool,
    pub is_reply: bool,
}

impl Message {
    pub fn new_text(
        channel_id: ChannelId,
        sender: UserId,
        recipient: UserId,
        text: String,
    ) -> Self {
        Self {
            id: MessageId::new(),
            session_id: None,
            channel_id,
            chat_id: String::new(),
            user_id: None,
            sender,
            recipient,
            content: MessageContent::Text { text },
            direction: Direction::Inbound,
            created_at: Utc::now(),
            metadata: serde_json::Value::default(),
        }
    }
}
