use serde_json::Value;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct ResponsePrefixContext {
    pub identity_name: Option<String>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub model_full: Option<String>,
    pub thinking_level: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelSelectionContext {
    pub provider: String,
    pub model: String,
    pub think_level: Option<String>,
}

pub struct ReplyPrefixContextBundle {
    pub prefix_context: Arc<RwLock<ResponsePrefixContext>>,
    pub response_prefix: Option<String>,
    pub response_prefix_context_provider: Box<dyn Fn() -> ResponsePrefixContext + Send + Sync>,
    pub on_model_selected: Box<dyn Fn(ModelSelectionContext) + Send + Sync>,
}

fn resolve_identity_name(cfg: &Value, agent_id: &str) -> Option<String> {
    cfg.get("agents")
        .and_then(|v| v.get(agent_id))
        .and_then(|a| a.get("identity"))
        .and_then(|i| i.get("name"))
        .and_then(|s| s.as_str())
        .map(|s| s.trim().to_string())
}

fn resolve_identity_name_prefix(cfg: &Value, agent_id: &str) -> Option<String> {
    resolve_identity_name(cfg, agent_id).map(|n| format!("[{}]", n))
}

fn get_channel_cfg<'a>(cfg: &'a Value, channel: &str) -> Option<&'a Value> {
    cfg.get("channels").and_then(|c| c.get(channel))
}

pub fn resolve_effective_messages_config(
    cfg: &Value,
    agent_id: &str,
    channel: Option<&str>,
    account_id: Option<&str>,
) -> (String, Option<String>) {
    // responsePrefix resolution similar to TS: account -> channel -> global
    if let (Some(ch), Some(acc)) = (channel, account_id) {
        if let Some(account_pref) = get_channel_cfg(cfg, ch)
            .and_then(|v| v.get("accounts"))
            .and_then(|a| a.get(acc))
            .and_then(|accv| accv.get("responsePrefix"))
        {
            if let Some(s) = account_pref.as_str() {
                if s == "auto" {
                    return (
                        resolve_message_prefix(cfg, agent_id, None),
                        resolve_identity_name_prefix(cfg, agent_id),
                    );
                }
                return (resolve_message_prefix(cfg, agent_id, None), Some(s.to_string()));
            }
        }
    }

    if let Some(ch) = channel {
        if let Some(channel_pref) = get_channel_cfg(cfg, ch).and_then(|v| v.get("responsePrefix")) {
            if let Some(s) = channel_pref.as_str() {
                if s == "auto" {
                    return (
                        resolve_message_prefix(cfg, agent_id, None),
                        resolve_identity_name_prefix(cfg, agent_id),
                    );
                }
                return (resolve_message_prefix(cfg, agent_id, None), Some(s.to_string()));
            }
        }
    }

    if let Some(global) = cfg.get("messages").and_then(|m| m.get("responsePrefix")) {
        if let Some(s) = global.as_str() {
            if s == "auto" {
                return (
                    resolve_message_prefix(cfg, agent_id, None),
                    resolve_identity_name_prefix(cfg, agent_id),
                );
            }
            return (resolve_message_prefix(cfg, agent_id, None), Some(s.to_string()));
        }
    }

    (resolve_message_prefix(cfg, agent_id, None), None)
}

fn resolve_message_prefix(cfg: &Value, agent_id: &str, _opts: Option<&str>) -> String {
    // simplified: return messages.messagePrefix or identity prefix or default
    if let Some(msg_pref) = cfg.get("messages").and_then(|m| m.get("messagePrefix")).and_then(|v| v.as_str()) {
        return msg_pref.to_string();
    }
    resolve_identity_name_prefix(cfg, agent_id).unwrap_or_else(|| "[krabkrab]".to_string())
}

pub fn create_reply_prefix_context(
    cfg: &Value,
    agent_id: &str,
    channel: Option<&str>,
    account_id: Option<&str>,
) -> ReplyPrefixContextBundle {
    let identity_name = resolve_identity_name(cfg, agent_id);
    let prefix_ctx = Arc::new(RwLock::new(ResponsePrefixContext {
        identity_name,
        provider: None,
        model: None,
        model_full: None,
        thinking_level: None,
    }));

    let ( _message_prefix, response_prefix ) = resolve_effective_messages_config(cfg, agent_id, channel, account_id);

    let provider_clone = Arc::clone(&prefix_ctx);
    let response_prefix_context_provider: Box<dyn Fn() -> ResponsePrefixContext + Send + Sync> = Box::new(move || {
        provider_clone.read().unwrap().clone()
    });

    let model_ctx = Arc::clone(&prefix_ctx);
    let on_model_selected: Box<dyn Fn(ModelSelectionContext) + Send + Sync> = Box::new(move |ctx: ModelSelectionContext| {
        let mut lock = model_ctx.write().unwrap();
        lock.provider = Some(ctx.provider.clone());
        lock.model = Some(ctx.model.clone());
        lock.model_full = Some(format!("{}/{}", ctx.provider, ctx.model));
        lock.thinking_level = ctx.think_level.clone();
    });

    ReplyPrefixContextBundle {
        prefix_context: prefix_ctx,
        response_prefix,
        response_prefix_context_provider,
        on_model_selected,
    }
}

pub fn create_reply_prefix_options(
    cfg: &Value,
    agent_id: &str,
    channel: Option<&str>,
    account_id: Option<&str>,
) -> (Option<String>, Box<dyn Fn() -> ResponsePrefixContext + Send + Sync>, Box<dyn Fn(ModelSelectionContext) + Send + Sync>) {
    let bundle = create_reply_prefix_context(cfg, agent_id, channel, account_id);
    (bundle.response_prefix, bundle.response_prefix_context_provider, bundle.on_model_selected)
}
