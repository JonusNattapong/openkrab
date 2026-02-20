//! session_key — Complex session key construction and parsing.
//! Ported from `openclaw/src/routing/session-key.ts`.
//!
//! Builds composite session keys from connector + channel + thread + agent +
//! account + DM scope, following the same format as OpenClaw.

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

// ─── Constants ────────────────────────────────────────────────────────────────

pub const DEFAULT_AGENT_ID: &str = "main";
pub const DEFAULT_MAIN_KEY: &str = "main";
pub const DEFAULT_ACCOUNT_ID: &str = "default";

lazy_static! {
    static ref VALID_ID_RE: Regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]{0,63}$").unwrap();
    static ref INVALID_CHARS_RE: Regex = Regex::new(r"[^a-z0-9_-]+").unwrap();
    static ref LEADING_DASH_RE: Regex = Regex::new(r"^-+").unwrap();
    static ref TRAILING_DASH_RE: Regex = Regex::new(r"-+$").unwrap();
}

// ─── DM Scope ─────────────────────────────────────────────────────────────────

/// DM session scope — controls how direct messages are grouped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum DmScope {
    /// All DMs share a single session per agent.
    #[default]
    Main,
    /// Each peer gets their own session.
    PerPeer,
    /// Per channel + peer.
    PerChannelPeer,
    /// Per account + channel + peer.
    PerAccountChannelPeer,
}

impl DmScope {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "per-peer" | "per_peer" => DmScope::PerPeer,
            "per-channel-peer" | "per_channel_peer" => DmScope::PerChannelPeer,
            "per-account-channel-peer" | "per_account_channel_peer" => {
                DmScope::PerAccountChannelPeer
            }
            _ => DmScope::Main,
        }
    }
}

// ─── Session key shape classification ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionKeyShape {
    Missing,
    Agent,
    LegacyOrAlias,
    MalformedAgent,
}

// ─── Parsed session key ───────────────────────────────────────────────────────

/// Parsed agent session key parts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedAgentSessionKey {
    pub agent_id: String,
    pub rest: String,
    pub full: String,
}

/// Parse `agent:<agent_id>:<rest>` format.
pub fn parse_agent_session_key(key: Option<&str>) -> Option<ParsedAgentSessionKey> {
    let raw = key?.trim();
    if raw.is_empty() {
        return None;
    }
    let lower = raw.to_lowercase();
    if !lower.starts_with("agent:") {
        return None;
    }
    let after = &lower[6..]; // skip "agent:"
    let colon_pos = after.find(':')?;
    let agent_id = &after[..colon_pos];
    if agent_id.is_empty() {
        return None;
    }
    let rest = &after[colon_pos + 1..];
    Some(ParsedAgentSessionKey {
        agent_id: agent_id.to_string(),
        rest: rest.to_string(),
        full: lower,
    })
}

// ─── Normalization helpers ────────────────────────────────────────────────────

fn normalize_token(value: Option<&str>) -> String {
    value.unwrap_or("").trim().to_lowercase()
}

/// Normalize main key.
pub fn normalize_main_key(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim();
    if trimmed.is_empty() {
        DEFAULT_MAIN_KEY.to_string()
    } else {
        trimmed.to_lowercase()
    }
}

/// Normalize agent ID — keep path-safe, shell-friendly.
pub fn normalize_agent_id(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim();
    if trimmed.is_empty() {
        return DEFAULT_AGENT_ID.to_string();
    }
    if VALID_ID_RE.is_match(trimmed) {
        return trimmed.to_lowercase();
    }
    // Best-effort: collapse invalid characters to "-"
    let lower = trimmed.to_lowercase();
    let replaced = INVALID_CHARS_RE.replace_all(&lower, "-");
    let no_leading = LEADING_DASH_RE.replace(&replaced, "");
    let no_trailing = TRAILING_DASH_RE.replace(&no_leading, "");
    let result: String = no_trailing.chars().take(64).collect();
    if result.is_empty() {
        DEFAULT_AGENT_ID.to_string()
    } else {
        result
    }
}

/// Normalize account ID.
pub fn normalize_account_id(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim();
    if trimmed.is_empty() {
        return DEFAULT_ACCOUNT_ID.to_string();
    }
    if VALID_ID_RE.is_match(trimmed) {
        return trimmed.to_lowercase();
    }
    let lower = trimmed.to_lowercase();
    let replaced = INVALID_CHARS_RE.replace_all(&lower, "-");
    let no_leading = LEADING_DASH_RE.replace(&replaced, "");
    let no_trailing = TRAILING_DASH_RE.replace(&no_leading, "");
    let result: String = no_trailing.chars().take(64).collect();
    if result.is_empty() {
        DEFAULT_ACCOUNT_ID.to_string()
    } else {
        result
    }
}

// ─── Session key classification ───────────────────────────────────────────────

/// Classify the shape of a session key.
pub fn classify_session_key_shape(key: Option<&str>) -> SessionKeyShape {
    let raw = key.unwrap_or("").trim();
    if raw.is_empty() {
        return SessionKeyShape::Missing;
    }
    if parse_agent_session_key(Some(raw)).is_some() {
        return SessionKeyShape::Agent;
    }
    if raw.to_lowercase().starts_with("agent:") {
        SessionKeyShape::MalformedAgent
    } else {
        SessionKeyShape::LegacyOrAlias
    }
}

/// Resolve agent ID from a session key.
pub fn resolve_agent_id_from_session_key(session_key: Option<&str>) -> String {
    let parsed = parse_agent_session_key(session_key);
    normalize_agent_id(parsed.as_ref().map(|p| p.agent_id.as_str()))
}

// ─── Session key builders ─────────────────────────────────────────────────────

/// Build the main session key for an agent.
pub fn build_agent_main_session_key(agent_id: &str, main_key: Option<&str>) -> String {
    let agent = normalize_agent_id(Some(agent_id));
    let main = normalize_main_key(main_key);
    format!("agent:{}:{}", agent, main)
}

/// Convert a request-side session key to a store-side session key.
pub fn to_agent_store_session_key(
    agent_id: &str,
    request_key: Option<&str>,
    main_key: Option<&str>,
) -> String {
    let raw = request_key.unwrap_or("").trim();
    if raw.is_empty() || raw == DEFAULT_MAIN_KEY {
        return build_agent_main_session_key(agent_id, main_key);
    }
    let lowered = raw.to_lowercase();
    if lowered.starts_with("agent:") {
        return lowered;
    }
    if lowered.starts_with("subagent:") {
        return format!("agent:{}:{}", normalize_agent_id(Some(agent_id)), lowered);
    }
    format!("agent:{}:{}", normalize_agent_id(Some(agent_id)), lowered)
}

/// Convert a store-side session key back to a request-side key.
pub fn to_agent_request_session_key(store_key: Option<&str>) -> Option<String> {
    let raw = store_key?.trim();
    if raw.is_empty() {
        return None;
    }
    parse_agent_session_key(Some(raw)).map(|p| p.rest)
}

/// Build a peer-specific session key, respecting DM scope and identity links.
pub fn build_agent_peer_session_key(params: PeerSessionKeyParams) -> String {
    let peer_kind = params.peer_kind.as_deref().unwrap_or("direct");

    if peer_kind == "direct" {
        let dm_scope = params.dm_scope.unwrap_or(DmScope::Main);
        let mut peer_id = params.peer_id.unwrap_or_default().trim().to_string();

        // Resolve identity links
        if dm_scope != DmScope::Main && !peer_id.is_empty() {
            if let Some(linked) = resolve_linked_peer_id(
                params.identity_links.as_ref(),
                params.channel.as_deref().unwrap_or(""),
                &peer_id,
            ) {
                peer_id = linked;
            }
        }

        peer_id = peer_id.to_lowercase();

        match dm_scope {
            DmScope::PerAccountChannelPeer if !peer_id.is_empty() => {
                let channel = normalize_token(params.channel.as_deref())
                    .chars()
                    .take(64)
                    .collect::<String>();
                let channel = if channel.is_empty() {
                    "unknown".to_string()
                } else {
                    channel
                };
                let account = normalize_account_id(params.account_id.as_deref());
                let agent = normalize_agent_id(Some(&params.agent_id));
                format!("agent:{}:{}:{}:direct:{}", agent, channel, account, peer_id)
            }
            DmScope::PerChannelPeer if !peer_id.is_empty() => {
                let channel = normalize_token(params.channel.as_deref())
                    .chars()
                    .take(64)
                    .collect::<String>();
                let channel = if channel.is_empty() {
                    "unknown".to_string()
                } else {
                    channel
                };
                let agent = normalize_agent_id(Some(&params.agent_id));
                format!("agent:{}:{}:direct:{}", agent, channel, peer_id)
            }
            DmScope::PerPeer if !peer_id.is_empty() => {
                let agent = normalize_agent_id(Some(&params.agent_id));
                format!("agent:{}:direct:{}", agent, peer_id)
            }
            _ => build_agent_main_session_key(&params.agent_id, params.main_key.as_deref()),
        }
    } else {
        // Group / channel
        let channel = normalize_token(params.channel.as_deref())
            .chars()
            .take(64)
            .collect::<String>();
        let channel = if channel.is_empty() {
            "unknown".to_string()
        } else {
            channel
        };
        let peer_id = params
            .peer_id
            .unwrap_or_default()
            .trim()
            .to_lowercase();
        let peer_id = if peer_id.is_empty() {
            "unknown".to_string()
        } else {
            peer_id
        };
        let agent = normalize_agent_id(Some(&params.agent_id));
        format!("agent:{}:{}:{}:{}", agent, channel, peer_kind, peer_id)
    }
}

/// Parameters for building a peer session key.
pub struct PeerSessionKeyParams {
    pub agent_id: String,
    pub main_key: Option<String>,
    pub channel: Option<String>,
    pub account_id: Option<String>,
    pub peer_kind: Option<String>,
    pub peer_id: Option<String>,
    pub identity_links: Option<std::collections::HashMap<String, Vec<String>>>,
    pub dm_scope: Option<DmScope>,
}

// ─── Thread session keys ──────────────────────────────────────────────────────

/// Resolve thread-aware session keys.
pub fn resolve_thread_session_keys(
    base_session_key: &str,
    thread_id: Option<&str>,
    parent_session_key: Option<&str>,
    use_suffix: bool,
) -> (String, Option<String>) {
    let tid = thread_id.unwrap_or("").trim();
    if tid.is_empty() {
        return (base_session_key.to_string(), None);
    }
    let normalized = tid.to_lowercase();
    let session_key = if use_suffix {
        format!("{}:thread:{}", base_session_key, normalized)
    } else {
        base_session_key.to_string()
    };
    (session_key, parent_session_key.map(|s| s.to_string()))
}

/// Build a group history key.
pub fn build_group_history_key(
    channel: &str,
    account_id: Option<&str>,
    peer_kind: &str,
    peer_id: &str,
) -> String {
    let ch = normalize_token(Some(channel));
    let ch = if ch.is_empty() {
        "unknown".to_string()
    } else {
        ch
    };
    let account = normalize_account_id(account_id);
    let pid = peer_id.trim().to_lowercase();
    let pid = if pid.is_empty() {
        "unknown".to_string()
    } else {
        pid
    };
    format!("{}:{}:{}:{}", ch, account, peer_kind, pid)
}

// ─── Subagent / cron / acp helpers ────────────────────────────────────────────

/// Get subagent depth from session key.
pub fn get_subagent_depth(key: Option<&str>) -> usize {
    let raw = key.unwrap_or("").to_lowercase();
    raw.matches("subagent:").count()
}

/// Check if this is a cron session key.
pub fn is_cron_session_key(key: Option<&str>) -> bool {
    let raw = key.unwrap_or("").to_lowercase();
    raw.contains(":cron:") || raw.ends_with(":cron")
}

/// Check if this is an ACP session key.
pub fn is_acp_session_key(key: Option<&str>) -> bool {
    let raw = key.unwrap_or("").to_lowercase();
    raw.contains(":acp:") || raw.ends_with(":acp")
}

/// Check if this is a subagent session key.
pub fn is_subagent_session_key(key: Option<&str>) -> bool {
    let raw = key.unwrap_or("").to_lowercase();
    raw.contains("subagent:")
}

// ─── Identity link resolution ─────────────────────────────────────────────────

fn resolve_linked_peer_id(
    identity_links: Option<&std::collections::HashMap<String, Vec<String>>>,
    channel: &str,
    peer_id: &str,
) -> Option<String> {
    let links = identity_links?;
    let peer_id = peer_id.trim();
    if peer_id.is_empty() {
        return None;
    }

    let mut candidates = std::collections::HashSet::new();
    let raw_candidate = normalize_token(Some(peer_id));
    if !raw_candidate.is_empty() {
        candidates.insert(raw_candidate);
    }
    let ch = normalize_token(Some(channel));
    if !ch.is_empty() {
        let scoped = format!("{}:{}", ch, normalize_token(Some(peer_id)));
        if !scoped.is_empty() {
            candidates.insert(scoped);
        }
    }
    if candidates.is_empty() {
        return None;
    }

    for (canonical, ids) in links {
        let canonical_name = canonical.trim();
        if canonical_name.is_empty() {
            continue;
        }
        for id in ids {
            let normalized = normalize_token(Some(id));
            if !normalized.is_empty() && candidates.contains(&normalized) {
                return Some(canonical_name.to_string());
            }
        }
    }
    None
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_agent_id_valid() {
        assert_eq!(normalize_agent_id(Some("Main")), "main");
        assert_eq!(normalize_agent_id(Some("my-agent")), "my-agent");
        assert_eq!(normalize_agent_id(Some("")), "main");
        assert_eq!(normalize_agent_id(None), "main");
    }

    #[test]
    fn normalize_agent_id_invalid_chars() {
        assert_eq!(normalize_agent_id(Some("My Agent!")), "my-agent");
    }

    #[test]
    fn normalize_account_id_cases() {
        assert_eq!(normalize_account_id(Some("Bot1")), "bot1");
        assert_eq!(normalize_account_id(Some("")), "default");
        assert_eq!(normalize_account_id(None), "default");
    }

    #[test]
    fn parse_agent_key() {
        let parsed = parse_agent_session_key(Some("agent:helper:main")).unwrap();
        assert_eq!(parsed.agent_id, "helper");
        assert_eq!(parsed.rest, "main");
    }

    #[test]
    fn parse_agent_key_none() {
        assert!(parse_agent_session_key(None).is_none());
        assert!(parse_agent_session_key(Some("")).is_none());
        assert!(parse_agent_session_key(Some("not-agent-key")).is_none());
    }

    #[test]
    fn parse_agent_key_malformed() {
        // "agent:" without enough parts
        assert!(parse_agent_session_key(Some("agent:")).is_none());
    }

    #[test]
    fn classify_shapes() {
        assert_eq!(classify_session_key_shape(None), SessionKeyShape::Missing);
        assert_eq!(classify_session_key_shape(Some("")), SessionKeyShape::Missing);
        assert_eq!(
            classify_session_key_shape(Some("agent:main:main")),
            SessionKeyShape::Agent
        );
        assert_eq!(
            classify_session_key_shape(Some("agent:")),
            SessionKeyShape::MalformedAgent
        );
        assert_eq!(
            classify_session_key_shape(Some("legacy-key")),
            SessionKeyShape::LegacyOrAlias
        );
    }

    #[test]
    fn build_main_key() {
        assert_eq!(
            build_agent_main_session_key("main", None),
            "agent:main:main"
        );
        assert_eq!(
            build_agent_main_session_key("helper", Some("custom")),
            "agent:helper:custom"
        );
    }

    #[test]
    fn store_key_roundtrip() {
        let store = to_agent_store_session_key("main", Some("test"), None);
        assert_eq!(store, "agent:main:test");

        let request = to_agent_request_session_key(Some(&store));
        assert_eq!(request, Some("test".to_string()));
    }

    #[test]
    fn store_key_empty() {
        let store = to_agent_store_session_key("main", None, None);
        assert_eq!(store, "agent:main:main");
    }

    #[test]
    fn peer_session_key_main_scope() {
        let key = build_agent_peer_session_key(PeerSessionKeyParams {
            agent_id: "main".into(),
            main_key: None,
            channel: Some("telegram".into()),
            account_id: None,
            peer_kind: Some("direct".into()),
            peer_id: Some("user123".into()),
            identity_links: None,
            dm_scope: Some(DmScope::Main),
        });
        // Main scope ignores peer_id
        assert_eq!(key, "agent:main:main");
    }

    #[test]
    fn peer_session_key_per_peer() {
        let key = build_agent_peer_session_key(PeerSessionKeyParams {
            agent_id: "main".into(),
            main_key: None,
            channel: Some("telegram".into()),
            account_id: None,
            peer_kind: Some("direct".into()),
            peer_id: Some("User123".into()),
            identity_links: None,
            dm_scope: Some(DmScope::PerPeer),
        });
        assert_eq!(key, "agent:main:direct:user123");
    }

    #[test]
    fn peer_session_key_per_channel_peer() {
        let key = build_agent_peer_session_key(PeerSessionKeyParams {
            agent_id: "main".into(),
            main_key: None,
            channel: Some("telegram".into()),
            account_id: None,
            peer_kind: Some("direct".into()),
            peer_id: Some("user123".into()),
            identity_links: None,
            dm_scope: Some(DmScope::PerChannelPeer),
        });
        assert_eq!(key, "agent:main:telegram:direct:user123");
    }

    #[test]
    fn peer_session_key_group() {
        let key = build_agent_peer_session_key(PeerSessionKeyParams {
            agent_id: "main".into(),
            main_key: None,
            channel: Some("discord".into()),
            account_id: None,
            peer_kind: Some("group".into()),
            peer_id: Some("guild_123".into()),
            identity_links: None,
            dm_scope: None,
        });
        assert_eq!(key, "agent:main:discord:group:guild_123");
    }

    #[test]
    fn thread_session_keys() {
        let (key, parent) =
            resolve_thread_session_keys("agent:main:main", Some("T456"), None, true);
        assert_eq!(key, "agent:main:main:thread:t456");
        assert!(parent.is_none());

        let (key, _) = resolve_thread_session_keys("agent:main:main", None, None, true);
        assert_eq!(key, "agent:main:main");
    }

    #[test]
    fn subagent_depth() {
        assert_eq!(get_subagent_depth(None), 0);
        assert_eq!(get_subagent_depth(Some("agent:main:main")), 0);
        assert_eq!(
            get_subagent_depth(Some("agent:main:subagent:child:main")),
            1
        );
        assert_eq!(
            get_subagent_depth(Some("agent:main:subagent:a:subagent:b:main")),
            2
        );
    }

    #[test]
    fn special_key_checks() {
        assert!(is_cron_session_key(Some("agent:main:cron:daily")));
        assert!(!is_cron_session_key(Some("agent:main:main")));
        assert!(is_acp_session_key(Some("agent:main:acp:req1")));
        assert!(is_subagent_session_key(Some(
            "agent:main:subagent:child:main"
        )));
    }

    #[test]
    fn identity_links_resolution() {
        let mut links = std::collections::HashMap::new();
        links.insert(
            "alice".to_string(),
            vec!["telegram:alice123".to_string(), "slack:alice_s".to_string()],
        );

        let key = build_agent_peer_session_key(PeerSessionKeyParams {
            agent_id: "main".into(),
            main_key: None,
            channel: Some("telegram".into()),
            account_id: None,
            peer_kind: Some("direct".into()),
            peer_id: Some("alice123".into()),
            identity_links: Some(links),
            dm_scope: Some(DmScope::PerPeer),
        });
        assert_eq!(key, "agent:main:direct:alice");
    }

    #[test]
    fn group_history_key() {
        let key = build_group_history_key("discord", Some("bot1"), "group", "guild_123");
        assert_eq!(key, "discord:bot1:group:guild_123");
    }
}
