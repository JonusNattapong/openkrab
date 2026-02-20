//! BlueBubbles server probe and Private API detection.
//! Ported from openkrab/extensions/bluebubbles/src/probe.ts

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::types::{BlueBubblesProbeResult, BlueBubblesServerInfo, DEFAULT_TIMEOUT_MS};

static SERVER_INFO_CACHE: once_cell::sync::Lazy<
    Arc<Mutex<HashMap<String, BlueBubblesServerInfo>>>,
> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub ok: bool,
    pub server_info: Option<BlueBubblesServerInfo>,
    pub error: Option<String>,
}

pub fn probe_server(
    base_url: &str,
    password: Option<&str>,
    timeout_ms: Option<u64>,
) -> Result<ProbeResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = super::types::build_api_url(base_url, "/api/v1/server/info", password);

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Ok(ProbeResult {
            ok: false,
            server_info: None,
            error: Some(format!("HTTP {}", response.status())),
        });
    }

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let data = json.get("data").unwrap_or(&json);

    let server_info = BlueBubblesServerInfo {
        os_version: data
            .get("os_version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        private_api: data.get("private_api").and_then(|v| v.as_bool()),
        bundle_id: data
            .get("bundle_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        app_version: data
            .get("app_version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    };

    Ok(ProbeResult {
        ok: true,
        server_info: Some(server_info.clone()),
        error: None,
    })
}

pub fn fetch_server_info(
    base_url: &str,
    password: Option<&str>,
    account_id: &str,
    timeout_ms: Option<u64>,
) -> Option<BlueBubblesServerInfo> {
    match probe_server(base_url, password, timeout_ms) {
        Ok(result) if result.ok => {
            if let Some(info) = &result.server_info {
                cache_server_info(account_id, info.clone());
            }
            result.server_info
        }
        _ => None,
    }
}

pub fn cache_server_info(account_id: &str, info: BlueBubblesServerInfo) {
    let mut cache = SERVER_INFO_CACHE.lock().unwrap();
    cache.insert(account_id.to_string(), info);
}

pub fn get_cached_server_info(account_id: &str) -> Option<BlueBubblesServerInfo> {
    let cache = SERVER_INFO_CACHE.lock().unwrap();
    cache.get(account_id).cloned()
}

pub fn is_macos_26_or_higher(account_id: &str) -> bool {
    let cache = SERVER_INFO_CACHE.lock().unwrap();
    if let Some(info) = cache.get(account_id) {
        if let Some(ref version) = info.os_version {
            return parse_macos_version(version) >= 26;
        }
    }
    false
}

pub fn get_private_api_status(account_id: &str) -> Option<bool> {
    let cache = SERVER_INFO_CACHE.lock().unwrap();
    cache.get(account_id).and_then(|info| info.private_api)
}

pub fn is_private_api_enabled(account_id: &str) -> bool {
    get_private_api_status(account_id).unwrap_or(false)
}

fn parse_macos_version(version: &str) -> i32 {
    let version = version.trim();
    if version.starts_with("macOS ") {
        let rest = &version[6..];
        let parts: Vec<&str> = rest.split('.').collect();
        if let Some(major) = parts.first() {
            return major.parse().unwrap_or(0);
        }
    }
    if version.starts_with("14") || version.starts_with("15") {
        return version
            .split('.')
            .next()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);
    }
    0
}

pub fn clear_cache(account_id: &str) {
    let mut cache = SERVER_INFO_CACHE.lock().unwrap();
    cache.remove(account_id);
}

pub fn clear_all_cache() {
    let mut cache = SERVER_INFO_CACHE.lock().unwrap();
    cache.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_macos_version() {
        assert_eq!(parse_macos_version("macOS 14.0"), 14);
        assert_eq!(parse_macos_version("macOS 15.1"), 15);
        assert_eq!(parse_macos_version("14.5"), 14);
        assert_eq!(parse_macos_version("invalid"), 0);
    }

    #[test]
    fn test_cache_server_info() {
        let info = BlueBubblesServerInfo {
            os_version: Some("macOS 15.0".to_string()),
            private_api: Some(true),
            bundle_id: None,
            app_version: None,
        };
        cache_server_info("test-account", info.clone());

        let cached = get_cached_server_info("test-account");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().os_version, Some("macOS 15.0".to_string()));

        clear_cache("test-account");
        assert!(get_cached_server_info("test-account").is_none());
    }

    #[test]
    fn test_is_macos_26_or_higher() {
        let info = BlueBubblesServerInfo {
            os_version: Some("macOS 26.0".to_string()),
            private_api: None,
            bundle_id: None,
            app_version: None,
        };
        cache_server_info("macos26", info);
        assert!(is_macos_26_or_higher("macos26"));

        let info_old = BlueBubblesServerInfo {
            os_version: Some("macOS 14.0".to_string()),
            private_api: None,
            bundle_id: None,
            app_version: None,
        };
        cache_server_info("macos14", info_old);
        assert!(!is_macos_26_or_higher("macos14"));

        clear_cache("macos26");
        clear_cache("macos14");
    }

    #[test]
    fn test_private_api_status() {
        let info = BlueBubblesServerInfo {
            os_version: None,
            private_api: Some(true),
            bundle_id: None,
            app_version: None,
        };
        cache_server_info("private-api-test", info);
        assert_eq!(get_private_api_status("private-api-test"), Some(true));
        assert!(is_private_api_enabled("private-api-test"));

        clear_cache("private-api-test");
    }
}
