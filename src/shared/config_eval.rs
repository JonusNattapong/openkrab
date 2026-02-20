//! Port of `openkrab/src/shared/config-eval.ts`
//!
//! Configuration evaluation utilities: truthiness checks, dot-path resolution,
//! runtime requirement evaluation, and binary lookup on `$PATH`.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

// ─── Truthiness ──────────────────────────────────────────────────────────────

/// Check whether a JSON value is "truthy" using JavaScript-like semantics.
pub fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i != 0
            } else if let Some(f) = n.as_f64() {
                f != 0.0
            } else {
                true
            }
        }
        Value::String(s) => !s.trim().is_empty(),
        _ => true, // objects and arrays are truthy
    }
}

// ─── Dot-path resolution ────────────────────────────────────────────────────

/// Resolve a dot-separated path (`"a.b.c"`) into a nested [`Value`].
///
/// Returns `None` if any intermediate segment is not an object or the final key
/// does not exist.
pub fn resolve_config_path<'a>(config: &'a Value, path_str: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = path_str.split('.').filter(|s| !s.is_empty()).collect();
    let mut current = config;
    for part in parts {
        match current.as_object() {
            Some(obj) => match obj.get(part) {
                Some(v) => current = v,
                None => return None,
            },
            None => return None,
        }
    }
    Some(current)
}

/// Check whether a config path is truthy, falling back to `defaults` when the
/// path is absent.
pub fn is_config_path_truthy_with_defaults(
    config: &Value,
    path_str: &str,
    defaults: &HashMap<String, bool>,
) -> bool {
    match resolve_config_path(config, path_str) {
        Some(v) => is_truthy(v),
        None => defaults.get(path_str).copied().unwrap_or(false),
    }
}

// ─── Runtime requirements ────────────────────────────────────────────────────

/// A set of requirements that must be satisfied for a feature to be enabled.
#[derive(Debug, Default, Clone)]
pub struct RuntimeRequires {
    /// All of these binaries must be present.
    pub bins: Vec<String>,
    /// At least one of these binaries must be present.
    pub any_bins: Vec<String>,
    /// All of these environment variables must be set.
    pub env: Vec<String>,
    /// All of these config paths must be truthy.
    pub config: Vec<String>,
}

/// Callbacks used by [`evaluate_runtime_requires`] to test the environment.
pub struct RuntimeRequiresCallbacks<'a> {
    pub has_bin: &'a dyn Fn(&str) -> bool,
    pub has_any_remote_bin: Option<&'a dyn Fn(&[String]) -> bool>,
    pub has_remote_bin: Option<&'a dyn Fn(&str) -> bool>,
    pub has_env: &'a dyn Fn(&str) -> bool,
    pub is_config_path_truthy: &'a dyn Fn(&str) -> bool,
}

/// Evaluate whether all runtime requirements are satisfied.
pub fn evaluate_runtime_requires(
    requires: Option<&RuntimeRequires>,
    cbs: &RuntimeRequiresCallbacks,
) -> bool {
    let requires = match requires {
        Some(r) => r,
        None => return true,
    };

    // required bins — all must be present (locally or remotely)
    for bin in &requires.bins {
        if (cbs.has_bin)(bin) {
            continue;
        }
        if let Some(remote) = cbs.has_remote_bin {
            if remote(bin) {
                continue;
            }
        }
        return false;
    }

    // any_bins — at least one must be present
    if !requires.any_bins.is_empty() {
        let any_found = requires.any_bins.iter().any(|b| (cbs.has_bin)(b));
        if !any_found {
            if let Some(remote) = cbs.has_any_remote_bin {
                if !remote(&requires.any_bins) {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    // env — all must be set
    for env_name in &requires.env {
        if !(cbs.has_env)(env_name) {
            return false;
        }
    }

    // config — all must be truthy
    for config_path in &requires.config {
        if !(cbs.is_config_path_truthy)(config_path) {
            return false;
        }
    }

    true
}

// ─── Platform detection ──────────────────────────────────────────────────────

/// Return the current platform string (mirrors `process.platform`).
pub fn resolve_runtime_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "win32"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

// ─── Binary lookup ──────────────────────────────────────────────────────────

fn windows_path_extensions() -> Vec<String> {
    let raw = env::var("PATHEXT").unwrap_or_default();
    let list: Vec<String> = if raw.is_empty() {
        vec![".EXE", ".CMD", ".BAT", ".COM"]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        raw.split(';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };
    let mut exts = vec![String::new()]; // bare name first
    exts.extend(list);
    exts
}

/// Check whether a binary named `bin` is reachable via `$PATH`.
///
/// On Windows, also checks `%PATHEXT%` extensions (`.EXE`, `.CMD`, …).
pub fn has_binary(bin: &str) -> bool {
    let path_env = env::var("PATH").unwrap_or_default();
    let parts: Vec<&str> = path_env
        .split(if cfg!(windows) { ';' } else { ':' })
        .filter(|s| !s.is_empty())
        .collect();

    let extensions: Vec<String> = if cfg!(windows) {
        windows_path_extensions()
    } else {
        vec![String::new()]
    };

    for dir in &parts {
        for ext in &extensions {
            let candidate: PathBuf = Path::new(dir).join(format!("{}{}", bin, ext));
            if candidate.exists() {
                // On Unix additionally check execute permission
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(meta) = fs::metadata(&candidate) {
                        if meta.permissions().mode() & 0o111 != 0 {
                            return true;
                        }
                    }
                    continue;
                }
                #[cfg(not(unix))]
                {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ── is_truthy ────────────────────────────────────────────────────────

    #[test]
    fn null_is_falsy() {
        assert!(!is_truthy(&json!(null)));
    }

    #[test]
    fn false_is_falsy() {
        assert!(!is_truthy(&json!(false)));
    }

    #[test]
    fn true_is_truthy() {
        assert!(is_truthy(&json!(true)));
    }

    #[test]
    fn zero_is_falsy() {
        assert!(!is_truthy(&json!(0)));
    }

    #[test]
    fn nonzero_is_truthy() {
        assert!(is_truthy(&json!(42)));
    }

    #[test]
    fn empty_string_is_falsy() {
        assert!(!is_truthy(&json!("   ")));
    }

    #[test]
    fn nonempty_string_is_truthy() {
        assert!(is_truthy(&json!("hello")));
    }

    #[test]
    fn object_is_truthy() {
        assert!(is_truthy(&json!({})));
    }

    #[test]
    fn array_is_truthy() {
        assert!(is_truthy(&json!([])));
    }

    // ── resolve_config_path ──────────────────────────────────────────────

    #[test]
    fn resolves_nested_path() {
        let cfg = json!({"a": {"b": {"c": 42}}});
        assert_eq!(resolve_config_path(&cfg, "a.b.c"), Some(&json!(42)));
    }

    #[test]
    fn returns_none_for_missing() {
        let cfg = json!({"a": 1});
        assert_eq!(resolve_config_path(&cfg, "a.b"), None);
    }

    // ── evaluate_runtime_requires ────────────────────────────────────────

    #[test]
    fn none_requirements_satisfied() {
        let cbs = RuntimeRequiresCallbacks {
            has_bin: &|_| false,
            has_any_remote_bin: None,
            has_remote_bin: None,
            has_env: &|_| false,
            is_config_path_truthy: &|_| false,
        };
        assert!(evaluate_runtime_requires(None, &cbs));
    }

    #[test]
    fn missing_bin_fails() {
        let req = RuntimeRequires {
            bins: vec!["git".to_string()],
            ..Default::default()
        };
        let cbs = RuntimeRequiresCallbacks {
            has_bin: &|_| false,
            has_any_remote_bin: None,
            has_remote_bin: None,
            has_env: &|_| true,
            is_config_path_truthy: &|_| true,
        };
        assert!(!evaluate_runtime_requires(Some(&req), &cbs));
    }

    #[test]
    fn any_bins_one_found() {
        let req = RuntimeRequires {
            any_bins: vec!["curl".to_string(), "wget".to_string()],
            ..Default::default()
        };
        let cbs = RuntimeRequiresCallbacks {
            has_bin: &|b| b == "wget",
            has_any_remote_bin: None,
            has_remote_bin: None,
            has_env: &|_| true,
            is_config_path_truthy: &|_| true,
        };
        assert!(evaluate_runtime_requires(Some(&req), &cbs));
    }

    // ── resolve_runtime_platform ────────────────────────────────────────

    #[test]
    fn platform_is_not_empty() {
        let plat = resolve_runtime_platform();
        assert!(!plat.is_empty());
    }
}
