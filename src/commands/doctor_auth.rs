//! doctor-auth — Auth profile health checks and repairs for doctor command.
//! Ported from `openclaw/src/commands/doctor-auth.ts` (Phase 6).

use crate::config::AppConfig;

/// Default OAuth warning threshold (7 days in milliseconds).
pub const DEFAULT_OAUTH_WARN_MS: u64 = 7 * 24 * 60 * 60 * 1000;

/// Format remaining time in a human-readable short format.
pub fn format_remaining_short(ms: u64) -> String {
    let days = ms / (24 * 60 * 60 * 1000);
    let hours = (ms % (24 * 60 * 60 * 1000)) / (60 * 60 * 1000);

    if days > 0 {
        format!("{}d", days)
    } else if hours > 0 {
        format!("{}h", hours)
    } else {
        "<1h".to_string()
    }
}

/// Check if Anthropic OAuth profile ID needs repair.
/// Returns (needs_repair, changes_description).
pub fn check_anthropic_oauth_profile_repair(cfg: &AppConfig) -> (bool, Vec<String>) {
    let mut changes = Vec::new();

    // Check for legacy profile ID "anthropic:default"
    if cfg.auth.profiles.contains_key("anthropic:default") {
        changes.push("Migrate anthropic:default → anthropic".to_string());
    }

    (changes.len() > 0, changes)
}

/// Prune auth order entries for removed profiles.
pub fn prune_auth_order(
    order: &Option<std::collections::HashMap<String, Vec<String>>>,
    profile_ids: &std::collections::HashSet<String>,
) -> (Option<std::collections::HashMap<String, Vec<String>>>, bool) {
    let order = match order {
        Some(o) => o,
        None => return (None, false),
    };

    let mut changed = false;
    let mut next: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for (provider, list) in order {
        let filtered: Vec<String> = list
            .iter()
            .filter(|id| !profile_ids.contains(*id))
            .cloned()
            .collect();

        if filtered.len() != list.len() {
            changed = true;
        }

        if !filtered.is_empty() {
            next.insert(provider.clone(), filtered);
        }
    }

    if next.is_empty() {
        (None, changed)
    } else {
        (Some(next), changed)
    }
}

/// Build auth health summary for display.
pub fn build_auth_health_summary(cfg: &AppConfig) -> String {
    let mut lines = Vec::new();

    lines.push(format!("Profiles: {}", cfg.auth.profiles.len()));

    for (id, profile) in &cfg.auth.profiles {
        let provider = profile
            .get("provider")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let status = if provider == "anthropic" && id == "anthropic:default" {
            "⚠️ legacy ID".to_string()
        } else {
            "✓".to_string()
        };
        lines.push(format!("  {}: {}", id, status));
    }

    lines.join("\n")
}

/// Note auth profile health issues.
pub fn note_auth_profile_health(cfg: &AppConfig) -> Vec<String> {
    let mut warnings = Vec::new();

    // Check for empty profiles
    if cfg.auth.profiles.is_empty() {
        warnings.push("Auth profiles section exists but is empty".to_string());
    }

    // Check for legacy profile IDs
    for id in cfg.auth.profiles.keys() {
        if id == "anthropic:default" {
            warnings.push(format!(
                "Legacy profile ID '{}' found. Run 'krabkrab doctor' to migrate.",
                id
            ));
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_format_remaining_short_days() {
        let ms = 3 * 24 * 60 * 60 * 1000; // 3 days
        assert_eq!(format_remaining_short(ms), "3d");
    }

    #[test]
    fn test_format_remaining_short_hours() {
        let ms = 5 * 60 * 60 * 1000; // 5 hours
        assert_eq!(format_remaining_short(ms), "5h");
    }

    #[test]
    fn test_format_remaining_short_less_than_hour() {
        let ms = 30 * 60 * 1000; // 30 minutes
        assert_eq!(format_remaining_short(ms), "<1h");
    }

    #[test]
    fn test_prune_auth_order_no_change() {
        let mut order = HashMap::new();
        order.insert("openai".to_string(), vec!["openai:default".to_string()]);

        let removed: HashSet<String> = ["other".to_string()].into_iter().collect();
        let (result, changed) = prune_auth_order(&Some(order), &removed);

        assert!(!changed);
        assert!(result.is_some());
        assert_eq!(result.unwrap().get("openai").unwrap().len(), 1);
    }

    #[test]
    fn test_prune_auth_order_with_change() {
        let mut order = HashMap::new();
        order.insert(
            "openai".to_string(),
            vec!["openai:default".to_string(), "removed".to_string()],
        );

        let removed: HashSet<String> = ["removed".to_string()].into_iter().collect();
        let (result, changed) = prune_auth_order(&Some(order), &removed);

        assert!(changed);
        assert!(result.is_some());
        assert_eq!(result.unwrap().get("openai").unwrap().len(), 1);
    }
}
