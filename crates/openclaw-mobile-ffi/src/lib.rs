// Minimal uniffi + wrapper for exposing a tiny API to mobile
use async_trait::async_trait;
use std::sync::Arc;

pub struct MobileBridge;

impl MobileBridge {
    pub fn new() -> Self {
        MobileBridge
    }

    pub fn ping(&self) -> String {
        "pong".to_string()
    }
}

// uniffi scaffolding will be added when generating bindings
