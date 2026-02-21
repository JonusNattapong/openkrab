use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::gateway::types::*;

/// Gateway WebSocket client for connecting to the gateway server
pub struct GatewayClient {
    url: Url,
}

impl GatewayClient {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub async fn connect(
        &self,
    ) -> Result<GatewayConnection, Box<dyn std::error::Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (write, read) = ws_stream.split();

        Ok(GatewayConnection {
            write: Box::pin(write),
            read: Box::pin(read),
            pending_requests: HashMap::new(),
        })
    }
}

/// Active WebSocket connection to the gateway server
pub struct GatewayConnection {
    write: std::pin::Pin<
        Box<dyn futures_util::Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Send>,
    >,
    read: std::pin::Pin<
        Box<
            dyn futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
                + Send,
        >,
    >,
    #[allow(dead_code)]
    pending_requests: HashMap<String, tokio::sync::oneshot::Sender<GatewayMessage>>,
}

impl GatewayConnection {
    pub async fn send(
        &mut self,
        message: GatewayMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string(&message)?;
        self.write.send(Message::Text(json)).await?;
        Ok(())
    }

    pub async fn receive(
        &mut self,
    ) -> Result<Option<GatewayMessage>, Box<dyn std::error::Error + Send + Sync>> {
        while let Some(msg) = self.read.next().await {
            let msg = msg?;
            match msg {
                Message::Text(text) => {
                    let message: GatewayMessage = serde_json::from_str(&text)?;
                    return Ok(Some(message));
                }
                Message::Close(_) => {
                    return Ok(None);
                }
                _ => continue,
            }
        }
        Ok(None)
    }

    pub async fn close(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.write.send(Message::Close(None)).await?;
        Ok(())
    }

    /// Send a chat message and wait for a response
    pub async fn send_chat(
        &mut self,
        session_key: String,
        message: String,
        attachments: Option<Vec<crate::gateway::types::Attachment>>,
    ) -> Result<GatewayMessage, Box<dyn std::error::Error + Send + Sync>> {
        let chat_msg = GatewayMessage::Chat {
            session_key,
            message,
            attachments,
        };

        self.send(chat_msg).await?;

        loop {
            match self.receive().await? {
                Some(GatewayMessage::Chat {
                    session_key,
                    message,
                    attachments,
                }) => {
                    return Ok(GatewayMessage::Chat {
                        session_key,
                        message,
                        attachments,
                    });
                }
                Some(GatewayMessage::Error { code, message }) => {
                    return Err(format!("Gateway error ({}): {}", code, message).into());
                }
                Some(_) => continue,
                None => return Err("Connection closed while waiting for chat reply".into()),
            }
        }
    }

    /// Get server status
    pub async fn get_status(
        &mut self,
    ) -> Result<GatewayMessage, Box<dyn std::error::Error + Send + Sync>> {
        // Wait for the next message (likely a status update)
        match self.receive().await? {
            Some(msg) => Ok(msg),
            None => Err("Connection closed while waiting for status".into()),
        }
    }
}

/// Builder for creating gateway client connections
pub struct GatewayClientBuilder {
    host: String,
    port: u16,
    secure: bool,
}

impl Default for GatewayClientBuilder {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 18789,
            secure: false,
        }
    }
}

impl GatewayClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    pub fn build(self) -> Result<GatewayClient, Box<dyn std::error::Error + Send + Sync>> {
        let scheme = if self.secure { "wss" } else { "ws" };
        let url = format!("{}://{}:{}/ws", scheme, self.host, self.port);
        let url = Url::parse(&url)?;
        Ok(GatewayClient::new(url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = GatewayClientBuilder::new()
            .host("localhost")
            .port(8080)
            .secure(true)
            .build()
            .unwrap();

        assert_eq!(client.url.scheme(), "wss");
        assert_eq!(client.url.host_str(), Some("localhost"));
        assert_eq!(client.url.port(), Some(8080));
    }
}
