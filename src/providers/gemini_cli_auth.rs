//! providers::gemini_cli_auth — Gemini CLI credential extractor.
//! Ported from `openclaw/extensions/google-gemini-cli-auth/oauth.ts` (Phase 15).
//!
//! Locates the installed `gemini` binary, finds the bundled `oauth2.js` file,
//! and regex-extracts the embedded OAuth client_id / client_secret.
//! Falls back to env-var overrides if the CLI is not installed.

use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

// ─── Env-var keys ─────────────────────────────────────────────────────────────

const CLIENT_ID_KEYS: &[&str] = &[
    "KRABKRAB_GEMINI_OAUTH_CLIENT_ID",
    "GEMINI_CLI_OAUTH_CLIENT_ID",
];
const CLIENT_SECRET_KEYS: &[&str] = &[
    "KRABKRAB_GEMINI_OAUTH_CLIENT_SECRET",
    "GEMINI_CLI_OAUTH_CLIENT_SECRET",
];

// ─── Public types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct GeminiCliCredentials {
    pub client_id: String,
    pub client_secret: Option<String>,
}

/// Project tier used when provisioning or discovering a GCP project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeminiTier {
    Free,
    Legacy,
    Standard,
}

impl GeminiTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Free => "free-tier",
            Self::Legacy => "legacy-tier",
            Self::Standard => "standard-tier",
        }
    }
}

// ─── Env-var resolution ───────────────────────────────────────────────────────

fn resolve_env(keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Ok(val) = std::env::var(key) {
            let v = val.trim().to_string();
            if !v.is_empty() {
                return Some(v);
            }
        }
    }
    None
}

// ─── PATH search ─────────────────────────────────────────────────────────────

/// Find the `gemini` binary on PATH (tries common extensions on Windows).
pub fn find_in_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    let exts: &[&str] = if cfg!(windows) {
        &[".cmd", ".bat", ".exe", ""]
    } else {
        &[""]
    };
    for dir in std::env::split_paths(&path_var) {
        for ext in exts {
            let candidate = dir.join(format!("{}{}", name, ext));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

/// Walk a directory tree up to `max_depth` looking for a file named `name`.
pub fn find_file_recursive(dir: &Path, name: &str, max_depth: usize) -> Option<PathBuf> {
    if max_depth == 0 {
        return None;
    }
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if fname.starts_with('.') {
            continue;
        }
        if path.is_file() && fname == name {
            return Some(path);
        }
        if path.is_dir() {
            if let Some(found) = find_file_recursive(&path, name, max_depth - 1) {
                return Some(found);
            }
        }
    }
    None
}

// ─── Credential extraction ────────────────────────────────────────────────────

/// Regex-extract the client_id and client_secret from the content of `oauth2.js`.
pub fn extract_credentials_from_js(content: &str) -> Option<GeminiCliCredentials> {
    // Google OAuth2 client IDs follow the pattern: <digits>-<alphanumeric>.apps.googleusercontent.com
    let id_re = regex_find(content, r"\d+-[a-z0-9]+\.apps\.googleusercontent\.com");
    // Google OAuth2 client secrets start with GOCSPX-
    let secret_re = regex_find(content, r"GOCSPX-[A-Za-z0-9_-]+");

    id_re.map(|client_id| GeminiCliCredentials {
        client_id,
        client_secret: secret_re,
    })
}

/// Minimal regex-find without the `regex` crate dependency.
/// Searches for the pattern using simple state-machine matching for the two known shapes.
fn regex_find(haystack: &str, pattern: &str) -> Option<String> {
    if pattern == r"\d+-[a-z0-9]+\.apps\.googleusercontent\.com" {
        // Match: <digits>-<alnum>.apps.googleusercontent.com
        let suffix = ".apps.googleusercontent.com";
        if let Some(pos) = haystack.find(suffix) {
            // Walk backwards to find the start of the match
            let before = &haystack[..pos];
            let start = before
                .rfind(|c: char| !c.is_ascii_alphanumeric() && c != '-')
                .map(|i| i + 1)
                .unwrap_or(0);
            let token = &haystack[start..pos + suffix.len()];
            // Must have at least one digit followed by '-'
            if token.contains('-')
                && token
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
            {
                return Some(token.to_string());
            }
        }
        return None;
    }
    if pattern == r"GOCSPX-[A-Za-z0-9_-]+" {
        let prefix = "GOCSPX-";
        if let Some(pos) = haystack.find(prefix) {
            let rest = &haystack[pos..];
            let end = rest
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                .unwrap_or(rest.len());
            return Some(rest[..end].to_string());
        }
        return None;
    }
    None
}

// ─── High-level resolve ───────────────────────────────────────────────────────

/// Resolve Gemini CLI OAuth credentials with fallback priority:
/// 1. Environment variable overrides
/// 2. Extraction from installed Gemini CLI
pub fn resolve_gemini_credentials() -> Result<GeminiCliCredentials> {
    // 1. Env-var overrides
    if let Some(client_id) = resolve_env(CLIENT_ID_KEYS) {
        let client_secret = resolve_env(CLIENT_SECRET_KEYS);
        return Ok(GeminiCliCredentials {
            client_id,
            client_secret,
        });
    }

    // 2. Extract from installed Gemini CLI
    if let Some(creds) = extract_from_installed_cli() {
        return Ok(creds);
    }

    bail!(
        "Gemini CLI not found. Install it first: \
         brew install gemini-cli (or npm install -g @google/gemini-cli), \
         or set GEMINI_CLI_OAUTH_CLIENT_ID."
    );
}

fn extract_from_installed_cli() -> Option<GeminiCliCredentials> {
    let gemini_path = find_in_path("gemini")?;
    // Resolve symlink if needed
    let resolved = std::fs::canonicalize(&gemini_path).unwrap_or(gemini_path);
    let gemini_cli_dir = resolved.parent()?.parent()?;

    let search_paths = [
        gemini_cli_dir.join("node_modules/@google/gemini-cli-core/dist/src/code_assist/oauth2.js"),
        gemini_cli_dir.join("node_modules/@google/gemini-cli-core/dist/code_assist/oauth2.js"),
    ];

    for p in &search_paths {
        if p.is_file() {
            if let Ok(content) = std::fs::read_to_string(p) {
                if let Some(creds) = extract_credentials_from_js(&content) {
                    return Some(creds);
                }
            }
        }
    }

    // Fallback: recursive search
    if let Some(found) = find_file_recursive(gemini_cli_dir, "oauth2.js", 10) {
        if let Ok(content) = std::fs::read_to_string(found) {
            return extract_credentials_from_js(&content);
        }
    }
    None
}

// ─── VPC-SC check ─────────────────────────────────────────────────────────────

/// Check if a JSON error payload is from a VPC Service Controls policy violation.
pub fn is_vpc_sc_affected(payload: &serde_json::Value) -> bool {
    let details = payload
        .get("error")
        .and_then(|e| e.get("details"))
        .and_then(|d| d.as_array());
    if let Some(arr) = details {
        return arr.iter().any(|item| {
            item.get("reason").and_then(|r| r.as_str()) == Some("SECURITY_POLICY_VIOLATED")
        });
    }
    false
}

// ─── Copilot-endpoint token parsing (re-export shim) ─────────────────────────

/// Parse a Copilot-style semicolon-delimited token to find the `proxy-ep=` value.
pub fn derive_api_base_url_from_token(token: &str) -> Option<String> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Find `proxy-ep=<value>`
    for part in trimmed.split(';') {
        let part = part.trim();
        if let Some(rest) = part.strip_prefix("proxy-ep=") {
            let host = rest
                .trim()
                .trim_start_matches("https://")
                .trim_start_matches("http://");
            if host.is_empty() {
                return None;
            }
            // Convert proxy.* -> api.*
            let api_host = if let Some(s) = host.strip_prefix("proxy.") {
                format!("api.{}", s)
            } else {
                host.to_string()
            };
            return Some(format!("https://{}", api_host));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_client_id() {
        let js = r#"const CLIENT_ID = '123456789-abcdefg.apps.googleusercontent.com';"#;
        let creds = extract_credentials_from_js(js).unwrap();
        assert!(creds.client_id.ends_with(".apps.googleusercontent.com"));
    }

    #[test]
    fn extract_client_secret() {
        let js = r#"const SECRET = 'GOCSPX-fakesecret123';"#;
        let creds = extract_credentials_from_js(js);
        // secret only, no client_id
        assert!(creds.is_none()); // no client_id -> returns None
    }

    #[test]
    fn extract_both() {
        let js = "client_id: '1234-abc.apps.googleusercontent.com', secret: 'GOCSPX-fakesecret456'";
        let creds = extract_credentials_from_js(js).unwrap();
        assert!(creds.client_id.contains("googleusercontent.com"));
        assert_eq!(creds.client_secret.as_deref(), Some("GOCSPX-fakesecret456"));
    }

    #[test]
    fn is_vpc_sc_affected_true() {
        let payload = serde_json::json!({
            "error": {"details": [{"reason": "SECURITY_POLICY_VIOLATED"}]}
        });
        assert!(is_vpc_sc_affected(&payload));
    }

    #[test]
    fn is_vpc_sc_affected_false() {
        let payload = serde_json::json!({"error": {"details": [{"reason": "OTHER"}]}});
        assert!(!is_vpc_sc_affected(&payload));
    }

    #[test]
    fn derive_api_base_url_proxy() {
        let token = "tid=x;proxy-ep=proxy.individual.githubcopilot.com;other=y";
        let url = derive_api_base_url_from_token(token).unwrap();
        assert_eq!(url, "https://api.individual.githubcopilot.com");
    }

    #[test]
    fn derive_api_base_url_no_proxy() {
        assert!(derive_api_base_url_from_token("tid=x;other=y").is_none());
    }

    #[test]
    fn derive_api_base_url_empty() {
        assert!(derive_api_base_url_from_token("").is_none());
    }
}
