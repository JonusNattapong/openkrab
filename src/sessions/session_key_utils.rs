//! Ported from `openclaw/src/sessions/session-key-utils.ts`

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedAgentSessionKey {
    pub agent_id: String,
    pub rest: String,
}

pub fn parse_agent_session_key(session_key: Option<&str>) -> Option<ParsedAgentSessionKey> {
    let raw = session_key.unwrap_or("").trim();
    if raw.is_empty() {
        return None;
    }

    let parts: Vec<&str> = raw.split(':').filter(|s| !s.is_empty()).collect();
    if parts.len() < 3 {
        return None;
    }
    if parts[0] != "agent" {
        return None;
    }

    let agent_id = parts[1].trim();
    let rest = parts[2..].join(":");

    if agent_id.is_empty() || rest.is_empty() {
        return None;
    }

    Some(ParsedAgentSessionKey {
        agent_id: agent_id.to_string(),
        rest,
    })
}

pub fn is_cron_run_session_key(session_key: Option<&str>) -> bool {
    let parsed = match parse_agent_session_key(session_key) {
        Some(p) => p,
        None => return false,
    };

    let rest = parsed.rest;
    if !rest.starts_with("cron:") {
        return false;
    }

    let parts: Vec<&str> = rest.split(':').filter(|s| !s.is_empty()).collect();
    // Matches ^cron:[^:]+:run:[^:]+$
    parts.len() == 4 && parts[0] == "cron" && parts[2] == "run"
}

pub fn is_cron_session_key(session_key: Option<&str>) -> bool {
    let parsed = match parse_agent_session_key(session_key) {
        Some(p) => p,
        None => return false,
    };
    parsed.rest.to_lowercase().starts_with("cron:")
}

pub fn is_subagent_session_key(session_key: Option<&str>) -> bool {
    let raw = session_key.unwrap_or("").trim();
    if raw.is_empty() {
        return false;
    }
    if raw.to_lowercase().starts_with("subagent:") {
        return true;
    }
    if let Some(parsed) = parse_agent_session_key(Some(raw)) {
        return parsed.rest.to_lowercase().starts_with("subagent:");
    }
    false
}

pub fn get_subagent_depth(session_key: Option<&str>) -> usize {
    let raw = session_key.unwrap_or("").trim().to_lowercase();
    if raw.is_empty() {
        return 0;
    }
    raw.split(":subagent:").count().saturating_sub(1)
}

pub fn is_acp_session_key(session_key: Option<&str>) -> bool {
    let raw = session_key.unwrap_or("").trim();
    if raw.is_empty() {
        return false;
    }
    let normalized = raw.to_lowercase();
    if normalized.starts_with("acp:") {
        return true;
    }
    if let Some(parsed) = parse_agent_session_key(Some(raw)) {
        return parsed.rest.to_lowercase().starts_with("acp:");
    }
    false
}

// ─── Key Normalization & Building ─────────────────────────────────────────────

pub const DEFAULT_AGENT_ID: &str = "main";
pub const DEFAULT_MAIN_KEY: &str = "main";

pub fn normalize_main_key(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim();
    if trimmed.is_empty() {
        DEFAULT_MAIN_KEY.to_string()
    } else {
        trimmed.to_lowercase()
    }
}

pub fn normalize_agent_id(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim();
    if trimmed.is_empty() {
        return DEFAULT_AGENT_ID.to_string();
    }

    // Ported regex logic: keep alphanumeric, underscore, dash.
    // best-effort: replace others with '-'
    let mut normalized = String::with_capacity(trimmed.len());
    let mut last_was_dash = false;

    for c in trimmed.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
            let nc = c.to_ascii_lowercase();
            if nc == '-' {
                if !normalized.is_empty() && !last_was_dash {
                    normalized.push(nc);
                    last_was_dash = true;
                }
            } else {
                normalized.push(nc);
                last_was_dash = false;
            }
        } else if !normalized.is_empty() && !last_was_dash {
            normalized.push('-');
            last_was_dash = true;
        }
    }

    // Trim leading/trailing dashes
    let result = normalized.trim_matches('-').to_string();
    if result.is_empty() {
        DEFAULT_AGENT_ID.to_string()
    } else if result.len() > 64 {
        result[..64].trim_matches('-').to_string()
    } else {
        result
    }
}

pub fn build_agent_main_session_key(agent_id: &str, main_key: Option<&str>) -> String {
    let aid = normalize_agent_id(Some(agent_id));
    let mk = normalize_main_key(main_key);
    format!("agent:{}:{}", aid, mk)
}

const THREAD_SESSION_MARKERS: &[&str] = &[":thread:", ":topic:"];
const GROUP_SESSION_MARKERS: &[&str] = &[":group:", ":channel:"];

pub fn is_thread_session_key(session_key: Option<&str>) -> bool {
    let raw = session_key.unwrap_or("").trim().to_lowercase();
    if raw.is_empty() {
        return false;
    }
    THREAD_SESSION_MARKERS.iter().any(|m| raw.contains(m))
}

pub fn is_group_session_key(session_key: Option<&str>) -> bool {
    let raw = session_key.unwrap_or("").trim().to_lowercase();
    if raw.is_empty() {
        return false;
    }
    GROUP_SESSION_MARKERS.iter().any(|m| raw.contains(m))
}

pub fn resolve_thread_parent_session_key(session_key: Option<&str>) -> Option<String> {
    let raw = session_key.unwrap_or("").trim();
    if raw.is_empty() {
        return None;
    }
    let normalized = raw.to_lowercase();
    let mut max_idx: i32 = -1;

    for marker in THREAD_SESSION_MARKERS {
        if let Some(idx) = normalized.rfind(marker) {
            if idx as i32 > max_idx {
                max_idx = idx as i32;
            }
        }
    }

    if max_idx <= 0 {
        return None;
    }

    let parent = raw[..max_idx as usize].trim();
    if parent.is_empty() {
        None
    } else {
        Some(parent.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_agent_session_key() {
        assert_eq!(
            parse_agent_session_key(Some("agent:bob:discord:123")),
            Some(ParsedAgentSessionKey {
                agent_id: "bob".into(),
                rest: "discord:123".into()
            })
        );
        assert_eq!(parse_agent_session_key(Some("user:discord")), None);
        assert_eq!(parse_agent_session_key(Some("agent:alice:")), None);
    }

    #[test]
    fn test_is_cron_run() {
        assert!(is_cron_run_session_key(Some(
            "agent:test:cron:daily:run:uuid"
        )));
        assert!(!is_cron_run_session_key(Some(
            "agent:test:cron:daily:fail:uuid"
        )));
        assert!(!is_cron_run_session_key(Some("agent:test:cron:daily:run")));
    }

    #[test]
    fn test_thread_resolution() {
        assert_eq!(
            resolve_thread_parent_session_key(Some("agent:bob:discord:group:123:thread:456")),
            Some("agent:bob:discord:group:123".to_string())
        );
        assert_eq!(
            resolve_thread_parent_session_key(Some("agent:bob:discord:group:123:topic:hello")),
            Some("agent:bob:discord:group:123".to_string())
        );
    }
}
