//! BlueBubbles configuration schema.
//! Ported from openkrab/extensions/bluebubbles/src/config-schema.ts

use serde::{Deserialize, Serialize};

use super::types::{BlueBubblesAccountConfig, BlueBubblesActionConfig, BlueBubblesConfig};

pub const DEFAULT_ACCOUNT_ID: &str = "default";
pub const DEFAULT_WEBHOOK_PATH: &str = "/webhook/bluebubbles";
pub const DEFAULT_TEXT_CHUNK_LIMIT: usize = 4000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedBlueBubblesAccount {
    pub account_id: String,
    pub name: Option<String>,
    pub enabled: bool,
    pub configured: bool,
    pub base_url: String,
    pub config: BlueBubblesAccountConfig,
}

impl Default for ResolvedBlueBubblesAccount {
    fn default() -> Self {
        Self {
            account_id: DEFAULT_ACCOUNT_ID.to_string(),
            name: None,
            enabled: true,
            configured: false,
            base_url: String::new(),
            config: BlueBubblesAccountConfig::default(),
        }
    }
}

pub fn list_account_ids(config: &BlueBubblesConfig) -> Vec<String> {
    let mut ids = Vec::new();

    if config.account.server_url.is_some() || config.account.password.is_some() {
        ids.push(DEFAULT_ACCOUNT_ID.to_string());
    }

    if let Some(ref accounts) = config.accounts {
        for id in accounts.keys() {
            if !ids.contains(id) {
                ids.push(id.clone());
            }
        }
    }

    ids.sort();
    ids
}

pub fn resolve_account(
    config: &BlueBubblesConfig,
    account_id: Option<&str>,
) -> ResolvedBlueBubblesAccount {
    let id = account_id.unwrap_or(DEFAULT_ACCOUNT_ID);

    if id == DEFAULT_ACCOUNT_ID {
        let base_url = config
            .account
            .server_url
            .clone()
            .unwrap_or_default()
            .trim_end_matches('/')
            .to_string();
        let configured = !base_url.is_empty() && config.account.password.is_some();

        return ResolvedBlueBubblesAccount {
            account_id: DEFAULT_ACCOUNT_ID.to_string(),
            name: config.account.name.clone(),
            enabled: config.account.enabled.unwrap_or(true),
            configured,
            base_url,
            config: config.account.clone(),
        };
    }

    if let Some(ref accounts) = config.accounts {
        if let Some(account_config) = accounts.get(id) {
            let base_url = account_config
                .server_url
                .clone()
                .or_else(|| config.account.server_url.clone())
                .unwrap_or_default()
                .trim_end_matches('/')
                .to_string();
            let password = account_config
                .password
                .clone()
                .or_else(|| config.account.password.clone());
            let configured = !base_url.is_empty() && password.is_some();

            return ResolvedBlueBubblesAccount {
                account_id: id.to_string(),
                name: account_config.name.clone(),
                enabled: account_config.enabled.unwrap_or(true),
                configured,
                base_url: base_url.clone(),
                config: BlueBubblesAccountConfig {
                    server_url: Some(base_url),
                    password,
                    ..account_config.clone()
                },
            };
        }
    }

    ResolvedBlueBubblesAccount::default()
}

pub fn resolve_default_account_id(config: &BlueBubblesConfig) -> Option<String> {
    let ids = list_account_ids(config);
    ids.first().cloned()
}

pub fn resolve_webhook_path(config: &BlueBubblesAccountConfig) -> String {
    config
        .webhook_path
        .clone()
        .unwrap_or_else(|| DEFAULT_WEBHOOK_PATH.to_string())
}

pub fn resolve_text_chunk_limit(config: &BlueBubblesAccountConfig) -> usize {
    config.text_chunk_limit.unwrap_or(DEFAULT_TEXT_CHUNK_LIMIT)
}

pub fn resolve_dm_policy(config: &BlueBubblesAccountConfig) -> String {
    config
        .dm_policy
        .clone()
        .unwrap_or_else(|| "pairing".to_string())
}

pub fn resolve_group_policy(config: &BlueBubblesAccountConfig) -> String {
    config
        .group_policy
        .clone()
        .unwrap_or_else(|| "allowlist".to_string())
}

pub fn resolve_group_require_mention(
    config: &BlueBubblesAccountConfig,
    chat_guid: Option<&str>,
) -> bool {
    if let Some(guid) = chat_guid {
        if let Some(ref groups) = config.groups {
            if let Some(group_config) = groups.get(guid) {
                return group_config.require_mention.unwrap_or(true);
            }
        }
    }
    true
}

pub fn resolve_allow_from(config: &BlueBubblesAccountConfig) -> Vec<String> {
    config
        .allow_from
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|s| s.to_string())
        .collect()
}

pub fn resolve_group_allow_from(config: &BlueBubblesAccountConfig) -> Vec<String> {
    config
        .group_allow_from
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|s| s.to_string())
        .collect()
}

pub fn is_action_enabled(actions: Option<&BlueBubblesActionConfig>, action: &str) -> bool {
    let actions = match actions {
        Some(a) => a,
        None => return true,
    };

    let enabled = match action {
        "reactions" => actions.reactions,
        "edit" => actions.edit,
        "unsend" => actions.unsend,
        "reply" => actions.reply,
        "send_with_effect" => actions.send_with_effect,
        "rename_group" => actions.rename_group,
        "add_participant" => actions.add_participant,
        "remove_participant" => actions.remove_participant,
        "leave_group" => actions.leave_group,
        "send_attachment" => actions.send_attachment,
        _ => return true,
    };

    enabled.unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_account_ids_empty() {
        let config = BlueBubblesConfig::default();
        let ids = list_account_ids(&config);
        assert!(ids.is_empty());
    }

    #[test]
    fn test_list_account_ids_with_default() {
        let config = BlueBubblesConfig {
            account: BlueBubblesAccountConfig {
                server_url: Some("http://localhost:8080".to_string()),
                password: Some("secret".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let ids = list_account_ids(&config);
        assert_eq!(ids, vec!["default"]);
    }

    #[test]
    fn test_resolve_account_configured() {
        let config = BlueBubblesConfig {
            account: BlueBubblesAccountConfig {
                server_url: Some("http://localhost:8080".to_string()),
                password: Some("secret".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let account = resolve_account(&config, None);
        assert!(account.configured);
        assert_eq!(account.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_resolve_webhook_path_default() {
        let config = BlueBubblesAccountConfig::default();
        assert_eq!(resolve_webhook_path(&config), DEFAULT_WEBHOOK_PATH);
    }

    #[test]
    fn test_is_action_enabled_default() {
        assert!(is_action_enabled(None, "reactions"));

        let actions = BlueBubblesActionConfig {
            reactions: Some(false),
            ..Default::default()
        };
        assert!(!is_action_enabled(Some(&actions), "reactions"));
        assert!(is_action_enabled(Some(&actions), "edit"));
    }
}
