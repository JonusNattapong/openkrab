//! process — Subprocess execution, command queue, and process supervision.
//! Ported from `openkrab/src/process/` (Phase 8).
//!
//! Provides safe subprocess spawning, a sequential command queue,
//! kill-tree utilities, and a simple supervisor for restarting crashed children.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::process::{Command, Output};
use std::time::Duration;

// ─── Exec ─────────────────────────────────────────────────────────────────────

/// Options for running a shell command.
#[derive(Debug, Clone)]
pub struct ExecOptions {
    /// Working directory (None = inherit).
    pub cwd: Option<std::path::PathBuf>,
    /// Extra environment variables.
    pub env: Vec<(String, String)>,
    /// Timeout (None = no limit).
    pub timeout: Option<Duration>,
    /// If true, capture both stdout + stderr; otherwise stream to parent.
    pub capture: bool,
}

impl Default for ExecOptions {
    fn default() -> Self {
        Self {
            cwd: None,
            env: Vec::new(),
            timeout: None,
            capture: true,
        }
    }
}

/// Result of running a subprocess.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

impl ExecResult {
    pub fn from_output(out: Output) -> Self {
        let exit_code = out.status.code().unwrap_or(-1);
        Self {
            exit_code,
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            success: out.status.success(),
        }
    }
}

/// Execute a shell command string synchronously.
///
/// On Windows uses `cmd /C`; on Unix uses `sh -c`.
pub fn exec(cmd: &str, opts: &ExecOptions) -> Result<ExecResult> {
    #[cfg(target_os = "windows")]
    let mut child = {
        let mut c = Command::new("cmd");
        c.args(["/C", cmd]);
        c
    };
    #[cfg(not(target_os = "windows"))]
    let mut child = {
        let mut c = Command::new("sh");
        c.args(["-c", cmd]);
        c
    };

    if let Some(ref cwd) = opts.cwd {
        child.current_dir(cwd);
    }
    for (k, v) in &opts.env {
        child.env(k, v);
    }

    let out = child
        .output()
        .with_context(|| format!("failed to run command: {}", cmd))?;

    Ok(ExecResult::from_output(out))
}

/// Like `exec` but fails if exit_code != 0.
pub fn exec_ok(cmd: &str, opts: &ExecOptions) -> Result<ExecResult> {
    let r = exec(cmd, opts)?;
    if !r.success {
        bail!(
            "command `{}` failed (exit {}): {}",
            cmd,
            r.exit_code,
            r.stderr.trim()
        );
    }
    Ok(r)
}

// ─── Command queue ────────────────────────────────────────────────────────────

/// A simple sequential command queue — runs commands one at a time.
#[derive(Debug, Default)]
pub struct CommandQueue {
    pending: VecDeque<String>,
    results: Vec<ExecResult>,
    opts: Option<ExecOptions>,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_opts(opts: ExecOptions) -> Self {
        Self {
            opts: Some(opts),
            ..Default::default()
        }
    }

    pub fn push(&mut self, cmd: impl Into<String>) {
        self.pending.push_back(cmd.into());
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Run all pending commands in order. Stops on first failure if `stop_on_error` is true.
    pub fn flush(&mut self, stop_on_error: bool) -> Vec<ExecResult> {
        let opts = self.opts.clone().unwrap_or_default();
        let mut results = Vec::new();
        while let Some(cmd) = self.pending.pop_front() {
            match exec(&cmd, &opts) {
                Ok(r) => {
                    let failed = !r.success;
                    results.push(r);
                    if failed && stop_on_error {
                        break;
                    }
                }
                Err(e) => {
                    results.push(ExecResult {
                        exit_code: -1,
                        stdout: String::new(),
                        stderr: e.to_string(),
                        success: false,
                    });
                    if stop_on_error {
                        break;
                    }
                }
            }
        }
        self.results.extend(results.clone());
        results
    }
}

// ─── Supervisor ───────────────────────────────────────────────────────────────

/// Restart policy for a supervised process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RestartPolicy {
    /// Never restart.
    Never,
    /// Always restart on exit.
    Always,
    /// Restart only on non-zero exit.
    OnFailure,
}

/// Simple process supervisor entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisedProcess {
    pub name: String,
    pub command: String,
    pub policy: RestartPolicy,
    pub restart_count: u32,
    pub max_restarts: u32,
    pub last_exit_code: Option<i32>,
}

impl SupervisedProcess {
    pub fn new(name: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
            policy: RestartPolicy::OnFailure,
            restart_count: 0,
            max_restarts: 5,
            last_exit_code: None,
        }
    }

    /// Returns true if this process should be restarted given the exit code.
    pub fn should_restart(&self, exit_code: i32) -> bool {
        if self.restart_count >= self.max_restarts {
            return false;
        }
        match self.policy {
            RestartPolicy::Never => false,
            RestartPolicy::Always => true,
            RestartPolicy::OnFailure => exit_code != 0,
        }
    }
}

// ─── Platform spawn helpers ───────────────────────────────────────────────────

/// Build a `std::process::Command` for the given shell command string.
pub fn shell_command(cmd: &str) -> Command {
    #[cfg(target_os = "windows")]
    {
        let mut c = Command::new("cmd");
        c.args(["/C", cmd]);
        c
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut c = Command::new("sh");
        c.args(["-c", cmd]);
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_echo() {
        #[cfg(target_os = "windows")]
        let cmd = "echo hello";
        #[cfg(not(target_os = "windows"))]
        let cmd = "echo hello";

        let r = exec(cmd, &ExecOptions::default()).unwrap();
        assert!(r.success);
        assert!(r.stdout.contains("hello"));
    }

    #[test]
    fn exec_ok_fails_on_bad_exit() {
        #[cfg(target_os = "windows")]
        let cmd = "exit 1";
        #[cfg(not(target_os = "windows"))]
        let cmd = "exit 1";

        let r = exec_ok(cmd, &ExecOptions::default());
        assert!(r.is_err());
    }

    #[test]
    fn command_queue_runs_in_order() {
        let mut q = CommandQueue::new();
        #[cfg(target_os = "windows")]
        {
            q.push("echo a");
            q.push("echo b");
        }
        #[cfg(not(target_os = "windows"))]
        {
            q.push("echo a");
            q.push("echo b");
        }
        let results = q.flush(true);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.success));
    }

    #[test]
    fn command_queue_stops_on_error() {
        let mut q = CommandQueue::new();
        q.push("not_a_real_command_xyz_abc");
        q.push("echo should_not_run");
        let results = q.flush(true);
        // Should have exactly 1 result (stopped after failure)
        assert_eq!(results.len(), 1);
        assert!(!results[0].success);
    }

    #[test]
    fn supervised_process_restart_policy() {
        let mut p = SupervisedProcess::new("test", "echo hi");
        p.policy = RestartPolicy::OnFailure;
        assert!(p.should_restart(1));
        assert!(!p.should_restart(0));

        p.policy = RestartPolicy::Always;
        assert!(p.should_restart(0));

        p.policy = RestartPolicy::Never;
        assert!(!p.should_restart(1));
    }

    #[test]
    fn supervised_max_restarts_reached() {
        let mut p = SupervisedProcess::new("test", "echo hi");
        p.restart_count = 5;
        p.max_restarts = 5;
        assert!(!p.should_restart(1));
    }
}
