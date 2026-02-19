//! infra — Infrastructure helpers: env vars, paths, workspace discovery.
//! Ported from `openclaw/src/infra/` (Phase 7).
//!
//! Centralises all OS-level path resolution so the rest of the codebase
//! never hardcodes `~/.config` or `./memory` directly.

use std::path::{Path, PathBuf};

// ─── Known directories ────────────────────────────────────────────────────────

/// Returns the platform home directory.
/// Checks `$HOME` (or `$USERPROFILE` on Windows), then falls back to `.`.
pub fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

/// Returns `$HOME/.config/krabkrab` (or `%APPDATA%\krabkrab` on Windows).
pub fn config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .map(|p| PathBuf::from(p).join("krabkrab"))
            .unwrap_or_else(|_| home_dir().join(".config").join("krabkrab"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("XDG_CONFIG_HOME")
            .map(|p| PathBuf::from(p).join("krabkrab"))
            .unwrap_or_else(|_| home_dir().join(".config").join("krabkrab"))
    }
}

/// Returns the default data directory for krabkrab (`$HOME/.local/share/krabkrab`).
pub fn data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        std::env::var("LOCALAPPDATA")
            .map(|p| PathBuf::from(p).join("krabkrab"))
            .unwrap_or_else(|_| home_dir().join(".krabkrab"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("XDG_DATA_HOME")
            .map(|p| PathBuf::from(p).join("krabkrab"))
            .unwrap_or_else(|_| home_dir().join(".local").join("share").join("krabkrab"))
    }
}

/// Returns the runtime/cache directory (`/tmp/krabkrab` or `%TEMP%\krabkrab`).
pub fn cache_dir() -> PathBuf {
    std::env::var("KRABKRAB_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::temp_dir().join("krabkrab")
        })
}

// ─── Workspace discovery ──────────────────────────────────────────────────────

/// Find the workspace root by walking up from `start` looking for a
/// `krabkrab.toml`, `Cargo.toml`, or `.git` marker.
pub fn find_workspace_root(start: &Path) -> PathBuf {
    let mut current = start.to_path_buf();
    loop {
        if current.join("krabkrab.toml").exists()
            || current.join("Cargo.toml").exists()
            || current.join(".git").exists()
        {
            return current;
        }
        if !current.pop() {
            break;
        }
    }
    start.to_path_buf()
}

/// Returns the memory directory relative to the workspace root.
pub fn memory_dir(workspace: &Path) -> PathBuf {
    workspace.join("memory")
}

/// Returns the logs directory relative to the workspace root.
pub fn logs_dir(workspace: &Path) -> PathBuf {
    workspace.join("logs")
}

// ─── Env helpers ──────────────────────────────────────────────────────────────

/// Read an env var, returning `None` if missing or empty.
pub fn env_optional(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

/// Read an env var or return a default.
pub fn env_or(key: &str, default: &str) -> String {
    env_optional(key).unwrap_or_else(|| default.to_string())
}

/// Read an env var or `Err(...)`.
pub fn env_required(key: &str) -> Result<String, String> {
    env_optional(key).ok_or_else(|| format!("required env var `{}` is not set", key))
}

/// Returns `true` if the process is running inside a Docker container.
pub fn is_docker() -> bool {
    Path::new("/.dockerenv").exists()
        || std::env::var("DOCKER_CONTAINER").is_ok()
}

/// Returns `true` if `CI` env var is set (GitHub Actions, GitLab CI, etc.).
pub fn is_ci() -> bool {
    std::env::var("CI").is_ok()
}

// ─── Profile resolution ───────────────────────────────────────────────────────

/// The active profile name (`KRABKRAB_PROFILE` env, default "default").
pub fn active_profile() -> String {
    env_or("KRABKRAB_PROFILE", "default")
}

/// Path to the config file for the given profile.
pub fn profile_config_path(profile: &str) -> PathBuf {
    config_dir().join(format!("{}.toml", profile))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_dir_is_non_empty() {
        let h = home_dir();
        assert!(!h.as_os_str().is_empty());
    }

    #[test]
    fn config_dir_ends_with_krabkrab() {
        let d = config_dir();
        assert!(d.to_str().unwrap_or("").contains("krabkrab"));
    }

    #[test]
    fn env_optional_missing_returns_none() {
        assert!(env_optional("___KRABKRAB_NO_SUCH_VAR___").is_none());
    }

    #[test]
    fn env_or_fallback() {
        assert_eq!(env_or("___KRABKRAB_NO_SUCH_VAR___", "fallback"), "fallback");
    }

    #[test]
    fn env_required_missing_is_err() {
        assert!(env_required("___KRABKRAB_NO_SUCH_VAR___").is_err());
    }

    #[test]
    fn active_profile_default() {
        // Without env var set, should be "default"
        if std::env::var("KRABKRAB_PROFILE").is_err() {
            assert_eq!(active_profile(), "default");
        }
    }

    #[test]
    fn workspace_root_finds_cargo_toml() {
        // The krabkrab crate has Cargo.toml at its root
        let start = std::env::current_dir().unwrap();
        let root = find_workspace_root(&start);
        assert!(root.exists());
    }
}
