#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatType {
    Direct,
    Group,
    Channel,
}

pub fn normalize_chat_type(raw: Option<&str>) -> Option<ChatType> {
    let value = raw?.trim().to_lowercase();
    if value.is_empty() {
        return None;
    }
    match value.as_str() {
        "direct" | "dm" => Some(ChatType::Direct),
        "group" => Some(ChatType::Group),
        "channel" => Some(ChatType::Channel),
        _ => None,
    }
}
