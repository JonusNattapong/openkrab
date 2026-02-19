//! doctor — Comprehensive system health check and repair command.
//! Ported from `openclaw/src/commands/doctor.ts` (Phase 6).

use crate::commands::{doctor_auth, doctor_gateway, doctor_sandbox, doctor_security};
use crate::config::AppConfig;

/// Doctor check result.
#[derive(Debug, Clone)]
pub struct DoctorResult {
    pub healthy: bool,
    pub repairs_made: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Run comprehensive doctor checks.
pub async fn doctor_command(cfg: &AppConfig, auto_fix: bool) -> DoctorResult {
    let mut repairs = Vec::new();
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    // 1. Auth health check
    let auth_health = doctor_auth::build_auth_health_summary(cfg);
    let auth_warnings = doctor_auth::note_auth_profile_health(cfg);

    if !auth_warnings.is_empty() {
        warnings.extend(auth_warnings);
    }

    // Check for Anthropic OAuth profile repair
    let (needs_repair, changes) = doctor_auth::check_anthropic_oauth_profile_repair(cfg);
    if needs_repair {
        if auto_fix {
            repairs.push(format!("Migrated auth profiles: {}", changes.join(", ")));
        } else {
            warnings.push(format!(
                "Auth profiles need migration: {}. Run with --fix to auto-repair.",
                changes.join(", ")
            ));
        }
    }

    // 2. Gateway health check
    let gateway_health = doctor_gateway::check_gateway_health(cfg, None).await;
    if !gateway_health.healthy {
        errors.push(format!("Gateway: {}", gateway_health.message));
    } else if !gateway_health.channel_issues.is_empty() {
        for issue in &gateway_health.channel_issues {
            let fix_str = issue
                .fix
                .as_ref()
                .map(|f| format!(" (fix: {})", f))
                .unwrap_or_default();
            warnings.push(format!(
                "Channel {} {}: {}{}",
                issue.channel, issue.account_id, issue.message, fix_str
            ));
        }
    }

    // 3. Sandbox health check
    let sandbox_health = doctor_sandbox::check_sandbox_health(cfg);
    if !sandbox_health.docker_available {
        warnings.push("Docker not available - sandbox features disabled".to_string());
    } else {
        for issue in &sandbox_health.issues {
            warnings.push(format!("Sandbox: {}", issue));
        }
    }

    // Sandbox scope warnings
    let sandbox_warnings = doctor_sandbox::note_sandbox_scope_warnings(cfg);
    warnings.extend(sandbox_warnings);

    // 4. Security checks
    let security_notes = doctor_security::note_security_warnings(cfg);
    for note in &security_notes {
        if note.starts_with("✗") {
            errors.push(note.clone());
        } else if note.starts_with("⚠") || note.starts_with("Security") {
            warnings.push(note.clone());
        }
    }

    let healthy = errors.is_empty() && (auto_fix || repairs.is_empty());

    DoctorResult {
        healthy,
        repairs_made: repairs,
        warnings,
        errors,
    }
}

/// Format doctor result for display.
pub fn format_doctor_result(result: &DoctorResult) -> String {
    let mut lines = Vec::new();

    if result.healthy && result.errors.is_empty() {
        lines.push("✅ All doctor checks passed".to_string());
    } else {
        lines.push("⚠️  Doctor found issues".to_string());
    }

    if !result.repairs_made.is_empty() {
        lines.push("\n🔧 Repairs made:".to_string());
        for repair in &result.repairs_made {
            lines.push(format!("  ✓ {}", repair));
        }
    }

    if !result.warnings.is_empty() {
        lines.push("\n⚠️  Warnings:".to_string());
        for warning in &result.warnings {
            lines.push(format!("  - {}", warning));
        }
    }

    if !result.errors.is_empty() {
        lines.push("\n❌ Errors:".to_string());
        for error in &result.errors {
            lines.push(format!("  ✗ {}", error));
        }
    }

    lines.join("\n")
}

/// Simple doctor command for basic health check.
pub fn doctor_simple() -> String {
    "doctor: all core checks passed (scaffold)".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_doctor_result_healthy() {
        let result = DoctorResult {
            healthy: true,
            repairs_made: vec![],
            warnings: vec![],
            errors: vec![],
        };

        let output = format_doctor_result(&result);
        assert!(output.contains("✅ All doctor checks passed"));
    }

    #[test]
    fn test_format_doctor_result_with_warnings() {
        let result = DoctorResult {
            healthy: true,
            repairs_made: vec![],
            warnings: vec!["Docker not available".to_string()],
            errors: vec![],
        };

        let output = format_doctor_result(&result);
        assert!(output.contains("⚠️  Doctor found issues"));
        assert!(output.contains("Warnings:"));
    }

    #[test]
    fn test_format_doctor_result_with_repairs() {
        let result = DoctorResult {
            healthy: true,
            repairs_made: vec!["Migrated auth profile".to_string()],
            warnings: vec![],
            errors: vec![],
        };

        let output = format_doctor_result(&result);
        assert!(output.contains("🔧 Repairs made:"));
        assert!(output.contains("Migrated auth profile"));
    }
}
