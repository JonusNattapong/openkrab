use std::fmt;

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

impl fmt::Display for AllowlistMatchSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AllowlistMatchSource::Wildcard => "wildcard",
            AllowlistMatchSource::Id => "id",
            AllowlistMatchSource::Name => "name",
            AllowlistMatchSource::Tag => "tag",
            AllowlistMatchSource::Username => "username",
            AllowlistMatchSource::PrefixedId => "prefixed-id",
            AllowlistMatchSource::PrefixedUser => "prefixed-user",
            AllowlistMatchSource::PrefixedName => "prefixed-name",
            AllowlistMatchSource::Slug => "slug",
            AllowlistMatchSource::Localpart => "localpart",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowlistMatch {
    pub allowed: bool,
    pub match_key: Option<String>,
    pub match_source: Option<AllowlistMatchSource>,
}

impl AllowlistMatch {
    pub fn none() -> Self {
        Self {
            allowed: false,
            match_key: None,
            match_source: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchMeta {
    pub match_key: Option<String>,
    pub match_source: Option<String>,
}

pub fn format_allowlist_match_meta(m: Option<&MatchMeta>) -> String {
    format!(
        "matchKey={} matchSource= {}",
        m.and_then(|x| x.match_key.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("none"),
        m.and_then(|x| x.match_source.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("none")
    )
}

pub fn resolve_allowlist_match_simple(
    allow_from: &Vec<String>,
    sender_id: &str,
    sender_name: Option<&str>,
) -> AllowlistMatch {
    let allow_norm: Vec<String> = allow_from
        .iter()
        .map(|entry| entry.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if allow_norm.is_empty() {
        return AllowlistMatch::none();
    }

    if allow_norm.iter().any(|s| s == "*") {
        return AllowlistMatch {
            allowed: true,
            match_key: Some("*".to_string()),
            match_source: Some(AllowlistMatchSource::Wildcard),
        };
    }

    let sender_id_l = sender_id.to_lowercase();
    if allow_norm.iter().any(|s| s == &sender_id_l) {
        return AllowlistMatch {
            allowed: true,
            match_key: Some(sender_id_l),
            match_source: Some(AllowlistMatchSource::Id),
        };
    }

    if let Some(name) = sender_name {
        let name_l = name.to_lowercase();
        if allow_norm.iter().any(|s| s == &name_l) {
            return AllowlistMatch {
                allowed: true,
                match_key: Some(name_l),
                match_source: Some(AllowlistMatchSource::Name),
            };
        }
    }

    AllowlistMatch::none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_none() {
        let s = format_allowlist_match_meta(None);
        assert!(s.contains("matchKey=none"));
    }

    #[test]
    fn test_resolve_wildcard() {
        let allow = vec!["*".to_string()];
        let m = resolve_allowlist_match_simple(&allow, "user1", Some("Alice"));
        assert!(m.allowed);
        assert_eq!(m.match_key.unwrap(), "*");
    }

    #[test]
    fn test_resolve_id_and_name() {
        let allow = vec!["alice".to_string(), "bob".to_string()];
        let m1 = resolve_allowlist_match_simple(&allow, "alice", Some("Alice"));
        assert!(m1.allowed);
        assert_eq!(m1.match_source.unwrap(), AllowlistMatchSource::Id);

        let m2 = resolve_allowlist_match_simple(&allow, "charlie", Some("bob"));
        assert!(m2.allowed);
        assert_eq!(m2.match_source.unwrap(), AllowlistMatchSource::Name);
    }
}
