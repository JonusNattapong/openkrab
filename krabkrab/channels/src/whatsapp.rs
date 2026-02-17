#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatsAppOutboundTargetResolution {
    Ok { to: String },
    Err { message: String },
}

#[derive(Debug, Clone)]
pub struct ResolveWhatsAppOutboundTargetParams {
    pub to: Option<String>,
    pub allow_from: Option<Vec<String>>,
    pub mode: Option<String>,
}

fn strip_whatsapp_target_prefixes(value: &str) -> String {
    let mut candidate = value.trim().to_string();

    loop {
        let lowered = candidate.to_ascii_lowercase();
        if lowered.starts_with("whatsapp:") {
            candidate = candidate["whatsapp:".len()..].trim().to_string();
            continue;
        }
        return candidate;
    }
}

fn normalize_e164(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(rest) = trimmed.strip_prefix('+') {
        if rest.chars().all(|c| c.is_ascii_digit()) && !rest.is_empty() {
            return Some(format!("+{rest}"));
        }
        return None;
    }

    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Some(format!("+{trimmed}"));
    }

    None
}

pub fn is_whatsapp_group_jid(value: &str) -> bool {
    let candidate = strip_whatsapp_target_prefixes(value);
    let lower = candidate.to_ascii_lowercase();

    if !lower.ends_with("@g.us") {
        return false;
    }

    let local_part = &candidate[..candidate.len() - "@g.us".len()];
    if local_part.is_empty() || local_part.contains('@') {
        return false;
    }

    local_part
        .split('-')
        .all(|chunk| !chunk.is_empty() && chunk.chars().all(|c| c.is_ascii_digit()))
}

pub fn is_whatsapp_user_target(value: &str) -> bool {
    extract_user_jid_phone(&strip_whatsapp_target_prefixes(value)).is_some()
}

fn extract_user_jid_phone(jid: &str) -> Option<String> {
    let lowered = jid.to_ascii_lowercase();

    if let Some(local) = lowered.strip_suffix("@s.whatsapp.net") {
        let mut parts = local.split(':');
        let phone = parts.next()?;
        let device = parts.next();

        if parts.next().is_some() {
            return None;
        }

        if !phone.chars().all(|c| c.is_ascii_digit()) || phone.is_empty() {
            return None;
        }

        if let Some(dev) = device {
            if !dev.chars().all(|c| c.is_ascii_digit()) || dev.is_empty() {
                return None;
            }
        }

        return Some(phone.to_string());
    }

    if let Some(local) = lowered.strip_suffix("@lid") {
        if local.chars().all(|c| c.is_ascii_digit()) && !local.is_empty() {
            return Some(local.to_string());
        }
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
        return Some(format!("{local_part}@g.us"));
    }

    if is_whatsapp_user_target(&candidate) {
        let phone = extract_user_jid_phone(&candidate)?;
        let normalized = normalize_e164(&phone)?;
        return (normalized.len() > 1).then_some(normalized);
    }

    if candidate.contains('@') {
        return None;
    }

    let normalized = normalize_e164(&candidate)?;
    (normalized.len() > 1).then_some(normalized)
}

fn missing_target_error() -> String {
    "Missing or invalid WhatsApp target; expected <E.164|group JID>".to_string()
}

pub fn resolve_whatsapp_outbound_target(
    params: ResolveWhatsAppOutboundTargetParams,
) -> WhatsAppOutboundTargetResolution {
    let trimmed = params.to.unwrap_or_default().trim().to_string();

    let allow_list_raw: Vec<String> = params
        .allow_from
        .unwrap_or_default()
        .into_iter()
        .map(|entry| entry.trim().to_string())
        .filter(|entry| !entry.is_empty())
        .collect();

    let has_wildcard = allow_list_raw.iter().any(|entry| entry == "*");

    let allow_list: Vec<String> = allow_list_raw
        .iter()
        .filter(|entry| entry.as_str() != "*")
        .filter_map(|entry| normalize_whatsapp_target(entry))
        .collect();

    if !trimmed.is_empty() {
        let normalized_to = match normalize_whatsapp_target(&trimmed) {
            Some(to) => to,
            None => {
                return WhatsAppOutboundTargetResolution::Err {
                    message: missing_target_error(),
                };
            }
        };

        if is_whatsapp_group_jid(&normalized_to) {
            return WhatsAppOutboundTargetResolution::Ok { to: normalized_to };
        }

        let mode = params.mode.unwrap_or_default();
        if mode == "implicit" || mode == "heartbeat" {
            if has_wildcard || allow_list.is_empty() || allow_list.contains(&normalized_to) {
                return WhatsAppOutboundTargetResolution::Ok { to: normalized_to };
            }

            return WhatsAppOutboundTargetResolution::Err {
                message: missing_target_error(),
            };
        }

        return WhatsAppOutboundTargetResolution::Ok { to: normalized_to };
    }

    WhatsAppOutboundTargetResolution::Err {
        message: missing_target_error(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_preserves_group_jids() {
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
    fn normalize_user_jids_to_e164() {
        assert_eq!(
            normalize_whatsapp_target("41796666864:0@s.whatsapp.net"),
            Some("+41796666864".to_string())
        );
        assert_eq!(
            normalize_whatsapp_target("1234567890:123@s.whatsapp.net"),
            Some("+1234567890".to_string())
        );
        assert_eq!(
            normalize_whatsapp_target("41796666864@s.whatsapp.net"),
            Some("+41796666864".to_string())
        );
    }

    #[test]
    fn normalize_lid_jids_to_e164() {
        assert_eq!(
            normalize_whatsapp_target("123456789@lid"),
            Some("+123456789".to_string())
        );
        assert_eq!(
            normalize_whatsapp_target("123456789@LID"),
            Some("+123456789".to_string())
        );
    }

    #[test]
    fn normalize_rejects_invalid_targets() {
        assert_eq!(normalize_whatsapp_target("wat"), None);
        assert_eq!(normalize_whatsapp_target("whatsapp:"), None);
        assert_eq!(normalize_whatsapp_target("@g.us"), None);
        assert_eq!(normalize_whatsapp_target("whatsapp:group:@g.us"), None);
        assert_eq!(
            normalize_whatsapp_target("whatsapp:group:120363401234567890@g.us"),
            None
        );
        assert_eq!(normalize_whatsapp_target("group:123456789-987654321@g.us"), None);
        assert_eq!(
            normalize_whatsapp_target(" WhatsApp:Group:123456789-987654321@G.US "),
            None
        );
        assert_eq!(normalize_whatsapp_target("abc@s.whatsapp.net"), None);
    }

    #[test]
    fn handles_repeated_prefixes() {
        assert_eq!(
            normalize_whatsapp_target("whatsapp:whatsapp:+1555"),
            Some("+1555".to_string())
        );
        assert_eq!(normalize_whatsapp_target("group:group:120@g.us"), None);
    }

    #[test]
    fn detects_user_targets() {
        assert!(is_whatsapp_user_target("41796666864:0@s.whatsapp.net"));
        assert!(is_whatsapp_user_target("1234567890@s.whatsapp.net"));
        assert!(is_whatsapp_user_target("123456789@lid"));
        assert!(is_whatsapp_user_target("123456789@LID"));
        assert!(!is_whatsapp_user_target("123@lid:0"));
        assert!(!is_whatsapp_user_target("abc@s.whatsapp.net"));
        assert!(!is_whatsapp_user_target("123456789-987654321@g.us"));
        assert!(!is_whatsapp_user_target("+1555123"));
    }

    #[test]
    fn detects_group_jids() {
        assert!(is_whatsapp_group_jid("120363401234567890@g.us"));
        assert!(is_whatsapp_group_jid("123456789-987654321@g.us"));
        assert!(is_whatsapp_group_jid("whatsapp:120363401234567890@g.us"));
        assert!(!is_whatsapp_group_jid("whatsapp:group:120363401234567890@g.us"));
        assert!(!is_whatsapp_group_jid("x@g.us"));
        assert!(!is_whatsapp_group_jid("@g.us"));
        assert!(!is_whatsapp_group_jid("120@g.usx"));
        assert!(!is_whatsapp_group_jid("+1555123"));
    }

    #[test]
    fn resolve_outbound_target_allows_valid_explicit() {
        let result = resolve_whatsapp_outbound_target(ResolveWhatsAppOutboundTargetParams {
            to: Some("1555123".to_string()),
            allow_from: None,
            mode: None,
        });

        assert_eq!(
            result,
            WhatsAppOutboundTargetResolution::Ok {
                to: "+1555123".to_string()
            }
        );
    }

    #[test]
    fn resolve_outbound_target_blocks_implicit_not_allowlisted() {
        let result = resolve_whatsapp_outbound_target(ResolveWhatsAppOutboundTargetParams {
            to: Some("1555000".to_string()),
            allow_from: Some(vec!["+1555123".to_string()]),
            mode: Some("implicit".to_string()),
        });

        assert!(matches!(result, WhatsAppOutboundTargetResolution::Err { .. }));
    }
}
