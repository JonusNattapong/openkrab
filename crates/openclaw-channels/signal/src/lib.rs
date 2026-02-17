use async_trait::async_trait;
use std::sync::Arc;

use openclaw_core::{Channel, ChannelId, ChannelType, Message, OpenClawError, Result};

/// Minimal Signal channel skeleton. Integration can be built around `signal-cli`
/// via IPC or a local daemon; for now provide the Channel trait surface so it's
/// present in the workspace.
pub struct SignalChannel {
    id: ChannelId,
    name: String,
}

impl SignalChannel {
    pub fn new(id: ChannelId, name: impl Into<String>) -> Self {
        Self { id, name: name.into() }
    }
}

#[async_trait]
impl Channel for SignalChannel {
    fn id(&self) -> ChannelId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Signal
    }

    fn config(&self) -> &openclaw_core::channel::ChannelConfig {
        unimplemented!("Config integration not wired yet")
    }

    async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting Signal channel: {}", self.name);
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping Signal channel: {}", self.name);
        Ok(())
    }

    async fn send_message(&self, _message: Message) -> Result<()> {
        tracing::info!("(signal) send_message called");
        Err(OpenClawError::Channel { message: "Not implemented".to_string() })
    }

    async fn receive_message(&self) -> Result<Message> {
        Err(OpenClawError::Channel { message: "Not implemented".to_string() })
    }

    fn is_connected(&self) -> bool {
        false
    }
}

pub fn create_stub(id: ChannelId, name: &str) -> Arc<dyn Channel> {
    Arc::new(SignalChannel::new(id, name.to_string()))
}
