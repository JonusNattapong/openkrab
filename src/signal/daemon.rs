//! signal::daemon — Signal-cli daemon auto-start/management.
//! Ported from `openkrab/src/signal/daemon.ts` (Phase 13).

use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

/// Daemon configuration options.
#[derive(Debug, Clone)]
pub struct DaemonOpts {
    pub cli_path: String,
    pub account: Option<String>,
    pub http_host: String,
    pub http_port: u16,
    pub receive_mode: Option<String>,
    pub ignore_attachments: bool,
    pub ignore_stories: bool,
    pub send_read_receipts: bool,
}

/// Daemon handle for controlling the spawned process.
pub struct DaemonHandle {
    child: Arc<Mutex<Option<Child>>>,
    shutdown_tx: mpsc::Sender<()>,
}

impl DaemonHandle {
    /// Stop the daemon process.
    pub async fn stop(&self) {
        let _ = self.shutdown_tx.send(()).await;

        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
        *child_guard = None;
    }
}

impl Drop for DaemonHandle {
    fn drop(&mut self) {
        // Try to stop the daemon when dropped
        // Use try_lock since Drop is synchronous
        if let Ok(mut child_guard) = self.child.try_lock() {
            if let Some(ref mut child) = *child_guard {
                // Note: This is synchronous and may not work properly in async context
                // In production, prefer calling stop() explicitly
                let _ = child.start_kill();
            }
        }
    }
}

/// Build command arguments for signal-cli daemon.
fn build_daemon_args(opts: &DaemonOpts) -> Vec<String> {
    let mut args = Vec::new();

    if let Some(ref account) = opts.account {
        args.push("-a".to_string());
        args.push(account.clone());
    }

    args.push("daemon".to_string());
    args.push("--http".to_string());
    args.push(format!("{}:{}", opts.http_host, opts.http_port));
    args.push("--no-receive-stdout".to_string());

    if let Some(ref receive_mode) = opts.receive_mode {
        args.push("--receive-mode".to_string());
        args.push(receive_mode.clone());
    }

    if opts.ignore_attachments {
        args.push("--ignore-attachments".to_string());
    }

    if opts.ignore_stories {
        args.push("--ignore-stories".to_string());
    }

    if opts.send_read_receipts {
        args.push("--send-read-receipts".to_string());
    }

    args
}

/// Spawn signal-cli daemon process.
pub async fn spawn_daemon(
    opts: DaemonOpts,
) -> Result<DaemonHandle, Box<dyn std::error::Error + Send + Sync>> {
    let args = build_daemon_args(&opts);

    let child = Command::new(&opts.cli_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let child_arc = Arc::new(Mutex::new(Some(child)));
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);

    // Spawn a task to handle child process output and shutdown
    let child_clone = child_arc.clone();
    tokio::spawn(async move {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                // Shutdown requested
                let mut child_guard = child_clone.lock().await;
                if let Some(ref mut child) = *child_guard {
                    let _ = child.kill().await;
                }
            }
            status = async {
                let mut child_guard = child_clone.lock().await;
                if let Some(ref mut child) = *child_guard {
                    return child.wait().await;
                }
                Ok::<_, std::io::Error>(std::process::ExitStatus::default())
            } => {
                match status {
                    Ok(exit_status) => {
                        if !exit_status.success() {
                            eprintln!("signal-cli daemon exited with status: {}", exit_status);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error waiting for signal-cli daemon: {}", e);
                    }
                }
            }
        }
    });

    Ok(DaemonHandle {
        child: child_arc,
        shutdown_tx,
    })
}

/// Check if daemon should auto-start based on config.
pub fn should_auto_start(config: &super::SignalConfig) -> bool {
    config.auto_start.unwrap_or_else(|| {
        config.api_base.starts_with("http://localhost")
            || config.api_base.starts_with("http://127.0.0.1")
    })
}

/// Create daemon options from signal config.
pub fn daemon_opts_from_config(config: &super::SignalConfig) -> DaemonOpts {
    let url_parts: Vec<&str> = config
        .api_base
        .trim_start_matches("http://")
        .split(':')
        .collect();
    let http_host = url_parts.get(0).unwrap_or(&"localhost").to_string();
    let http_port = url_parts
        .get(1)
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    DaemonOpts {
        cli_path: config
            .cli_path
            .clone()
            .unwrap_or_else(|| "signal-cli".to_string()),
        account: config.account.clone(),
        http_host,
        http_port,
        receive_mode: Some("manual".to_string()),
        ignore_attachments: false,
        ignore_stories: true,
        send_read_receipts: config.send_read_receipts.unwrap_or(false),
    }
}

/// Wait for daemon to be ready by checking health endpoint.
pub async fn wait_for_daemon_ready(
    client: &reqwest::Client,
    api_base: &str,
    timeout_ms: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let health_url = format!("{}/api/v1/check", api_base);
    let start_time = std::time::Instant::now();
    let timeout_duration = std::time::Duration::from_millis(timeout_ms);

    while start_time.elapsed() < timeout_duration {
        match client.get(&health_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                return Ok(());
            }
            _ => {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }

    Err("Daemon startup timeout".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_daemon_args_basic() {
        let opts = DaemonOpts {
            cli_path: "signal-cli".to_string(),
            account: Some("+1234567890".to_string()),
            http_host: "localhost".to_string(),
            http_port: 8080,
            receive_mode: Some("manual".to_string()),
            ignore_attachments: false,
            ignore_stories: true,
            send_read_receipts: false,
        };

        let args = build_daemon_args(&opts);
        assert!(args.contains(&"daemon".to_string()));
        assert!(args.contains(&"--http".to_string()));
        assert!(args.contains(&"localhost:8080".to_string()));
        assert!(args.contains(&"--ignore-stories".to_string()));
    }

    #[test]
    fn should_auto_start_localhost() {
        let config = super::super::SignalConfig {
            phone_number: "+1234567890".to_string(),
            api_base: "http://localhost:8080".to_string(),
            account: None,
            cli_path: None,
            auto_start: None,
            startup_timeout_ms: None,
            send_read_receipts: None,
        };
        assert!(should_auto_start(&config));
    }

    #[test]
    fn should_not_auto_start_remote() {
        let config = super::super::SignalConfig {
            phone_number: "+1234567890".to_string(),
            api_base: "https://signal.example.com".to_string(),
            account: None,
            cli_path: None,
            auto_start: None,
            startup_timeout_ms: None,
            send_read_receipts: None,
        };
        assert!(!should_auto_start(&config));
    }
}
