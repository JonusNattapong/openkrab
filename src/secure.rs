use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};

const NONCE_SIZE: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptedValue {
    pub nonce: String,
    pub ciphertext: String,
}

pub struct SecretBox {
    key: [u8; 32],
}

impl SecretBox {
    pub fn new(password: &str) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let mut key = [0u8; 32];
        key.copy_from_slice(&hasher.finalize());
        Self { key }
    }

    pub fn from_key(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn encrypt(&self, plaintext: &str) -> EncryptedValue {
        let cipher = Aes256Gcm::new_from_slice(&self.key).expect("valid key size");
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .expect("encryption should not fail");

        EncryptedValue {
            nonce: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, nonce_bytes),
            ciphertext: base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                ciphertext,
            ),
        }
    }

    pub fn decrypt(&self, encrypted: &EncryptedValue) -> Option<String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key).ok()?;

        let nonce_bytes =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &encrypted.nonce)
                .ok()?;
        let ciphertext = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &encrypted.ciphertext,
        )
        .ok()?;

        if nonce_bytes.len() != NONCE_SIZE {
            return None;
        }

        let nonce = Nonce::from_slice(&nonce_bytes);
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).ok()?;

        String::from_utf8(plaintext).ok()
    }
}

pub fn generate_encryption_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}
