//! tools::bash_pty — Advanced Bash PTY tool for interactive shell command execution.
//!
//! Provides a PTY (pseudo-terminal) based shell execution environment that supports:
//! - Interactive command execution with real terminal emulation
//! - Persistent shell sessions with state preservation
//! - Terminal resizing and ANSI support
//! - Timeout and output limits
//! - Cross-platform support (Unix PTY / Windows ConPTY)
//! - Global process registry for session management
//! - Signal handling for graceful termination
//!
//! This is designed for agent tools that need to run shell commands with
//! full terminal capabilities, such as interactive CLI tools, TUI applications,
//! or commands that require a real TTY.

use anyhow::{bail, Context, Result};
use portable_pty::{Child, CommandBuilder, NativePtySystem, PtyPair, PtySize, PtySystem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ─── Global Process Registry ──────────────────────────────────────────────────

lazy_static::lazy_static! {
    static ref SESSION_REGISTRY: Arc<Mutex<HashMap<String, BashPtySessionHandle>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

/// A handle to a registered session for external management.
pub struct BashPtySessionHandle {
    pub id: String,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub command_count: u64,
}

/// Session metadata for listing and management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub created_at_ms: u64,
    pub last_activity_ms: u64,
    pub duration_ms: u64,
    pub command_count: u64,
}

/// Register a new session in the global registry.
pub fn register_session(id: String) -> Result<()> {
    let now = Instant::now();
    let handle = BashPtySessionHandle {
        id: id.clone(),
        created_at: now,
        last_activity: now,
        command_count: 0,
    };

    let mut registry = SESSION_REGISTRY
        .lock()
        .map_err(|e| anyhow::anyhow!("failed to lock session registry: {}", e))?;

    // Clean up old sessions if too many
    if registry.len() >= 100 {
        let oldest = registry
            .iter()
            .min_by_key(|(_, h)| h.last_activity)
            .map(|(k, _)| k.clone());
        if let Some(key) = oldest {
            registry.remove(&key);
        }
    }

    registry.insert(id, handle);
    Ok(())
}

/// Update session activity timestamp.
pub fn touch_session(id: &str) -> Result<()> {
    let mut registry = SESSION_REGISTRY
        .lock()
        .map_err(|e| anyhow::anyhow!("failed to lock session registry: {}", e))?;

    if let Some(handle) = registry.get_mut(id) {
        handle.last_activity = Instant::now();
        handle.command_count += 1;
    }

    Ok(())
}

/// Remove a session from the registry.
pub fn unregister_session(id: &str) -> Result<()> {
    let mut registry = SESSION_REGISTRY
        .lock()
        .map_err(|e| anyhow::anyhow!("failed to lock session registry: {}", e))?;

    registry.remove(id);
    Ok(())
}

/// List all active sessions.
pub fn list_sessions() -> Result<Vec<SessionInfo>> {
    let registry = SESSION_REGISTRY
        .lock()
        .map_err(|e| anyhow::anyhow!("failed to lock session registry: {}", e))?;

    let now = Instant::now();
    let infos: Vec<SessionInfo> = registry
        .values()
        .map(|h| SessionInfo {
            id: h.id.clone(),
            created_at_ms: now.duration_since(h.created_at).as_millis() as u64,
            last_activity_ms: now.duration_since(h.last_activity).as_millis() as u64,
            duration_ms: h.created_at.elapsed().as_millis() as u64,
            command_count: h.command_count,
        })
        .collect();

    Ok(infos)
}

/// Kill all sessions older than the given duration.
pub fn cleanup_stale_sessions(max_age: Duration) -> Result<usize> {
    let mut registry = SESSION_REGISTRY
        .lock()
        .map_err(|e| anyhow::anyhow!("failed to lock session registry: {}", e))?;

    let now = Instant::now();
    let to_remove: Vec<String> = registry
        .iter()
        .filter(|(_, h)| now.duration_since(h.last_activity) > max_age)
        .map(|(k, _)| k.clone())
        .collect();

    let count = to_remove.len();
    for id in to_remove {
        registry.remove(&id);
    }

    Ok(count)
}

// ─── Signal Handling ──────────────────────────────────────────────────────────

/// Signal types supported for process control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessSignal {
    /// Interrupt (Ctrl+C)
    Sigint,
    /// Terminate
    Sigterm,
    /// Kill (force)
    Sigkill,
}

impl ProcessSignal {
    /// Convert to platform-specific signal value.
    #[cfg(unix)]
    pub fn as_nix_signal(&self) -> nix::sys::signal::Signal {
        use nix::sys::signal::Signal;
        match self {
            ProcessSignal::Sigint => Signal::SIGINT,
            ProcessSignal::Sigterm => Signal::SIGTERM,
            ProcessSignal::Sigkill => Signal::SIGKILL,
        }
    }
}

/// Send a signal to a child process (Unix only for now).
#[cfg(unix)]
pub fn send_signal(child: &mut Box<dyn Child>, signal: ProcessSignal) -> Result<()> {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    if let Some(pid) = child.process_id() {
        let nix_signal = signal.as_nix_signal();
        kill(Pid::from_raw(pid as i32), Some(nix_signal))
            .with_context(|| format!("failed to send {:?} to process {}", signal, pid))?;
        Ok(())
    } else {
        bail!("process ID not available")
    }
}

#[cfg(not(unix))]
pub fn send_signal(_child: &mut Box<dyn Child>, _signal: ProcessSignal) -> Result<()> {
    // Windows doesn't support POSIX signals; use taskkill or similar if needed
    bail!("signal sending not supported on this platform")
}

/// Gracefully terminate a child process.
pub fn graceful_terminate(child: &mut Box<dyn Child>, timeout: Duration) -> Result<bool> {
    // Try SIGTERM first
    if send_signal(child, ProcessSignal::Sigterm).is_ok() {
        let start = Instant::now();
        while start.elapsed() < timeout {
            match child.try_wait()? {
                Some(_) => return Ok(true),
                None => std::thread::sleep(Duration::from_millis(100)),
            }
        }
    }

    // Force kill if still running
    if send_signal(child, ProcessSignal::Sigkill).is_ok() {
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(5) {
            match child.try_wait()? {
                Some(_) => return Ok(true),
                None => std::thread::sleep(Duration::from_millis(100)),
            }
        }
    }

    Ok(false)
}

// ─── Configuration ────────────────────────────────────────────────────────────

/// Configuration for Bash PTY sessions.
#[derive(Debug, Clone)]
pub struct BashPtyConfig {
    /// Shell to use (default: "bash" on Unix, "powershell" on Windows).
    pub shell: String,
    /// Working directory for the session.
    pub cwd: Option<PathBuf>,
    /// Environment variables to set.
    pub env: Vec<(String, String)>,
    /// Default terminal size.
    pub size: PtySize,
    /// Default timeout for commands.
    pub timeout: Duration,
    /// Maximum output bytes to capture.
    pub max_output_bytes: usize,
    /// Session ID for registry tracking.
    pub session_id: Option<String>,
    /// Whether to auto-register in global registry.
    pub auto_register: bool,
}

impl Default for BashPtyConfig {
    fn default() -> Self {
        Self {
            shell: default_shell(),
            cwd: None,
            env: Vec::new(),
            size: PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            },
            timeout: Duration::from_secs(60),
            max_output_bytes: 1024 * 1024, // 1MB
            session_id: None,
            auto_register: true,
        }
    }
}

fn default_shell() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("COMSPEC").unwrap_or_else(|_| "powershell.exe".into())
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".into())
    }
}

// ─── Session Types ────────────────────────────────────────────────────────────

/// A Bash PTY session handle.
pub struct BashPtySession {
    pty_pair: PtyPair,
    config: BashPtyConfig,
    start_time: Instant,
    child: Option<Box<dyn Child>>,
    session_id: String,
    command_count: u64,
}

impl Drop for BashPtySession {
    fn drop(&mut self) {
        // Gracefully terminate child process
        if let Some(ref mut child) = self.child {
            let _ = graceful_terminate(child, Duration::from_secs(5));
        }
        // Unregister from global registry
        let _ = unregister_session(&self.session_id);
    }
}

/// Result of executing a command in a PTY.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BashPtyResult {
    /// Exit code of the command (None if killed/timed out).
    pub exit_code: Option<i32>,
    /// Combined stdout/stderr output from the PTY.
    pub output: String,
    /// Whether the command completed successfully.
    pub success: bool,
    /// Whether the command timed out.
    pub timed_out: bool,
    /// Duration of execution.
    pub duration_ms: u64,
}

/// Request to execute a command in a PTY.
#[derive(Debug, Clone, Deserialize)]
pub struct BashPtyRequest {
    /// Command to execute.
    pub command: String,
    /// Working directory (overrides session default).
    pub cwd: Option<PathBuf>,
    /// Timeout in seconds (overrides default).
    pub timeout_secs: Option<u64>,
    /// Maximum output bytes (overrides default).
    pub max_output_bytes: Option<usize>,
    /// Environment variables to set for this command only.
    pub env: Option<Vec<(String, String)>>,
}

// ─── Session Management ───────────────────────────────────────────────────────

impl BashPtySession {
    /// Create a new Bash PTY session with the given configuration.
    pub fn new(config: BashPtyConfig) -> Result<Self> {
        let pty_system = NativePtySystem::default();
        let pty_pair = pty_system
            .openpty(config.size.clone())
            .context("failed to open PTY")?;

        // Build the shell command
        let mut cmd_builder = CommandBuilder::new(&config.shell);
        cmd_builder.env("TERM", "xterm-256color");
        cmd_builder.env("PTY_MODE", "agent");

        // Set working directory
        if let Some(ref cwd) = config.cwd {
            cmd_builder.cwd(cwd);
        }

        // Set environment variables
        for (key, value) in &config.env {
            cmd_builder.env(key, value);
        }

        // Spawn the shell
        let child = pty_pair
            .slave
            .spawn_command(cmd_builder)
            .context("failed to spawn shell in PTY")?;

        // Generate or use provided session ID
        let session_id = config
            .session_id
            .clone()
            .unwrap_or_else(|| format!("pty-{}", uuid::Uuid::new_v4()));

        // Register in global registry
        if config.auto_register {
            register_session(session_id.clone())?;
        }

        Ok(Self {
            pty_pair,
            config,
            start_time: Instant::now(),
            child: Some(child),
            session_id,
            command_count: 0,
        })
    }

    /// Get the session ID.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Send a signal to the shell process.
    pub fn signal(&mut self, signal: ProcessSignal) -> Result<()> {
        if let Some(ref mut child) = self.child {
            send_signal(child, signal)
        } else {
            bail!("no child process")
        }
    }

    /// Gracefully terminate the session.
    pub fn terminate(&mut self, timeout: Duration) -> Result<bool> {
        if let Some(ref mut child) = self.child {
            graceful_terminate(child, timeout)
        } else {
            Ok(true)
        }
    }

    /// Execute a command in this PTY session.
    pub fn execute(&mut self, request: &BashPtyRequest) -> Result<BashPtyResult> {
        let start = Instant::now();
        let timeout = request
            .timeout_secs
            .map(Duration::from_secs)
            .unwrap_or(self.config.timeout);
        let max_bytes = request
            .max_output_bytes
            .unwrap_or(self.config.max_output_bytes);

        // Track activity in registry
        self.command_count += 1;
        if self.config.auto_register {
            let _ = touch_session(&self.session_id);
        }

        // Get writer to send command
        let mut writer = self
            .pty_pair
            .master
            .take_writer()
            .context("failed to get PTY writer")?;

        // Get reader to read output
        let mut reader = self
            .pty_pair
            .master
            .try_clone_reader()
            .context("failed to get PTY reader")?;

        // Change directory if specified
        if let Some(ref cwd) = request.cwd {
            let cd_cmd = format!(
                "cd '{}' 2>/dev/null || cd \"{}\"\n",
                cwd.display(),
                cwd.display()
            );
            writer.write_all(cd_cmd.as_bytes())?;
            writer.flush()?;
            // Small delay to let cd complete
            std::thread::sleep(Duration::from_millis(50));
            // Drain any output from cd
            let mut drain_buf = [0u8; 1024];
            let _ = reader.read(&mut drain_buf);
        }

        // Set environment variables for this command
        if let Some(ref env_vars) = request.env {
            for (key, value) in env_vars {
                let export_cmd = format!("export {}='{}'\n", key, value.replace('\'', "'\"'\"'"));
                writer.write_all(export_cmd.as_bytes())?;
            }
            if !env_vars.is_empty() {
                writer.flush()?;
                std::thread::sleep(Duration::from_millis(50));
                let mut drain_buf = [0u8; 1024];
                let _ = reader.read(&mut drain_buf);
            }
        }

        // Send the command with a marker to detect completion
        let marker = format!("__BASH_PTY_DONE_{}__", rand::random::<u64>());
        let cmd_with_marker = format!("{}; echo \"{}:$?\"\n", request.command, marker);

        writer.write_all(cmd_with_marker.as_bytes())?;
        writer.flush()?;

        // Read output until we see the marker or timeout
        let mut output = String::new();
        let mut buf = vec![0u8; 4096];
        let mut timed_out = false;
        let marker_pattern = format!("{}:", marker);

        loop {
            if start.elapsed() > timeout {
                timed_out = true;
                break;
            }

            // Set a read timeout
            self.pty_pair
                .master
                .set_blocking(false)
                .context("failed to set non-blocking mode")?;

            match reader.read(&mut buf) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let chunk = String::from_utf8_lossy(&buf[..n]);
                    output.push_str(&chunk);

                    // Check for marker
                    if let Some(pos) = output.find(&marker_pattern) {
                        // Extract exit code
                        let after_marker = &output[pos + marker_pattern.len()..];
                        if let Some(newline_pos) = after_marker.find('\n') {
                            let exit_code_str = &after_marker[..newline_pos];
                            let exit_code = exit_code_str.parse::<i32>().ok();

                            // Truncate output to remove marker
                            output.truncate(pos);

                            return Ok(BashPtyResult {
                                exit_code,
                                output: truncate_output(&output, max_bytes),
                                success: exit_code == Some(0),
                                timed_out: false,
                                duration_ms: start.elapsed().as_millis() as u64,
                            });
                        }
                    }

                    // Check output size limit
                    if output.len() > max_bytes * 2 {
                        output.truncate(max_bytes);
                        output.push_str("\n[output truncated due to size limit]");
                        timed_out = true;
                        break;
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        std::thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                    return Err(e.into());
                }
            }
        }

        Ok(BashPtyResult {
            exit_code: None,
            output: truncate_output(&output, max_bytes),
            success: false,
            timed_out,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Resize the PTY terminal.
    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        self.pty_pair
            .master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("failed to resize PTY")
    }

    /// Get session duration.
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

// ─── Helper Functions ─────────────────────────────────────────────────────────

fn truncate_output(output: &str, max_bytes: usize) -> String {
    if output.len() <= max_bytes {
        output.to_string()
    } else {
        let mut truncated = output[..max_bytes].to_string();
        truncated.push_str("\n[output truncated]");
        truncated
    }
}

/// Execute a single command in a new PTY session (convenience function).
pub fn execute_pty_command(
    request: &BashPtyRequest,
    config: Option<BashPtyConfig>,
) -> Result<BashPtyResult> {
    let config = config.unwrap_or_default();
    let mut session = BashPtySession::new(config)?;
    session.execute(request)
}

/// Execute a command string directly (simplest API).
pub fn bash_pty(command: &str, timeout_secs: Option<u64>) -> Result<BashPtyResult> {
    let request = BashPtyRequest {
        command: command.to_string(),
        cwd: None,
        timeout_secs,
        max_output_bytes: None,
        env: None,
    };
    execute_pty_command(&request, None)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bash_pty_echo() {
        let result = bash_pty("echo 'hello world'", Some(5)).unwrap();
        assert!(result.success);
        assert!(result.output.contains("hello world"));
        assert_eq!(result.exit_code, Some(0));
    }

    #[test]
    fn test_bash_pty_exit_code() {
        let result = bash_pty("exit 42", Some(5)).unwrap();
        assert!(!result.success);
        assert_eq!(result.exit_code, Some(42));
    }

    #[test]
    fn test_bash_pty_env_var() {
        let request = BashPtyRequest {
            command: "echo $TEST_VAR".to_string(),
            cwd: None,
            timeout_secs: Some(5),
            max_output_bytes: None,
            env: Some(vec![("TEST_VAR".to_string(), "test_value".to_string())]),
        };
        let result = execute_pty_command(&request, None).unwrap();
        assert!(result.success);
        assert!(result.output.contains("test_value"));
    }

    #[test]
    fn test_bash_pty_pwd() {
        let request = BashPtyRequest {
            command: "pwd".to_string(),
            cwd: Some(PathBuf::from("/")),
            timeout_secs: Some(5),
            max_output_bytes: None,
            env: None,
        };
        let result = execute_pty_command(&request, None).unwrap();
        assert!(result.success);
        assert!(result.output.contains('/'));
    }

    #[test]
    fn test_bash_pty_timeout() {
        let result = bash_pty("sleep 10", Some(1));
        // Should either timeout or return an error
        if let Ok(r) = result {
            assert!(r.timed_out);
        }
    }

    #[test]
    fn test_truncate_output() {
        let long = "a".repeat(2000);
        let truncated = truncate_output(&long, 1000);
        assert!(truncated.len() <= 1020); // 1000 + truncation message
        assert!(truncated.contains("[output truncated]"));
    }

    #[test]
    fn test_session_registry() {
        // Clear any existing sessions
        let existing = list_sessions().unwrap();
        for s in existing {
            unregister_session(&s.id).unwrap();
        }

        // Register a test session
        let session_id = "test-session-123".to_string();
        register_session(session_id.clone()).unwrap();

        // List should contain our session
        let sessions = list_sessions().unwrap();
        assert!(sessions.iter().any(|s| s.id == session_id));

        // Touch the session
        touch_session(&session_id).unwrap();

        // Unregister
        unregister_session(&session_id).unwrap();

        // Should be gone
        let sessions = list_sessions().unwrap();
        assert!(!sessions.iter().any(|s| s.id == session_id));
    }

    #[test]
    fn test_cleanup_stale_sessions() {
        // Clear existing
        let existing = list_sessions().unwrap();
        for s in existing {
            unregister_session(&s.id).unwrap();
        }

        // Register a session
        let session_id = "stale-session".to_string();
        register_session(session_id.clone()).unwrap();

        // Cleanup with 0 duration should remove all
        let cleaned = cleanup_stale_sessions(Duration::from_nanos(1)).unwrap();
        assert!(cleaned >= 1);

        let sessions = list_sessions().unwrap();
        assert!(!sessions.iter().any(|s| s.id == session_id));
    }

    #[test]
    fn test_session_with_id() {
        let config = BashPtyConfig {
            session_id: Some("my-custom-id".to_string()),
            auto_register: true,
            ..Default::default()
        };

        let session = BashPtySession::new(config).unwrap();
        assert_eq!(session.session_id(), "my-custom-id");

        // Should be in registry
        let sessions = list_sessions().unwrap();
        assert!(sessions.iter().any(|s| s.id == "my-custom-id"));

        // Cleanup
        unregister_session("my-custom-id").unwrap();
    }

    #[test]
    fn test_process_signal_enum() {
        // Just verify the enum variants exist
        let _sigint = ProcessSignal::Sigint;
        let _sigterm = ProcessSignal::Sigterm;
        let _sigkill = ProcessSignal::Sigkill;
    }
}
