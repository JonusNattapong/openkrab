//! providers::minimax_oauth — MiniMax Portal device-code OAuth.
//! Ported from `openkrab/extensions/minimax-portal-auth/oauth.ts` (Phase 15).
//!
//! MiniMax uses a device-code–style flow:
//!   1. Request a `user_code` + `verification_uri` from the code endpoint.
//!   2. Show the code to the user and open the verification URL.
//!   3. Poll the token endpoint with exponential back-off until approved or expired.

use anyhow::{bail, Result};
use base64::Engine;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ─── Region ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MiniMaxRegion {
    Cn,
    Global,
}

impl Default for MiniMaxRegion {
    fn default() -> Self {
        Self::Global
    }
}

impl MiniMaxRegion {
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::Cn => "https://api.minimaxi.com",
            Self::Global => "https://api.minimax.io",
        }
    }
    pub fn client_id(&self) -> &'static str {
        "78257093-7e40-4613-99e0-527b14b39113"
    }
    pub fn code_endpoint(&self) -> String {
        format!("{}/oauth/code", self.base_url())
    }
    pub fn token_endpoint(&self) -> String {
        format!("{}/oauth/token", self.base_url())
    }
}

// ─── OAuth constants ──────────────────────────────────────────────────────────

const SCOPE: &str = "group_id profile model.completion";
const GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:user_code";
const MAX_POLL_INTERVAL_MS: u64 = 10_000;

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniMaxAuthorization {
    pub user_code: String,
    pub verification_uri: String,
    pub expired_in: u64,
    pub interval: Option<u64>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniMaxToken {
    pub access: String,
    pub refresh: String,
    /// Unix timestamp ms when the token expires.
    pub expires: u64,
    pub resource_url: Option<String>,
    pub notification_message: Option<String>,
}

// ─── Internal poll response ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct TokenPollResponse {
    status: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expired_in: Option<u64>,
    resource_url: Option<String>,
    notification_message: Option<String>,
}

#[derive(Debug)]
pub enum PollResult {
    Success(MiniMaxToken),
    Pending { message: Option<String> },
    Error(String),
}

// ─── PKCE helpers (re-use oauth module types) ─────────────────────────────────

pub struct MiniMaxPkce {
    pub verifier: String,
    pub challenge: String,
    pub state: String,
}

impl MiniMaxPkce {
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let verifier = hex::encode(bytes);
        let challenge = base64url_sha256(verifier.as_bytes());
        let mut state_bytes = [0u8; 8];
        rand::thread_rng().fill_bytes(&mut state_bytes);
        let state = hex::encode(state_bytes);
        Self {
            verifier,
            challenge,
            state,
        }
    }
}

fn base64url_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(result)
}

fn form_encode(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

fn percent_encode(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_ascii_alphanumeric() || "-._~".contains(c) {
                vec![c]
            } else {
                c.to_string()
                    .bytes()
                    .flat_map(|b| format!("%{:02X}", b).chars().collect::<Vec<_>>())
                    .collect()
            }
        })
        .collect()
}

// ─── Sync API (no async for the pure logic) ───────────────────────────────────

/// Parse the code endpoint response.
pub fn parse_authorization(body: &str, expected_state: &str) -> Result<MiniMaxAuthorization> {
    #[derive(Deserialize)]
    struct Raw {
        user_code: Option<String>,
        verification_uri: Option<String>,
        expired_in: Option<u64>,
        interval: Option<u64>,
        state: Option<String>,
        error: Option<String>,
    }
    let raw: Raw = serde_json::from_str(body)?;
    if let Some(e) = raw.error {
        bail!("MiniMax OAuth authorization failed: {}", e);
    }
    let user_code = raw
        .user_code
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("missing user_code in MiniMax response"))?;
    let verification_uri = raw
        .verification_uri
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("missing verification_uri in MiniMax response"))?;
    let state = raw
        .state
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("missing state in MiniMax response"))?;
    if state != expected_state {
        bail!("MiniMax OAuth state mismatch: possible CSRF attack");
    }
    Ok(MiniMaxAuthorization {
        user_code,
        verification_uri,
        expired_in: raw.expired_in.unwrap_or(0),
        interval: raw.interval,
        state,
    })
}

/// Parse a token poll response.
pub fn parse_poll_result(body: &str) -> PollResult {
    let raw: TokenPollResponse = match serde_json::from_str(body) {
        Ok(r) => r,
        Err(e) => return PollResult::Error(format!("failed to parse: {}", e)),
    };
    match raw.status.as_deref() {
        Some("error") => PollResult::Error("An error occurred. Please try again later".into()),
        Some("success") => {
            let access = match raw.access_token.filter(|s| !s.is_empty()) {
                Some(t) => t,
                None => {
                    return PollResult::Error(
                        "MiniMax OAuth returned incomplete token payload".into(),
                    )
                }
            };
            let refresh = match raw.refresh_token.filter(|s| !s.is_empty()) {
                Some(t) => t,
                None => return PollResult::Error("MiniMax OAuth returned no refresh_token".into()),
            };
            let expires = raw.expired_in.unwrap_or(0);
            PollResult::Success(MiniMaxToken {
                access,
                refresh,
                expires,
                resource_url: raw.resource_url,
                notification_message: raw.notification_message,
            })
        }
        _ => PollResult::Pending {
            message: Some("current user code is not authorized".into()),
        },
    }
}

/// Build the form body for the code request.
pub fn build_code_request_body(challenge: &str, state: &str, region: MiniMaxRegion) -> String {
    form_encode(&[
        ("response_type", "code"),
        ("client_id", region.client_id()),
        ("scope", SCOPE),
        ("code_challenge", challenge),
        ("code_challenge_method", "S256"),
        ("state", state),
    ])
}

/// Build the form body for the token poll request.
pub fn build_token_poll_body(user_code: &str, verifier: &str, region: MiniMaxRegion) -> String {
    form_encode(&[
        ("grant_type", GRANT_TYPE),
        ("client_id", region.client_id()),
        ("user_code", user_code),
        ("code_verifier", verifier),
    ])
}

/// Compute next poll interval with exponential back-off capped at 10s.
pub fn next_poll_interval_ms(current_ms: u64) -> u64 {
    let next = (current_ms * 3 / 2).min(MAX_POLL_INTERVAL_MS);
    next.max(2_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_endpoints() {
        assert!(MiniMaxRegion::Cn.code_endpoint().contains("minimaxi.com"));
        assert!(MiniMaxRegion::Global
            .token_endpoint()
            .contains("minimax.io"));
    }

    #[test]
    fn parse_authorization_ok() {
        let body = r#"{"user_code":"ABC123","verification_uri":"https://example.com/verify","expired_in":1800,"state":"mystate"}"#;
        let auth = parse_authorization(body, "mystate").unwrap();
        assert_eq!(auth.user_code, "ABC123");
        assert_eq!(auth.verification_uri, "https://example.com/verify");
    }

    #[test]
    fn parse_authorization_state_mismatch() {
        let body = r#"{"user_code":"X","verification_uri":"https://x.com","expired_in":1800,"state":"wrongstate"}"#;
        assert!(parse_authorization(body, "expectedstate").is_err());
    }

    #[test]
    fn parse_poll_result_success() {
        let body =
            r#"{"status":"success","access_token":"acc","refresh_token":"ref","expired_in":3600}"#;
        assert!(matches!(parse_poll_result(body), PollResult::Success(_)));
    }

    #[test]
    fn parse_poll_result_pending() {
        let body = r#"{"status":"pending"}"#;
        assert!(matches!(
            parse_poll_result(body),
            PollResult::Pending { .. }
        ));
    }

    #[test]
    fn parse_poll_result_error() {
        let body = r#"{"status":"error"}"#;
        assert!(matches!(parse_poll_result(body), PollResult::Error(_)));
    }

    #[test]
    fn build_code_request_body_contains_params() {
        let body = build_code_request_body("chal", "st", MiniMaxRegion::Global);
        assert!(body.contains("code_challenge=chal"));
        assert!(body.contains("state=st"));
        assert!(body.contains("code_challenge_method=S256"));
    }

    #[test]
    fn next_poll_interval_backoff() {
        assert_eq!(next_poll_interval_ms(2000), 3000);
        assert_eq!(next_poll_interval_ms(8000), 10_000);
        assert_eq!(next_poll_interval_ms(100), 2000); // min floor
    }
}
