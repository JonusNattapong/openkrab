use serde_json::{Map, Value};
use std::env;

use crate::channels::chat_type::normalize_chat_type;

pub const DEFAULT_ACCOUNT_ID: &str = "default";

#[derive(Debug, Clone)]
pub struct ResolvedSlackAccount {
    pub account_id: String,
    pub enabled: bool,
    pub name: Option<String>,
    pub bot_token: Option<String>,
    pub app_token: Option<String>,
    pub bot_token_source: String,
    pub app_token_source: String,
    pub config: Value,
    pub reply_to_mode: Option<String>,
    pub reply_to_mode_by_chat_type: Option<Map<String, Value>>,
}

fn normalize_account_id(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim();
    if trimmed.is_empty() {
        return DEFAULT_ACCOUNT_ID.to_string();
    }
    // Best-effort: accept alnum + _- up to 64, else collapse invalid chars to '-'
    let valid = regex::Regex::new(r"^[a-z0-9][a-z0-9_-]{0,63}$").unwrap();
    if valid.is_match(trimmed) {
        return trimmed.to_lowercase();
    }
    let invalid = regex::Regex::new(r"[^a-z0-9_-]+", ).unwrap();
    let mut s = trimmed.to_lowercase();
    s = invalid.replace_all(&s, "-").to_string();
    s.truncate(64);
    if s.is_empty() { DEFAULT_ACCOUNT_ID.to_string() } else { s }
}

fn merge_configs(base: &Value, overlay: &Value) -> Value {
    match (base, overlay) {
        (Value::Object(bm), Value::Object(om)) => {
            let mut out = bm.clone();
            for (k, v) in om {
                out.insert(k.clone(), merge_configs(out.get(k).unwrap_or(&Value::Null), v));
            }
            Value::Object(out)
        }
        (_, o) => o.clone(),
    }
}

pub fn resolve_slack_account(cfg: &Value, account_id: Option<&str>) -> ResolvedSlackAccount {
    let account_id = normalize_account_id(account_id);
    let base = cfg
        .get("channels")
        .and_then(|c| c.get("slack"))
        .cloned()
        .unwrap_or(Value::Object(Map::new()));
    let accounts = base.get("accounts").and_then(|v| v.as_object()).cloned();
    let account_cfg = accounts
        .as_ref()
        .and_then(|m| m.get(&account_id))
        .cloned()
        .unwrap_or(Value::Object(Map::new()));
    let merged = merge_configs(&base, &account_cfg);

    let base_enabled = base.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let account_enabled = merged.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let enabled = base_enabled && account_enabled;

    let allow_env = account_id == DEFAULT_ACCOUNT_ID;
    let env_bot = if allow_env { env::var("SLACK_BOT_TOKEN").ok() } else { None };
    let env_app = if allow_env { env::var("SLACK_APP_TOKEN").ok() } else { None };

    let config_bot = merged.get("botToken").and_then(|v| v.as_str()).map(|s| s.to_string());
    let config_app = merged.get("appToken").and_then(|v| v.as_str()).map(|s| s.to_string());
    let bot_token = config_bot.clone().or(env_bot.clone());
    let app_token = config_app.clone().or(env_app.clone());
    let bot_token_source = if config_bot.is_some() { "config" } else if env_bot.is_some() { "env" } else { "none" };
    let app_token_source = if config_app.is_some() { "config" } else if env_app.is_some() { "env" } else { "none" };

    let reply_to_mode = merged.get("replyToMode").and_then(|v| v.as_str()).map(|s| s.to_string());
    let reply_to_mode_by_chat_type = merged
        .get("replyToModeByChatType")
        .and_then(|v| v.as_object())
        .cloned();

    ResolvedSlackAccount {
        account_id,
        enabled,
        name: merged.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
        bot_token,
        app_token,
        bot_token_source: bot_token_source.to_string(),
        app_token_source: app_token_source.to_string(),
        config: merged,
        reply_to_mode,
        reply_to_mode_by_chat_type,
    }
}

pub fn list_enabled_slack_accounts(cfg: &Value) -> Vec<ResolvedSlackAccount> {
    let ids = cfg
        .get("channels")
        .and_then(|c| c.get("slack"))
        .and_then(|s| s.get("accounts"))
        .and_then(|a| a.as_object())
        .map(|m| m.keys().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    ids.into_iter()
        .map(|id| resolve_slack_account(cfg, Some(&id)))
        .filter(|acc| acc.enabled)
        .collect()
}

pub fn resolve_slack_reply_to_mode(account: &ResolvedSlackAccount, chat_type: Option<&str>) -> String {
    if let Some(normalized) = normalize_chat_type(chat_type) {
        let key = match normalized {
            crate::channels::chat_type::ChatType::Direct => "direct",
            crate::channels::chat_type::ChatType::Group => "group",
            crate::channels::chat_type::ChatType::Channel => "channel",
        };
        if let Some(map) = &account.reply_to_mode_by_chat_type {
            if let Some(mode_val) = map.get(key).and_then(|v| v.as_str()) {
                return mode_val.to_string();
            }
        }
        if key == "direct" {
            if let Some(dm_cfg) = account.config.get("dm").and_then(|v| v.get("replyToMode")).and_then(|v| v.as_str()) {
                return dm_cfg.to_string();
            }
        }
    }
    account.reply_to_mode.clone().unwrap_or_else(|| "off".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_normalize_account_id_empty() {
        assert_eq!(normalize_account_id(None), DEFAULT_ACCOUNT_ID.to_string());
    }

    #[test]
    fn test_resolve_reply_to_mode_override() {
        let cfg = json!({
            "channels": {
                "slack": {
                    "accounts": {
                        "default": {
                            "replyToMode": "first",
                            "replyToModeByChatType": { "direct": "all" },
                            "dm": { "replyToMode": "first" }
                        }
                    }
                }
            }
        });
        let acc = resolve_slack_account(&cfg, Some("default"));
        assert_eq!(resolve_slack_reply_to_mode(&acc, Some("direct")), "all".to_string());
        assert_eq!(resolve_slack_reply_to_mode(&acc, Some("group")), "first".to_string());
    }
}
