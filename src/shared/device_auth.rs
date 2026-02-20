//! Port of `openkrab/src/shared/device-auth.ts`
//!
//! Types and helpers for the device-auth token store used by the pairing
//! protocol.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

/// A single device-auth entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthEntry {
    pub token: String,
    pub role: String,
    pub scopes: Vec<String>,
    #[serde(rename = "updatedAtMs")]
    pub updated_at_ms: u64,
}

/// Versioned device-auth store persisted to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthStore {
    pub version: u32,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub tokens: HashMap<String, DeviceAuthEntry>,
}

/// Normalize a device-auth role string (trim whitespace).
pub fn normalize_device_auth_role(role: &str) -> String {
    role.trim().to_string()
}

/// Normalize a list of scopes: trim whitespace, deduplicate, and sort.
pub fn normalize_device_auth_scopes(scopes: Option<&[String]>) -> Vec<String> {
    let scopes = match scopes {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut set = BTreeSet::new();
    for scope in scopes {
        let trimmed = scope.trim().to_string();
        if !trimmed.is_empty() {
            set.insert(trimmed);
        }
    }
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_role_trims() {
        assert_eq!(normalize_device_auth_role("  admin  "), "admin");
    }

    #[test]
    fn normalize_role_empty() {
        assert_eq!(normalize_device_auth_role(""), "");
    }

    #[test]
    fn normalize_scopes_dedup_and_sort() {
        let scopes = vec![
            " write ".to_string(),
            "read".to_string(),
            "write".to_string(),
            "".to_string(),
        ];
        let result = normalize_device_auth_scopes(Some(&scopes));
        assert_eq!(result, vec!["read", "write"]);
    }

    #[test]
    fn normalize_scopes_none() {
        let result = normalize_device_auth_scopes(None);
        assert!(result.is_empty());
    }

    #[test]
    fn normalize_scopes_empty_vec() {
        let result = normalize_device_auth_scopes(Some(&[]));
        assert!(result.is_empty());
    }

    #[test]
    fn store_roundtrip() {
        let store = DeviceAuthStore {
            version: 1,
            device_id: "d-123".to_string(),
            tokens: HashMap::new(),
        };
        let json = serde_json::to_string(&store).unwrap();
        let back: DeviceAuthStore = serde_json::from_str(&json).unwrap();
        assert_eq!(back.version, 1);
        assert_eq!(back.device_id, "d-123");
    }
}
