//! sandbox — Sandbox management commands.
//! Ported from `openkrab/src/commands/sandbox.ts` (Phase 6).

use crate::commands::doctor_sandbox::{
    docker_image_exists, is_docker_available, DEFAULT_SANDBOX_BROWSER_IMAGE,
    DEFAULT_SANDBOX_COMMON_IMAGE, DEFAULT_SANDBOX_IMAGE,
};
use crate::config::AppConfig;
use std::process::Command;

/// Sandbox container info.
#[derive(Debug, Clone)]
pub struct SandboxContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub running: bool,
}

/// Sandbox list result.
#[derive(Debug, Clone)]
pub struct SandboxListResult {
    pub containers: Vec<SandboxContainerInfo>,
    pub images_present: Vec<String>,
    pub images_missing: Vec<String>,
}

/// List all sandbox containers.
pub fn sandbox_list_command(cfg: &AppConfig) -> String {
    if !is_docker_available() {
        return "Docker is not available. Install Docker to use sandbox features.".to_string();
    }

    let mut lines = vec!["Sandbox Containers:".to_string()];

    // Get running containers
    match Command::new("docker")
        .args([
            "ps",
            "-a",
            "--filter",
            "label=krabkrab.sandbox=true",
            "--format",
            "{{.ID}}|{{.Names}}|{{.Image}}|{{.Status}}",
        ])
        .output()
    {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                lines.push("  No sandbox containers found".to_string());
            } else {
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split('|').collect();
                    if parts.len() >= 4 {
                        let running = parts[3].starts_with("Up");
                        let status_icon = if running { "✓" } else { "✗" };
                        lines.push(format!(
                            "  {} {} ({}) - {}",
                            status_icon, parts[1], parts[2], parts[3]
                        ));
                    }
                }
            }
        }
        _ => {
            lines.push("  Unable to list containers".to_string());
        }
    }

    // Check images
    lines.push(String::new());
    lines.push("Sandbox Images:".to_string());

    let images = vec![
        ("default", resolve_sandbox_image(cfg)),
        ("browser", resolve_sandbox_browser_image(cfg)),
        ("common", resolve_sandbox_common_image(cfg)),
    ];

    for (name, image) in images {
        let present = docker_image_exists(&image);
        let icon = if present { "✓" } else { "✗" };
        lines.push(format!("  {} {}: {}", icon, name, image));
    }

    lines.join("\n")
}

/// Build sandbox images.
pub fn sandbox_build_command(browser: bool, common: bool) -> String {
    if !is_docker_available() {
        return "Docker is not available. Install Docker to use sandbox features.".to_string();
    }

    let mut lines = vec!["Building sandbox images...".to_string()];

    // Build default image
    lines.push("Building default sandbox image...".to_string());
    lines.push(
        "  (This would run: docker build -t krabkrab/sandbox:latest -f Dockerfile.sandbox .)"
            .to_string(),
    );

    if browser {
        lines.push("Building browser sandbox image...".to_string());
        lines.push("  (This would run: docker build -t krabkrab/sandbox-browser:latest -f Dockerfile.sandbox.browser .)".to_string());
    }

    if common {
        lines.push("Building common sandbox image...".to_string());
        lines.push("  (This would run: docker build -t krabkrab/sandbox-common:latest -f Dockerfile.sandbox.common .)".to_string());
    }

    lines.push(String::new());
    lines.push(
        "Note: In a full implementation, this would execute the actual docker build commands."
            .to_string(),
    );

    lines.join("\n")
}

/// Recreate sandbox containers.
pub fn sandbox_recreate_command(_cfg: &AppConfig, force: bool) -> String {
    if !is_docker_available() {
        return "Docker is not available. Install Docker to use sandbox features.".to_string();
    }

    let mut lines = vec!["Recreating sandbox containers...".to_string()];

    if force {
        lines.push("Force mode: Will stop and remove existing containers".to_string());
    }

    lines.push(String::new());
    lines.push("This would:".to_string());
    lines.push("  1. Stop running sandbox containers".to_string());
    lines.push("  2. Remove old containers".to_string());
    lines.push("  3. Pull latest images (if configured)".to_string());
    lines.push("  4. Start new containers with fresh state".to_string());

    lines.join("\n")
}

/// Explain sandbox configuration.
pub fn sandbox_explain_command(cfg: &AppConfig) -> String {
    let mut lines = vec![
        "Sandbox Configuration Explanation".to_string(),
        "═════════════════════════════════".to_string(),
        String::new(),
    ];

    lines.push("The sandbox provides isolated execution environments for tools.".to_string());
    lines.push(String::new());

    lines.push("Docker Images:".to_string());
    lines.push(format!("  Default: {}", resolve_sandbox_image(cfg)));
    lines.push(format!("  Browser: {}", resolve_sandbox_browser_image(cfg)));
    lines.push(format!("  Common:  {}", resolve_sandbox_common_image(cfg)));
    lines.push(String::new());

    lines.push("Security Features:".to_string());
    lines.push("  • Container isolation".to_string());
    lines.push("  • Network restrictions".to_string());
    lines.push("  • Filesystem sandboxing".to_string());
    lines.push("  • Resource limits".to_string());
    lines.push(String::new());

    lines.push("Configuration (agents.defaults.sandbox):".to_string());
    lines.push(format!(
        "  Docker image: {}",
        cfg.agents
            .defaults
            .sandbox
            .docker
            .image
            .as_deref()
            .unwrap_or("default")
    ));
    lines.push(format!(
        "  Network mode: {}",
        cfg.agents
            .defaults
            .sandbox
            .docker
            .network
            .as_deref()
            .unwrap_or("bridge")
    ));
    lines.push(format!(
        "  Binds: {:?}",
        cfg.agents.defaults.sandbox.docker.binds
    ));

    lines.join("\n")
}

/// Format sandbox status.
pub fn format_sandbox_status(running: bool) -> String {
    if running {
        "● running".to_string()
    } else {
        "○ stopped".to_string()
    }
}

/// Format simple sandbox status.
pub fn format_simple_sandbox_status(running: bool) -> String {
    if running {
        "up".to_string()
    } else {
        "down".to_string()
    }
}

/// Count running items.
pub fn count_running<T: AsRef<[U]>, U: SandboxRunning>(items: T) -> usize {
    items.as_ref().iter().filter(|i| i.is_running()).count()
}

/// Trait for sandbox running status.
pub trait SandboxRunning {
    fn is_running(&self) -> bool;
}

impl SandboxRunning for SandboxContainerInfo {
    fn is_running(&self) -> bool {
        self.running
    }
}

fn resolve_sandbox_image(cfg: &AppConfig) -> String {
    cfg.agents
        .defaults
        .sandbox
        .docker
        .image
        .clone()
        .unwrap_or_else(|| DEFAULT_SANDBOX_IMAGE.to_string())
}

fn resolve_sandbox_browser_image(cfg: &AppConfig) -> String {
    cfg.agents
        .defaults
        .sandbox
        .docker
        .browser
        .as_ref()
        .and_then(|b| b.image.clone())
        .unwrap_or_else(|| DEFAULT_SANDBOX_BROWSER_IMAGE.to_string())
}

fn resolve_sandbox_common_image(cfg: &AppConfig) -> String {
    cfg.agents
        .defaults
        .sandbox
        .docker
        .common
        .as_ref()
        .and_then(|c| c.image.clone())
        .unwrap_or_else(|| DEFAULT_SANDBOX_COMMON_IMAGE.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_sandbox_status() {
        assert_eq!(format_sandbox_status(true), "● running");
        assert_eq!(format_sandbox_status(false), "○ stopped");
    }

    #[test]
    fn test_format_simple_sandbox_status() {
        assert_eq!(format_simple_sandbox_status(true), "up");
        assert_eq!(format_simple_sandbox_status(false), "down");
    }
}
