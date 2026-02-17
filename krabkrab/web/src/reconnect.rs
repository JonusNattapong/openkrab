#[derive(Debug, Clone, Copy)]
pub struct ReconnectPolicy {
    pub initial_ms: u64,
    pub max_ms: u64,
    pub factor: f64,
    pub jitter: f64,
    pub max_attempts: usize,
}

pub const DEFAULT_HEARTBEAT_SECONDS: u64 = 60;
pub const DEFAULT_RECONNECT_POLICY: ReconnectPolicy = ReconnectPolicy {
    initial_ms: 2_000,
    max_ms: 30_000,
    factor: 1.8,
    jitter: 0.25,
    max_attempts: 12,
};

#[derive(Debug, Clone, Default)]
pub struct WebReconnectConfig {
    pub heartbeat_seconds: Option<u64>,
    pub initial_ms: Option<u64>,
    pub max_ms: Option<u64>,
    pub factor: Option<f64>,
    pub jitter: Option<f64>,
    pub max_attempts: Option<usize>,
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

pub fn resolve_heartbeat_seconds(cfg: &WebReconnectConfig, override_seconds: Option<u64>) -> u64 {
    let candidate = override_seconds.or(cfg.heartbeat_seconds).unwrap_or(DEFAULT_HEARTBEAT_SECONDS);
    if candidate > 0 { candidate } else { DEFAULT_HEARTBEAT_SECONDS }
}

pub fn resolve_reconnect_policy(cfg: &WebReconnectConfig) -> ReconnectPolicy {
    let mut p = DEFAULT_RECONNECT_POLICY;

    if let Some(v) = cfg.initial_ms { p.initial_ms = v.max(250); }
    if let Some(v) = cfg.max_ms { p.max_ms = v.max(p.initial_ms); }
    if let Some(v) = cfg.factor { p.factor = clamp(v, 1.1, 10.0); }
    if let Some(v) = cfg.jitter { p.jitter = clamp(v, 0.0, 1.0); }
    if let Some(v) = cfg.max_attempts { p.max_attempts = v; }

    p
}

pub fn compute_backoff(policy: ReconnectPolicy, attempt: usize) -> u64 {
    if attempt == 0 {
        return policy.initial_ms;
    }

    let exp = policy.factor.powi(attempt as i32);
    let mut delay = (policy.initial_ms as f64 * exp) as u64;
    if delay > policy.max_ms {
        delay = policy.max_ms;
    }
    delay
}

pub fn new_connection_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("web-{now}-{id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_heartbeat_falls_back_to_default() {
        let cfg = WebReconnectConfig::default();
        assert_eq!(resolve_heartbeat_seconds(&cfg, None), DEFAULT_HEARTBEAT_SECONDS);
    }

    #[test]
    fn resolve_policy_clamps_values() {
        let cfg = WebReconnectConfig {
            initial_ms: Some(10),
            max_ms: Some(100),
            factor: Some(100.0),
            jitter: Some(2.0),
            max_attempts: Some(3),
            ..WebReconnectConfig::default()
        };

        let p = resolve_reconnect_policy(&cfg);
        assert_eq!(p.initial_ms, 250);
        assert_eq!(p.max_ms, 250);
        assert_eq!(p.factor, 10.0);
        assert_eq!(p.jitter, 1.0);
        assert_eq!(p.max_attempts, 3);
    }
}
