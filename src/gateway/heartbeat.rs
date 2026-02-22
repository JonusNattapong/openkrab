//! heartbeat — Periodic heartbeat runner for connectivity checks.
//! Ported from OpenClaw's `startHeartbeatRunner` concept.
//!
//! Runs a periodic check to verify that channels and gateway connections
//! are alive, emitting status events and optionally restarting unhealthy services.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;

// ─── Types ────────────────────────────────────────────────────────────────────

/// Status of a single heartbeat target.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatStatus {
    Healthy,
    Degraded { reason: String },
    Unhealthy { reason: String },
    Unknown,
}

/// A heartbeat report for a single target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatReport {
    pub target: String,
    pub status: HeartbeatStatus,
    pub latency_ms: Option<u64>,
    pub checked_at: DateTime<Utc>,
    pub consecutive_failures: u32,
}

/// Configuration for the heartbeat runner.
#[derive(Debug, Clone)]
pub struct HeartbeatConfig {
    /// Interval between heartbeat checks.
    pub interval: Duration,
    /// Grace period after startup before first check.
    pub startup_grace: Duration,
    /// Number of failures before marking as unhealthy.
    pub failure_threshold: u32,
    /// Maximum consecutive failures before giving up.
    pub max_failures: u32,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(60),
            startup_grace: Duration::from_secs(30),
            failure_threshold: 3,
            max_failures: 10,
        }
    }
}

/// Trait for heartbeat check targets.
#[async_trait::async_trait]
pub trait HeartbeatTarget: Send + Sync {
    /// Name of this target (e.g. "gateway", "telegram", etc.).
    fn name(&self) -> &str;

    /// Perform a health check. Returns Ok(latency_ms) or Err.
    async fn check(&self) -> Result<u64>;

    /// Whether this target should be checked.
    fn is_enabled(&self) -> bool {
        true
    }
}

// ─── Heartbeat state ──────────────────────────────────────────────────────────

#[derive(Debug)]
struct TargetState {
    consecutive_failures: u32,
    last_success: Option<DateTime<Utc>>,
    last_failure: Option<DateTime<Utc>>,
    last_latency_ms: Option<u64>,
}

impl Default for TargetState {
    fn default() -> Self {
        Self {
            consecutive_failures: 0,
            last_success: None,
            last_failure: None,
            last_latency_ms: None,
        }
    }
}

// ─── Heartbeat runner ─────────────────────────────────────────────────────────

/// Handle to a running heartbeat runner.
pub struct HeartbeatRunner {
    stop_tx: mpsc::Sender<()>,
    handle: Mutex<Option<tokio::task::JoinHandle<()>>>,
    state: Arc<Mutex<HashMap<String, TargetState>>>,
}

impl HeartbeatRunner {
    /// Get the latest heartbeat reports.
    pub fn get_reports(&self) -> Vec<HeartbeatReport> {
        let state = self.state.lock().unwrap();
        state
            .iter()
            .map(|(name, s)| {
                let status = if s.consecutive_failures == 0 {
                    HeartbeatStatus::Healthy
                } else if s.consecutive_failures < 3 {
                    HeartbeatStatus::Degraded {
                        reason: format!("{} consecutive failures", s.consecutive_failures),
                    }
                } else {
                    HeartbeatStatus::Unhealthy {
                        reason: format!("{} consecutive failures", s.consecutive_failures),
                    }
                };
                HeartbeatReport {
                    target: name.clone(),
                    status,
                    latency_ms: s.last_latency_ms,
                    checked_at: s.last_success.or(s.last_failure).unwrap_or_else(Utc::now),
                    consecutive_failures: s.consecutive_failures,
                }
            })
            .collect()
    }

    /// Stop the heartbeat runner.
    pub async fn stop(&self) -> Result<()> {
        let _ = self.stop_tx.send(()).await;
        let handle = self.handle.lock().unwrap().take();
        if let Some(handle) = handle {
            let _ = handle.await;
        }
        Ok(())
    }
}

/// Start the heartbeat runner with the given targets and configuration.
pub fn start_heartbeat_runner(
    config: HeartbeatConfig,
    targets: Vec<Box<dyn HeartbeatTarget>>,
    on_report: Option<mpsc::UnboundedSender<Vec<HeartbeatReport>>>,
) -> HeartbeatRunner {
    let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
    let state: Arc<Mutex<HashMap<String, TargetState>>> = Arc::new(Mutex::new(HashMap::new()));
    let state_clone = state.clone();

    // Initialize state
    {
        let mut s = state.lock().unwrap();
        for target in &targets {
            s.insert(target.name().to_string(), TargetState::default());
        }
    }

    let handle = tokio::spawn(async move {
        // Wait for startup grace period
        tokio::select! {
            _ = tokio::time::sleep(config.startup_grace) => {},
            _ = stop_rx.recv() => return,
        }

        let targets = Arc::new(targets);
        let failure_threshold = config.failure_threshold;
        let max_failures = config.max_failures;

        loop {
            // Run checks
            let mut reports = Vec::new();

            for target in targets.iter() {
                if !target.is_enabled() {
                    continue;
                }

                let now = Utc::now();
                let check_result = target.check().await;

                let mut s = state_clone.lock().unwrap();
                let target_state = s.entry(target.name().to_string()).or_default();

                let status = match check_result {
                    Ok(latency_ms) => {
                        target_state.consecutive_failures = 0;
                        target_state.last_success = Some(now);
                        target_state.last_latency_ms = Some(latency_ms);
                        HeartbeatStatus::Healthy
                    }
                    Err(err) => {
                        target_state.consecutive_failures += 1;
                        target_state.last_failure = Some(now);

                        if target_state.consecutive_failures >= max_failures {
                            HeartbeatStatus::Unhealthy {
                                reason: format!(
                                    "exceeded max failures ({}): {}",
                                    max_failures, err
                                ),
                            }
                        } else if target_state.consecutive_failures >= failure_threshold {
                            HeartbeatStatus::Unhealthy {
                                reason: format!("{}", err),
                            }
                        } else {
                            HeartbeatStatus::Degraded {
                                reason: format!("{}", err),
                            }
                        }
                    }
                };

                reports.push(HeartbeatReport {
                    target: target.name().to_string(),
                    status,
                    latency_ms: target_state.last_latency_ms,
                    checked_at: now,
                    consecutive_failures: target_state.consecutive_failures,
                });
            }

            // Emit reports if subscriber is set
            if let Some(ref tx) = on_report {
                let _ = tx.send(reports);
            }

            // Wait for next interval or stop signal
            tokio::select! {
                _ = tokio::time::sleep(config.interval) => {},
                _ = stop_rx.recv() => {
                    tracing::info!("heartbeat runner stopped");
                    break;
                },
            }
        }
    });

    HeartbeatRunner {
        stop_tx,
        handle: Mutex::new(Some(handle)),
        state,
    }
}

// ─── Built-in targets ─────────────────────────────────────────────────────────

/// A simple TCP connectivity check target.
#[derive(Debug)]
pub struct TcpHeartbeatTarget {
    name: String,
    host: String,
    port: u16,
    timeout: Duration,
}

impl TcpHeartbeatTarget {
    pub fn new(name: impl Into<String>, host: impl Into<String>, port: u16) -> Self {
        Self {
            name: name.into(),
            host: host.into(),
            port,
            timeout: Duration::from_secs(5),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[async_trait::async_trait]
impl HeartbeatTarget for TcpHeartbeatTarget {
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> Result<u64> {
        let start = std::time::Instant::now();
        let addr = format!("{}:{}", self.host, self.port);

        match tokio::time::timeout(self.timeout, tokio::net::TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => Ok(start.elapsed().as_millis() as u64),
            Ok(Err(e)) => Err(anyhow::anyhow!("connection failed: {}", e)),
            Err(_) => Err(anyhow::anyhow!(
                "connection timeout after {:?}",
                self.timeout
            )),
        }
    }
}

/// A simple HTTP health check target.
#[derive(Debug)]
pub struct HttpHeartbeatTarget {
    name: String,
    url: String,
    timeout: Duration,
}

impl HttpHeartbeatTarget {
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            timeout: Duration::from_secs(10),
        }
    }
}

#[async_trait::async_trait]
impl HeartbeatTarget for HttpHeartbeatTarget {
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> Result<u64> {
        let start = std::time::Instant::now();
        let client = reqwest::Client::builder().timeout(self.timeout).build()?;

        let response = client.get(&self.url).send().await?;
        if response.status().is_success() {
            Ok(start.elapsed().as_millis() as u64)
        } else {
            Err(anyhow::anyhow!("HTTP {}", response.status()))
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTarget {
        name: String,
        should_fail: bool,
    }

    #[async_trait::async_trait]
    impl HeartbeatTarget for MockTarget {
        fn name(&self) -> &str {
            &self.name
        }

        async fn check(&self) -> Result<u64> {
            if self.should_fail {
                Err(anyhow::anyhow!("mock failure"))
            } else {
                Ok(1)
            }
        }
    }

    #[test]
    fn heartbeat_config_defaults() {
        let config = HeartbeatConfig::default();
        assert_eq!(config.interval, Duration::from_secs(60));
        assert_eq!(config.startup_grace, Duration::from_secs(30));
        assert_eq!(config.failure_threshold, 3);
    }

    #[tokio::test]
    async fn heartbeat_start_stop() {
        let (report_tx, mut report_rx) = mpsc::unbounded_channel();
        let runner = start_heartbeat_runner(
            HeartbeatConfig {
                interval: Duration::from_millis(50),
                startup_grace: Duration::from_millis(10),
                failure_threshold: 2,
                max_failures: 5,
            },
            vec![Box::new(MockTarget {
                name: "test".to_string(),
                should_fail: false,
            })],
            Some(report_tx),
        );

        // Wait for at least one report
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Check reports
        let reports = runner.get_reports();
        assert!(!reports.is_empty());

        // Stop
        runner.stop().await.unwrap();
    }

    #[tokio::test]
    async fn heartbeat_failure_tracking() {
        let runner = start_heartbeat_runner(
            HeartbeatConfig {
                interval: Duration::from_millis(30),
                startup_grace: Duration::from_millis(5),
                failure_threshold: 2,
                max_failures: 5,
            },
            vec![Box::new(MockTarget {
                name: "failing".to_string(),
                should_fail: true,
            })],
            None,
        );

        // Wait for a few checks
        tokio::time::sleep(Duration::from_millis(150)).await;

        let reports = runner.get_reports();
        assert!(!reports.is_empty());
        let failing = reports.iter().find(|r| r.target == "failing").unwrap();
        assert!(failing.consecutive_failures > 0);
        assert!(matches!(
            failing.status,
            HeartbeatStatus::Degraded { .. } | HeartbeatStatus::Unhealthy { .. }
        ));

        runner.stop().await.unwrap();
    }
}
