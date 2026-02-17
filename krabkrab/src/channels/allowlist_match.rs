#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowlistMatchSource {
    Wildcard,
    Id,
    Name,
    Tag,
    Username,
    PrefixedId,
    PrefixedUser,
    PrefixedName,
    Slug,
    Localpart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowlistMatch {
    pub allowed: bool,
    pub match_key: Option<String>,
    pub match_source: Option<AllowlistMatchSource>,
}

impl AllowlistMatch {
    pub fn none() -> Self {
        Self { allowed: false, match_key: None, match_source: None }
    }
}

pub fn format_allowlist_match_meta(m: Option<(&str, &str)>) -> String {
    match m {
        Some((k, s)) => format!("matchKey={} matchSource={}", k, s),
        None => "matchKey=none matchSource=none".to_string(),
    }
}

pub fn resolve_allowlist_match_simple(
    allow_from: &[String],
    sender_id: &str,
    sender_name: Option<&str>,
) -> AllowlistMatch {
    let allow_from_norm: Vec<String> = allow_from
        .iter()
        .map(|e| e.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if allow_from_norm.is_empty() {
        return AllowlistMatch::none();
    }
    if allow_from_norm.iter().any(|s| s == "*") {
        return AllowlistMatch { allowed: true, match_key: Some("*".to_string()), match_source: Some(AllowlistMatchSource::Wildcard) };
    }

    let sid = sender_id.to_lowercase();
    if allow_from_norm.iter().any(|s| s == &sid) {
        return AllowlistMatch { allowed: true, match_key: Some(sid), match_source: Some(AllowlistMatchSource::Id) };
    }

    if let Some(name) = sender_name.map(|s| s.to_lowercase()) {
        if allow_from_norm.iter().any(|s| s == &name) {
            return AllowlistMatch { allowed: true, match_key: Some(name), match_source: Some(AllowlistMatchSource::Name) };
        }
    }

    AllowlistMatch::none()
}
