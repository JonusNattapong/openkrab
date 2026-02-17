use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use teloxide::{prelude::*, Bot};
use teloxide::types::Message as TeloxideMessage;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

use openclaw_core::{
    Channel, ChannelId, ChannelType, Direction, Message as OpenClawMessage, MessageContent, MessageId, MessageMetadata,
    OpenClawError, Result, UserId,
};
use openclaw_core::channel::ChannelConfig;

pub struct TelegramChannel {
    config: ChannelConfig,
    token: String,
    connected: Arc<AtomicBool>,
    message_receiver: Arc<RwLock<Option<mpsc::Receiver<OpenClawMessage>>>>,
    bot: Arc<RwLock<Option<Bot>>>,
    dispatcher_handle: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl TelegramChannel {
    pub fn new(token: impl Into<String>, config: ChannelConfig) -> Self {
        Self {
            config,
            token: token.into(),
            connected: Arc::new(AtomicBool::new(false)),
            message_receiver: Arc::new(RwLock::new(None)),
            bot: Arc::new(RwLock::new(None)),
            dispatcher_handle: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_token(token: impl Into<String>, config: ChannelConfig) -> Self {
        Self::new(token, config)
    }

    async fn start_polling(&self) -> Result<()> {
        let token = std::sync::Arc::new(self.token.clone());
        let (tx, rx) = mpsc::channel::<OpenClawMessage>(100);

        {
            let mut receiver = self.message_receiver.write().await;
            *receiver = Some(rx);
        }

        let connected = self.connected.clone();
        let channel_id = self.config.id;

        let handle = tokio::spawn(async move {
            let bot = Bot::new(&*token);

            let handler = dptree::entry()
                .branch(Update::filter_message().endpoint(move |msg: TeloxideMessage, bot: Bot| {
                    let tx = tx.clone();
                    // clone cheap handles here so the outer closure implements `Fn`
                    let channel_id = channel_id.clone();
                    let token = token.clone();
                    async move {
                        // Convert teloxide Message to openclaw Message
                        let chat_id = msg.chat.id.to_string();
                        let user_id = msg.from.as_ref().map(|u| u.id.to_string()).unwrap_or_default();
                        let caption = msg.caption().map(|c| c.to_string());

                        let content = if let Some(text) = msg.text() {
                            MessageContent::Text { text: text.to_string() }
                        } else if let Some(photo) = msg.photo() {
                            // Get largest photo and attempt to download via Bot API
                            let mut data: Option<Vec<u8>> = None;
                            if let Some(photo_size) = photo.last() {
                                 // PhotoSize contains a `file` with `id`
                                 let file_id = photo_size.file.id.clone();
                                 if let Ok(file) = bot.get_file(&file_id).await {
                                     let path = file.path; // teloxide File.path is String/Option depending on version
                                     if !path.is_empty() {
                                         let url = format!("https://api.telegram.org/file/bot{}/{}", token.as_ref(), path);
                                         if let Ok(resp) = reqwest::get(&url).await {
                                             if let Ok(bytes) = resp.bytes().await {
                                                 data = Some(bytes.to_vec());
                                             }
                                         }
                                     }
                                 }
                            }
                            MessageContent::Image {
                                url: None,
                                data: data,
                                caption,
                            }
                        } else if let Some(document) = msg.document() {
                            let mut data: Vec<u8> = Vec::new();
                             let file_id = document.file.id.clone();
                             if let Ok(file) = bot.get_file(&file_id).await {
                                 let path = file.path;
                                 if !path.is_empty() {
                                     let url = format!("https://api.telegram.org/file/bot{}/{}", token.as_ref(), path);
                                     if let Ok(resp) = reqwest::get(&url).await {
                                         if let Ok(bytes) = resp.bytes().await {
                                             data = bytes.to_vec();
                                         }
                                     }
                                 }
                             }
                            MessageContent::File {
                                name: document.file_name.clone().unwrap_or_default(),
                                data,
                                mime_type: document.mime_type.clone().map(|m| m.to_string()).unwrap_or_default(),
                            }
                        } else if let Some(sticker) = msg.sticker() {
                            let mut data: Vec<u8> = Vec::new();
                             let file_id = sticker.file.id.clone();
                             if let Ok(file) = bot.get_file(&file_id).await {
                                 let path = file.path;
                                 if !path.is_empty() {
                                     let url = format!("https://api.telegram.org/file/bot{}/{}", token.as_ref(), path);
                                     if let Ok(resp) = reqwest::get(&url).await {
                                         if let Ok(bytes) = resp.bytes().await {
                                             data = bytes.to_vec();
                                         }
                                     }
                                 }
                             }
                            MessageContent::Sticker {
                                data,
                            }
                        } else {
                            MessageContent::Text { text: "Unsupported media".to_string() }
                        };

                        let openclaw_msg = OpenClawMessage {
                            id: MessageId::new(),
                            session_id: None,
                            channel_id,
                            chat_id: chat_id.clone(),
                            user_id: Some(user_id.clone()),
                            sender: UserId::new(), // TODO: Map telegram user ID
                            recipient: UserId::new(), // TODO: Map chat ID
                            content,
                            direction: Direction::Inbound,
                            created_at: chrono::Utc::now(),
                            metadata: serde_json::json!({
                                "reply_to": null,
                                "mentions": [],
                                "is_forwarded": false,
                                "is_reply": false,
                            }),
                        };

                        let _ = tx.send(openclaw_msg).await;
                        Ok::<(), teloxide::RequestError>(())
                    }
                }));

            connected.store(true, Ordering::SeqCst);

            let mut dispatcher = Dispatcher::builder(bot, handler)
                .enable_ctrlc_handler()
                .build();

            dispatcher.dispatch().await;

            connected.store(false, Ordering::SeqCst);
        });

        {
            let mut handle_lock = self.dispatcher_handle.write().await;
            *handle_lock = Some(handle);
        }

        {
            let mut bot_lock = self.bot.write().await;
            // Store bot so tests or other parts can access it if needed
            *bot_lock = Some(bot.clone());
        }

        Ok(())
    }
}

#[async_trait]
impl Channel for TelegramChannel {
    fn id(&self) -> ChannelId {
        self.config.id
    }

    fn name(&self) -> &str {
        &self.config.name
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Telegram
    }

    fn config(&self) -> &ChannelConfig {
        &self.config
    }

    async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting Telegram channel: {}", self.config.name);

        self.start_polling().await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        self.connected.store(true, Ordering::SeqCst);

        tracing::info!("Telegram channel started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping Telegram channel: {}", self.config.name);

        self.connected.store(false, Ordering::SeqCst);

        // TODO: Stop dispatcher
        let mut handle_lock = self.dispatcher_handle.write().await;
        if let Some(handle) = handle_lock.take() {
            handle.abort();
        }

        tracing::info!("Telegram channel stopped");
        Ok(())
    }

    async fn send_message(&self, message: OpenClawMessage) -> Result<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(OpenClawError::Channel {
                message: "Telegram not connected".to_string(),
            });
        }

        // TODO: Implement actual sending
        // For now, just log and return success
        tracing::trace!("Would send message to Telegram: {:?}", message.content);
        Ok(())
    }

    async fn receive_message(&self) -> Result<OpenClawMessage> {
        let mut receiver = self.message_receiver.write().await;
        if let Some(ref mut rx) = *receiver {
            match rx.recv().await {
                Some(msg) => Ok(msg),
                None => Err(OpenClawError::Channel {
                    message: "Message channel closed".to_string(),
                }),
            }
        } else {
            Err(OpenClawError::Channel {
                message: "Message receiver not initialized".to_string(),
            })
        }
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }
}
