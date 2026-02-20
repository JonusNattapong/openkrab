use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Keep server maxPayload aligned with gateway client maxPayload so high-res canvas snapshots
/// don't get disconnected mid-invoke with "Max payload size exceeded".
pub const MAX_PAYLOAD_BYTES: usize = 25 * 1024 * 1024;
pub const MAX_BUFFERED_BYTES: usize = 50 * 1024 * 1024; // per-connection send buffer limit (2x max payload)

static DEFAULT_MAX_CHAT_HISTORY_MESSAGES_BYTES: usize = 6 * 1024 * 1024; // keep history responses comfortably under client WS limits
static MAX_CHAT_HISTORY_MESSAGES_BYTES: Lazy<Mutex<usize>> =
    Lazy::new(|| Mutex::new(DEFAULT_MAX_CHAT_HISTORY_MESSAGES_BYTES));

pub fn get_max_chat_history_messages_bytes() -> usize {
    *MAX_CHAT_HISTORY_MESSAGES_BYTES.lock().unwrap()
}

#[cfg(test)]
pub fn __set_max_chat_history_messages_bytes_for_test(value: Option<usize>) {
    if std::env::var("VITEST").is_ok() || std::env::var("OPENKRAB_TEST").is_ok() {
        let mut lock = MAX_CHAT_HISTORY_MESSAGES_BYTES.lock().unwrap();
        *lock = value.unwrap_or(DEFAULT_MAX_CHAT_HISTORY_MESSAGES_BYTES);
    }
}

pub const DEFAULT_HANDSHAKE_TIMEOUT_MS: u64 = 10_000;

pub fn get_handshake_timeout_ms() -> u64 {
    if cfg!(test) && std::env::var("OPENKRAB_TEST_HANDSHAKE_TIMEOUT_MS").is_ok() {
        if let Ok(parsed) = std::env::var("OPENKRAB_TEST_HANDSHAKE_TIMEOUT_MS")
            .unwrap()
            .parse::<u64>()
        {
            if parsed > 0 {
                return parsed;
            }
        }
    }
    DEFAULT_HANDSHAKE_TIMEOUT_MS
}

pub const TICK_INTERVAL_MS: u64 = 30_000;
pub const HEALTH_REFRESH_INTERVAL_MS: u64 = 60_000;
pub const DEDUPE_TTL_MS: u64 = 5 * 60_000;
pub const DEDUPE_MAX: usize = 1000;

/// Default gateway port
pub const DEFAULT_PORT: u16 = 18789;
