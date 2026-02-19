//! providers::qwen_oauth — Qwen Portal OAuth token refresh.
//! Ported from `openclaw/src/providers/qwen-portal-oauth.ts` (Phase 16).
//!
//! Refreshes a Qwen Portal access token using a stored refresh token.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

pub const QWEN_OAUTH_BASE_URL: &str = "https://chat.qwen.ai";
pub const QWEN_OAUTH_TOKEN_ENDPOINT: &str = "https://chat.qwen.ai/api/v1/oauth2/token";
pub const QWEN_OAUTH_CLIENT_ID: &str = "f0304373b74a44d2b584a3fb70ca9e56";

// ─── Credential types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QwenCredentials {
    pub access: String,
    pub refresh: Option<String>,
    /// Unix timestamp ms when the access token expires.
    pub expires: u64,
}

impl QwenCredentials {
    pub fn is_expired(&self) -> bool {
        let now = now_ms();
        now >= self.expires
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ─── Refresh response ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct QwenRefreshResponse {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
}

impl QwenRefreshResponse {
    pub fn into_credentials(self, old: &QwenCredentials) -> Result<QwenCredentials> {
        let access = self
            .access_token
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Qwen OAuth refresh response missing access token"))?;
        let expires_in = self
            .expires_in
            .ok_or_else(|| anyhow::anyhow!("Qwen OAuth refresh response missing expires_in"))?;
        let expires = now_ms() + expires_in * 1000;
        let refresh = self
            .refresh_token
            .filter(|s| !s.is_empty())
            .or_else(|| old.refresh.clone());
        Ok(QwenCredentials {
            access,
            refresh,
            expires,
        })
    }
}

// ─── Form body builder ────────────────────────────────────────────────────────

/// Build the URL-encoded form body for the refresh token request.
pub fn build_refresh_body(refresh_token: &str) -> String {
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", QWEN_OAUTH_CLIENT_ID),
    ];
    params
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

/// Parse a Qwen token refresh response body (JSON string → `QwenRefreshResponse`).
pub fn parse_refresh_response(body: &str) -> Result<QwenRefreshResponse> {
    Ok(serde_json::from_str(body)?)
}

/// Validate that credentials have a refresh token before attempting a refresh.
pub fn validate_has_refresh(creds: &QwenCredentials) -> Result<&str> {
    match creds.refresh.as_deref().filter(|s| !s.trim().is_empty()) {
        Some(r) => Ok(r),
        None => bail!("Qwen OAuth refresh token missing; re-authenticate."),
    }
}

/// Async token refresh using `reqwest`.
pub async fn refresh_qwen_credentials(
    client: &reqwest::Client,
    old: &QwenCredentials,
) -> Result<QwenCredentials> {
    let refresh_token = validate_has_refresh(old)?;
    let body = build_refresh_body(refresh_token);

    let resp = client
        .post(QWEN_OAUTH_TOKEN_ENDPOINT)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .body(body)
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        if status.as_u16() == 400 {
            bail!(
                "Qwen OAuth refresh token expired or invalid. \
                 Re-authenticate with `krabkrab models auth login --provider qwen-portal`."
            );
        }
        bail!("Qwen OAuth refresh failed: {}", text.trim());
    }

    let payload: QwenRefreshResponse = resp.json().await?;
    payload.into_credentials(old)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_refresh_body_contains_params() {
        let body = build_refresh_body("ref_token_abc");
        assert!(body.contains("grant_type=refresh_token"));
        assert!(body.contains("refresh_token=ref_token_abc"));
        assert!(body.contains("client_id=f0304373b74a44d2b584a3fb70ca9e56"));
    }

    #[test]
    fn parse_refresh_response_ok() {
        let json = r#"{"access_token":"new_acc","refresh_token":"new_ref","expires_in":3600}"#;
        let r = parse_refresh_response(json).unwrap();
        assert_eq!(r.access_token.as_deref(), Some("new_acc"));
        assert_eq!(r.expires_in, Some(3600));
    }

    #[test]
    fn into_credentials_ok() {
        let old = QwenCredentials {
            access: "old".into(),
            refresh: Some("old_ref".into()),
            expires: 0,
        };
        let resp = QwenRefreshResponse {
            access_token: Some("new".into()),
            refresh_token: None, // keep old refresh
            expires_in: Some(3600),
        };
        let new = resp.into_credentials(&old).unwrap();
        assert_eq!(new.access, "new");
        assert_eq!(new.refresh.as_deref(), Some("old_ref")); // preserved from old
    }

    #[test]
    fn into_credentials_missing_access() {
        let old = QwenCredentials {
            access: "x".into(),
            refresh: None,
            expires: 0,
        };
        let resp = QwenRefreshResponse {
            access_token: None,
            refresh_token: None,
            expires_in: Some(3600),
        };
        assert!(resp.into_credentials(&old).is_err());
    }

    #[test]
    fn validate_has_refresh_ok() {
        let c = QwenCredentials {
            access: "a".into(),
            refresh: Some("r".into()),
            expires: 0,
        };
        assert!(validate_has_refresh(&c).is_ok());
    }

    #[test]
    fn validate_has_refresh_missing() {
        let c = QwenCredentials {
            access: "a".into(),
            refresh: None,
            expires: 0,
        };
        assert!(validate_has_refresh(&c).is_err());
    }

    #[test]
    fn qwen_credentials_expiry() {
        let expired = QwenCredentials {
            access: "a".into(),
            refresh: None,
            expires: 1,
        };
        assert!(expired.is_expired());
        let future = QwenCredentials {
            access: "a".into(),
            refresh: None,
            expires: now_ms() + 60_000,
        };
        assert!(!future.is_expired());
    }
}
