use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlackChannelLookup {
    pub id: String,
    pub name: String,
    pub archived: bool,
    pub is_private: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlackChannelResolution {
    pub input: String,
    pub resolved: bool,
    pub id: Option<String>,
    pub name: Option<String>,
    pub archived: Option<bool>,
}

fn resolve_by_name(name: &str, channels: &[SlackChannelLookup]) -> Option<SlackChannelLookup> {
    let target = name.trim().to_lowercase();
    if target.is_empty() { return None; }
    let matches: Vec<_> = channels.iter().filter(|c| c.name.to_lowercase() == target).cloned().collect();
    if matches.is_empty() { return None; }
    let active = matches.iter().find(|c| !c.archived).cloned();
    Some(active.unwrap_or_else(|| matches[0].clone()))
}

pub fn resolve_slack_channel_allowlist(entries: &[String], channels: &[SlackChannelLookup]) -> Vec<SlackChannelResolution> {
    let mut out = Vec::new();
    for input in entries {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            out.push(SlackChannelResolution { input: input.clone(), resolved: false, id: None, name: None, archived: None });
            continue;
        }
        if trimmed.starts_with("<#") && trimmed.ends_with('>') {
            // format: <#C123|name>
            let inner = &trimmed[2..trimmed.len()-1];
            let parts: Vec<_> = inner.splitn(2, '|').collect();
            let id = parts.get(0).map(|s| s.to_uppercase());
            let name = parts.get(1).map(|s| s.trim().to_string());
            if let Some(id) = id {
                let m = channels.iter().find(|c| c.id == id);
                out.push(SlackChannelResolution { input: input.clone(), resolved: true, id: Some(id), name: name.or(m.map(|x| x.name.clone())), archived: m.map(|x| x.archived) });
                continue;
            }
        }
        let pref = trimmed.trim_start_matches(|c| c == 's' || c == 'S' || c == 'l' || c == 'L' || c == 'a' || c == 'A' || c == 'c' || c == 'C' || c == 'h' || c == 'H' || c == 'n' || c == 'N' || c == ':' );
        if pref.starts_with('#') {
            let name = pref.trim_start_matches('#').to_string();
            if let Some(m) = resolve_by_name(&name, channels) {
                out.push(SlackChannelResolution { input: input.clone(), resolved: true, id: Some(m.id.clone()), name: Some(m.name.clone()), archived: Some(m.archived) });
                continue;
            }
        }
        // If raw looks like ID (starts with C or G)
        if pref.len() > 0 && (pref.starts_with('C') || pref.starts_with('G')) {
            let id = pref.to_uppercase();
            let m = channels.iter().find(|c| c.id == id);
            out.push(SlackChannelResolution { input: input.clone(), resolved: true, id: Some(id.clone()), name: m.map(|x| x.name.clone()).or(Some(pref.to_string())), archived: m.map(|x| x.archived) });
            continue;
        }
        out.push(SlackChannelResolution { input: input.clone(), resolved: false, id: None, name: None, archived: None });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_channels() -> Vec<SlackChannelLookup> {
        vec![
            SlackChannelLookup { id: "C1".to_string(), name: "general".to_string(), archived: false, is_private: false },
            SlackChannelLookup { id: "C2".to_string(), name: "random".to_string(), archived: true, is_private: false },
        ]
    }

    #[test]
    fn test_resolve_by_mention() {
        let channels = sample_channels();
        let entries = vec!["<#C1|general>".to_string()];
        let r = resolve_slack_channel_allowlist(&entries, &channels);
        assert!(r[0].resolved);
        assert_eq!(r[0].id.as_deref(), Some("C1"));
    }

    #[test]
    fn test_resolve_by_name() {
        let channels = sample_channels();
        let entries = vec!["#general".to_string()];
        let r = resolve_slack_channel_allowlist(&entries, &channels);
        assert!(r[0].resolved);
        assert_eq!(r[0].id.as_deref(), Some("C1"));
    }
}
