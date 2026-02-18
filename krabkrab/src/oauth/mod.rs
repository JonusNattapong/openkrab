//! oauth — Generic OAuth 2.0 PKCE helper.
//! Ported from `openclaw/extensions/google-antigravity-auth/index.ts` (Phase 14).
//!
//! Provides PKCE (Proof Key for Code Exchange) helpers, authorization URL builder,
//! callback URL parser, and token exchange — usable for Google, MiniMax, Qwen etc.

use anyhow::{bail, Result};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use rand::RngCore;
use base64::Engine;

// ─── PKCE ─────────────────────────────────────────────────────────────────────

/// A PKCE (S256) code verifier + challenge pair.
#[derive(Debug, Clone)]
pub struct PkceChallenge {
    pub verifier: String,
    pub challenge: String, // Base64url-encoded SHA-256 of verifier
}

impl PkceChallenge {
    /// Generate a new random PKCE pair.
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let verifier = hex::encode(bytes);
        let challenge = base64url_sha256(verifier.as_bytes());
        Self { verifier, challenge }
    }

    /// Generate with explicit verifier bytes (for tests / deterministic flows).
    pub fn from_verifier(verifier: impl Into<String>) -> Self {
        let v = verifier.into();
        let challenge = base64url_sha256(v.as_bytes());
        Self { verifier: v, challenge }
    }
}

fn base64url_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(result)
}

// ─── OAuth config ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
}

// ─── Auth URL ─────────────────────────────────────────────────────────────────

/// Build an OAuth 2.0 authorization URL with PKCE (S256).
pub fn build_auth_url(
    cfg: &OAuthClientConfig,
    challenge: &str,
    state: &str,
    extra_params: &[(&str, &str)],
) -> String {
    let scope = cfg.scopes.join(" ");
    let mut params = vec![
        ("client_id", cfg.client_id.as_str()),
        ("response_type", "code"),
        ("redirect_uri", cfg.redirect_uri.as_str()),
        ("scope", &scope),
        ("code_challenge", challenge),
        ("code_challenge_method", "S256"),
        ("state", state),
        ("access_type", "offline"),
        ("prompt", "consent"),
    ];
    params.extend_from_slice(extra_params);

    let query: String = params.iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    format!("{}?{}", cfg.auth_url, query)
}

fn url_encode(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_ascii_alphanumeric() || "-._~".contains(c) {
                vec![c]
            } else {
                // percent-encode
                c.to_string()
                    .bytes()
                    .flat_map(|b| format!("%{:02X}", b).chars().collect::<Vec<_>>())
                    .collect()
            }
        })
        .collect()
}

// ─── Callback URL parsing ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OAuthCallback {
    pub code: String,
    pub state: String,
}

/// Parse a redirect callback URL (e.g. http://localhost:51121/oauth-callback?code=...&state=...).
pub fn parse_callback_url(url_str: &str) -> Result<OAuthCallback> {
    let trimmed = url_str.trim();
    if trimmed.is_empty() {
        bail!("No input provided");
    }

    // Parse query string
    let query_start = trimmed.find('?').ok_or_else(|| anyhow::anyhow!("Missing query string in callback URL"))?;
    let query = &trimmed[query_start + 1..];

    let mut code = None;
    let mut state = None;

    for pair in query.split('&') {
        let mut kv = pair.splitn(2, '=');
        let key = kv.next().unwrap_or("").trim();
        let val = url_decode(kv.next().unwrap_or("").trim());
        match key {
            "code" => code = Some(val),
            "state" => state = Some(val),
            _ => {}
        }
    }

    let code = code.filter(|s| !s.is_empty()).ok_or_else(|| anyhow::anyhow!("Missing 'code' parameter in URL"))?;
    let state = state.filter(|s| !s.is_empty()).ok_or_else(|| anyhow::anyhow!("Missing 'state' parameter in URL"))?;

    Ok(OAuthCallback { code, state })
}

fn url_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = std::str::from_utf8(&bytes[i+1..i+3]) {
                if let Ok(b) = u8::from_str_radix(hex, 16) {
                    out.push(b as char);
                    i += 3;
                    continue;
                }
            }
        } else if bytes[i] == b'+' {
            out.push(' ');
            i += 1;
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

// ─── Token types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    /// Unix timestamp (ms) when access_token expires (pre-buffered by 5 min).
    pub expires_at_ms: u64,
    pub email: Option<String>,
}

impl OAuthTokens {
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        now >= self.expires_at_ms
    }
}

// ─── Token exchange response ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TokenExchangeResponse {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
}

impl TokenExchangeResponse {
    pub fn into_tokens(self) -> Result<OAuthTokens> {
        let access = self.access_token
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Token exchange returned no access_token"))?;
        let refresh = self.refresh_token
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Token exchange returned no refresh_token"))?;

        let expires_in = self.expires_in.unwrap_or(0);
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        // Buffer 5 minutes early
        let expires_at_ms = now_ms + expires_in * 1000 - 5 * 60 * 1000;

        Ok(OAuthTokens { access_token: access, refresh_token: refresh, expires_at_ms, email: None })
    }
}

// ─── Async helpers ────────────────────────────────────────────────────────────

/// Exchange an authorization code for tokens.
pub async fn exchange_code(
    client: &reqwest::Client,
    cfg: &OAuthClientConfig,
    code: &str,
    verifier: &str,
) -> Result<OAuthTokens> {
    let params = [
        ("client_id", cfg.client_id.as_str()),
        ("client_secret", cfg.client_secret.as_str()),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", cfg.redirect_uri.as_str()),
        ("code_verifier", verifier),
    ];
    let resp: TokenExchangeResponse = client
        .post(&cfg.token_url)
        .form(&params)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    resp.into_tokens()
}

/// Refresh an access token using a refresh token.
pub async fn refresh_token(
    client: &reqwest::Client,
    cfg: &OAuthClientConfig,
    refresh: &str,
) -> Result<OAuthTokens> {
    let params = [
        ("client_id", cfg.client_id.as_str()),
        ("client_secret", cfg.client_secret.as_str()),
        ("refresh_token", refresh),
        ("grant_type", "refresh_token"),
    ];
    let resp: TokenExchangeResponse = client
        .post(&cfg.token_url)
        .form(&params)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    resp.into_tokens()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pkce_from_verifier() {
        let p = PkceChallenge::from_verifier("abc123");
        assert!(!p.challenge.is_empty());
        assert_ne!(p.verifier, p.challenge);
    }

    #[test]
    fn build_auth_url_contains_params() {
        let cfg = OAuthClientConfig {
            client_id: "cid".into(),
            client_secret: "secret".into(),
            redirect_uri: "http://localhost/cb".into(),
            auth_url: "https://auth.example.com/oauth".into(),
            token_url: "https://auth.example.com/token".into(),
            scopes: vec!["openid".into(), "email".into()],
        };
        let url = build_auth_url(&cfg, "chal123", "state456", &[]);
        assert!(url.contains("client_id=cid"));
        assert!(url.contains("code_challenge=chal123"));
        assert!(url.contains("state=state456"));
        assert!(url.contains("code_challenge_method=S256"));
    }

    #[test]
    fn parse_callback_url_ok() {
        let url = "http://localhost:51121/cb?code=auth_code_abc&state=xyz123";
        let cb = parse_callback_url(url).unwrap();
        assert_eq!(cb.code, "auth_code_abc");
        assert_eq!(cb.state, "xyz123");
    }

    #[test]
    fn parse_callback_url_missing_code() {
        let url = "http://localhost:51121/cb?state=xyz";
        assert!(parse_callback_url(url).is_err());
    }

    #[test]
    fn parse_callback_url_missing_state() {
        let url = "http://localhost:51121/cb?code=abc";
        assert!(parse_callback_url(url).is_err());
    }

    #[test]
    fn parse_callback_url_empty() {
        assert!(parse_callback_url("").is_err());
    }

    #[test]
    fn oauth_tokens_expiry() {
        let tokens = OAuthTokens {
            access_token: "tok".into(),
            refresh_token: "ref".into(),
            expires_at_ms: 1, // far in the past
            email: None,
        };
        assert!(tokens.is_expired());
    }

    #[test]
    fn oauth_tokens_not_expired() {
        let future = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
            + 3600 * 1000;
        let tokens = OAuthTokens {
            access_token: "tok".into(),
            refresh_token: "ref".into(),
            expires_at_ms: future,
            email: None,
        };
        assert!(!tokens.is_expired());
    }

    #[test]
    fn token_exchange_response_missing_access() {
        let resp = TokenExchangeResponse {
            access_token: None, refresh_token: Some("r".into()), expires_in: Some(3600),
        };
        assert!(resp.into_tokens().is_err());
    }

    #[test]
    fn token_exchange_response_missing_refresh() {
        let resp = TokenExchangeResponse {
            access_token: Some("a".into()), refresh_token: None, expires_in: Some(3600),
        };
        assert!(resp.into_tokens().is_err());
    }

    #[test]
    fn pkce_generate_random() {
        let p1 = PkceChallenge::generate();
        let p2 = PkceChallenge::generate();
        assert_ne!(p1.verifier, p2.verifier);
        assert_ne!(p1.challenge, p2.challenge);
        assert_eq!(p1.verifier.len(), 64);
    }

    #[test]
    fn pkce_from_verifier_deterministic() {
        let p = PkceChallenge::from_verifier("test_verifier_12345");
        assert_eq!(p.verifier, "test_verifier_12345");
        assert!(!p.challenge.is_empty());
    }
}
