use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{ChannelId, ChannelType, Message, OpenClawError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub id: ChannelId,
    pub name: String,
    pub channel_type: ChannelType,
    pub enabled: bool,
    pub config: serde_json::Value,
}

#[async_trait]
pub trait Channel: Send + Sync {
    fn id(&self) -> ChannelId;
    fn name(&self) -> &str;
    fn channel_type(&self) -> ChannelType;
    fn config(&self) -> &ChannelConfig;

    async fn start(&mut self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
    async fn send_message(&self, message: Message) -> Result<()>;
    async fn receive_message(&self) -> Result<Message>;

    fn is_connected(&self) -> bool;
}

pub struct ChannelRegistry {
    channels: std::sync::RwLock<Vec<(ChannelId, Arc<dyn Channel>)>>,
}

impl ChannelRegistry {
    pub fn new() -> Self {
        Self {
            channels: std::sync::RwLock::new(Vec::new()),
        }
    }

    pub fn register(&self, channel: Arc<dyn Channel>) -> Result<()> {
        let id = channel.id();
        let mut channels = self.channels.write().map_err(|_| OpenClawError::Internal {
            message: "Failed to acquire channel lock".to_string(),
        })?;
        channels.push((id, channel));
        Ok(())
    }

    pub fn unregister(&self, id: &ChannelId) -> Result<()> {
        let mut channels = self.channels.write().map_err(|_| OpenClawError::Internal {
            message: "Failed to acquire channel lock".to_string(),
        })?;
        channels.retain(|(cid, _)| cid != id);
        Ok(())
    }

    pub fn get(&self, id: &ChannelId) -> Option<Arc<dyn Channel>> {
        let channels = self.channels.read().ok()?;
        for (cid, channel) in channels.iter() {
            if cid == id {
                return Some(channel.clone());
            }
        }
        None
    }

    pub fn list(&self) -> Vec<ChannelId> {
        self.channels
            .read()
            .map(|c| c.iter().map(|(id, _)| *id).collect())
            .unwrap_or_default()
    }
}

impl Default for ChannelRegistry {
    fn default() -> Self {
        Self::new()
    }
}
