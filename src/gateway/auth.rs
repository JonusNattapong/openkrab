use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication modes supported by the gateway
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthMode {
    None,
    Token,
    Password,
    TrustedProxy,
}

/// Resolved authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedAuth {
    pub mode: AuthMode,
    pub token: Option<String>,
    pub password: Option<String>,
    pub allow_tailscale: bool,
    pub trusted_proxy: Option<TrustedProxyConfig>,
}

/// Trusted proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedProxyConfig {
    pub networks: Vec<String>,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub ok: bool,
    pub method: Option<String>,
    pub user: Option<String>,
    pub reason: Option<String>,
    pub rate_limited: Option<bool>,
    pub retry_after_ms: Option<u64>,
}

/// Authentication context for a request
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub client_ip: Option<String>,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            client_ip: None,
            headers: HashMap::new(),
            query_params: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }

    pub fn with_client_ip(mut self, ip: impl Into<String>) -> Self {
        self.client_ip = Some(ip.into());
        self
    }
}

/// Authenticator trait for different authentication methods
pub trait Authenticator: Send + Sync {
    fn authenticate(&self, ctx: &AuthContext) -> AuthResult;
}

/// No authentication - allows all requests
pub struct NoAuth;

impl Authenticator for NoAuth {
    fn authenticate(&self, _ctx: &AuthContext) -> AuthResult {
        AuthResult {
            ok: true,
            method: Some("none".to_string()),
            user: None,
            reason: None,
            rate_limited: None,
            retry_after_ms: None,
        }
    }
}

/// Token-based authentication
pub struct TokenAuth {
    token: String,
}

impl TokenAuth {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Authenticator for TokenAuth {
    fn authenticate(&self, ctx: &AuthContext) -> AuthResult {
        let auth_header = ctx.headers.get("authorization");
        let token_param = ctx.query_params.get("token");

        let provided_token = auth_header
            .and_then(|h| h.strip_prefix("Bearer "))
            .or_else(|| token_param.as_deref());

        match provided_token {
            Some(token) if token == self.token => AuthResult {
                ok: true,
                method: Some("token".to_string()),
                user: None,
                reason: None,
                rate_limited: None,
                retry_after_ms: None,
            },
            _ => AuthResult {
                ok: false,
                method: Some("token".to_string()),
                user: None,
                reason: Some("Invalid token".to_string()),
                rate_limited: None,
                retry_after_ms: None,
            },
        }
    }
}

/// Password-based authentication
pub struct PasswordAuth {
    username: String,
    password: String,
}

impl PasswordAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Authenticator for PasswordAuth {
    fn authenticate(&self, ctx: &AuthContext) -> AuthResult {
        let auth_header = ctx.headers.get("authorization");

        if let Some(header) = auth_header {
            if let Some(credentials) = header.strip_prefix("Basic ") {
                if let Ok(decoded) = base64::decode(credentials) {
                    if let Ok(creds_str) = String::from_utf8(decoded) {
                        let parts: Vec<&str> = creds_str.split(':').collect();
                        if parts.len() == 2 {
                            let username = parts[0];
                            let password = parts[1];

                            if username == self.username && password == self.password {
                                return AuthResult {
                                    ok: true,
                                    method: Some("password".to_string()),
                                    user: Some(username.to_string()),
                                    reason: None,
                                    rate_limited: None,
                                    retry_after_ms: None,
                                };
                            }
                        }
                    }
                }
            }
        }

        AuthResult {
            ok: false,
            method: Some("password".to_string()),
            user: None,
            reason: Some("Invalid credentials".to_string()),
            rate_limited: None,
            retry_after_ms: None,
        }
    }
}

/// Authentication manager that handles different auth methods
pub struct AuthManager {
    authenticator: Box<dyn Authenticator>,
}

impl AuthManager {
    pub fn new(auth_config: ResolvedAuth) -> Self {
        let authenticator: Box<dyn Authenticator> = match auth_config.mode {
            AuthMode::None => Box::new(NoAuth),
            AuthMode::Token => {
                let token = auth_config.token.unwrap_or_default();
                Box::new(TokenAuth::new(token))
            }
            AuthMode::Password => {
                let username = "admin".to_string(); // Default username
                let password = auth_config.password.unwrap_or_default();
                Box::new(PasswordAuth::new(username, password))
            }
            AuthMode::TrustedProxy => Box::new(NoAuth), // Simplified for now
        };

        Self { authenticator }
    }

    pub fn authenticate(&self, ctx: &AuthContext) -> AuthResult {
        self.authenticator.authenticate(ctx)
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new(ResolvedAuth {
            mode: AuthMode::None,
            token: None,
            password: None,
            allow_tailscale: false,
            trusted_proxy: None,
        })
    }
}

pub fn is_loopback_address(ip: &str) -> bool {
    ip == "127.0.0.1" || ip == "::1" || ip.starts_with("127.")
}

pub fn is_local_request(client_ip: Option<&str>) -> bool {
    client_ip.map_or(false, |ip| is_loopback_address(ip))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_auth() {
        let auth = NoAuth;
        let ctx = AuthContext::new();
        let result = auth.authenticate(&ctx);
        assert!(result.ok);
        assert_eq!(result.method, Some("none".to_string()));
    }

    #[test]
    fn test_token_auth_success() {
        let auth = TokenAuth::new("secret-token".to_string());
        let ctx = AuthContext::new().with_header("authorization", "Bearer secret-token");
        let result = auth.authenticate(&ctx);
        assert!(result.ok);
        assert_eq!(result.method, Some("token".to_string()));
    }

    #[test]
    fn test_token_auth_failure() {
        let auth = TokenAuth::new("secret-token".to_string());
        let ctx = AuthContext::new().with_header("authorization", "Bearer wrong-token");
        let result = auth.authenticate(&ctx);
        assert!(!result.ok);
        assert_eq!(result.reason, Some("Invalid token".to_string()));
    }

    #[test]
    fn test_password_auth_success() {
        let auth = PasswordAuth::new("admin".to_string(), "password".to_string());
        let credentials = base64::encode("admin:password");
        let ctx = AuthContext::new().with_header("authorization", format!("Basic {}", credentials));
        let result = auth.authenticate(&ctx);
        assert!(result.ok);
        assert_eq!(result.method, Some("password".to_string()));
        assert_eq!(result.user, Some("admin".to_string()));
    }

    #[test]
    fn test_is_loopback_address() {
        assert!(is_loopback_address("127.0.0.1"));
        assert!(is_loopback_address("::1"));
        assert!(!is_loopback_address("192.168.1.1"));
    }
}
