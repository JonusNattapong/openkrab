use axum::{
    body::Body,
    extract::{Request, State as AxumState},
    http::{HeaderMap, StatusCode},
};
use std::sync::Arc;

pub mod key {
    pub const API_KEY_HEADER: &str = "x-api-key";
}

#[derive(Clone)]
pub struct AuthState {
    pub api_keys: Arc<Vec<String>>,
}

impl AuthState {
    pub fn new(api_keys: Vec<String>) -> Self {
        Self {
            api_keys: Arc::new(api_keys),
        }
    }

    pub fn is_valid(&self, key: &str) -> bool {
        self.api_keys.is_empty() || self.api_keys.contains(&key.to_string())
    }
}

pub fn require_auth(state: &AuthState, headers: &HeaderMap) -> Result<(), StatusCode> {
    if state.api_keys.is_empty() {
        return Ok(());
    }

    match headers.get(key::API_KEY_HEADER) {
        Some(key) => {
            if let Ok(key_str) = key.to_str() {
                if state.is_valid(key_str) {
                    return Ok(());
                }
            }
        }
        None => {}
    }

    Err(StatusCode::UNAUTHORIZED)
}
