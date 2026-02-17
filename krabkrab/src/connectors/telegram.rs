use crate::common::Message;

pub fn normalize_inbound(text: &str, chat_id: i64, user_id: i64) -> Message {
    Message {
        id: format!("tg:{chat_id}:{user_id}"),
        text: text.to_string(),
        from: Some(crate::common::UserId(format!("tg:{user_id}"))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[telegram] {text}")
}
