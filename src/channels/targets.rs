use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessagingTargetKind {
    User,
    Channel,
}

impl ToString for MessagingTargetKind {
    fn to_string(&self) -> String {
        match self {
            MessagingTargetKind::User => "user".to_string(),
            MessagingTargetKind::Channel => "channel".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessagingTarget {
    pub kind: MessagingTargetKind,
    pub id: String,
    pub raw: String,
    pub normalized: String,
}

pub fn normalize_target_id(kind: &MessagingTargetKind, id: &str) -> String {
    format!("{}:{}", kind.to_string(), id).to_lowercase()
}

pub fn build_messaging_target(kind: MessagingTargetKind, id: &str, raw: &str) -> MessagingTarget {
    let normalized = normalize_target_id(&kind, id);
    MessagingTarget {
        kind,
        id: id.to_string(),
        raw: raw.to_string(),
        normalized,
    }
}

pub fn ensure_target_id(
    candidate: &str,
    pattern: &Regex,
    error_message: &str,
) -> Result<String, String> {
    if !pattern.is_match(candidate) {
        return Err(error_message.to_string());
    }
    Ok(candidate.to_string())
}

pub fn parse_target_mention(
    raw: &str,
    mention_pattern: &Regex,
    kind: MessagingTargetKind,
) -> Option<MessagingTarget> {
    if let Some(caps) = mention_pattern.captures(raw) {
        if let Some(m) = caps.get(1) {
            return Some(build_messaging_target(kind, m.as_str(), raw));
        }
    }
    None
}

pub fn parse_target_prefix(
    raw: &str,
    prefix: &str,
    kind: MessagingTargetKind,
) -> Option<MessagingTarget> {
    if !raw.starts_with(prefix) {
        return None;
    }
    let id = raw[prefix.len()..].trim();
    if id.is_empty() {
        None
    } else {
        Some(build_messaging_target(kind, id, raw))
    }
}

pub fn parse_target_prefixes(
    raw: &str,
    prefixes: &[(String, MessagingTargetKind)],
) -> Option<MessagingTarget> {
    for (prefix, kind) in prefixes {
        if let Some(t) = parse_target_prefix(raw, prefix, kind.clone()) {
            return Some(t);
        }
    }
    None
}

pub fn require_target_kind(
    platform: &str,
    target: Option<&MessagingTarget>,
    kind: &MessagingTargetKind,
) -> Result<String, String> {
    let kind_label = kind.to_string();
    if target.is_none() {
        return Err(format!("{} {} id is required.", platform, kind_label));
    }
    let t = target.unwrap();
    if &t.kind != kind {
        return Err(format!(
            "{} {} id is required (use {}:<id>).",
            platform, kind_label, kind_label
        ));
    }
    Ok(t.id.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_normalize_and_build() {
        let t = build_messaging_target(MessagingTargetKind::User, "Alice", "@Alice");
        assert_eq!(t.normalized, "user:alice");
    }

    #[test]
    fn test_parse_mention() {
        let re = Regex::new("@([A-Za-z0-9_\\-]+)").unwrap();
        let m = parse_target_mention("hello @bob!", &re, MessagingTargetKind::User);
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.id, "bob");
    }

    #[test]
    fn test_parse_prefixes() {
        let prefixes = vec![
            ("#".to_string(), MessagingTargetKind::Channel),
            ("@".to_string(), MessagingTargetKind::User),
        ];
        let p = parse_target_prefixes("#general", &prefixes);
        assert!(p.is_some());
        let p = p.unwrap();
        assert_eq!(p.kind, MessagingTargetKind::Channel);
        assert_eq!(p.id, "general");
    }

    #[test]
    fn test_require_target_kind() {
        let t = build_messaging_target(MessagingTargetKind::User, "u1", "@u1");
        let res = require_target_kind("slack", Some(&t), &MessagingTargetKind::User).unwrap();
        assert_eq!(res, "u1");
    }
}
