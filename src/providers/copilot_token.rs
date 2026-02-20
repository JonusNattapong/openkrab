//! providers::copilot_token — GitHub Copilot session token resolver.
//! Ported from `openkrab/src/providers/github-copilot-token.ts` (Phase 15).
//!
//! Fetches a short-lived Copilot API token from GitHub using a stored OAuth token,
//! with file-based caching and `proxy-ep=` base-URL derivation.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

pub const COPILOT_TOKEN_URL: &str = "https://api.github.com/copilot_internal/v2/token";
pub const DEFAULT_COPILOT_API_BASE_URL: &str = "https://api.individual.githubcopilot.com";
/// Safety margin: expire cache 5 minutes before actual expiry.
const EXPIRY_MARGIN_MS: u64 = 5 * 60 * 1_000;

// ─── Cached token ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCopilotToken {
    pub token: String,
    /// Unix timestamp in milliseconds.
    pub expires_at: u64,
    /// Unix timestamp in milliseconds when cache was written.
    pub updated_at: u64,
}

impl CachedCopilotToken {
    pub fn is_usable(&self) -> bool {
        let now = now_ms();
        self.expires_at > now + EXPIRY_MARGIN_MS
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ─── Token exchange response ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct TokenApiResponse {
    token: Option<String>,
    expires_at: Option<serde_json::Value>,
}

/// Parse the GitHub Copilot token endpoint response.
/// `expires_at` may be seconds (< 10_000_000_000) or milliseconds.
pub fn parse_token_response(value: &serde_json::Value) -> Result<(String, u64)> {
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("unexpected Copilot token response shape"))?;

    let token = obj
        .get("token")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Copilot token response missing token"))?;

    let expires_at_ms = match obj.get("expires_at") {
        Some(serde_json::Value::Number(n)) => {
            let raw = n
                .as_u64()
                .or_else(|| n.as_f64().map(|f| f as u64))
                .ok_or_else(|| anyhow::anyhow!("Copilot token: invalid expires_at number"))?;
            // GitHub sends seconds; defensively accept ms too
            if raw > 10_000_000_000 {
                raw
            } else {
                raw * 1000
            }
        }
        Some(serde_json::Value::String(s)) => {
            let raw: u64 = s
                .trim()
                .parse()
                .map_err(|_| anyhow::anyhow!("Copilot token: invalid expires_at string"))?;
            if raw > 10_000_000_000 {
                raw
            } else {
                raw * 1000
            }
        }
        _ => bail!("Copilot token response missing expires_at"),
    };

    Ok((token, expires_at_ms))
}

// ─── proxy-ep derivation ──────────────────────────────────────────────────────

/// Extract the `proxy-ep=` field from a Copilot token string and convert to an API base URL.
/// Token format: `tid=<id>;proxy-ep=<host>;...`
pub fn derive_api_base_url(token: &str) -> Option<String> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return None;
    }
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
            // pi-ai convention: proxy.* → api.*
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

// ─── Cache file helpers ───────────────────────────────────────────────────────

/// Load a cached token from a JSON file. Returns `None` if the file is missing or invalid.
pub fn load_cached_token(path: &std::path::Path) -> Option<CachedCopilotToken> {
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Save a token to a JSON cache file. Creates parent directories if needed.
pub fn save_cached_token(path: &std::path::Path, token: &CachedCopilotToken) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(token)?;
    std::fs::write(path, json)?;
    Ok(())
}

// ─── Resolved token ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ResolvedCopilotToken {
    pub token: String,
    pub expires_at: u64,
    pub base_url: String,
    pub source: String,
}

/// Resolve a Copilot API token: returns from cache if still valid, otherwise fetches fresh.
/// Uses the provided HTTP client (or a default) for network calls.
pub async fn resolve_copilot_token(
    github_token: &str,
    cache_path: &std::path::Path,
) -> Result<ResolvedCopilotToken> {
    // 1. Try cache
    if let Some(cached) = load_cached_token(cache_path) {
        if cached.is_usable() {
            let base_url = derive_api_base_url(&cached.token)
                .unwrap_or_else(|| DEFAULT_COPILOT_API_BASE_URL.to_string());
            return Ok(ResolvedCopilotToken {
                token: cached.token,
                expires_at: cached.expires_at,
                base_url,
                source: format!("cache:{}", cache_path.display()),
            });
        }
    }

    // 2. Fetch fresh token
    let client = reqwest::Client::new();
    let resp = client
        .get(COPILOT_TOKEN_URL)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", github_token))
        .send()
        .await?;

    if !resp.status().is_success() {
        bail!("Copilot token exchange failed: HTTP {}", resp.status());
    }

    let json: serde_json::Value = resp.json().await?;
    let (token, expires_at) = parse_token_response(&json)?;
    let base_url =
        derive_api_base_url(&token).unwrap_or_else(|| DEFAULT_COPILOT_API_BASE_URL.to_string());

    let cached = CachedCopilotToken {
        token: token.clone(),
        expires_at,
        updated_at: now_ms(),
    };
    // Best-effort cache write — ignore errors
    let _ = save_cached_token(cache_path, &cached);

    Ok(ResolvedCopilotToken {
        token,
        expires_at,
        base_url,
        source: format!("fetched:{}", COPILOT_TOKEN_URL),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_token_response_seconds() {
        let json = serde_json::json!({"token": "tid=abc;proxy-ep=proxy.example.com", "expires_at": 9999999999u64});
        let (tok, exp) = parse_token_response(&json).unwrap();
        assert!(tok.contains("proxy-ep"));
        // 9999999999 > 10_000_000_000? no, so * 1000
        assert_eq!(exp, 9_999_999_999_000);
    }

    #[test]
    fn parse_token_response_ms() {
        let json = serde_json::json!({"token": "t", "expires_at": 99999999999999u64});
        let (_, exp) = parse_token_response(&json).unwrap();
        assert_eq!(exp, 99_999_999_999_999); // already ms
    }

    #[test]
    fn parse_token_response_missing_token() {
        let json = serde_json::json!({"expires_at": 1234567890});
        assert!(parse_token_response(&json).is_err());
    }

    #[test]
    fn parse_token_response_string_expires() {
        let json = serde_json::json!({"token": "tok", "expires_at": "1234567890"});
        let (_, exp) = parse_token_response(&json).unwrap();
        assert_eq!(exp, 1_234_567_890_000); // seconds -> ms
    }

    #[test]
    fn derive_api_base_url_converts_proxy() {
        let tok = "tid=x;proxy-ep=proxy.individual.githubcopilot.com;extra=y";
        assert_eq!(
            derive_api_base_url(tok).unwrap(),
            "https://api.individual.githubcopilot.com"
        );
    }

    #[test]
    fn derive_api_base_url_no_proxy_ep() {
        assert!(derive_api_base_url("tid=x;other=y").is_none());
    }

    #[test]
    fn derive_api_base_url_empty() {
        assert!(derive_api_base_url("").is_none());
    }

    #[test]
    fn cached_token_usability() {
        let far_future = now_ms() + 60 * 60 * 1000; // 1 hour ahead
        let usable = CachedCopilotToken {
            token: "t".into(),
            expires_at: far_future,
            updated_at: now_ms(),
        };
        assert!(usable.is_usable());

        let expired = CachedCopilotToken {
            token: "t".into(),
            expires_at: 1,
            updated_at: 1,
        };
        assert!(!expired.is_usable());
    }
}
