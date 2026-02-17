#[derive(Debug, Clone)]
pub struct WebListenerCloseReason {
    pub status: Option<u16>,
    pub is_logged_out: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum WebChatType {
    Direct,
    Group,
}

#[derive(Debug, Clone)]
pub struct WebInboundMessage {
    pub id: Option<String>,
    pub from: String,
    pub conversation_id: String,
    pub to: String,
    pub account_id: String,
    pub body: String,
    pub push_name: Option<String>,
    pub timestamp: Option<i64>,
    pub chat_type: WebChatType,
    pub chat_id: String,
    pub sender_jid: Option<String>,
    pub sender_e164: Option<String>,
    pub sender_name: Option<String>,
    pub mentioned_jids: Vec<String>,
    pub media_path: Option<String>,
    pub media_type: Option<String>,
    pub media_file_name: Option<String>,
    pub media_url: Option<String>,
}
