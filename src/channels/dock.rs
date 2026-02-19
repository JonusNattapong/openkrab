use serde_json::Value;

// Minimal `dock` port: provide `format_lower` and `normalize_target` helpers and
// `build_direct_or_group_thread_tool_context` equivalent for shared use.

pub fn format_lower(allow_from: &[String]) -> Vec<String> {
    allow_from
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

pub fn build_direct_or_group_thread_tool_context(
    context: &serde_json::Map<String, Value>,
    has_replied_ref: Option<String>,
) -> (Option<String>, Option<String>, Option<String>) {
    let chat_type = context
        .get("ChatType")
        .and_then(|v| v.as_str())
        .map(|s| s.to_lowercase());
    let is_direct = chat_type.as_deref() == Some("direct");
    let channel_id = if is_direct {
        context
            .get("From")
            .and_then(|v| v.as_str())
            .or_else(|| context.get("To").and_then(|v| v.as_str()))
            .map(|s| s.trim().to_string())
    } else {
        context
            .get("To")
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string())
    };
    let thread_ts = context
        .get("ReplyToId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    (channel_id, thread_ts, has_replied_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_format_lower() {
        let in_vals = vec![" Alice ".to_string(), "BOB".to_string(), "".to_string()];
        let out = format_lower(&in_vals);
        assert_eq!(out, vec!["alice".to_string(), "bob".to_string()]);
    }

    #[test]
    fn test_build_direct_context() {
        let mut ctx = serde_json::Map::new();
        ctx.insert("ChatType".to_string(), json!("Direct"));
        ctx.insert("From".to_string(), json!(" user1 "));
        ctx.insert("ReplyToId".to_string(), json!("r1"));
        let res = build_direct_or_group_thread_tool_context(&ctx, Some("h".to_string()));
        assert_eq!(res.0.unwrap(), "user1");
        assert_eq!(res.1.unwrap(), "r1");
    }
}
