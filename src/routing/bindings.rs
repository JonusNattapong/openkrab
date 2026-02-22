//! bindings — Channel-Agent binding resolution.
//! Ported from `openclaw/src/routing/bindings.ts`.
//!
//! Maps channels to agents via config-driven bindings.
//! Allows multi-account connectors to route to different agents.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::routing::session_key::{normalize_account_id, normalize_agent_id};

// ─── Types ────────────────────────────────────────────────────────────────────

pub use crate::openkrab_config::{AgentBinding, BindingMatch};

/// Binding configuration extracted from main config.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BindingsConfig {
    /// List of all bindings.
    #[serde(default)]
    pub bindings: Vec<AgentBinding>,
    /// Default agent ID if no binding matches.
    #[serde(default = "default_agent")]
    pub default_agent_id: String,
}

fn default_agent() -> String {
    "main".to_string()
}

// ─── Normalization ────────────────────────────────────────────────────────────

fn normalize_channel_id(raw: Option<&str>) -> Option<String> {
    let trimmed = raw?.trim().to_lowercase();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Resolved match components.
struct ResolvedMatch {
    agent_id: String,
    account_id: String,
    channel_id: String,
}

fn resolve_binding_match(binding: &AgentBinding) -> Option<ResolvedMatch> {
    let channel_id = normalize_channel_id(Some(&binding.match_.channel))?;
    let raw_account = binding
        .match_
        .account_id
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_string();
    if raw_account.is_empty() || raw_account == "*" {
        return None;
    }
    Some(ResolvedMatch {
        agent_id: normalize_agent_id(Some(&binding.agent_id)),
        account_id: normalize_account_id(Some(&raw_account)),
        channel_id,
    })
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// List all bindings from the config.
pub fn list_bindings(cfg: &BindingsConfig) -> &[AgentBinding] {
    &cfg.bindings
}

/// List bound account IDs for a specific channel.
pub fn list_bound_account_ids(cfg: &BindingsConfig, channel_id: &str) -> Vec<String> {
    let normalized_channel = match normalize_channel_id(Some(channel_id)) {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut ids = BTreeSet::new();
    for binding in &cfg.bindings {
        if let Some(resolved) = resolve_binding_match(binding) {
            if resolved.channel_id == normalized_channel {
                ids.insert(resolved.account_id);
            }
        }
    }
    ids.into_iter().collect()
}

/// Resolve default agent's bound account ID for a channel.
pub fn resolve_default_agent_bound_account_id(
    cfg: &BindingsConfig,
    channel_id: &str,
) -> Option<String> {
    let normalized_channel = normalize_channel_id(Some(channel_id))?;
    let default_agent = normalize_agent_id(Some(&cfg.default_agent_id));

    for binding in &cfg.bindings {
        if let Some(resolved) = resolve_binding_match(binding) {
            if resolved.channel_id == normalized_channel && resolved.agent_id == default_agent {
                return Some(resolved.account_id);
            }
        }
    }
    None
}

/// Build a map: channel_id → (agent_id → [account_ids]).
pub fn build_channel_account_bindings(
    cfg: &BindingsConfig,
) -> HashMap<String, BTreeMap<String, Vec<String>>> {
    let mut map: HashMap<String, BTreeMap<String, Vec<String>>> = HashMap::new();

    for binding in &cfg.bindings {
        if let Some(resolved) = resolve_binding_match(binding) {
            let by_agent = map.entry(resolved.channel_id).or_default();
            let list = by_agent.entry(resolved.agent_id).or_default();
            if !list.contains(&resolved.account_id) {
                list.push(resolved.account_id);
            }
        }
    }
    map
}

/// Resolve the preferred account ID given bound accounts and a default.
pub fn resolve_preferred_account_id(
    account_ids: &[String],
    default_account_id: &str,
    bound_accounts: &[String],
) -> String {
    if !bound_accounts.is_empty() {
        bound_accounts[0].clone()
    } else if !account_ids.is_empty() {
        account_ids[0].clone()
    } else {
        default_account_id.to_string()
    }
}

/// Resolve which agent should handle a message from a specific channel/account.
pub fn resolve_agent_binding(
    bindings: &[AgentBinding],
    channel_id: &str,
    target_channel: &str,
    account_id: &str,
) -> Option<String> {
    for binding in bindings {
        let channel_match = binding.match_.channel.as_str();
        if channel_match == "" {
            continue; // Technically invalid to have empty channel, but just in case
        }

        let account_match = binding.match_.account_id.as_deref().unwrap_or("*");

        let channel_ok =
            channel_match == "*" || channel_match == channel_id || channel_match == target_channel;
        let account_ok = account_match == "*" || account_match == account_id;

        if channel_ok && account_ok {
            return Some(binding.agent_id.clone());
        }
    }
    None
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_binding(agent: &str, channel: &str, account: &str) -> AgentBinding {
        AgentBinding {
            agent_id: agent.to_string(),
            match_: BindingMatch {
                channel: channel.to_string(),
                account_id: Some(account.to_string()),
                peer: None,
                guild_id: None,
                team_id: None,
                roles: None,
            },
        }
    }

    fn make_config(bindings: Vec<AgentBinding>) -> BindingsConfig {
        BindingsConfig {
            bindings,
            default_agent_id: "main".to_string(),
        }
    }

    #[test]
    fn list_bound_accounts() {
        let cfg = make_config(vec![
            make_binding("main", "telegram", "bot1"),
            make_binding("main", "telegram", "bot2"),
            make_binding("helper", "slack", "slackbot"),
        ]);
        let accounts = list_bound_account_ids(&cfg, "telegram");
        assert_eq!(accounts, vec!["bot1", "bot2"]);
    }

    #[test]
    fn list_bound_accounts_empty() {
        let cfg = make_config(vec![]);
        let accounts = list_bound_account_ids(&cfg, "telegram");
        assert!(accounts.is_empty());
    }

    #[test]
    fn resolve_default_bound() {
        let cfg = make_config(vec![
            make_binding("main", "telegram", "default-bot"),
            make_binding("helper", "telegram", "helper-bot"),
        ]);
        let result = resolve_default_agent_bound_account_id(&cfg, "telegram");
        assert_eq!(result, Some("default-bot".to_string()));
    }

    #[test]
    fn resolve_default_none() {
        let cfg = make_config(vec![make_binding("helper", "telegram", "bot1")]);
        let result = resolve_default_agent_bound_account_id(&cfg, "telegram");
        assert!(result.is_none());
    }

    #[test]
    fn build_channel_bindings_map() {
        let cfg = make_config(vec![
            make_binding("main", "telegram", "bot1"),
            make_binding("main", "telegram", "bot2"),
            make_binding("helper", "slack", "slackbot"),
        ]);
        let map = build_channel_account_bindings(&cfg);
        assert!(map.contains_key("telegram"));
        assert!(map.contains_key("slack"));
        assert_eq!(map["telegram"]["main"].len(), 2);
        assert_eq!(map["slack"]["helper"].len(), 1);
    }

    #[test]
    fn preferred_account_id() {
        assert_eq!(
            resolve_preferred_account_id(&["a".into()], "default", &["bound".into()]),
            "bound"
        );
        assert_eq!(
            resolve_preferred_account_id(&["a".into()], "default", &[]),
            "a"
        );
        assert_eq!(resolve_preferred_account_id(&[], "default", &[]), "default");
    }

    #[test]
    fn wildcard_account_ignored() {
        let binding = AgentBinding {
            agent_id: "main".to_string(),
            match_: BindingMatch {
                channel: "telegram".to_string(),
                account_id: Some("*".to_string()),
                peer: None,
                guild_id: None,
                team_id: None,
                roles: None,
            },
        };
        let cfg = make_config(vec![binding]);
        let accounts = list_bound_account_ids(&cfg, "telegram");
        assert!(accounts.is_empty());
    }

    #[test]
    fn normalize_channel_case_insensitive() {
        let cfg = make_config(vec![make_binding("main", "Telegram", "bot1")]);
        let accounts = list_bound_account_ids(&cfg, "TELEGRAM");
        assert_eq!(accounts, vec!["bot1"]);
    }
}
