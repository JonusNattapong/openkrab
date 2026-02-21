//! signature — Code signing and verification for plugins.
//!
//! Provides ed25519 signature verification for plugin manifests and binaries.

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

/// Verify a plugin signature using ed25519
///
/// # Arguments
/// * `data` - The data to verify (usually manifest JSON)
/// * `signature_b64` - Base64-encoded signature
/// * `public_key_b64` - Base64-encoded public key
///
/// # Returns
/// * `Ok(true)` if signature is valid
/// * `Ok(false)` if signature is invalid
/// * `Err` if verification fails
pub fn verify_signature(data: &[u8], signature_b64: &str, public_key_b64: &str) -> Result<bool> {
    // Decode signature
    let signature = BASE64
        .decode(signature_b64)
        .context("Failed to decode signature")?;

    // Decode public key
    let public_key = BASE64
        .decode(public_key_b64)
        .context("Failed to decode public key")?;

    verify_ed25519(data, &signature, &public_key)
}

/// Calculate SHA-256 hash of data
pub fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Calculate fingerprint of a public key
pub fn calculate_key_fingerprint(public_key_b64: &str) -> Result<String> {
    let public_key = BASE64
        .decode(public_key_b64)
        .context("Failed to decode public key")?;

    let mut hasher = Sha256::new();
    hasher.update(&public_key);
    Ok(format!("{:x}", hasher.finalize())[..16].to_string())
}

/// Verify that a public key fingerprint matches
pub fn verify_key_fingerprint(public_key_b64: &str, expected_fingerprint: &str) -> Result<bool> {
    let actual = calculate_key_fingerprint(public_key_b64)?;
    Ok(actual.eq_ignore_ascii_case(expected_fingerprint))
}

fn verify_ed25519(data: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool> {
    let pk_bytes: [u8; 32] = public_key
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid ed25519 public key length: expected 32 bytes"))?;
    let sig_bytes: [u8; 64] = signature
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid ed25519 signature length: expected 64 bytes"))?;

    let verifying_key =
        VerifyingKey::from_bytes(&pk_bytes).context("Failed to parse ed25519 public key")?;
    let signature = Signature::from_bytes(&sig_bytes);

    Ok(verifying_key.verify(data, &signature).is_ok())
}

fn decode_public_key_bytes(public_key_b64: &str) -> Option<Vec<u8>> {
    let decoded = BASE64.decode(public_key_b64).ok()?;
    if decoded.len() == 32 {
        Some(decoded)
    } else {
        None
    }
}

/// Plugin signature validator
pub struct SignatureValidator {
    trusted_keys: Vec<String>,
    require_signatures: bool,
}

impl SignatureValidator {
    pub fn new(trusted_keys: Vec<String>, require_signatures: bool) -> Self {
        Self {
            trusted_keys,
            require_signatures,
        }
    }

    /// Validate a plugin manifest signature
    pub fn validate_manifest(
        &self,
        manifest_json: &[u8],
        signature: Option<&str>,
        key_fingerprint: Option<&str>,
    ) -> Result<ValidationResult> {
        // Check if signature is required but missing
        if self.require_signatures && signature.is_none() {
            return Ok(ValidationResult::Missing);
        }

        // If no signature required and none provided, allow
        if !self.require_signatures && signature.is_none() {
            return Ok(ValidationResult::Allowed);
        }

        let sig = match signature {
            Some(s) => s,
            None => return Ok(ValidationResult::Missing),
        };

        // Verify fingerprint matches if provided
        if let Some(fp) = key_fingerprint {
            // Check if fingerprint is in trusted keys
            if !self.trusted_keys.iter().any(|k| k.eq_ignore_ascii_case(fp)) {
                return Ok(ValidationResult::UntrustedKey);
            }

            // Resolve matching public key from trusted entries.
            let mut matched_key: Option<&str> = None;
            for entry in &self.trusted_keys {
                if let Some(pk) = decode_public_key_bytes(entry) {
                    let mut hasher = Sha256::new();
                    hasher.update(&pk);
                    let derived = format!("{:x}", hasher.finalize())[..16].to_string();
                    if derived.eq_ignore_ascii_case(fp) {
                        matched_key = Some(entry.as_str());
                        break;
                    }
                }
            }

            let Some(public_key_b64) = matched_key else {
                return Ok(ValidationResult::Invalid);
            };

            return match verify_signature(manifest_json, sig, public_key_b64) {
                Ok(true) => Ok(ValidationResult::Valid),
                Ok(false) => Ok(ValidationResult::Invalid),
                Err(e) => Err(e),
            };
        }

        // No fingerprint provided: verify against any trusted public key.
        for entry in &self.trusted_keys {
            if decode_public_key_bytes(entry).is_none() {
                continue;
            }
            if verify_signature(manifest_json, sig, entry)? {
                return Ok(ValidationResult::Valid);
            }
        }

        Ok(ValidationResult::Invalid)
    }

    /// Add a trusted key
    pub fn add_trusted_key(&mut self, fingerprint: String) {
        if !self.trusted_keys.contains(&fingerprint) {
            self.trusted_keys.push(fingerprint);
        }
    }

    /// Check if key is trusted
    pub fn is_key_trusted(&self, fingerprint: &str) -> bool {
        self.trusted_keys.iter().any(|k| {
            k.eq_ignore_ascii_case(fingerprint)
                || calculate_key_fingerprint(k)
                    .map(|fp| fp.eq_ignore_ascii_case(fingerprint))
                    .unwrap_or(false)
        })
    }
}

/// Signature validation result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    /// Signature is valid and trusted
    Valid,
    /// Signature is invalid
    Invalid,
    /// Signature is missing
    Missing,
    /// Key is not in trusted list
    UntrustedKey,
    /// No signature required, allowed
    Allowed,
}

impl ValidationResult {
    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid | ValidationResult::Allowed)
    }

    /// Check if validation failed
    pub fn is_failed(&self) -> bool {
        !self.is_valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hash() {
        let data = b"hello world";
        let hash = calculate_hash(data);
        assert_eq!(hash.len(), 64); // SHA-256 hex string
    }

    #[test]
    fn test_signature_validator() {
        let validator = SignatureValidator::new(vec![], false);

        // Should allow unsigned when not required
        let result = validator.validate_manifest(b"{}", None, None).unwrap();
        assert!(result.is_valid());

        // Should require signature when configured
        let validator_strict = SignatureValidator::new(vec![], true);
        let result = validator_strict
            .validate_manifest(b"{}", None, None)
            .unwrap();
        assert!(!result.is_valid());
    }

    #[test]
    fn test_key_fingerprint() {
        let key = BASE64.encode(b"test_public_key_12345");
        let fp = calculate_key_fingerprint(&key).unwrap();
        assert_eq!(fp.len(), 16);

        let valid = verify_key_fingerprint(&key, &fp).unwrap();
        assert!(valid);
    }
}
