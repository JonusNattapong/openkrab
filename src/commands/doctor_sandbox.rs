//! doctor_sandbox — Sandbox health checks for doctor command.
//! Ported from `openclaw/src/commands/doctor-sandbox.ts` (Phase 6).

use crate::config::AppConfig;
use std::process::Command;

/// Default sandbox Docker images.
pub const DEFAULT_SANDBOX_IMAGE: &str = "krabkrab/sandbox:latest";
pub const DEFAULT_SANDBOX_BROWSER_IMAGE: &str = "krabkrab/sandbox-browser:latest";
pub const DEFAULT_SANDBOX_COMMON_IMAGE: &str = "krabkrab/sandbox-common:latest";

/// Sandbox health status.
#[derive(Debug, Clone)]
pub struct SandboxHealth {
    pub docker_available: bool,
    pub default_image_present: bool,
    pub browser_image_present: bool,
    pub common_image_present: bool,
    pub issues: Vec<String>,
}

/// Check if Docker is available.
pub fn is_docker_available() -> bool {
    match Command::new("docker")
        .args(["version", "--format", "{{.Server.Version}}"])
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Check if a Docker image exists locally.
pub fn docker_image_exists(image: &str) -> bool {
    match Command::new("docker")
        .args(["image", "inspect", image])
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Resolve sandbox Docker image from config or default.
pub fn resolve_sandbox_image(cfg: &AppConfig) -> String {
    cfg.agents
        .defaults
        .sandbox
        .docker
        .image
        .clone()
        .unwrap_or_else(|| DEFAULT_SANDBOX_IMAGE.to_string())
}

/// Resolve sandbox browser image from config or default.
pub fn resolve_sandbox_browser_image(cfg: &AppConfig) -> String {
    cfg.agents
        .defaults
        .sandbox
        .docker
        .browser
        .as_ref()
        .and_then(|b| b.image.clone())
        .unwrap_or_else(|| DEFAULT_SANDBOX_BROWSER_IMAGE.to_string())
}

/// Resolve sandbox common image from config or default.
pub fn resolve_sandbox_common_image(cfg: &AppConfig) -> String {
    cfg.agents
        .defaults
        .sandbox
        .docker
        .common
        .as_ref()
        .and_then(|c| c.image.clone())
        .unwrap_or_else(|| DEFAULT_SANDBOX_COMMON_IMAGE.to_string())
}

/// Check sandbox health comprehensively.
pub fn check_sandbox_health(cfg: &AppConfig) -> SandboxHealth {
    let mut issues = Vec::new();

    // Check Docker availability
    let docker_available = is_docker_available();
    if !docker_available {
        issues.push("Docker is not available. Install Docker to use sandbox features.".to_string());
        return SandboxHealth {
            docker_available: false,
            default_image_present: false,
            browser_image_present: false,
            common_image_present: false,
            issues,
        };
    }

    // Check images
    let default_image = resolve_sandbox_image(cfg);
    let browser_image = resolve_sandbox_browser_image(cfg);
    let common_image = resolve_sandbox_common_image(cfg);

    let default_image_present = docker_image_exists(&default_image);
    let browser_image_present = docker_image_exists(&browser_image);
    let common_image_present = docker_image_exists(&common_image);

    if !default_image_present {
        issues.push(format!(
            "Default sandbox image '{}' not found. Run 'krabkrab sandbox build' to build it.",
            default_image
        ));
    }

    if !browser_image_present {
        issues.push(format!(
            "Browser sandbox image '{}' not found. Run 'krabkrab sandbox build --browser' to build it.",
            browser_image
        ));
    }

    if !common_image_present {
        issues.push(format!(
            "Common sandbox image '{}' not found. Run 'krabkrab sandbox build --common' to build it.",
            common_image
        ));
    }

    SandboxHealth {
        docker_available,
        default_image_present,
        browser_image_present,
        common_image_present,
        issues,
    }
}

/// Format sandbox health for display.
pub fn format_sandbox_health(health: &SandboxHealth) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "Docker: {}",
        if health.docker_available {
            "✓ available"
        } else {
            "✗ not available"
        }
    ));

    if health.docker_available {
        lines.push(format!(
            "Default image: {}",
            if health.default_image_present {
                "✓"
            } else {
                "✗ missing"
            }
        ));
        lines.push(format!(
            "Browser image: {}",
            if health.browser_image_present {
                "✓"
            } else {
                "✗ missing"
            }
        ));
        lines.push(format!(
            "Common image: {}",
            if health.common_image_present {
                "✓"
            } else {
                "✗ missing"
            }
        ));
    }

    if !health.issues.is_empty() {
        lines.push("\nIssues:".to_string());
        for issue in &health.issues {
            lines.push(format!("  - {}", issue));
        }
    }

    lines.join("\n")
}

/// Note sandbox scope warnings based on config.
pub fn note_sandbox_scope_warnings(cfg: &AppConfig) -> Vec<String> {
    let mut warnings = Vec::new();

    let sandbox = &cfg.agents.defaults.sandbox;

    // Check for potentially dangerous bind mounts
    if let Some(binds) = &sandbox.docker.binds {
        for bind in binds {
            if bind.starts_with("/etc") || bind.starts_with("/root") {
                warnings.push(format!(
                    "Potentially dangerous bind mount: {}. Consider using a more restrictive path.",
                    bind
                ));
            }
        }
    }

    // Check network mode
    if let Some(network) = &sandbox.docker.network {
        if network == "host" {
            warnings.push("Sandbox using host network mode. This reduces isolation.".to_string());
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_sandbox_health_all_good() {
        let health = SandboxHealth {
            docker_available: true,
            default_image_present: true,
            browser_image_present: true,
            common_image_present: true,
            issues: vec![],
        };

        let output = format_sandbox_health(&health);
        assert!(output.contains("Docker: ✓ available"));
        assert!(output.contains("Default image: ✓"));
    }

    #[test]
    fn test_format_sandbox_health_docker_unavailable() {
        let health = SandboxHealth {
            docker_available: false,
            default_image_present: false,
            browser_image_present: false,
            common_image_present: false,
            issues: vec!["Docker is not available".to_string()],
        };

        let output = format_sandbox_health(&health);
        assert!(output.contains("Docker: ✗ not available"));
        assert!(output.contains("Issues:"));
    }
}
