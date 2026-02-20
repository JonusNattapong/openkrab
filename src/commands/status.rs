//! status — System status command with comprehensive health checks.
//! Ported from `openkrab/src/commands/status.ts` (Phase 6).

use crate::commands::{
    status_daemon::{
        format_agent_local_status, format_daemon_status, get_agent_local_status, get_daemon_status,
    },
    status_summary::{build_status_summary, format_status_summary},
    status_update::{check_for_updates, format_update_one_liner},
};
use crate::config::AppConfig;

/// Status command options.
#[derive(Debug, Clone, Default)]
pub struct StatusOptions {
    pub json: bool,
    pub all: bool,
    pub check_updates: bool,
}

/// Run status command with full checks.
pub async fn status_command(cfg: &AppConfig, opts: StatusOptions) -> String {
    let mut lines = Vec::new();

    // Header
    lines.push("🦀 krabkrab Status".to_string());
    lines.push(String::new());

    // 1. Status summary
    let summary = build_status_summary(cfg);
    lines.push(format_status_summary(&summary));
    lines.push(String::new());

    // 2. Daemon status
    let daemon_status = get_daemon_status();
    lines.push(format_daemon_status(&daemon_status));

    // 3. Agent local status
    let agent_status = get_agent_local_status();
    lines.push(format_agent_local_status(&agent_status));
    lines.push(String::new());

    // 4. Update check (if requested)
    if opts.check_updates {
        let current_version = env!("CARGO_PKG_VERSION");
        match check_for_updates(current_version).await {
            Ok(update_result) => {
                lines.push(format_update_one_liner(&update_result));
            }
            Err(e) => {
                lines.push(format!("Update check failed: {}", e));
            }
        }
        lines.push(String::new());
    }

    // 5. Extended info (--all)
    if opts.all {
        lines.push("--- Extended Information ---".to_string());

        // Provider registry
        let registry = crate::providers::default_registry_from_env();
        let providers = registry.list();
        lines.push(format!("Available providers: {}", providers.join(", ")));

        // Config path
        if let Ok(config_path) = crate::config::config_path() {
            lines.push(format!("Config path: {}", config_path.display()));
        }

        lines.push(String::new());
    }

    // Footer
    if summary.warnings.is_empty() && daemon_status.running {
        lines.push("✅ System healthy".to_string());
    } else if !summary.warnings.is_empty() {
        lines.push(format!("⚠️  {} warning(s) found", summary.warnings.len()));
    }

    lines.join("\n")
}

/// Simple status command (legacy compatibility).
pub fn status_simple() -> String {
    let registry = crate::providers::default_registry_from_env();
    let providers = registry.list();
    let provider_text = if providers.is_empty() {
        "none".to_string()
    } else {
        providers.join(", ")
    };

    format!(
        "OK — {} providers registered [{}]",
        providers.len(),
        provider_text
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_simple() {
        let status = status_simple();
        assert!(status.contains("OK"));
        assert!(status.contains("providers registered"));
    }
}
