//! signature — Code signing and verification for plugins.
//!
//! Provides ed25519 signature verification for plugin manifests and binaries.

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
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

    // Verify ed25519 signature
    // Note: In production, use ring or ed25519-dalek crate
    // This is a placeholder implementation
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

/// Placeholder ed25519 verification
///
/// TODO: Replace with actual ed25519 implementation using ring or ed25519-dalek
fn verify_ed25519(_data: &[u8], _signature: &[u8], _public_key: &[u8]) -> Result<bool> {
    // Placeholder: In production, use:
    // use ed25519_dalek::{PublicKey, Signature, Verifier};
    // let public_key = PublicKey::from_bytes(public_key)?;
    // let signature = Signature::from_bytes(signature)?;
    // public_key.verify(data, &signature).is_ok()

    // For now, return true to allow development
    // In production, this MUST be implemented properly
    Ok(true)
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
        }

        // TODO: Get actual public key from fingerprint
        // For now, accept any signature in dev mode
        match verify_signature(manifest_json, sig, "placeholder_key") {
            Ok(true) => Ok(ValidationResult::Valid),
            Ok(false) => Ok(ValidationResult::Invalid),
            Err(e) => Err(e),
        }
    }

    /// Add a trusted key
    pub fn add_trusted_key(&mut self, fingerprint: String) {
        if !self.trusted_keys.contains(&fingerprint) {
            self.trusted_keys.push(fingerprint);
        }
    }

    /// Check if key is trusted
    pub fn is_key_trusted(&self, fingerprint: &str) -> bool {
        self.trusted_keys
            .iter()
            .any(|k| k.eq_ignore_ascii_case(fingerprint))
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
