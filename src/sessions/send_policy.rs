use crate::channels::chat_type::normalize_chat_type;
use crate::openkrab_config::SessionConfig;
use crate::sessions::{SendPolicy, Session};

pub fn normalize_send_policy(raw: Option<&str>) -> Option<SendPolicy> {
    match raw?.trim().to_lowercase().as_str() {
        "allow" => Some(SendPolicy::Allow),
        "deny" => Some(SendPolicy::Deny),
        _ => None,
    }
}

fn normalize_match_value(raw: Option<&str>) -> Option<String> {
    let value = raw?.trim().to_lowercase();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn strip_agent_session_key_prefix(key: Option<&str>) -> Option<String> {
    let key = key?;
    let parts: Vec<&str> = key.split(':').filter(|s| !s.is_empty()).collect();
    // Canonical agent session keys: agent:<agentId>:<sessionKey...>
    if parts.len() >= 3 && parts[0] == "agent" {
        Some(parts[2..].join(":"))
    } else {
        Some(key.to_string())
    }
}

fn derive_channel_from_key(key: Option<&str>) -> Option<String> {
    let normalized = strip_agent_session_key_prefix(key)?;
    let parts: Vec<&str> = normalized.split(':').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && (parts[1] == "group" || parts[1] == "channel") {
        normalize_match_value(Some(parts[0]))
    } else {
        None
    }
}

fn derive_chat_type_from_key(key: Option<&str>) -> Option<String> {
    let normalized = strip_agent_session_key_prefix(key)?;
    if normalized.contains(":group:") {
        Some("group".to_string())
    } else if normalized.contains(":channel:") {
        Some("channel".to_string())
    } else {
        None
    }
}

pub struct ResolveSendPolicyParams<'a> {
    pub cfg: &'a SessionConfig,
    pub entry: Option<&'a Session>,
    pub session_key: Option<&'a str>,
    pub channel: Option<&'a str>,
    pub chat_type: Option<&'a str>,
}

pub fn resolve_send_policy(params: ResolveSendPolicyParams) -> SendPolicy {
    // 1. Session-level override
    if let Some(entry) = params.entry {
        if let Some(policy) = entry.send_policy {
            return policy;
        }
        // Fallback to metadata for legacy compatibility if needed
        if let Some(m) = entry.get_meta("send_policy_override") {
            if let Some(o) = normalize_send_policy(Some(m)) {
                return o;
            }
        }
    }

    // 2. Global policy rules
    let policy = match &params.cfg.send_policy {
        Some(p) => p,
        None => return SendPolicy::Allow,
    };

    let channel_val = normalize_match_value(params.channel)
        .or_else(|| {
            params
                .entry
                .and_then(|e| normalize_match_value(e.channel.as_deref()))
        })
        .or_else(|| {
            params
                .entry
                .and_then(|e| normalize_match_value(e.last_channel.as_deref()))
        })
        .or_else(|| derive_channel_from_key(params.session_key));

    let chat_type_val = normalize_chat_type(params.chat_type)
        .or_else(|| {
            params
                .entry
                .and_then(|e| normalize_chat_type(e.chat_type.as_deref()))
        })
        .or_else(|| normalize_chat_type(derive_chat_type_from_key(params.session_key).as_deref()));

    let raw_session_key = params.session_key.unwrap_or("").to_lowercase();
    let stripped_session_key = strip_agent_session_key_prefix(params.session_key)
        .unwrap_or_default()
        .to_lowercase();

    let mut allowed_match = false;

    if let Some(rules) = &policy.rules {
        for rule in rules {
            let action = normalize_send_policy(rule.action.as_deref()).unwrap_or(SendPolicy::Allow);

            if let Some(match_cfg) = &rule.match_conditions {
                if let Some(mc) = normalize_match_value(match_cfg.channel.as_deref()) {
                    if Some(&mc) != channel_val.as_ref() {
                        continue;
                    }
                }
                if let Some(mct) = normalize_chat_type(match_cfg.chat_type.as_deref()) {
                    if Some(&mct) != chat_type_val.as_ref() {
                        continue;
                    }
                }
                if let Some(mrp) = normalize_match_value(match_cfg.raw_key_prefix.as_deref()) {
                    if !raw_session_key.starts_with(&mrp) {
                        continue;
                    }
                }
                if let Some(mp) = normalize_match_value(match_cfg.key_prefix.as_deref()) {
                    if !raw_session_key.starts_with(&mp) && !stripped_session_key.starts_with(&mp) {
                        continue;
                    }
                }
            }

            if action == SendPolicy::Deny {
                return SendPolicy::Deny;
            }
            allowed_match = true;
        }
    }

    if allowed_match {
        return SendPolicy::Allow;
    }

    normalize_send_policy(policy.default.as_deref()).unwrap_or(SendPolicy::Allow)
}
