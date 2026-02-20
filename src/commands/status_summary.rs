//! status_summary — Status summary formatting for status command.
//! Ported from `openkrab/src/commands/status.summary.ts` (Phase 6).

use crate::config::AppConfig;
use serde::{Deserialize, Serialize};

/// Status summary for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusSummary {
    pub version: String,
    pub gateway_running: bool,
    pub gateway_mode: String,
    pub channels_configured: usize,
    pub channels_healthy: usize,
    pub auth_profiles: usize,
    pub memory_enabled: bool,
    pub sandbox_ready: bool,
    pub warnings: Vec<String>,
}

/// Build status summary from config and runtime state.
pub fn build_status_summary(cfg: &AppConfig) -> StatusSummary {
    let mut warnings = Vec::new();

    // Count channels
    let channels_configured =
        cfg.channels.slack.len() + cfg.channels.telegram.len() + cfg.channels.discord.len();

    // Estimate healthy channels (simplified)
    let channels_healthy = channels_configured; // Would check actual health in real impl

    // Count auth profiles
    let auth_profiles = cfg.auth.profiles.len();

    // Check memory
    let memory_enabled = cfg.memory.enabled.unwrap_or(false);

    // Check sandbox (simplified)
    let sandbox_ready = crate::commands::doctor_sandbox::is_docker_available();

    // Generate warnings
    if channels_configured == 0 {
        warnings.push("No channels configured. Run 'krabkrab onboard' to set up.".to_string());
    }

    if auth_profiles == 0 {
        warnings.push(
            "No auth profiles configured. Run 'krabkrab auth add' to add credentials.".to_string(),
        );
    }

    if !memory_enabled {
        warnings
            .push("Memory system disabled. Enable in config for persistent context.".to_string());
    }

    StatusSummary {
        version: env!("CARGO_PKG_VERSION").to_string(),
        gateway_running: false, // Would check actual gateway status
        gateway_mode: cfg
            .gateway
            .mode
            .clone()
            .unwrap_or_else(|| "local".to_string()),
        channels_configured,
        channels_healthy,
        auth_profiles,
        memory_enabled,
        sandbox_ready,
        warnings,
    }
}

/// Format status summary for display.
pub fn format_status_summary(summary: &StatusSummary) -> String {
    let mut lines = vec![
        format!("Version: {}", summary.version),
        format!(
            "Gateway: {} ({} mode)",
            if summary.gateway_running {
                "✓ running"
            } else {
                "✗ stopped"
            },
            summary.gateway_mode
        ),
        format!(
            "Channels: {}/{} healthy",
            summary.channels_healthy, summary.channels_configured
        ),
        format!("Auth Profiles: {}", summary.auth_profiles),
        format!(
            "Memory: {}",
            if summary.memory_enabled {
                "✓ enabled"
            } else {
                "✗ disabled"
            }
        ),
        format!(
            "Sandbox: {}",
            if summary.sandbox_ready {
                "✓ ready"
            } else {
                "✗ not available"
            }
        ),
    ];

    if !summary.warnings.is_empty() {
        lines.push("\nWarnings:".to_string());
        for warning in &summary.warnings {
            lines.push(format!("  ⚠ {}", warning));
        }
    }

    lines.join("\n")
}

/// Redact sensitive values from status summary.
pub fn redact_sensitive_status_summary(summary: &mut StatusSummary) {
    // In a real implementation, this would redact tokens, passwords, etc.
    // For now, just clear any potentially sensitive warnings
    summary.warnings.retain(|w| {
        !w.to_lowercase().contains("token")
            && !w.to_lowercase().contains("password")
            && !w.to_lowercase().contains("secret")
    });
}

/// Format tokens in compact notation (K = thousands).
pub fn format_tokens_compact(tokens: u64) -> String {
    if tokens >= 1_000_000 {
        format!("{:.1}M", tokens as f64 / 1_000_000.0)
    } else if tokens >= 1_000 {
        format!("{:.1}K", tokens as f64 / 1_000.0)
    } else {
        tokens.to_string()
    }
}

/// Format duration in human-readable form.
pub fn format_duration(ms: u64) -> String {
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{}d {}h", days, hours % 24)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_tokens_compact() {
        assert_eq!(format_tokens_compact(500), "500");
        assert_eq!(format_tokens_compact(1500), "1.5K");
        assert_eq!(format_tokens_compact(1_500_000), "1.5M");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30_000), "30s");
        assert_eq!(format_duration(90_000), "1m 30s");
        assert_eq!(format_duration(3_600_000), "1h 0m");
        assert_eq!(format_duration(86_400_000), "1d 0h");
    }

    #[test]
    fn test_redact_sensitive() {
        let mut summary = StatusSummary {
            version: "1.0.0".to_string(),
            gateway_running: true,
            gateway_mode: "local".to_string(),
            channels_configured: 1,
            channels_healthy: 1,
            auth_profiles: 1,
            memory_enabled: true,
            sandbox_ready: true,
            warnings: vec!["Token expired".to_string(), "Normal warning".to_string()],
        };

        redact_sensitive_status_summary(&mut summary);
        assert_eq!(summary.warnings.len(), 1);
        assert_eq!(summary.warnings[0], "Normal warning");
    }
}
