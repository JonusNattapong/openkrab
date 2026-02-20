//! tools::open_prose — OpenProse VM skill command router.
//! Ported from `openkrab/extensions/open-prose/` (Phase 21).
//!
//! OpenProse is a programming language for AI sessions.
//! This module provides command routing, target resolution, and workspace
//! migration helpers. The actual VM semantics live in the `prose.md` skill
//! document; this module handles the structural / path-resolution layer.
//!
//! Reference: `openkrab/extensions/open-prose/skills/prose/SKILL.md`

use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

// ─── Command kind ────────────────────────────────────────────────────────────

/// Top-level `prose` sub-commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProseCommand {
    /// `prose run <target>` — execute a local file, URL, or registry shorthand.
    Run { target: String },
    /// `prose compile <file>` — validate syntax without running.
    Compile { file: String },
    /// `prose help` — display help and examples.
    Help,
    /// `prose examples` — list or run bundled example programs.
    Examples { name: Option<String> },
    /// `prose update` — migrate legacy workspace paths.
    Update,
    /// Any other sub-command, forwarded verbatim.
    Other { command: String, args: Vec<String> },
}

/// Parse a space-separated `prose` invocation into a `ProseCommand`.
///
/// Input is the tail after the leading `prose` token, e.g. `"run foo.prose"`.
pub fn parse_prose_command(input: &str) -> ProseCommand {
    let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
    match parts.as_slice() {
        ["run", target, ..] => ProseCommand::Run {
            target: target.trim().to_string(),
        },
        ["compile", file, ..] => ProseCommand::Compile {
            file: file.trim().to_string(),
        },
        ["help"] | ["help", ..] => ProseCommand::Help,
        ["examples"] => ProseCommand::Examples { name: None },
        ["examples", name, ..] => ProseCommand::Examples {
            name: Some(name.trim().to_string()),
        },
        ["update"] | ["update", ..] => ProseCommand::Update,
        [cmd, rest @ ..] => ProseCommand::Other {
            command: cmd.to_string(),
            args: rest.iter().map(|s| s.to_string()).collect(),
        },
        [] => ProseCommand::Help,
    }
}

// ─── Target resolution ───────────────────────────────────────────────────────

/// How a `prose run` target should be loaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProseTargetKind {
    /// A local file path.
    LocalFile(PathBuf),
    /// A direct HTTP/HTTPS URL.
    RemoteUrl(String),
    /// A `handle/slug` registry shorthand → `https://p.prose.md/{path}`.
    RegistryShorthand(String),
}

/// Resolve a raw target string to a `ProseTargetKind`.
///
/// Rules:
/// - Starts with `http://` or `https://` → `RemoteUrl`
/// - Contains `/` but no protocol → `RegistryShorthand`
/// - Otherwise → `LocalFile`
pub fn resolve_target(target: &str) -> ProseTargetKind {
    let t = target.trim();
    if t.starts_with("http://") || t.starts_with("https://") {
        return ProseTargetKind::RemoteUrl(t.to_string());
    }
    if t.contains('/') {
        return ProseTargetKind::RegistryShorthand(t.to_string());
    }
    ProseTargetKind::LocalFile(PathBuf::from(t))
}

/// Convert a `RegistryShorthand` to its canonical fetch URL.
pub fn registry_shorthand_to_url(shorthand: &str) -> String {
    format!("https://p.prose.md/{}", shorthand.trim_start_matches('/'))
}

// ─── Known examples ───────────────────────────────────────────────────────────

/// Keyword→filename table for well-known bundled examples.
pub const KNOWN_EXAMPLES: &[(&str, &str)] = &[
    ("hello", "examples/01-hello-world.prose"),
    ("hello world", "examples/01-hello-world.prose"),
    ("gas town", "examples/28-gas-town.prose"),
    ("gastown", "examples/28-gas-town.prose"),
    ("captain", "examples/29-captains-chair.prose"),
    ("chair", "examples/29-captains-chair.prose"),
    ("forge", "examples/37-the-forge.prose"),
    ("browser", "examples/37-the-forge.prose"),
    ("parallel", "examples/16-parallel-reviews.prose"),
    ("pipeline", "examples/21-pipeline-operations.prose"),
    ("error", "examples/22-error-handling.prose"),
    ("retry", "examples/22-error-handling.prose"),
];

/// Look up a bundled example filename by keyword (case-insensitive partial match).
pub fn find_example_by_keyword(keyword: &str) -> Option<&'static str> {
    let lower = keyword.trim().to_lowercase();
    KNOWN_EXAMPLES
        .iter()
        .find(|(kw, _)| kw.contains(lower.as_str()) || lower.contains(kw))
        .map(|(_, path)| *path)
}

// ─── Workspace migration ──────────────────────────────────────────────────────

/// Result of a `prose update` (migration) run.
#[derive(Debug, Default)]
pub struct MigrationResult {
    pub converted_state_json: bool,
    pub renamed_execution_dir: bool,
    pub created_agents_dir: bool,
    pub already_up_to_date: bool,
}

/// Run `prose update` workspace migration against the given base directory.
///
/// - Converts `.prose/state.json` → `.prose/.env` (JSON → key=value)
/// - Renames `.prose/execution/` → `.prose/runs/`
/// - Creates `.prose/agents/` if missing
pub fn run_migration(base: &Path) -> Result<MigrationResult> {
    let mut result = MigrationResult::default();

    // 1. Convert .prose/state.json → .prose/.env
    let state_json = base.join(".prose").join("state.json");
    if state_json.exists() {
        let content = std::fs::read_to_string(&state_json)?;
        let parsed: serde_json::Value = serde_json::from_str(&content)?;
        let mut env_lines = Vec::new();
        if let Some(obj) = parsed.as_object() {
            for (k, v) in obj {
                let val_str = match v {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                env_lines.push(format!("{}={}", k, val_str));
            }
        }
        let env_path = base.join(".prose").join(".env");
        std::fs::write(&env_path, env_lines.join("\n"))?;
        std::fs::remove_file(&state_json)?;
        result.converted_state_json = true;
    }

    // 2. Rename .prose/execution/ → .prose/runs/
    let execution_dir = base.join(".prose").join("execution");
    let runs_dir = base.join(".prose").join("runs");
    if execution_dir.exists() && !runs_dir.exists() {
        std::fs::rename(&execution_dir, &runs_dir)?;
        result.renamed_execution_dir = true;
    }

    // 3. Create .prose/agents/ if missing
    let agents_dir = base.join(".prose").join("agents");
    if !agents_dir.exists() {
        std::fs::create_dir_all(&agents_dir)?;
        result.created_agents_dir = true;
    }

    if !result.converted_state_json && !result.renamed_execution_dir && !result.created_agents_dir {
        result.already_up_to_date = true;
    }

    Ok(result)
}

/// Format the migration result as human-readable output (mirrors the TS spec output).
pub fn format_migration_output(result: &MigrationResult) -> String {
    if result.already_up_to_date {
        return "Workspace already up to date. No migration needed.".to_string();
    }
    let mut lines = vec!["Migrating OpenProse workspace...".to_string()];
    if result.converted_state_json {
        lines.push("  Converted .prose/state.json -> .prose/.env".to_string());
    }
    if result.renamed_execution_dir {
        lines.push("  Renamed .prose/execution/ -> .prose/runs/".to_string());
    }
    if result.created_agents_dir {
        lines.push("  Created .prose/agents/".to_string());
    }
    lines.push("Migration complete. Your workspace is up to date.".to_string());
    lines.join("\n")
}

// ─── State mode ───────────────────────────────────────────────────────────────

/// OpenProse state backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateMode {
    /// Default: file-based under `.prose/runs/{id}/`.
    Filesystem,
    /// In-conversation (ephemeral, no persistence).
    InContext,
    /// SQLite (experimental): `.prose/runs/{id}/state.db`.
    Sqlite,
    /// PostgreSQL (experimental).
    Postgres,
}

impl StateMode {
    pub fn from_flag(flag: Option<&str>) -> Self {
        match flag {
            Some("--in-context") | Some("in-context") => Self::InContext,
            Some("--state=sqlite") | Some("sqlite") => Self::Sqlite,
            Some("--state=postgres") | Some("postgres") => Self::Postgres,
            _ => Self::Filesystem,
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_run_command() {
        let cmd = parse_prose_command("run foo.prose");
        assert_eq!(
            cmd,
            ProseCommand::Run {
                target: "foo.prose".to_string()
            }
        );
    }

    #[test]
    fn parse_compile_command() {
        let cmd = parse_prose_command("compile bar.prose");
        assert_eq!(
            cmd,
            ProseCommand::Compile {
                file: "bar.prose".to_string()
            }
        );
    }

    #[test]
    fn parse_help_command() {
        assert_eq!(parse_prose_command("help"), ProseCommand::Help);
        assert_eq!(parse_prose_command("help --all"), ProseCommand::Help);
    }

    #[test]
    fn parse_examples_no_name() {
        assert_eq!(
            parse_prose_command("examples"),
            ProseCommand::Examples { name: None }
        );
    }

    #[test]
    fn parse_examples_with_name() {
        let cmd = parse_prose_command("examples gastown");
        assert_eq!(
            cmd,
            ProseCommand::Examples {
                name: Some("gastown".to_string())
            }
        );
    }

    #[test]
    fn parse_update_command() {
        assert_eq!(parse_prose_command("update"), ProseCommand::Update);
    }

    #[test]
    fn parse_empty_defaults_to_help() {
        assert_eq!(parse_prose_command(""), ProseCommand::Help);
    }

    #[test]
    fn resolve_target_https_url() {
        let t = resolve_target("https://example.com/foo.prose");
        assert_eq!(
            t,
            ProseTargetKind::RemoteUrl("https://example.com/foo.prose".to_string())
        );
    }

    #[test]
    fn resolve_target_registry_shorthand() {
        let t = resolve_target("alice/code-review");
        assert_eq!(
            t,
            ProseTargetKind::RegistryShorthand("alice/code-review".to_string())
        );
    }

    #[test]
    fn resolve_target_local_file() {
        let t = resolve_target("my-program.prose");
        assert_eq!(
            t,
            ProseTargetKind::LocalFile(PathBuf::from("my-program.prose"))
        );
    }

    #[test]
    fn registry_shorthand_to_url_formats_correctly() {
        assert_eq!(
            registry_shorthand_to_url("alice/code-review"),
            "https://p.prose.md/alice/code-review"
        );
        // Leading slash is stripped
        assert_eq!(
            registry_shorthand_to_url("/alice/code-review"),
            "https://p.prose.md/alice/code-review"
        );
    }

    #[test]
    fn find_example_by_keyword_known() {
        assert_eq!(
            find_example_by_keyword("gastown"),
            Some("examples/28-gas-town.prose")
        );
        assert_eq!(
            find_example_by_keyword("hello world"),
            Some("examples/01-hello-world.prose")
        );
    }

    #[test]
    fn find_example_by_keyword_unknown() {
        assert!(find_example_by_keyword("nonexistent-example-xyz").is_none());
    }

    #[test]
    fn state_mode_from_flag() {
        assert_eq!(
            StateMode::from_flag(Some("--in-context")),
            StateMode::InContext
        );
        assert_eq!(
            StateMode::from_flag(Some("--state=sqlite")),
            StateMode::Sqlite
        );
        assert_eq!(
            StateMode::from_flag(Some("--state=postgres")),
            StateMode::Postgres
        );
        assert_eq!(StateMode::from_flag(None), StateMode::Filesystem);
        assert_eq!(StateMode::from_flag(Some("--other")), StateMode::Filesystem);
    }

    #[test]
    fn migration_already_up_to_date() {
        let tmp = tempfile::tempdir().unwrap();
        // No legacy files → already up to date
        let prose_dir = tmp.path().join(".prose");
        std::fs::create_dir_all(&prose_dir).unwrap();
        // pre-create agents/ to avoid the "created" flag
        std::fs::create_dir_all(prose_dir.join("agents")).unwrap();
        let result = run_migration(tmp.path()).unwrap();
        assert!(result.already_up_to_date);
        assert!(!result.converted_state_json);
        assert!(!result.renamed_execution_dir);
    }

    #[test]
    fn migration_converts_state_json() {
        let tmp = tempfile::tempdir().unwrap();
        let prose_dir = tmp.path().join(".prose");
        std::fs::create_dir_all(&prose_dir).unwrap();
        std::fs::create_dir_all(prose_dir.join("agents")).unwrap();
        let state_json = prose_dir.join("state.json");
        std::fs::write(
            &state_json,
            r#"{"OPENPROSE_TELEMETRY":"enabled","USER_ID":"u-1"}"#,
        )
        .unwrap();
        let result = run_migration(tmp.path()).unwrap();
        assert!(result.converted_state_json);
        assert!(!state_json.exists());
        let env = std::fs::read_to_string(prose_dir.join(".env")).unwrap();
        assert!(env.contains("OPENPROSE_TELEMETRY=enabled"));
        assert!(env.contains("USER_ID=u-1"));
    }

    #[test]
    fn migration_renames_execution_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let prose_dir = tmp.path().join(".prose");
        std::fs::create_dir_all(prose_dir.join("execution")).unwrap();
        std::fs::create_dir_all(prose_dir.join("agents")).unwrap();
        let result = run_migration(tmp.path()).unwrap();
        assert!(result.renamed_execution_dir);
        assert!(prose_dir.join("runs").exists());
        assert!(!prose_dir.join("execution").exists());
    }

    #[test]
    fn format_migration_output_up_to_date() {
        let r = MigrationResult {
            already_up_to_date: true,
            ..Default::default()
        };
        assert!(format_migration_output(&r).contains("up to date"));
    }

    #[test]
    fn format_migration_output_with_changes() {
        let r = MigrationResult {
            converted_state_json: true,
            renamed_execution_dir: true,
            ..Default::default()
        };
        let out = format_migration_output(&r);
        assert!(out.contains("state.json"));
        assert!(out.contains("execution"));
    }
}
