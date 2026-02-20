//! Monitor manager — manages connector monitors for inbound messages

use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use crate::channels::{MessageHandler, MonitorOptions};
use crate::connectors;

/// Monitor manager for all connector monitors
pub struct MonitorManager {
    monitors: HashMap<String, Box<dyn MonitorHandle>>,
    message_handler: Arc<dyn MessageHandler>,
}

impl MonitorManager {
    pub fn new(message_handler: Arc<dyn MessageHandler>) -> Self {
        Self {
            monitors: HashMap::new(),
            message_handler,
        }
    }

    /// Start monitoring for a connector
    pub async fn start_monitor(
        &mut self,
        connector: &str,
        account_id: Option<String>,
    ) -> Result<()> {
        let options = MonitorOptions {
            account_id: account_id.clone(),
            verbose: false,
            heartbeat_seconds: None,
        };

        match connector {
            "whatsapp" => {
                let monitor_result = connectors::whatsapp_monitor::monitor_whatsapp_provider(
                    options,
                    self.message_handler.clone(),
                )
                .await?;

                self.monitors.insert(
                    format!("whatsapp:{}", account_id.unwrap_or_default()),
                    Box::new(WhatsAppMonitorHandle {
                        stop_tx: monitor_result.stop_tx,
                        handle: Mutex::new(Some(monitor_result.handle)),
                    }),
                );
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported connector: {}", connector));
            }
        }

        Ok(())
    }

    /// Stop monitoring for a connector
    pub async fn stop_monitor(
        &mut self,
        connector: &str,
        account_id: Option<String>,
    ) -> Result<()> {
        let key = format!("{}:{}", connector, account_id.unwrap_or_default());
        if let Some(monitor) = self.monitors.remove(&key) {
            monitor.stop().await?;
        }
        Ok(())
    }

    /// Get status of all monitors
    pub fn get_status(&self) -> HashMap<String, serde_json::Value> {
        let mut status = HashMap::new();

        for (key, _monitor) in &self.monitors {
            // In real implementation, would get actual status from monitor
            status.insert(
                key.clone(),
                serde_json::json!({
                    "running": true,
                    "connected": true
                }),
            );
        }

        status
    }
}

/// Trait for monitor handles
#[async_trait::async_trait]
pub trait MonitorHandle: Send + Sync {
    async fn stop(&self) -> Result<()>;
}

/// WhatsApp monitor handle
pub struct WhatsAppMonitorHandle {
    stop_tx: mpsc::Sender<()>,
    handle: Mutex<Option<tokio::task::JoinHandle<Result<()>>>>,
}

#[async_trait::async_trait]
impl MonitorHandle for WhatsAppMonitorHandle {
    async fn stop(&self) -> Result<()> {
        let _ = self.stop_tx.send(()).await;
        let handle = self.handle.lock().unwrap().take();
        if let Some(handle) = handle {
            let _ = handle.await;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockMessageHandler;

    #[async_trait::async_trait]
    impl MessageHandler for MockMessageHandler {
        async fn handle_message(&self, _message: serde_json::Value) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_monitor_manager_creation() {
        let handler = Arc::new(MockMessageHandler);
        let manager = MonitorManager::new(handler);
        assert!(manager.monitors.is_empty());
    }

    #[tokio::test]
    async fn test_unsupported_connector() {
        let handler = Arc::new(MockMessageHandler);
        let mut manager = MonitorManager::new(handler);

        let result = manager.start_monitor("unsupported", None).await;
        assert!(result.is_err());
    }
}
