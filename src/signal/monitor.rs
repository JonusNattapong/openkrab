//! signal::monitor — Signal message monitor with daemon management.
//! Ported from `openclaw/src/signal/monitor.ts` (Phase 13).

use anyhow::Result;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::client::{health_check, SseEvent};
use super::daemon::{should_auto_start, daemon_opts_from_config, spawn_daemon, wait_for_daemon_ready, DaemonHandle};
use super::{parse_inbound, SignalConfig, SignalEvent, SignalEventSender, SignalEventReceiver, create_signal_channel};

/// Signal monitor with automatic daemon management.
pub struct Monitor {
    client: Client,
    config: Arc<SignalConfig>,
    daemon_handle: Option<DaemonHandle>,
}

impl Monitor {
    pub fn new(config: SignalConfig) -> Self {
        Self {
            client: Client::new(),
            config: Arc::new(config),
            daemon_handle: None,
        }
    }

    /// Start the monitor with automatic daemon management.
    pub async fn start(&mut self) -> Result<SignalEventReceiver> {
        let (event_tx, event_rx) = create_signal_channel();

        // Auto-start daemon if configured
        if should_auto_start(&self.config) {
            println!("Auto-starting signal-cli daemon...");

            let daemon_opts = daemon_opts_from_config(&self.config);
            match spawn_daemon(daemon_opts).await {
                Ok(handle) => {
                    self.daemon_handle = Some(handle);

                    // Wait for daemon to be ready
                    let timeout_ms = self.config.startup_timeout_ms.unwrap_or(30_000);
                    if let Err(e) = wait_for_daemon_ready(&self.client, &self.config.api_base, timeout_ms).await {
                        eprintln!("Warning: Daemon startup failed: {}", e);
                        let _ = event_tx.send(SignalEvent::Error(format!("Daemon startup failed: {}", e))).await;
                    } else {
                        println!("Signal daemon ready");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to start signal daemon: {}", e);
                    let _ = event_tx.send(SignalEvent::Error(format!("Failed to start daemon: {}", e))).await;
                }
            }
        }

        // Check if signal-cli API is available
        match health_check(&self.client, &self.config.api_base, Some(5000)).await {
            Ok(check) if check.ok => {
                let _ = event_tx.send(SignalEvent::Connected).await;
            }
            _ => {
                let _ = event_tx.send(SignalEvent::Error("Signal API not available".to_string())).await;
            }
        }

        // Start SSE monitoring
        let config = self.config.clone();
        let event_tx_clone = event_tx.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::run_sse_monitor(&config, event_tx_clone).await {
                let _ = event_tx_clone.send(SignalEvent::Error(format!("SSE monitor error: {}", e))).await;
            }
        });

        Ok(event_rx)
    }

    async fn run_sse_monitor(
        config: &Arc<SignalConfig>,
        event_tx: SignalEventSender,
    ) -> Result<()> {
        loop {
            let url = format!("{}/api/v1/events?account={}", config.api_base, config.resolve_account());

            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    let (_write, mut read) = ws_stream.split();

                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                // Parse as SignalInbound
                                if let Ok(inbound) = serde_json::from_str::<super::SignalInbound>(&text) {
                                    if let Some(parsed) = parse_inbound(&inbound) {
                                        let _ = event_tx.send(SignalEvent::Message(parsed)).await;
                                    }
                                }
                            }
                            Ok(Message::Ping(_)) => {
                                // Pong is handled automatically
                            }
                            Ok(Message::Close(_)) => {
                                break;
                            }
                            Err(e) => {
                                let _ = event_tx.send(SignalEvent::Error(format!("WebSocket error: {}", e))).await;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                Err(e) => {
                    let _ = event_tx.send(SignalEvent::Error(format!("WebSocket connection failed: {}", e))).await;
                    // Reconnect after delay
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    /// Stop the monitor and daemon.
    pub async fn stop(&mut self) {
        if let Some(handle) = self.daemon_handle.take() {
            handle.stop().await;
        }
    }
}

/// Run a simple Signal monitor for testing or CLI use.
pub async fn run_monitor(config: SignalConfig) -> Result<()> {
    let mut monitor = Monitor::new(config);
    let mut events = monitor.start().await?;

    println!("Signal monitor started. Press Ctrl+C to stop.");

    while let Some(event) = events.recv().await {
        match event {
            SignalEvent::Message(msg) => {
                println!("[Signal] {}: {}", msg.from, msg.text);
            }
            SignalEvent::Reaction { from, emoji, target_timestamp, .. } => {
                println!("[Signal] {} reacted with {} at {}", from, emoji, target_timestamp);
            }
            SignalEvent::Connected => {
                println!("[Signal] Connected");
            }
            SignalEvent::Disconnected => {
                println!("[Signal] Disconnected");
            }
            SignalEvent::Error(e) => {
                eprintln!("[Signal] Error: {}", e);
            }
            SignalEvent::Receipt { .. } => {}
        }
    }

    monitor.stop().await;
    Ok(())
}