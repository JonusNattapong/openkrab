//! WhatsApp connector monitor — port of `openclaw/extensions/whatsapp/src/monitorWebChannel`

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use crate::channels::{MessageHandler, MonitorOptions};

/// WhatsApp message from monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppMessage {
    pub id: String,
    pub from: String,
    pub to: String,
    pub body: String,
    pub timestamp: u64,
    pub message_type: String,
}

/// WhatsApp client for monitoring (placeholder for Web WhatsApp integration)
pub struct WhatsAppClient {
    // In real implementation, this would connect to WhatsApp Web
    connected: bool,
}

impl WhatsAppClient {
    pub fn new() -> Result<Self> {
        Ok(Self { connected: false })
    }

    pub async fn connect(&mut self) -> Result<()> {
        // Simulate connection to WhatsApp Web
        sleep(Duration::from_secs(1)).await;
        self.connected = true;
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        self.connected = false;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

/// WhatsApp monitor status
#[derive(Debug, Clone)]
pub struct WhatsAppMonitorStatus {
    pub running: bool,
    pub connected: bool,
    pub reconnect_attempts: u32,
    pub last_connected_at: Option<std::time::SystemTime>,
    pub last_disconnect: Option<String>,
    pub last_message_at: Option<std::time::SystemTime>,
    pub last_event_at: Option<std::time::SystemTime>,
    pub last_error: Option<String>,
}

/// WhatsApp monitor result
pub struct WhatsAppMonitorResult {
    pub status: WhatsAppMonitorStatus,
    pub stop_tx: mpsc::Sender<()>,
    pub handle: tokio::task::JoinHandle<Result<()>>,
}

impl Default for WhatsAppMonitorStatus {
    fn default() -> Self {
        Self {
            running: true,
            connected: false,
            reconnect_attempts: 0,
            last_connected_at: None,
            last_disconnect: None,
            last_message_at: None,
            last_event_at: None,
            last_error: None,
        }
    }
}

/// Monitor WhatsApp provider for inbound messages
pub async fn monitor_whatsapp_provider(
    options: MonitorOptions,
    message_handler: Arc<dyn MessageHandler>,
) -> Result<WhatsAppMonitorResult> {
    let (stop_tx, mut stop_rx) = mpsc::channel(1);
    let status = Arc::new(Mutex::new(WhatsAppMonitorStatus::default()));

    let status_clone = status.clone();
    let message_handler_clone = message_handler.clone();

    let handle = tokio::spawn(async move {
        let client = WhatsAppClient::new()
            .map_err(|e| anyhow!("Failed to create WhatsApp client: {}", e))?;

        // Main monitoring loop
        loop {
            tokio::select! {
                _ = stop_rx.recv() => {
                    println!("WhatsApp monitor stopping...");
                    break;
                }
                result = monitor_loop(&client, &status_clone, &message_handler_clone, &options) => {
                    if let Err(e) = result {
                        let backoff_ms = {
                            let mut status = status_clone.lock().unwrap();
                            status.last_error = Some(e.to_string());
                            status.connected = false;
                            let backoff = calculate_backoff(status.reconnect_attempts);
                            status.reconnect_attempts += 1;
                            backoff
                        };

                        println!("WhatsApp monitor error: {}. Reconnecting in {}ms...", e, backoff_ms);
                        sleep(Duration::from_millis(backoff_ms)).await;
                    }
                }
            }
        }

        // Cleanup
        {
            let mut status = status_clone.lock().unwrap();
            status.running = false;
            status.connected = false;
        }

        Ok(())
    });

    let current_status = status.lock().unwrap().clone();

    Ok(WhatsAppMonitorResult {
        status: current_status,
        stop_tx,
        handle,
    })
}

async fn monitor_loop(
    client: &WhatsAppClient,
    status: &Arc<Mutex<WhatsAppMonitorStatus>>,
    message_handler: &Arc<dyn MessageHandler>,
    options: &MonitorOptions,
) -> Result<()> {
    // Attempt to connect
    {
        let mut status_guard = status.lock().unwrap();
        status_guard.connected = false;
        status_guard.last_error = None;
    }

    // Simulate connection (in real implementation, this would connect to WhatsApp Web)
    println!("Connecting to WhatsApp...");
    sleep(Duration::from_secs(1)).await; // Simulate connection delay

    {
        let mut status_guard = status.lock().unwrap();
        status_guard.connected = true;
        status_guard.last_connected_at = Some(std::time::SystemTime::now());
        status_guard.reconnect_attempts = 0;
    }

    println!("WhatsApp connected successfully");

    // Main message polling loop
    loop {
        // Check for stop signal
        if let Ok(_) = tokio::time::timeout(Duration::from_millis(100), async {
            // This would be non-blocking in real implementation
        })
        .await
        {
            break;
        }

        // Poll for messages (in real implementation, this would listen to WebSocket/events)
        match poll_messages(client, options).await {
            Ok(messages) => {
                for message in messages {
                    {
                        let mut status_guard = status.lock().unwrap();
                        status_guard.last_message_at = Some(std::time::SystemTime::now());
                        status_guard.last_event_at = Some(std::time::SystemTime::now());
                    } // guard dropped here before .await

                    // Handle the message
                    let message_value =
                        serde_json::to_value(&message).unwrap_or_else(|_| serde_json::Value::Null);
                    if let Err(e) = message_handler.handle_message(message_value).await {
                        println!("Error handling WhatsApp message: {}", e);
                    }
                }
            }
            Err(e) => {
                let mut status_guard = status.lock().unwrap();
                status_guard.last_error = Some(e.to_string());
                status_guard.connected = false;
                status_guard.last_disconnect = Some(format!("Connection error: {}", e));
                return Err(e);
            }
        }

        // Small delay between polls
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

async fn poll_messages(
    _client: &WhatsAppClient,
    _options: &MonitorOptions,
) -> Result<Vec<WhatsAppMessage>> {
    // In real implementation, this would:
    // 1. Listen to WebSocket events from WhatsApp Web
    // 2. Parse incoming messages
    // 3. Convert to WhatsAppMessage structs
    // 4. Return them

    // For now, return empty vector (no messages)
    Ok(vec![])
}

fn calculate_backoff(attempt: u32) -> u64 {
    // Exponential backoff: 1s, 2s, 4s, 8s, 16s, max 30s
    let base_ms = 1000u64;
    let max_ms = 30000u64;
    let backoff_ms = base_ms * (1 << attempt.min(4));
    backoff_ms.min(max_ms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockMessageHandler;

    #[async_trait::async_trait]
    impl MessageHandler for MockMessageHandler {
        async fn handle_message(&self, _message: WhatsAppMessage) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_monitor_creation() {
        let handler = Arc::new(MockMessageHandler);
        let options = MonitorOptions::default();

        let result = monitor_whatsapp_provider(options, handler).await;
        assert!(result.is_ok());

        let monitor_result = result.unwrap();

        // Stop the monitor
        let _ = monitor_result.stop_tx.send(()).await;
        let _ = monitor_result.handle.await;
    }

    #[test]
    fn test_backoff_calculation() {
        assert_eq!(calculate_backoff(0), 1000);
        assert_eq!(calculate_backoff(1), 2000);
        assert_eq!(calculate_backoff(2), 4000);
        assert_eq!(calculate_backoff(3), 8000);
        assert_eq!(calculate_backoff(4), 16000);
        assert_eq!(calculate_backoff(5), 30000); // max
        assert_eq!(calculate_backoff(10), 30000); // still max
    }
}
