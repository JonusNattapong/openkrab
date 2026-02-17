use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve_default_web_auth_dir() -> PathBuf {
    PathBuf::from(".krabkrab/credentials/web")
}

pub fn resolve_web_creds_path(auth_dir: &Path) -> PathBuf {
    auth_dir.join("creds.json")
}

pub fn has_web_creds_sync(auth_dir: &Path) -> bool {
    resolve_web_creds_path(auth_dir).exists()
}

pub fn web_auth_exists(auth_dir: Option<&Path>) -> bool {
    let dir = auth_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(resolve_default_web_auth_dir);
    has_web_creds_sync(&dir)
}

pub fn logout_web(auth_dir: Option<&Path>) -> Result<bool> {
    let dir = auth_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(resolve_default_web_auth_dir);

    if !dir.exists() {
        return Ok(false);
    }

    fs::remove_dir_all(&dir)?;
    Ok(true)
}
