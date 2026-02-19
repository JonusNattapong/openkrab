//! matrix::credentials — Matrix token storage and management.
//! Ported from `openclaw/extensions/matrix/src/matrix/credentials.ts` (Phase 13).

use std::fs;
use std::path::PathBuf;

/// Matrix authentication data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatrixAuth {
    pub access_token: String,
    pub user_id: String,
    pub device_id: Option<String>,
    pub homeserver: String,
}

/// Matrix resolved configuration with auth.
#[derive(Debug, Clone)]
pub struct MatrixResolvedConfig {
    pub config: super::MatrixConfig,
    pub auth: Option<MatrixAuth>,
}

impl MatrixResolvedConfig {
    pub fn new(config: super::MatrixConfig) -> Self {
        Self { config, auth: None }
    }

    pub fn with_auth(config: super::MatrixConfig, auth: MatrixAuth) -> Self {
        Self {
            config,
            auth: Some(auth),
        }
    }

    /// Load auth from storage if available.
    pub fn load_auth(
        &mut self,
        account_id: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = get_matrix_auth_path(account_id);
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let auth: MatrixAuth = serde_json::from_str(&content)?;
            self.auth = Some(auth);
        }
        Ok(())
    }

    /// Save auth to storage.
    pub fn save_auth(&self, account_id: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(auth) = &self.auth {
            let path = get_matrix_auth_path(account_id);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let content = serde_json::to_string_pretty(auth)?;
            fs::write(&path, content)?;
        }
        Ok(())
    }
}

/// Get path for Matrix auth storage.
pub fn get_matrix_auth_path(account_id: Option<&str>) -> PathBuf {
    let home = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut path = home.join("krabkrab").join("matrix");

    if let Some(account) = account_id {
        path.push(format!("auth_{}.json", account));
    } else {
        path.push("auth.json");
    }

    path
}

/// Resolve Matrix configuration with auth loading.
pub fn resolve_matrix_config(
    config: super::MatrixConfig,
    account_id: Option<&str>,
) -> Result<MatrixResolvedConfig, Box<dyn std::error::Error>> {
    let mut resolved = MatrixResolvedConfig::new(config);
    resolved.load_auth(account_id)?;
    Ok(resolved)
}

/// Resolve Matrix auth separately.
pub fn resolve_matrix_auth(
    account_id: Option<&str>,
) -> Result<Option<MatrixAuth>, Box<dyn std::error::Error>> {
    let path = get_matrix_auth_path(account_id);
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path)?;
    let auth: MatrixAuth = serde_json::from_str(&content)?;
    Ok(Some(auth))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn matrix_auth_serialize() {
        let auth = MatrixAuth {
            access_token: "token123".to_string(),
            user_id: "@user:matrix.org".to_string(),
            device_id: Some("device123".to_string()),
            homeserver: "https://matrix.org".to_string(),
        };

        let json = serde_json::to_string(&auth).unwrap();
        let parsed: MatrixAuth = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.access_token, "token123");
        assert_eq!(parsed.user_id, "@user:matrix.org");
        assert_eq!(parsed.device_id, Some("device123".to_string()));
        assert_eq!(parsed.homeserver, "https://matrix.org");
    }

    #[test]
    fn matrix_resolved_config_with_auth() {
        let config = super::super::MatrixConfig::default();
        let auth = MatrixAuth {
            access_token: "token123".to_string(),
            user_id: "@user:matrix.org".to_string(),
            device_id: None,
            homeserver: "https://matrix.org".to_string(),
        };

        let resolved = MatrixResolvedConfig::with_auth(config, auth.clone());
        assert_eq!(
            resolved.auth.as_ref().unwrap().access_token,
            auth.access_token
        );
    }

    #[test]
    fn get_matrix_auth_path_default() {
        let path = get_matrix_auth_path(None);
        assert!(path.to_string_lossy().contains("matrix/auth.json"));
    }

    #[test]
    fn get_matrix_auth_path_with_account() {
        let path = get_matrix_auth_path(Some("account1"));
        assert!(path.to_string_lossy().contains("matrix/auth_account1.json"));
    }
}
