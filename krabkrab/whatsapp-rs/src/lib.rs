use regex::Regex;

fn strip_whatsapp_target_prefixes(value: &str) -> String {
    let mut candidate = value.trim().to_string();
    loop {
        let before = candidate.clone();
        candidate = Regex::new(r"(?i)^whatsapp:")
            .unwrap()
            .replace(&candidate, "")
            .trim()
            .to_string();
        if candidate == before {
            return candidate;
        }
    }
}

pub fn is_whatsapp_group_jid(value: &str) -> bool {
    let candidate = strip_whatsapp_target_prefixes(value);
    let lower = candidate.to_lowercase();
    if !lower.ends_with("@g.us") {
        return false;
    }
    let local = &candidate[..candidate.len() - "@g.us".len()];
    if local.is_empty() || local.contains('@') {
        return false;
    }
    let re = Regex::new(r"^[0-9]+(-[0-9]+)*$").unwrap();
    re.is_match(local)
}

fn extract_user_jid_phone(jid: &str) -> Option<String> {
    // ^(\d+)(?::\d+)?@s\.whatsapp\.net$
    let re_user = Regex::new(r"^(?i)(\d+)(?::\d+)?@s\.whatsapp\.net$").unwrap();
    if let Some(caps) = re_user.captures(jid) {
        if let Some(m) = caps.get(1) {
            return Some(m.as_str().to_string());
        }
    }
    let re_lid = Regex::new(r"(?i)^(\d+)@lid$").unwrap();
    if let Some(caps) = re_lid.captures(jid) {
        if let Some(m) = caps.get(1) {
            return Some(m.as_str().to_string());
        }
    }
    None
}

fn normalize_e164(number: &str) -> String {
    let without_prefix = Regex::new(r"(?i)^whatsapp:")
        .unwrap()
        .replace(number, "")
        .to_string()
        .trim()
        .to_string();
    let digits: String = without_prefix.chars().filter(|c| c.is_digit(10) || *c == '+').collect();
    if digits.starts_with('+') {
        format!("+{}", digits.trim_start_matches('+'))
    } else {
        format!("+{}", digits)
    }
}

pub fn to_whatsapp_jid(number: &str) -> String {
    let without_prefix = Regex::new(r"(?i)^whatsapp:")
        .unwrap()
        .replace(number, "")
        .to_string()
        .trim()
        .to_string();
    if without_prefix.contains('@') {
        return without_prefix;
    }
    let e164 = normalize_e164(&without_prefix);
    let digits: String = e164.chars().filter(|c| c.is_digit(10)).collect();
    format!("{}@s.whatsapp.net", digits)
}

pub fn jid_to_e164(jid: &str) -> Option<String> {
    jid_to_e164_with_opts(jid, None, None)
}

pub fn jid_to_e164_with_opts(jid: &str, auth_dir: Option<&str>, lid_mapping_dirs: Option<&[&str]>) -> Option<String> {
    // Direct match: ^(\d+)(?::\d+)?@(s.whatsapp.net|hosted)$
    let re_direct = Regex::new(r"(?i)^(\d+)(?::\d+)?@(s\.whatsapp\.net|hosted)$").unwrap();
    if let Some(caps) = re_direct.captures(jid) {
        if let Some(m) = caps.get(1) {
            return Some(format!("+{}", m.as_str()));
        }
    }

    // LID formats: ^(\d+)(?::\d+)?@(lid|hosted.lid)$
    let re_lid = Regex::new(r"(?i)^(\d+)(?::\d+)?@(lid|hosted\.lid)$").unwrap();
    if let Some(caps) = re_lid.captures(jid) {
        if let Some(m) = caps.get(1) {
            let lid = m.as_str();
            if let Some(found) = read_lid_reverse_mapping(lid, auth_dir, lid_mapping_dirs) {
                return Some(found);
            }
        }
    }

    None

}

fn read_lid_reverse_mapping(lid: &str, auth_dir: Option<&str>, lid_mapping_dirs: Option<&[&str]>) -> Option<String> {
    use std::fs;
    use std::path::PathBuf;

    let filename = format!("lid-mapping-{}_reverse.json", lid);
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Some(a) = auth_dir {
        candidates.push(PathBuf::from(a));
    }
    if let Some(dirs) = lid_mapping_dirs {
        for d in dirs { candidates.push(PathBuf::from(d)); }
    }
    // Try ./credentials and ./ by default
    candidates.push(PathBuf::from("./"));
    candidates.push(PathBuf::from("./credentials"));

    for dir in candidates {
        let path = dir.join(&filename);
        if !path.exists() {
            continue;
        }
        match fs::read_to_string(&path) {
            Ok(data) => {
                // mapping could be a JSON string or number
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&data) {
                    if json_value.is_string() {
                        if let Some(s) = json_value.as_str() {
                            let normalized = normalize_e164(s);
                            return Some(normalized);
                        }
                    } else if json_value.is_number() {
                        let s = json_value.to_string();
                        let normalized = normalize_e164(&s);
                        return Some(normalized);
                    }
                }
            }
            Err(_) => continue,
        }
    }
    None
}

pub fn is_whatsapp_user_target(value: &str) -> bool {
    let candidate = strip_whatsapp_target_prefixes(value);
    let re_user = Regex::new(r"(?i)^(\d+)(?::\d+)?@s\.whatsapp\.net$").unwrap();
    let re_lid = Regex::new(r"(?i)^(\d+)@lid$").unwrap();
    re_user.is_match(&candidate) || re_lid.is_match(&candidate)
}

pub fn normalize_whatsapp_target(value: &str) -> Option<String> {
    let candidate = strip_whatsapp_target_prefixes(value);
    if candidate.is_empty() {
        return None;
    }
    if is_whatsapp_group_jid(&candidate) {
        let local = &candidate[..candidate.len() - "@g.us".len()];
        return Some(format!("{}@g.us", local));
    }
    if is_whatsapp_user_target(&candidate) {
        if let Some(phone) = extract_user_jid_phone(&candidate) {
            let normalized = normalize_e164(&phone);
            return if normalized.len() > 1 { Some(normalized) } else { None };
        }
        return None;
    }
    if candidate.contains('@') {
        return None;
    }
    let normalized = normalize_e164(&candidate);
    if normalized.len() > 1 {
        Some(normalized)
    } else {
        None
    }
}

pub enum WhatsAppOutboundTargetResolution {
    Ok { to: String },
    Err { error: String },
}

pub fn resolve_whatsapp_outbound_target(
    to: Option<&str>,
    allow_from: Option<&[&str]>,
    mode: Option<&str>,
) -> WhatsAppOutboundTargetResolution {
    let trimmed = to.unwrap_or("").trim();
    let allow_list_raw: Vec<String> = allow_from
        .unwrap_or(&[])
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let has_wildcard = allow_list_raw.iter().any(|e| e == "*");
    let allow_list: Vec<String> = allow_list_raw
        .iter()
        .filter(|e| *e != "*")
        .filter_map(|e| normalize_whatsapp_target(e))
        .collect();

    if !trimmed.is_empty() {
        if let Some(normalized_to) = normalize_whatsapp_target(trimmed) {
            if is_whatsapp_group_jid(&normalized_to) {
                return WhatsAppOutboundTargetResolution::Ok { to: normalized_to };
            }
            if mode == Some("implicit") || mode == Some("heartbeat") {
                if has_wildcard || allow_list.is_empty() {
                    return WhatsAppOutboundTargetResolution::Ok { to: normalized_to };
                }
                if allow_list.iter().any(|e| e == &normalized_to) {
                    return WhatsAppOutboundTargetResolution::Ok { to: normalized_to };
                }
                return WhatsAppOutboundTargetResolution::Err {
                    error: missing_target_error(),
                };
            }
            return WhatsAppOutboundTargetResolution::Ok { to: normalized_to };
        } else {
            return WhatsAppOutboundTargetResolution::Err {
                error: missing_target_error(),
            };
        }
    }

    WhatsAppOutboundTargetResolution::Err {
        error: missing_target_error(),
    }
}

fn missing_target_error() -> String {
    "missing target for WhatsApp: expected <E.164|group JID>".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_jid_preserve() {
        assert_eq!(normalize_whatsapp_target("120363401234567890@g.us"), Some("120363401234567890@g.us".into()));
        assert_eq!(normalize_whatsapp_target("123456789-987654321@g.us"), Some("123456789-987654321@g.us".into()));
        assert_eq!(normalize_whatsapp_target("whatsapp:120363401234567890@g.us"), Some("120363401234567890@g.us".into()));
    }

    #[test]
    fn test_normalizes_direct_jid() {
        assert_eq!(normalize_whatsapp_target("1555123@s.whatsapp.net"), Some("+1555123".into()));
    }

    #[test]
    fn test_device_suffix() {
        assert_eq!(normalize_whatsapp_target("41796666864:0@s.whatsapp.net"), Some("+41796666864".into()));
        assert_eq!(normalize_whatsapp_target("1234567890:123@s.whatsapp.net"), Some("+1234567890".into()));
        assert_eq!(normalize_whatsapp_target("41796666864@s.whatsapp.net"), Some("+41796666864".into()));
    }

    #[test]
    fn test_lid_jids() {
        assert_eq!(normalize_whatsapp_target("123456789@lid"), Some("+123456789".into()));
        assert_eq!(normalize_whatsapp_target("123456789@LID"), Some("+123456789".into()));
    }

    #[test]
    fn test_reject_invalid() {
        assert_eq!(normalize_whatsapp_target("wat"), None);
        assert_eq!(normalize_whatsapp_target("whatsapp:"), None);
        assert_eq!(normalize_whatsapp_target("@g.us"), None);
    }

    #[test]
    fn test_repeated_prefixes() {
        assert_eq!(normalize_whatsapp_target("whatsapp:whatsapp:+1555"), Some("+1555".into()));
    }
}
