//! polls — Long-polling and short-polling utilities.
//! Ported from `openclaw/src/polls.ts` (Phase 7).
//!
//! Provides a generic polling loop with exponential back-off, jitter,
//! and a stop signal. Used by connector update loops (e.g. Telegram getUpdates).

use std::time::Duration;
use tokio::time::sleep;

// ─── Poll config ──────────────────────────────────────────────────────────────

/// Configuration for a polling loop.
#[derive(Debug, Clone)]
pub struct PollConfig {
    /// Initial delay between polls.
    pub initial_interval: Duration,
    /// Maximum delay (cap for back-off).
    pub max_interval: Duration,
    /// Back-off multiplier on errors (1.0 = no back-off).
    pub backoff_factor: f64,
    /// Jitter fraction (0.0–1.0) added to each delay.
    pub jitter: f64,
    /// Maximum consecutive errors before giving up (None = retry forever).
    pub max_errors: Option<u32>,
}

impl Default for PollConfig {
    fn default() -> Self {
        Self {
            initial_interval: Duration::from_millis(500),
            max_interval: Duration::from_secs(30),
            backoff_factor: 2.0,
            jitter: 0.1,
            max_errors: None,
        }
    }
}

impl PollConfig {
    pub fn fast() -> Self {
        Self {
            initial_interval: Duration::from_millis(100),
            max_interval: Duration::from_secs(5),
            ..Default::default()
        }
    }

    pub fn slow() -> Self {
        Self {
            initial_interval: Duration::from_secs(5),
            max_interval: Duration::from_secs(60),
            ..Default::default()
        }
    }
}

// ─── Back-off calculator ──────────────────────────────────────────────────────

/// Calculate the next sleep duration given current interval and config.
pub fn next_interval(current: Duration, config: &PollConfig, error: bool) -> Duration {
    if !error {
        return config.initial_interval;
    }
    let next_secs =
        (current.as_secs_f64() * config.backoff_factor).min(config.max_interval.as_secs_f64());

    // Add jitter: up to ±jitter fraction of next_secs
    let jitter_delta = next_secs * config.jitter;
    let jitter_offset = (rand_f64() * 2.0 - 1.0) * jitter_delta;
    let final_secs = (next_secs + jitter_offset).max(0.001);

    Duration::from_secs_f64(final_secs.min(config.max_interval.as_secs_f64()))
}

fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (t % 1_000_000) as f64 / 1_000_000.0
}

// ─── Poll loop ────────────────────────────────────────────────────────────────

/// Result of a single poll tick.
#[derive(Debug)]
pub enum PollResult<T> {
    /// Successful result.
    Ok(T),
    /// Transient error — will retry after back-off.
    Err(String),
    /// Terminal error — stop the loop.
    Fatal(String),
    /// Stop signal received.
    Stop,
}

/// Run `tick_fn` in a loop until it returns `Fatal` or `Stop`.
/// Applies back-off on errors.
///
/// Returns the number of successful ticks completed.
pub async fn poll_loop<F, Fut, T>(
    config: PollConfig,
    mut stop_rx: tokio::sync::watch::Receiver<bool>,
    mut tick_fn: F,
) -> u64
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = PollResult<T>>,
{
    let mut interval = config.initial_interval;
    let mut error_count: u32 = 0;
    let mut success_count: u64 = 0;

    loop {
        // Check stop signal
        if *stop_rx.borrow() {
            break;
        }

        let result = tick_fn().await;

        match result {
            PollResult::Ok(_) => {
                error_count = 0;
                success_count += 1;
                interval = config.initial_interval;
            }
            PollResult::Err(msg) => {
                error_count += 1;
                eprintln!("[poll] error #{}: {}", error_count, msg);
                if let Some(max) = config.max_errors {
                    if error_count >= max {
                        eprintln!("[poll] max errors reached, stopping");
                        break;
                    }
                }
                interval = next_interval(interval, &config, true);
            }
            PollResult::Fatal(msg) => {
                eprintln!("[poll] fatal: {}", msg);
                break;
            }
            PollResult::Stop => break,
        }

        // Wait before next tick (or stop if signal arrives)
        tokio::select! {
            _ = sleep(interval) => {}
            _ = stop_rx.changed() => {
                if *stop_rx.borrow() { break; }
            }
        }
    }

    success_count
}

// ─── Simple interval ticker ───────────────────────────────────────────────────

/// A simple ticker that fires every `interval`.
/// Returns a channel receiver — send `true` to stop.
pub fn start_ticker(
    interval: Duration,
) -> (
    tokio::sync::mpsc::Receiver<()>,
    tokio::sync::watch::Sender<bool>,
) {
    let (tick_tx, tick_rx) = tokio::sync::mpsc::channel::<()>(8);
    let (stop_tx, mut stop_rx) = tokio::sync::watch::channel(false);

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = sleep(interval) => {
                    if tick_tx.send(()).await.is_err() {
                        break;
                    }
                }
                _ = stop_rx.changed() => {
                    if *stop_rx.borrow() { break; }
                }
            }
        }
    });

    (tick_rx, stop_tx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_off_increases_on_error() {
        let cfg = PollConfig::default();
        let init = cfg.initial_interval;
        let next = next_interval(init, &cfg, true);
        // With backoff_factor=2.0 + jitter, next should be > initial but <= max
        assert!(next >= init);
        assert!(next <= cfg.max_interval);
    }

    #[test]
    fn back_off_resets_on_success() {
        let cfg = PollConfig::default();
        let big = Duration::from_secs(20);
        let next = next_interval(big, &cfg, false);
        assert_eq!(next, cfg.initial_interval);
    }

    #[test]
    fn poll_config_fast() {
        let cfg = PollConfig::fast();
        assert!(cfg.initial_interval < Duration::from_secs(1));
    }

    #[test]
    fn poll_config_slow() {
        let cfg = PollConfig::slow();
        assert!(cfg.initial_interval >= Duration::from_secs(1));
    }

    #[tokio::test]
    async fn poll_loop_stops_on_signal() {
        let cfg = PollConfig {
            initial_interval: Duration::from_millis(10),
            ..Default::default()
        };
        let (stop_tx, stop_rx) = tokio::sync::watch::channel(false);
        let count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let count2 = count.clone();

        let handle = tokio::spawn(async move {
            poll_loop(cfg, stop_rx, || {
                let c = count2.clone();
                async move {
                    c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    PollResult::<()>::Ok(())
                }
            })
            .await
        });

        // Let it run a few ticks
        tokio::time::sleep(Duration::from_millis(50)).await;
        stop_tx.send(true).unwrap();
        let success = handle.await.unwrap();
        assert!(success > 0);
    }
}
