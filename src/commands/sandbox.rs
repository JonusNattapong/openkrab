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
    let status = Command::new("docker")
        .args([
            "build",
            "-t",
            "krabkrab/sandbox:latest",
            "-f",
            "Dockerfile.sandbox",
            ".",
        ])
        .status();
    lines.push(format!("  Result: {:?}", status));

    if browser {
        lines.push("Building browser sandbox image...".to_string());
        let status = Command::new("docker")
            .args([
                "build",
                "-t",
                "krabkrab/sandbox-browser:latest",
                "-f",
                "Dockerfile.sandbox.browser",
                ".",
            ])
            .status();
        lines.push(format!("  Result: {:?}", status));
    }

    if common {
        lines.push("Building common sandbox image...".to_string());
        let status = Command::new("docker")
            .args([
                "build",
                "-t",
                "krabkrab/sandbox-common:latest",
                "-f",
                "Dockerfile.sandbox.common",
                ".",
            ])
            .status();
        lines.push(format!("  Result: {:?}", status));
    }

    lines.push("Build complete.".to_string());
    lines.join("\n")
}

/// Recreate sandbox containers.
pub fn sandbox_recreate_command(cfg: &AppConfig, force: bool) -> String {
    if !is_docker_available() {
        return "Docker is not available. Install Docker to use sandbox features.".to_string();
    }

    let mut lines = vec!["Recreating sandbox containers...".to_string()];

    if force {
        let output = Command::new("docker")
            .args(["ps", "-qa", "--filter", "label=krabkrab.sandbox=true"])
            .output()
            .expect("Failed to execute docker ps");

        let stdout = String::from_utf8_lossy(&output.stdout);
        for id in stdout.lines() {
            let id = id.trim();
            if !id.is_empty() {
                lines.push(format!("Removing container {}", id));
                let rm_status = Command::new("docker").args(["rm", "-f", id]).status();
                lines.push(format!("  Result: {:?}", rm_status));
            }
        }
    }

    let network = cfg
        .agents
        .defaults
        .sandbox
        .docker
        .network
        .clone()
        .or_else(|| cfg.agents.defaults.sandbox.network.clone());
    let binds = cfg
        .agents
        .defaults
        .sandbox
        .docker
        .binds
        .clone()
        .or_else(|| cfg.agents.defaults.sandbox.binds.clone())
        .unwrap_or_default();

    let targets = vec![
        ("krabkrab-sandbox-default", resolve_sandbox_image(cfg)),
        (
            "krabkrab-sandbox-browser",
            resolve_sandbox_browser_image(cfg),
        ),
        ("krabkrab-sandbox-common", resolve_sandbox_common_image(cfg)),
    ];

    for (name, image) in targets {
        if !docker_image_exists(&image) {
            lines.push(format!("Skipping {} (image missing: {})", name, image));
            continue;
        }

        match ensure_sandbox_container(name, &image, network.as_deref(), &binds) {
            Ok(action) => lines.push(format!("{}: {}", name, action)),
            Err(e) => lines.push(format!("{}: failed ({})", name, e)),
        }
    }

    lines.join("\n")
}

fn ensure_sandbox_container(
    name: &str,
    image: &str,
    network: Option<&str>,
    binds: &[String],
) -> Result<String, String> {
    let inspect = Command::new("docker")
        .args(["inspect", "-f", "{{.State.Running}}", name])
        .output()
        .map_err(|e| format!("docker inspect failed: {e}"))?;

    if inspect.status.success() {
        let running = String::from_utf8_lossy(&inspect.stdout).trim().to_string();
        if running.eq_ignore_ascii_case("true") {
            return Ok(format!("already running ({})", image));
        }

        let start = Command::new("docker")
            .args(["start", name])
            .status()
            .map_err(|e| format!("docker start failed: {e}"))?;
        if start.success() {
            return Ok(format!("started ({})", image));
        }
        return Err("docker start returned non-zero status".to_string());
    }

    let mut args: Vec<String> = vec![
        "run".to_string(),
        "-d".to_string(),
        "--name".to_string(),
        name.to_string(),
        "--label".to_string(),
        "krabkrab.sandbox=true".to_string(),
    ];

    if let Some(net) = network.filter(|s| !s.trim().is_empty()) {
        args.push("--network".to_string());
        args.push(net.to_string());
    }

    for bind in binds {
        if !bind.trim().is_empty() {
            args.push("-v".to_string());
            args.push(bind.clone());
        }
    }

    args.push(image.to_string());

    let run = Command::new("docker")
        .args(args)
        .status()
        .map_err(|e| format!("docker run failed: {e}"))?;
    if run.success() {
        Ok(format!("created ({})", image))
    } else {
        Err("docker run returned non-zero status".to_string())
    }
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
