use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::{ChannelId, Http};
use tokio::sync::RwLock;

use openclaw_core::{
    Channel, ChannelId as OpenClawChannelId, ChannelType, Message as OpenClawMessage,
    MessageContent, OpenClawError, Result,
};
use openclaw_core::channel::ChannelConfig;

pub struct DiscordChannel {
    config: ChannelConfig,
    token: String,
    connected: Arc<AtomicBool>,
    http_client: Arc<RwLock<Option<Arc<Http>>>>,
}

impl DiscordChannel {
    pub fn new(token: impl Into<String>, config: ChannelConfig) -> Self {
        Self {
            config,
            token: token.into(),
            connected: Arc::new(AtomicBool::new(false)),
            http_client: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_token(token: impl Into<String>, config: ChannelConfig) -> Self {
        Self::new(token, config)
    }

    async fn start_client(&self) -> Result<()> {
        let token = self.token.clone();

        let intents = serenity::model::gateway::GatewayIntents::GUILDS
            | serenity::model::gateway::GatewayIntents::GUILD_MESSAGES
            | serenity::model::gateway::GatewayIntents::MESSAGE_CONTENT
            | serenity::model::gateway::GatewayIntents::DIRECT_MESSAGES;

        let mut client = serenity::Client::builder(&token, intents)
            .await
            .map_err(|e| OpenClawError::Channel {
                message: format!("Failed to create Discord client: {}", e),
            })?;

        let http = client.http.clone();
        {
            let mut http_lock = self.http_client.write().await;
            *http_lock = Some(http);
        }

        tokio::spawn(async move {
            if let Err(e) = client.start().await {
                tracing::error!("Discord client error: {}", e);
            }
        });

        Ok(())
    }
}

#[async_trait]
impl Channel for DiscordChannel {
    fn id(&self) -> OpenClawChannelId {
        self.config.id
    }

    fn name(&self) -> &str {
        &self.config.name
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Discord
    }

    fn config(&self) -> &ChannelConfig {
        &self.config
    }

    async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting Discord channel: {}", self.config.name);

        self.start_client().await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        self.connected.store(true, Ordering::SeqCst);

        tracing::info!("Discord channel started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping Discord channel: {}", self.config.name);

        self.connected.store(false, Ordering::SeqCst);

        tracing::info!("Discord channel stopped");
        Ok(())
    }

    async fn send_message(&self, message: OpenClawMessage) -> Result<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(OpenClawError::Channel {
                message: "Discord not connected".to_string(),
            });
        }

        let http_lock = self.http_client.read().await;
        let http = http_lock.as_ref().ok_or_else(|| OpenClawError::Channel {
            message: "HTTP client not initialized".to_string(),
        })?;

        let content = match &message.content {
            MessageContent::Text { text } => text.clone(),
            MessageContent::Image { caption, .. } => caption.clone().unwrap_or_default(),
            _ => {
                return Err(OpenClawError::Channel {
                    message: "Unsupported message content type".to_string(),
                });
            }
        };

        let channel_id = self.config.config.get("channel_id")
            .and_then(|v| v.as_u64())
            .map(ChannelId::new)
            .ok_or_else(|| OpenClawError::Channel {
                message: "No channel_id configured".to_string(),
            })?;

        let create_message = serenity::all::CreateMessage::new()
            .content(&content);

        if let Err(e) = channel_id.send_message(http, create_message).await {
            return Err(OpenClawError::Channel {
                message: format!("Failed to send message: {}", e),
            });
        }

        tracing::trace!("Message sent to Discord channel {}", channel_id);
        Ok(())
    }

    async fn receive_message(&self) -> Result<OpenClawMessage> {
        Err(OpenClawError::Channel {
            message: "Event listening not yet implemented".to_string(),
        })
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }
}
