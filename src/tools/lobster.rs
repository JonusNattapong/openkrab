//! tools::lobster — Lobster workflow pipeline runner.
//! Ported from `openclaw/extensions/lobster/src/lobster-tool.ts` (Phase 14).
//!
//! Invokes the `lobster` binary as a sandboxed subprocess with:
//! - CWD containment (relative paths only, must stay within workspace)
//! - Stdout byte limit + timeout
//! - Typed JSON envelope output
//! - Tolerant noise-prefix parser

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LobsterConfig {
    pub lobster_path: Option<PathBuf>,
    pub timeout: Duration,
    pub max_stdout_bytes: usize,
}

impl Default for LobsterConfig {
    fn default() -> Self {
        Self { lobster_path: None, timeout: Duration::from_secs(20), max_stdout_bytes: 512_000 }
    }
}

// ─── Envelope types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LobsterEnvelope { Ok(LobsterOkEnvelope), Err(LobsterErrEnvelope) }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobsterOkEnvelope {
    pub ok: bool,
    pub status: LobsterStatus,
    pub output: Vec<serde_json::Value>,
    #[serde(rename = "requiresApproval")]
    pub requires_approval: Option<ApprovalRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LobsterStatus { Ok, NeedsApproval, Cancelled }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    #[serde(rename = "type")]
    pub kind: String,
    pub prompt: String,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "resumeToken")]
    pub resume_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobsterErrEnvelope { pub ok: bool, pub error: LobsterError }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobsterError {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub message: String,
}

// ─── Actions ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum LobsterAction {
    Run { pipeline: String, args_json: Option<String> },
    Resume { token: String, approve: bool },
}

// ─── Security: path validation ────────────────────────────────────────────────

pub fn resolve_executable(path_override: Option<&Path>) -> Result<String> {
    match path_override {
        None => Ok("lobster".into()),
        Some(p) => {
            if !p.is_absolute() { bail!("lobster_path must be an absolute path"); }
            let filename = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();
            let allowed = if cfg!(windows) {
                vec!["lobster.exe", "lobster.cmd", "lobster.bat"]
            } else {
                vec!["lobster"]
            };
            if !allowed.contains(&filename.as_str()) {
                bail!("lobster_path must point to the lobster executable (not {})", filename);
            }
            if !p.is_file() { bail!("lobster_path must exist and be a file"); }
            Ok(p.to_string_lossy().into_owned())
        }
    }
}

/// Lexically normalize a path (collapse `.` and `..` without filesystem access).
fn lexical_normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => { out.pop(); }
            std::path::Component::CurDir => {}
            c => out.push(c),
        }
    }
    out
}

pub fn resolve_cwd(workspace_root: &Path, cwd_rel: Option<&str>) -> Result<PathBuf> {
    match cwd_rel {
        None | Some("") => Ok(workspace_root.to_path_buf()),
        Some(rel) => {
            let rel = rel.trim();
            if Path::new(rel).is_absolute() { bail!("cwd must be a relative path"); }
            // Lexically normalize BEFORE checking containment to catch `../` traversal
            let resolved = lexical_normalize(&workspace_root.join(rel));
            let norm_root = lexical_normalize(workspace_root);
            let root_str = norm_root.to_string_lossy().to_lowercase();
            let res_str = resolved.to_string_lossy().to_lowercase();
            if !res_str.starts_with(&*root_str) {
                bail!("cwd must stay within the workspace directory");
            }
            Ok(resolved)
        }
    }
}

// ─── Build argv ───────────────────────────────────────────────────────────────

pub fn build_argv(action: &LobsterAction) -> Result<Vec<String>> {
    match action {
        LobsterAction::Run { pipeline, args_json } => {
            if pipeline.trim().is_empty() { bail!("pipeline is required"); }
            let mut argv = vec!["run".into(), "--mode".into(), "tool".into(), pipeline.clone()];
            if let Some(json) = args_json.as_deref().filter(|s| !s.trim().is_empty()) {
                argv.push("--args-json".into());
                argv.push(json.into());
            }
            Ok(argv)
        }
        LobsterAction::Resume { token, approve } => {
            if token.trim().is_empty() { bail!("token is required"); }
            Ok(vec![
                "resume".into(), "--token".into(), token.clone(),
                "--approve".into(), if *approve { "yes" } else { "no" }.into(),
            ])
        }
    }
}

// ─── Parse envelope ───────────────────────────────────────────────────────────

/// Scan FORWARD for the first `{` or `[` — strips leading noise/log output.
fn find_first_json_prefix(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b'{' || bytes[i] == b'[' {
            return Some(&s[i..]);
        }
    }
    None
}

fn envelope_from_value(v: serde_json::Value) -> Result<LobsterEnvelope> {
    match v.get("ok") {
        Some(serde_json::Value::Bool(true)) => {
            Ok(LobsterEnvelope::Ok(serde_json::from_value(v)?))
        }
        Some(serde_json::Value::Bool(false)) => {
            Ok(LobsterEnvelope::Err(serde_json::from_value(v)?))
        }
        _ => bail!("lobster returned invalid JSON envelope (missing 'ok' field)"),
    }
}

/// Parse Lobster JSON envelope from subprocess stdout.
/// Tolerates leading log/warning lines before the JSON.
pub fn parse_envelope(stdout: &str) -> Result<LobsterEnvelope> {
    let trimmed = stdout.trim();

    // Direct parse (clean output)
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return envelope_from_value(v);
    }

    // Tolerance: strip any leading noise before the first `{` or `[`
    if let Some(prefix) = find_first_json_prefix(trimmed) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(prefix) {
            return envelope_from_value(v);
        }
    }

    bail!("lobster returned invalid JSON");
}

// ─── Subprocess runner ────────────────────────────────────────────────────────

pub fn run_lobster_sync(
    exec: &str,
    argv: &[String],
    cwd: &Path,
    timeout: Duration,
    max_stdout_bytes: usize,
) -> Result<String> {
    use std::process::Command;
    let mut cmd = Command::new(exec);
    cmd.args(argv).current_dir(cwd).env("LOBSTER_MODE", "tool");
    if let Ok(node_opts) = std::env::var("NODE_OPTIONS") {
        if node_opts.contains("--inspect") { cmd.env_remove("NODE_OPTIONS"); }
    }
    let start = std::time::Instant::now();
    let output = cmd.output()?;
    if start.elapsed() > timeout {
        bail!("lobster subprocess timed out");
    }
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if stdout.len() > max_stdout_bytes {
        bail!("lobster output exceeded maxStdoutBytes ({})", max_stdout_bytes);
    }
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("lobster failed ({}): {}", output.status, stderr.trim());
    }
    Ok(stdout)
}

pub fn execute(cfg: &LobsterConfig, workspace_root: &Path, action: &LobsterAction, cwd_rel: Option<&str>) -> Result<LobsterEnvelope> {
    let exec = resolve_executable(cfg.lobster_path.as_deref())?;
    let cwd = resolve_cwd(workspace_root, cwd_rel)?;
    let argv = build_argv(action)?;
    let stdout = run_lobster_sync(&exec, &argv, &cwd, cfg.timeout, cfg.max_stdout_bytes)?;
    parse_envelope(&stdout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn resolve_cwd_stays_in_workspace() {
        let root = PathBuf::from("/workspace");
        assert!(resolve_cwd(&root, Some("../escape")).is_err());
        assert!(resolve_cwd(&root, Some("/absolute")).is_err());
    }

    #[test]
    fn resolve_cwd_relative_ok() {
        let root = PathBuf::from("/workspace");
        let r = resolve_cwd(&root, Some("subdir")).unwrap();
        assert!(r.starts_with(&root));
    }

    #[test]
    fn resolve_cwd_none_returns_root() {
        let root = PathBuf::from("/workspace");
        assert_eq!(resolve_cwd(&root, None).unwrap(), root);
    }

    #[test]
    fn build_argv_run() {
        let action = LobsterAction::Run { pipeline: "my-pipeline".into(), args_json: None };
        let argv = build_argv(&action).unwrap();
        assert_eq!(argv, vec!["run", "--mode", "tool", "my-pipeline"]);
    }

    #[test]
    fn build_argv_run_with_args() {
        let action = LobsterAction::Run {
            pipeline: "pipe".into(),
            args_json: Some(r#"{"key":"val"}"#.into()),
        };
        let argv = build_argv(&action).unwrap();
        assert!(argv.contains(&"--args-json".to_string()));
    }

    #[test]
    fn build_argv_resume() {
        let action = LobsterAction::Resume { token: "tok123".into(), approve: true };
        let argv = build_argv(&action).unwrap();
        assert!(argv.contains(&"yes".to_string()));
    }

    #[test]
    fn parse_envelope_ok() {
        let json = r#"{"ok":true,"status":"ok","output":[],"requiresApproval":null}"#;
        let env = parse_envelope(json).unwrap();
        assert!(matches!(env, LobsterEnvelope::Ok(_)));
    }

    #[test]
    fn parse_envelope_err() {
        let json = r#"{"ok":false,"error":{"message":"pipeline not found"}}"#;
        let env = parse_envelope(json).unwrap();
        assert!(matches!(env, LobsterEnvelope::Err(_)));
    }

    #[test]
    fn parse_envelope_with_noise() {
        let noisy = "Warning: some debug output\n{\"ok\":true,\"status\":\"ok\",\"output\":[],\"requiresApproval\":null}";
        let env = parse_envelope(noisy).unwrap();
        assert!(matches!(env, LobsterEnvelope::Ok(_)));
    }

    #[test]
    fn build_argv_empty_pipeline_errors() {
        let action = LobsterAction::Run { pipeline: "".into(), args_json: None };
        assert!(build_argv(&action).is_err());
    }

    #[test]
    fn resolve_executable_none_returns_lobster() {
        assert_eq!(resolve_executable(None).unwrap(), "lobster");
    }
}
