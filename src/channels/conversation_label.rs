use crate::channels::chat_type::ChatType;

fn extract_conversation_id(from: Option<&str>) -> Option<String> {
    let trimmed = from?.trim();
    if trimmed.is_empty() {
        return None;
    }
    let parts: Vec<&str> = trimmed.split(':').filter(|s| !s.is_empty()).collect();
    if !parts.is_empty() {
        Some(parts[parts.len() - 1].to_string())
    } else {
        Some(trimmed.to_string())
    }
}

fn should_append_id(id: &str) -> bool {
    if id.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    if id.contains("@g.us") {
        return true;
    }
    false
}

#[derive(Debug, Default)]
pub struct MsgContext {
    pub conversation_label: Option<String>,
    pub thread_label: Option<String>,
    pub chat_type: Option<String>,
    pub sender_name: Option<String>,
    pub from: Option<String>,
    pub group_channel: Option<String>,
    pub group_subject: Option<String>,
    pub group_space: Option<String>,
}

pub fn resolve_conversation_label(ctx: &MsgContext) -> Option<String> {
    if let Some(explicit) = ctx
        .conversation_label
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        return Some(explicit.to_string());
    }
    if let Some(thread_label) = ctx
        .thread_label
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        return Some(thread_label.to_string());
    }

    let chat_type = crate::channels::chat_type::normalize_chat_type(ctx.chat_type.as_deref());
    if matches!(chat_type, Some(ChatType::Direct)) {
        if let Some(name) = ctx
            .sender_name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            return Some(name.to_string());
        }
        if let Some(from) = ctx
            .from
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            return Some(from.to_string());
        }
        return None;
    }

    // Build base from group/channel/space/from in order
    let base = if let Some(v) = ctx
        .group_channel
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        v.to_string()
    } else if let Some(v) = ctx
        .group_subject
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        v.to_string()
    } else if let Some(v) = ctx
        .group_space
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        v.to_string()
    } else if let Some(v) = ctx
        .from
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        v.to_string()
    } else {
        String::new()
    };

    if base.is_empty() {
        return None;
    }

    let id = extract_conversation_id(ctx.from.as_deref());
    if id.is_none() {
        return Some(base);
    }
    let id = id.unwrap();
    if !should_append_id(&id) {
        return Some(base);
    }
    if base == id {
        return Some(base);
    }
    if base.contains(&id) {
        return Some(base);
    }
    if base.to_lowercase().contains(" id:") {
        return Some(base);
    }
    if base.starts_with('#') || base.starts_with('@') {
        return Some(base);
    }
    Some(format!("{} id:{}", base, id))
}
