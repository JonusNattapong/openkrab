use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Channel configuration skeleton.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub id: String,
    pub enabled: bool,
}

impl ChannelConfig {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), enabled: true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelMatchSource {
    Direct,
    Parent,
    Wildcard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelEntryMatch<T>
where
    T: Clone,
{
    pub entry: Option<T>,
    pub key: Option<String>,
    pub wildcard_entry: Option<T>,
    pub wildcard_key: Option<String>,
    pub parent_entry: Option<T>,
    pub parent_key: Option<String>,
    pub match_key: Option<String>,
    pub match_source: Option<ChannelMatchSource>,
}

impl<T> ChannelEntryMatch<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            entry: None,
            key: None,
            wildcard_entry: None,
            wildcard_key: None,
            parent_entry: None,
            parent_key: None,
            match_key: None,
            match_source: None,
        }
    }
}

pub fn normalize_channel_slug(value: &str) -> String {
    let s = value.trim().to_lowercase();
    let s = s.strip_prefix('#').unwrap_or(&s).to_string();

    let mut out = String::with_capacity(s.len());
    let mut last_was_dash = false;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_was_dash = false;
        } else {
            if !last_was_dash {
                out.push('-');
                last_was_dash = true;
            }
        }
    }
    // trim leading/trailing dashes
    let out = out.trim_matches('-').to_string();
    out
}

pub fn build_channel_key_candidates<'a, I>(keys: I) -> Vec<String>
where
    I: IntoIterator<Item = Option<&'a str>>,
{
    let mut seen = HashSet::new();
    let mut candidates = Vec::new();
    for opt in keys {
        if let Some(k) = opt {
            let trimmed = k.trim();
            if trimmed.is_empty() || seen.contains(trimmed) {
                continue;
            }
            seen.insert(trimmed.to_string());
            candidates.push(trimmed.to_string());
        }
    }
    candidates
}

pub fn resolve_channel_entry_match<T>(entries: &HashMap<String, T>, keys: &[String], wildcard_key: Option<&str>) -> ChannelEntryMatch<T>
where
    T: Clone,
{
    let mut m = ChannelEntryMatch::new();
    for key in keys {
        if entries.contains_key(key) {
            m.entry = entries.get(key).cloned();
            m.key = Some(key.clone());
            break;
        }
    }
    if let Some(wk) = wildcard_key {
        if entries.contains_key(wk) {
            m.wildcard_entry = entries.get(wk).cloned();
            m.wildcard_key = Some(wk.to_string());
        }
    }
    m
}

pub fn resolve_channel_entry_match_with_fallback<T, F>(
    entries: &HashMap<String, T>,
    keys: &[String],
    parent_keys: Option<&[String]>,
    wildcard_key: Option<&str>,
    normalize_key: Option<F>,
) -> ChannelEntryMatch<T>
where
    T: Clone,
    F: Fn(&str) -> Option<String>,
{
    let direct = resolve_channel_entry_match(entries, keys, wildcard_key);

    if direct.entry.is_some() && direct.key.is_some() {
        let mut d = direct.clone();
        d.match_key = d.key.clone();
        d.match_source = Some(ChannelMatchSource::Direct);
        return d;
    }

    if let Some(norm) = &normalize_key {
        let normalized_keys: Vec<String> = keys
            .iter()
            .filter_map(|k| norm(k))
            .filter(|s| !s.is_empty())
            .collect();
        if !normalized_keys.is_empty() {
            for (entry_key, entry) in entries.iter() {
                if let Some(ne) = norm(entry_key) {
                    if !ne.is_empty() && normalized_keys.contains(&ne) {
                        let mut res = direct.clone();
                        res.entry = Some(entry.clone());
                        res.key = Some(entry_key.clone());
                        res.match_key = Some(entry_key.clone());
                        res.match_source = Some(ChannelMatchSource::Direct);
                        return res;
                    }
                }
            }
        }
    }

    if let Some(pk) = parent_keys {
        if !pk.is_empty() {
            let parent = resolve_channel_entry_match(entries, pk, wildcard_key);
            if parent.entry.is_some() && parent.key.is_some() {
                let mut res = direct.clone();
                res.entry = parent.entry.clone();
                res.key = parent.key.clone();
                res.parent_entry = parent.entry.clone();
                res.parent_key = parent.key.clone();
                res.match_key = parent.key.clone();
                res.match_source = Some(ChannelMatchSource::Parent);
                return res;
            }
            if let Some(norm) = &normalize_key {
                let normalized_parent_keys: Vec<String> = pk.iter().filter_map(|k| norm(k)).filter(|s| !s.is_empty()).collect();
                if !normalized_parent_keys.is_empty() {
                    for (entry_key, entry) in entries.iter() {
                        if let Some(ne) = norm(entry_key) {
                            if !ne.is_empty() && normalized_parent_keys.contains(&ne) {
                                let mut res = direct.clone();
                                res.entry = Some(entry.clone());
                                res.key = Some(entry_key.clone());
                                res.parent_entry = Some(entry.clone());
                                res.parent_key = Some(entry_key.clone());
                                res.match_key = Some(entry_key.clone());
                                res.match_source = Some(ChannelMatchSource::Parent);
                                return res;
                            }
                        }
                    }
                }
            }
        }
    }

    if direct.wildcard_entry.is_some() && direct.wildcard_key.is_some() {
        let mut res = direct.clone();
        res.entry = res.wildcard_entry.clone();
        res.key = res.wildcard_key.clone();
        res.match_key = res.wildcard_key.clone();
        res.match_source = Some(ChannelMatchSource::Wildcard);
        return res;
    }

    direct
}

pub fn resolve_nested_allowlist_decision(params: (bool, bool, bool, bool)) -> bool {
    let (outer_configured, outer_matched, inner_configured, inner_matched) = params;
    if !outer_configured {
        return true;
    }
    if !outer_matched {
        return false;
    }
    if !inner_configured {
        return true;
    }
    inner_matched
}
