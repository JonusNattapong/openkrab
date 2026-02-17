use crate::common::{Message, UserId};

pub fn normalize_inbound(text: &str, channel: &str, user: &str) -> Message {
    Message {
        id: format!("slack:{channel}:{user}"),
        text: text.to_string(),
        from: Some(UserId(format!("slack:{user}"))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[slack] {text}")
}
