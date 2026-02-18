use serde_json::{Map, Value};

use crate::slack::{resolve_slack_account, resolve_slack_reply_to_mode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelThreadingToolContext {
    pub current_channel_id: Option<String>,
    pub current_thread_ts: Option<String>,
    pub reply_to_mode: String,
    pub has_replied_ref: Option<bool>,
}

pub fn build_slack_threading_tool_context(
    cfg: &Value,
    account_id: Option<&str>,
    context: &Map<String, Value>,
    has_replied_ref: Option<bool>,
) -> ChannelThreadingToolContext {
    let account = resolve_slack_account(cfg, account_id);
    let chat_type = context.get("ChatType").and_then(|v| v.as_str());
    let configured_reply_to_mode = resolve_slack_reply_to_mode(&account, chat_type);
    let effective_reply_to_mode = if context.get("ThreadLabel").is_some() {
        "all".to_string()
    } else {
        configured_reply_to_mode
    };

    let to = context.get("To").and_then(|v| v.as_str()).map(|s| s.to_string());
    let current_channel_id = to.and_then(|t| {
        if t.starts_with("channel:") {
            Some(t["channel:".len()..].to_string())
        } else {
            None
        }
    });

    let thread_id = context
        .get("MessageThreadId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| context.get("ReplyToId").and_then(|v| v.as_str()).map(|s| s.to_string()));

    ChannelThreadingToolContext {
        current_channel_id,
        current_thread_ts: thread_id,
        reply_to_mode: effective_reply_to_mode,
        has_replied_ref,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_build_threading_tool_context_channel() {
        let cfg = json!({
            "channels": { "slack": { "accounts": { "default": { "replyToMode": "first" } } } }
        });
        let mut ctx = Map::new();
        ctx.insert("To".to_string(), Value::String("channel:C123".to_string()));
        ctx.insert("ReplyToId".to_string(), Value::String("r1".to_string()));
        let out = build_slack_threading_tool_context(&cfg, Some("default"), &ctx, Some(true));
        assert_eq!(out.current_channel_id.unwrap(), "C123".to_string());
        assert_eq!(out.current_thread_ts.unwrap(), "r1".to_string());
        assert_eq!(out.reply_to_mode, "first".to_string());
        assert_eq!(out.has_replied_ref, Some(true));
    }

    #[test]
    fn test_build_threading_tool_context_threadlabel_overrides() {
        let cfg = json!({
            "channels": { "slack": { "accounts": { "default": { "replyToMode": "first" } } } }
        });
        let mut ctx = Map::new();
        ctx.insert("To".to_string(), Value::String("channel:C1".to_string()));
        ctx.insert("MessageThreadId".to_string(), Value::String("m1".to_string()));
        ctx.insert("ThreadLabel".to_string(), Value::String("tlabel".to_string()));
        let out = build_slack_threading_tool_context(&cfg, Some("default"), &ctx, None);
        assert_eq!(out.reply_to_mode, "all".to_string());
    }
}
