use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::infra::outbound::missing_target_error;
use crate::utils::normalize_e164;

static WHATSAPP_USER_JID_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\d+)(?::\d+)?@s\.whatsapp\.net$").unwrap());
static WHATSAPP_LID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)@lid$").unwrap());

fn strip_whatsapp_target_prefixes(value: &str) -> String {
    let mut candidate = value.trim().to_string();
    loop {
        let before = candidate.clone();
        if let Some(prefix) = candidate.get(0..9) {
            if prefix.eq_ignore_ascii_case("whatsapp:") {
                candidate = candidate[9..].trim().to_string();
                continue;
            }
        }
        if candidate == before {
            return candidate.trim().to_string();
        }
    }
}

pub fn is_whatsapp_group_jid(value: &str) -> bool {
    let candidate = strip_whatsapp_target_prefixes(value);
    let lower = candidate.to_lowercase();
    if !lower.ends_with("@g.us") {
        return false;
    }
    let local_part = &candidate[..candidate.len() - "@g.us".len()];
    if local_part.is_empty() || local_part.contains('@') {
        return false;
    }
    local_part
        .split('-')
        .all(|segment| !segment.is_empty() && segment.chars().all(|c| c.is_ascii_digit()))
}

pub fn is_whatsapp_user_target(value: &str) -> bool {
    let candidate = strip_whatsapp_target_prefixes(value);
    WHATSAPP_USER_JID_RE.is_match(&candidate) || WHATSAPP_LID_RE.is_match(&candidate)
}

fn extract_user_jid_phone(jid: &str) -> Option<String> {
    if let Some(caps) = WHATSAPP_USER_JID_RE.captures(jid) {
        return caps.get(1).map(|m| m.as_str().to_string());
    }
    if let Some(caps) = WHATSAPP_LID_RE.captures(jid) {
        return caps.get(1).map(|m| m.as_str().to_string());
    }
    None
}

pub fn normalize_whatsapp_target(value: &str) -> Option<String> {
    let candidate = strip_whatsapp_target_prefixes(value);
    if candidate.is_empty() {
        return None;
    }
    if is_whatsapp_group_jid(&candidate) {
        let local_part = &candidate[..candidate.len() - "@g.us".len()];
        return Some(format!("{}@g.us", local_part));
    }
    if is_whatsapp_user_target(&candidate) {
        let phone = extract_user_jid_phone(&candidate)?;
        let normalized = normalize_e164(&phone);
        return (normalized.len() > 1).then(|| normalized);
    }
    if candidate.contains('@') {
        return None;
    }
    let normalized = normalize_e164(&candidate);
    (normalized.len() > 1).then(|| normalized)
}

pub fn resolve_whatsapp_outbound_target(
    to: Option<&str>,
    allow_from: &[String],
    mode: Option<&str>,
) -> Result<String> {
    let trimmed = to.unwrap_or("").trim();
    let allow_list_raw: Vec<String> = allow_from
        .iter()
        .map(|entry| entry.trim().to_string())
        .filter(|entry| !entry.is_empty())
        .collect();
    let has_wildcard = allow_list_raw.iter().any(|entry| entry == "*");
    let allow_list: Vec<String> = allow_list_raw
        .iter()
        .filter(|entry| entry != &"*")
        .filter_map(|entry| normalize_whatsapp_target(entry))
        .collect();

    let target = trimmed.trim();
    if target.is_empty() {
        return Err(missing_target_error("WhatsApp", Some("<E.164|group JID>")));
    }
    let normalized_to = normalize_whatsapp_target(target)
        .ok_or_else(|| missing_target_error("WhatsApp", Some("<E.164|group JID>")))?;
    if is_whatsapp_group_jid(&normalized_to) {
        return Ok(normalized_to);
    }

    let normalized_mode = mode.map(|m| m.trim().to_ascii_lowercase());
    match normalized_mode.as_deref() {
        Some("implicit") | Some("heartbeat") => {
            if has_wildcard || allow_list.is_empty() || allow_list.contains(&normalized_to) {
                Ok(normalized_to)
            } else {
                Err(missing_target_error("WhatsApp", Some("<E.164|group JID>")))
            }
        }
        _ => Ok(normalized_to),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_whatsapp_target_handles_groups() {
        assert_eq!(
            normalize_whatsapp_target("120363401234567890@g.us"),
            Some("120363401234567890@g.us".to_string())
        );
        assert_eq!(
            normalize_whatsapp_target("123456789-987654321@g.us"),
            Some("123456789-987654321@g.us".to_string())
        );
        assert_eq!(
            normalize_whatsapp_target("whatsapp:120363401234567890@g.us"),
            Some("120363401234567890@g.us".to_string())
        );
    }

    #[test]
    fn normalize_whatsapp_target_handles_user_jids() {
        assert_eq!(
            normalize_whatsapp_target("1555123@s.whatsapp.net"),
            Some("+1555123".to_string())
        );
        assert_eq!(
            normalize_whatsapp_target("41796666864:0@s.whatsapp.net"),
            Some("+41796666864".to_string())
        );
        assert_eq!(
            normalize_whatsapp_target("123456789@lid"),
            Some("+123456789".to_string())
        );
        assert!(normalize_whatsapp_target("abc@s.whatsapp.net").is_none());
    }

    #[test]
    fn normalize_whatsapp_target_rejects_ambiguous_inputs() {
        assert!(normalize_whatsapp_target("wat").is_none());
        assert!(normalize_whatsapp_target("whatsapp:").is_none());
        assert!(normalize_whatsapp_target("@g.us").is_none());
        assert!(normalize_whatsapp_target("group:123456789@g.us").is_none());
        assert!(normalize_whatsapp_target("whatsapp:group:120@g.us").is_none());
    }

    #[test]
    fn resolve_whatsapp_outbound_target_requires_allowlist() {
        let err = resolve_whatsapp_outbound_target(None, &[], None).unwrap_err();
        assert!(err.to_string().contains("WhatsApp"));
    }

    #[test]
    fn resolve_whatsapp_outbound_target_obeys_mode() {
        let allow = vec!["+1555123".to_string()];
        let resolved =
            resolve_whatsapp_outbound_target(Some("+1555123"), &allow, Some("implicit")).unwrap();
        assert_eq!(resolved, "+1555123");
        assert!(
            resolve_whatsapp_outbound_target(Some("+1555124"), &allow, Some("implicit")).is_err()
        );
    }
}
