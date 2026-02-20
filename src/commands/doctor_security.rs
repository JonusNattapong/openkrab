//! doctor_security — Security checks for doctor command.
//! Ported from `openclaw/src/commands/doctor-security.ts` (Phase 6).

use crate::config::AppConfig;
use crate::security::{validate_sandbox_security, SandboxConfig};

/// Security check result.
#[derive(Debug, Clone)]
pub struct SecurityCheckResult {
    pub passed: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Run all security checks.
pub fn check_security(cfg: &AppConfig) -> SecurityCheckResult {
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    // Check sandbox security
    let sandbox_cfg = SandboxConfig {
        binds: cfg.agents.defaults.sandbox.docker.binds.clone(),
        network: cfg.agents.defaults.sandbox.docker.network.clone(),
        seccomp_profile: cfg.agents.defaults.sandbox.docker.seccomp_profile.clone(),
        apparmor_profile: cfg.agents.defaults.sandbox.docker.apparmor_profile.clone(),
    };

    if let Err(e) = validate_sandbox_security(&sandbox_cfg) {
        errors.push(format!("Sandbox security: {}", e));
    }

    // Check for insecure gateway settings
    if cfg.gateway.auth_token.is_some() && cfg.gateway.auth_token.as_ref().unwrap().len() < 32 {
        warnings.push(
            "Gateway auth token is short. Consider using a longer, more secure token.".to_string(),
        );
    }

    // Check for exposed sensitive paths in config
    if let Some(log_dir) = &cfg.logging.directory {
        if log_dir.contains("/tmp") || log_dir.contains("/var/tmp") {
            warnings.push(format!(
                "Log directory '{}' is in a shared temporary location. Consider using a private directory.",
                log_dir
            ));
        }
    }

    SecurityCheckResult {
        passed: errors.is_empty(),
        warnings,
        errors,
    }
}

/// Note security warnings for display.
pub fn note_security_warnings(cfg: &AppConfig) -> Vec<String> {
    let result = check_security(cfg);
    let mut notes = Vec::new();

    if !result.errors.is_empty() {
        notes.push("Security errors found:".to_string());
        for error in &result.errors {
            notes.push(format!("  ✗ {}", error));
        }
    }

    if !result.warnings.is_empty() {
        notes.push("Security warnings:".to_string());
        for warning in &result.warnings {
            notes.push(format!("  ⚠ {}", warning));
        }
    }

    if result.passed && result.warnings.is_empty() {
        notes.push("✓ No security issues detected".to_string());
    }

    notes
}

/// Format security check result for display.
pub fn format_security_check(result: &SecurityCheckResult) -> String {
    let mut lines = Vec::new();

    if result.passed && result.warnings.is_empty() && result.errors.is_empty() {
        lines.push("✓ All security checks passed".to_string());
    } else {
        if !result.errors.is_empty() {
            lines.push("Errors:".to_string());
            for error in &result.errors {
                lines.push(format!("  ✗ {}", error));
            }
        }

        if !result.warnings.is_empty() {
            if !result.errors.is_empty() {
                lines.push(String::new());
            }
            lines.push("Warnings:".to_string());
            for warning in &result.warnings {
                lines.push(format!("  ⚠ {}", warning));
            }
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_security_check_all_passed() {
        let result = SecurityCheckResult {
            passed: true,
            warnings: vec![],
            errors: vec![],
        };

        let output = format_security_check(&result);
        assert!(output.contains("✓ All security checks passed"));
    }

    #[test]
    fn test_format_security_check_with_warnings() {
        let result = SecurityCheckResult {
            passed: true,
            warnings: vec!["Short auth token".to_string()],
            errors: vec![],
        };

        let output = format_security_check(&result);
        assert!(output.contains("Warnings:"));
        assert!(output.contains("Short auth token"));
    }

    #[test]
    fn test_format_security_check_with_errors() {
        let result = SecurityCheckResult {
            passed: false,
            warnings: vec![],
            errors: vec!["Insecure bind mount".to_string()],
        };

        let output = format_security_check(&result);
        assert!(output.contains("Errors:"));
        assert!(output.contains("Insecure bind mount"));
    }
}
