//! status_daemon — Daemon status checking for status command.
//! Ported from `openclaw/src/commands/status.daemon.ts` (Phase 6).

use std::process::Command;

/// Daemon status information.
#[derive(Debug, Clone)]
pub struct DaemonStatus {
    pub running: bool,
    pub pid: Option<u32>,
    pub uptime_ms: Option<u64>,
    pub memory_mb: Option<f64>,
    pub version: Option<String>,
}

/// Check if daemon is running (simplified - checks for process).
pub fn get_daemon_status() -> DaemonStatus {
    // Try to find krabkrab-gateway process
    #[cfg(target_os = "windows")]
    let output = Command::new("tasklist")
        .args([
            "/FI",
            "IMAGENAME eq krabkrab-gateway.exe",
            "/FO",
            "CSV",
            "/NH",
        ])
        .output();

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("pgrep")
        .args(["-f", "krabkrab-gateway"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let running = !stdout.trim().is_empty();

            DaemonStatus {
                running,
                pid: None, // Would parse from output in real impl
                uptime_ms: None,
                memory_mb: None,
                version: None,
            }
        }
        _ => DaemonStatus {
            running: false,
            pid: None,
            uptime_ms: None,
            memory_mb: None,
            version: None,
        },
    }
}

/// Get local agent status (simplified).
pub fn get_agent_local_status() -> AgentLocalStatus {
    AgentLocalStatus {
        running: false,
        last_activity: None,
        session_count: 0,
    }
}

/// Local agent status.
#[derive(Debug, Clone)]
pub struct AgentLocalStatus {
    pub running: bool,
    pub last_activity: Option<String>,
    pub session_count: usize,
}

/// Format daemon status for display.
pub fn format_daemon_status(status: &DaemonStatus) -> String {
    if status.running {
        let mut parts = vec!["Daemon: ✓ running".to_string()];

        if let Some(pid) = status.pid {
            parts.push(format!("  PID: {}", pid));
        }

        if let Some(uptime) = status.uptime_ms {
            parts.push(format!(
                "  Uptime: {}",
                crate::commands::status_summary::format_duration(uptime)
            ));
        }

        if let Some(mem) = status.memory_mb {
            parts.push(format!("  Memory: {:.1} MB", mem));
        }

        if let Some(ver) = &status.version {
            parts.push(format!("  Version: {}", ver));
        }

        parts.join("\n")
    } else {
        "Daemon: ✗ not running".to_string()
    }
}

/// Format agent local status for display.
pub fn format_agent_local_status(status: &AgentLocalStatus) -> String {
    let mut parts = vec![format!(
        "Local Agent: {}",
        if status.running {
            "✓ running"
        } else {
            "✗ stopped"
        }
    )];

    if let Some(last) = &status.last_activity {
        parts.push(format!("  Last activity: {}", last));
    }

    parts.push(format!("  Sessions: {}", status.session_count));

    parts.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_daemon_status_running() {
        let status = DaemonStatus {
            running: true,
            pid: Some(1234),
            uptime_ms: Some(3_600_000),
            memory_mb: Some(128.5),
            version: Some("1.0.0".to_string()),
        };

        let output = format_daemon_status(&status);
        assert!(output.contains("✓ running"));
        assert!(output.contains("PID: 1234"));
    }

    #[test]
    fn test_format_daemon_status_stopped() {
        let status = DaemonStatus {
            running: false,
            pid: None,
            uptime_ms: None,
            memory_mb: None,
            version: None,
        };

        let output = format_daemon_status(&status);
        assert!(output.contains("✗ not running"));
    }
}
