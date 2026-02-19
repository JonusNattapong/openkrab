//! status_update — Update checking for status command.
//! Ported from `openclaw/src/commands/status.update.ts` (Phase 6).

use serde::{Deserialize, Serialize};

/// Update check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    pub current_version: String,
    pub latest_version: Option<String>,
    pub update_available: bool,
    pub release_notes_url: Option<String>,
}

/// Check for updates from GitHub releases.
pub async fn check_for_updates(current_version: &str) -> anyhow::Result<UpdateCheckResult> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let url = "https://api.github.com/repos/openkrab/openkrab/releases/latest";

    let resp = client
        .get(url)
        .header("User-Agent", "krabkrab-cli")
        .send()
        .await?;

    if !resp.status().is_success() {
        return Ok(UpdateCheckResult {
            current_version: current_version.to_string(),
            latest_version: None,
            update_available: false,
            release_notes_url: None,
        });
    }

    let json: serde_json::Value = resp.json().await?;

    let latest_version = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .map(|s| s.trim_start_matches('v').to_string());

    let release_notes_url = json
        .get("html_url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let update_available = latest_version
        .as_ref()
        .map(|v| version_is_newer(v, current_version))
        .unwrap_or(false);

    Ok(UpdateCheckResult {
        current_version: current_version.to_string(),
        latest_version,
        update_available,
        release_notes_url,
    })
}

/// Compare two version strings (semver-like).
fn version_is_newer(new: &str, current: &str) -> bool {
    let parse =
        |s: &str| -> Vec<u32> { s.split('.').filter_map(|p| p.parse::<u32>().ok()).collect() };

    let new_parts = parse(new);
    let current_parts = parse(current);

    for (n, c) in new_parts.iter().zip(current_parts.iter()) {
        if n > c {
            return true;
        }
        if n < c {
            return false;
        }
    }

    new_parts.len() > current_parts.len()
}

/// Format update availability as a hint.
pub fn format_update_available_hint(result: &UpdateCheckResult) -> Option<String> {
    if !result.update_available {
        return None;
    }

    let latest = result.latest_version.as_ref()?;
    Some(format!(
        "Update available: {} → {}. Run 'krabkrab update' to upgrade.",
        result.current_version, latest
    ))
}

/// Format update as one-liner.
pub fn format_update_one_liner(result: &UpdateCheckResult) -> String {
    if result.update_available {
        format!(
            "⬆️  {} → {}",
            result.current_version,
            result.latest_version.as_deref().unwrap_or("unknown")
        )
    } else {
        format!("✓ {} (latest)", result.current_version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_newer() {
        assert!(version_is_newer("1.2.0", "1.1.0"));
        assert!(version_is_newer("2.0.0", "1.9.9"));
        assert!(!version_is_newer("1.1.0", "1.2.0"));
        assert!(!version_is_newer("1.0.0", "1.0.0"));
    }

    #[test]
    fn test_format_update_available() {
        let result = UpdateCheckResult {
            current_version: "1.0.0".to_string(),
            latest_version: Some("1.1.0".to_string()),
            update_available: true,
            release_notes_url: None,
        };

        let hint = format_update_available_hint(&result);
        assert!(hint.is_some());
        assert!(hint.unwrap().contains("1.0.0 → 1.1.0"));
    }

    #[test]
    fn test_format_update_one_liner_available() {
        let result = UpdateCheckResult {
            current_version: "1.0.0".to_string(),
            latest_version: Some("1.1.0".to_string()),
            update_available: true,
            release_notes_url: None,
        };

        let line = format_update_one_liner(&result);
        assert!(line.contains("⬆️"));
        assert!(line.contains("1.0.0 → 1.1.0"));
    }

    #[test]
    fn test_format_update_one_liner_latest() {
        let result = UpdateCheckResult {
            current_version: "1.0.0".to_string(),
            latest_version: Some("1.0.0".to_string()),
            update_available: false,
            release_notes_url: None,
        };

        let line = format_update_one_liner(&result);
        assert!(line.contains("✓"));
        assert!(line.contains("latest"));
    }
}
