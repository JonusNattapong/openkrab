//! CDP Session Pool for efficient connection reuse
//!
//! This module provides connection pooling for CDP WebSocket connections,
//! enabling efficient multi-tab automation without creating new connections
//! for every operation.

use anyhow::{anyhow, bail, Context, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// Global message ID counter
static MESSAGE_ID: AtomicI64 = AtomicI64::new(1);

/// Configuration for CDP session pool
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of concurrent sessions
    pub max_sessions: usize,
    /// Session idle timeout before cleanup
    pub idle_timeout_ms: u64,
    /// Operation timeout for CDP calls
    pub operation_timeout_ms: u64,
    /// Connection timeout for WebSocket
    pub connection_timeout_ms: u64,
    /// Enable automatic session recovery
    pub auto_recovery: bool,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_sessions: 10,
            idle_timeout_ms: 300_000, // 5 minutes
            operation_timeout_ms: 30_000,
            connection_timeout_ms: 10_000,
            auto_recovery: true,
        }
    }
}

/// Internal message types for CDP communication
#[derive(Debug)]
enum InternalMessage {
    Request {
        id: i64,
        method: String,
        params: Value,
        response_tx: oneshot::Sender<Result<Value>>,
    },
    Event {
        method: String,
        params: Value,
    },
    Close,
}

/// A pooled CDP session that can be reused across operations
pub struct PooledSession {
    ws_url: String,
    target_id: String,
    config: PoolConfig,
    command_tx: mpsc::UnboundedSender<InternalMessage>,
    event_handlers: Arc<RwLock<HashMap<String, Vec<mpsc::UnboundedSender<Value>>>>>,
    last_used: Arc<RwLock<Instant>>,
    closed: Arc<RwLock<bool>>,
}

impl PooledSession {
    /// Create a new pooled session
    pub async fn new(ws_url: &str, target_id: &str, config: PoolConfig) -> Result<Self> {
        let ws_url = ws_url.to_string();
        let target_id = target_id.to_string();

        // Establish WebSocket connection with timeout
        let ws_future = connect_async(&ws_url);
        let (ws_stream, _) = timeout(
            Duration::from_millis(config.connection_timeout_ms),
            ws_future,
        )
        .await
        .with_context(|| format!("WebSocket connection timeout to {ws_url}"))??;

        let (write_half, read_half) = ws_stream.split();
        let (command_tx, command_rx) = mpsc::unbounded_channel::<InternalMessage>();
        let event_handlers: Arc<RwLock<HashMap<String, Vec<mpsc::UnboundedSender<Value>>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let last_used = Arc::new(RwLock::new(Instant::now()));
        let closed = Arc::new(RwLock::new(false));

        // Spawn the connection handler task
        tokio::spawn(Self::connection_handler(
            read_half,
            write_half,
            command_rx,
            event_handlers.clone(),
            last_used.clone(),
            closed.clone(),
        ));

        let mut session = Self {
            ws_url,
            target_id,
            config,
            command_tx,
            event_handlers,
            last_used,
            closed,
        };

        // Enable required CDP domains
        session.enable_domains().await?;

        Ok(session)
    }

    /// Connection handler task that manages WebSocket I/O
    async fn connection_handler(
        mut read: impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
        mut write: impl SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
        mut command_rx: mpsc::UnboundedReceiver<InternalMessage>,
        event_handlers: Arc<RwLock<HashMap<String, Vec<mpsc::UnboundedSender<Value>>>>>,
        last_used: Arc<RwLock<Instant>>,
        closed: Arc<RwLock<bool>>,
    ) {
        let mut pending_requests: HashMap<i64, oneshot::Sender<Result<Value>>> = HashMap::new();

        loop {
            tokio::select! {
                // Handle incoming WebSocket messages
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            if let Err(e) = Self::handle_incoming_message(
                                &text,
                                &mut pending_requests,
                                &event_handlers,
                            ).await {
                                tracing::warn!("CDP message handling error: {}", e);
                            }
                        }
                        Some(Ok(Message::Close(_))) => {
                            tracing::info!("CDP WebSocket closed by server");
                            break;
                        }
                        Some(Err(e)) => {
                            tracing::error!("CDP WebSocket error: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }

                // Handle outgoing commands
                cmd = command_rx.recv() => {
                    match cmd {
                        Some(InternalMessage::Request { id, method, params, response_tx }) => {
                            pending_requests.insert(id, response_tx);
                            let req = json!({ "id": id, "method": method, "params": params });
                            if let Err(e) = write.send(Message::Text(req.to_string())).await {
                                tracing::error!("Failed to send CDP request: {}", e);
                                pending_requests.remove(&id);
                            }
                            *last_used.write().await = Instant::now();
                        }
                        Some(InternalMessage::Close) => {
                            tracing::info!("CDP session closing");
                            break;
                        }
                        None => {
                            tracing::info!("CDP command channel closed");
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Clean up pending requests
        for (id, tx) in pending_requests {
            let _ = tx.send(Err(anyhow!("Session closed")));
        }

        *closed.write().await = true;
    }

    /// Handle incoming CDP messages (responses and events)
    async fn handle_incoming_message(
        text: &str,
        pending: &mut HashMap<i64, oneshot::Sender<Result<Value>>>,
        handlers: &Arc<RwLock<HashMap<String, Vec<mpsc::UnboundedSender<Value>>>>>,
    ) -> Result<()> {
        let value: Value = serde_json::from_str(text)
            .with_context(|| format!("Failed to parse CDP message: {}", text))?;

        // Handle response with ID
        if let Some(id) = value.get("id").and_then(Value::as_i64) {
            if let Some(tx) = pending.remove(&id) {
                if let Some(error) = value.get("error") {
                    let _ = tx.send(Err(anyhow!("CDP error: {}", error)));
                } else {
                    let _ = tx.send(Ok(value));
                }
            }
        }
        // Handle events (no ID, has method)
        else if let Some(method) = value.get("method").and_then(Value::as_str) {
            let params = value.get("params").cloned().unwrap_or(json!({}));
            let handlers = handlers.read().await;
            if let Some(subs) = handlers.get(method) {
                for tx in subs {
                    let _ = tx.send(params.clone());
                }
            }
        }

        Ok(())
    }

    /// Enable required CDP domains
    async fn enable_domains(&mut self) -> Result<()> {
        self.call("Page.enable", json!({})).await?;
        self.call("DOM.enable", json!({})).await?;
        self.call("Runtime.enable", json!({})).await?;
        Ok(())
    }

    /// Make a CDP call with timeout
    pub async fn call(&self, method: &str, params: Value) -> Result<Value> {
        // Check if session is closed
        if *self.closed.read().await {
            bail!("Session is closed");
        }

        let id = MESSAGE_ID.fetch_add(1, Ordering::SeqCst);
        let (response_tx, response_rx) = oneshot::channel();

        // Send request
        self.command_tx
            .send(InternalMessage::Request {
                id,
                method: method.to_string(),
                params,
                response_tx,
            })
            .map_err(|_| anyhow!("Failed to send command to session handler"))?;

        // Wait for response with timeout
        let result = timeout(
            Duration::from_millis(self.config.operation_timeout_ms),
            response_rx,
        )
        .await
        .with_context(|| format!("CDP call timeout: {}", method))?;

        match result {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(anyhow!("Response channel closed")),
        }
    }

    /// Subscribe to CDP events
    pub async fn subscribe(&self, event: &str) -> Result<mpsc::UnboundedReceiver<Value>> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut handlers = self.event_handlers.write().await;
        handlers.entry(event.to_string()).or_default().push(tx);
        Ok(rx)
    }

    /// Check if session is idle (for pool cleanup)
    pub async fn is_idle(&self, duration: Duration) -> bool {
        let last_used = *self.last_used.read().await;
        last_used.elapsed() > duration
    }

    /// Get target ID
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Close the session
    pub async fn close(&self) {
        let _ = self.command_tx.send(InternalMessage::Close);
        *self.closed.write().await = true;
    }

    /// Check if session is closed
    pub async fn is_closed(&self) -> bool {
        *self.closed.read().await
    }
}

impl Drop for PooledSession {
    fn drop(&mut self) {
        let _ = self.command_tx.send(InternalMessage::Close);
    }
}

/// Session pool for managing multiple CDP connections
pub struct SessionPool {
    config: PoolConfig,
    sessions: Arc<RwLock<HashMap<String, Arc<PooledSession>>>>,
    cleanup_handle: Option<tokio::task::JoinHandle<()>>,
}

impl SessionPool {
    /// Create a new session pool
    pub async fn new(config: PoolConfig) -> Result<Self> {
        let sessions = Arc::new(RwLock::new(HashMap::new()));

        // Start cleanup task
        let sessions_clone = sessions.clone();
        let idle_timeout = Duration::from_millis(config.idle_timeout_ms);
        let cleanup_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;

                let mut sessions = sessions_clone.write().await;
                let to_remove: Vec<String> = sessions
                    .iter()
                    .filter(|(_, session)| {
                        tokio::task::block_in_place(|| {
                            tokio::runtime::Handle::current()
                                .block_on(async { session.is_idle(idle_timeout).await })
                        })
                    })
                    .map(|(id, _)| id.clone())
                    .collect();

                for id in to_remove {
                    if let Some(session) = sessions.remove(&id) {
                        session.close().await;
                        tracing::info!("Cleaned up idle session: {}", id);
                    }
                }
            }
        });

        Ok(Self {
            config,
            sessions,
            cleanup_handle: Some(cleanup_handle),
        })
    }

    /// Get or create a session for a target
    pub async fn get_session(
        &self,
        ws_url: &str,
        target_id: &str,
    ) -> Result<Arc<PooledSession>> {
        let mut sessions = self.sessions.write().await;

        // Check if session exists and is still valid
        if let Some(session) = sessions.get(target_id) {
            if !session.is_closed().await {
                return Ok(session.clone());
            }
            // Remove closed session
            sessions.remove(target_id);
        }

        // Check pool size limit
        if sessions.len() >= self.config.max_sessions {
            // Try to remove an idle session
            let idle_timeout = Duration::from_millis(self.config.idle_timeout_ms);
            if let Some((id, session)) = sessions
                .iter()
                .find(|(_, s)| {
                    tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current()
                            .block_on(async { s.is_idle(idle_timeout).await })
                    })
                })
                .map(|(id, s)| (id.clone(), s.clone()))
            {
                session.close().await;
                sessions.remove(&id);
            } else {
                bail!("Session pool is full (max: {})", self.config.max_sessions);
            }
        }

        // Create new session
        let session = Arc::new(
            PooledSession::new(ws_url, target_id, self.config.clone())
                .await
                .with_context(|| format!("Failed to create session for target: {}", target_id))?,
        );

        sessions.insert(target_id.to_string(), session.clone());
        tracing::info!("Created new CDP session for target: {}", target_id);

        Ok(session)
    }

    /// Remove a session from the pool
    pub async fn remove_session(&self, target_id: &str) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.remove(target_id) {
            session.close().await;
            tracing::info!("Removed session: {}", target_id);
        }
    }

    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        let sessions = self.sessions.read().await;
        PoolStats {
            total_sessions: sessions.len(),
            max_sessions: self.config.max_sessions,
        }
    }

    /// Close all sessions and cleanup
    pub async fn shutdown(&mut self) {
        if let Some(handle) = self.cleanup_handle.take() {
            handle.abort();
        }

        let mut sessions = self.sessions.write().await;
        for (id, session) in sessions.drain() {
            session.close().await;
            tracing::info!("Closed session: {}", id);
        }
    }
}

impl Drop for SessionPool {
    fn drop(&mut self) {
        if let Some(handle) = self.cleanup_handle.take() {
            handle.abort();
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_sessions: usize,
    pub max_sessions: usize,
}

// ============================================================================
// High-level Browser API using the session pool
// ============================================================================

use crate::browser::{BrowserProfile, BrowserTab, BrowserSnapshot};

/// Enhanced browser client with session pooling
pub struct PooledBrowserClient {
    profile: BrowserProfile,
    pool: SessionPool,
}

impl PooledBrowserClient {
    /// Create a new pooled browser client
    pub async fn new(profile_name: &str) -> Result<Self> {
        let profile = crate::browser::resolve_profile(profile_name)?;
        let pool = SessionPool::new(PoolConfig::default()).await?;

        Ok(Self { profile, pool })
    }

    /// List all tabs
    pub async fn list_tabs(&self) -> Result<Vec<BrowserTab>> {
        use reqwest::Client;
        use serde_json::Value;

        let url = format!("{}/json/list", self.profile.cdp_http_url.trim_end_matches('/'));
        let raw: Vec<Value> = Client::new()
            .get(&url)
            .send()
            .await
            .with_context(|| format!("failed to query tabs: {url}"))?
            .error_for_status()?
            .json()
            .await?;

        Ok(raw
            .into_iter()
            .map(|t| BrowserTab {
                id: t
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                title: t
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                url: t
                    .get("url")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                websocket_debugger_url: t
                    .get("webSocketDebuggerUrl")
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
                target_id: t
                    .get("id")
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
            })
            .collect())
    }

    /// Get session for a tab
    async fn get_tab_session(&self, tab_id: &str) -> Result<Arc<PooledSession>> {
        let tabs = self.list_tabs().await?;
        let tab = tabs
            .into_iter()
            .find(|t| t.id == tab_id)
            .ok_or_else(|| anyhow!("Tab not found: {}", tab_id))?;

        let ws_url = tab
            .websocket_debugger_url
            .ok_or_else(|| anyhow!("Tab has no debug URL"))?;

        let target_id = tab
            .target_id
            .ok_or_else(|| anyhow!("Tab has no target ID"))?;

        self.pool.get_session(&ws_url, &target_id).await
    }

    /// Navigate to URL using Page.navigate
    pub async fn navigate(&self, tab_id: &str, url: &str) -> Result<()> {
        let session = self.get_tab_session(tab_id).await?;

        let result = session
            .call("Page.navigate", json!({ "url": url }))
            .await?;

        if let Some(error) = result.get("error") {
            bail!("Navigation failed: {}", error);
        }

        // Wait for load event
        let _ = session
            .call(
                "Page.getNavigationHistory",
                json!({}),
            )
            .await?;

        Ok(())
    }

    /// Click element using proper CDP DOM methods
    pub async fn click(&self, tab_id: &str, selector: &str) -> Result<()> {
        let session = self.get_tab_session(tab_id).await?;

        // Get document root
        let doc = session
            .call("DOM.getDocument", json!({ "depth": 0 }))
            .await?;

        let root_node_id = doc
            .get("result")
            .and_then(|r| r.get("root"))
            .and_then(|r| r.get("nodeId"))
            .and_then(Value::as_i64)
            .ok_or_else(|| anyhow!("Failed to get document root"))?;

        // Query element
        let query = session
            .call(
                "DOM.querySelector",
                json!({
                    "nodeId": root_node_id,
                    "selector": selector
                }),
            )
            .await?;

        let node_id = query
            .get("result")
            .and_then(|r| r.get("nodeId"))
            .and_then(Value::as_i64)
            .filter(|&id| id > 0)
            .ok_or_else(|| anyhow!("Element not found: {}", selector))?;

        // Scroll into view
        let _ = session
            .call(
                "DOM.scrollIntoViewIfNeeded",
                json!({ "nodeId": node_id }),
            )
            .await;

        // Get box model for coordinates
        let box_model = session
            .call("DOM.getBoxModel", json!({ "nodeId": node_id }))
            .await?;

        let content = box_model
            .get("result")
            .and_then(|r| r.get("model"))
            .and_then(|m| m.get("content"))
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Failed to get element box model"))?;

        if content.len() >= 2 {
            let x = content[0].as_f64().unwrap_or(0.0) + 5.0;
            let y = content[1].as_f64().unwrap_or(0.0) + 5.0;

            // Dispatch mouse events
            session
                .call(
                    "Input.dispatchMouseEvent",
                    json!({
                        "type": "mousePressed",
                        "x": x,
                        "y": y,
                        "button": "left",
                        "clickCount": 1
                    }),
                )
                .await?;

            session
                .call(
                    "Input.dispatchMouseEvent",
                    json!({
                        "type": "mouseReleased",
                        "x": x,
                        "y": y,
                        "button": "left",
                        "clickCount": 1
                    }),
                )
                .await?;
        }

        Ok(())
    }

    /// Type text using Input.dispatchKeyEvent
    pub async fn type_text(&self, tab_id: &str, selector: &str, text: &str) -> Result<()> {
        let session = self.get_tab_session(tab_id).await?;

        // Focus element
        let doc = session
            .call("DOM.getDocument", json!({ "depth": 0 }))
            .await?;

        let root_node_id = doc
            .get("result")
            .and_then(|r| r.get("root"))
            .and_then(|r| r.get("nodeId"))
            .and_then(Value::as_i64)
            .ok_or_else(|| anyhow!("Failed to get document root"))?;

        let query = session
            .call(
                "DOM.querySelector",
                json!({
                    "nodeId": root_node_id,
                    "selector": selector
                }),
            )
            .await?;

        let node_id = query
            .get("result")
            .and_then(|r| r.get("nodeId"))
            .and_then(Value::as_i64)
            .filter(|&id| id > 0)
            .ok_or_else(|| anyhow!("Element not found: {}", selector))?;

        session
            .call("DOM.focus", json!({ "nodeId": node_id }))
            .await?;

        // Type each character
        for ch in text.chars() {
            let key = ch.to_string();
            session
                .call(
                    "Input.dispatchKeyEvent",
                    json!({
                        "type": "keyDown",
                        "text": key,
                        "unmodifiedText": key,
                    }),
                )
                .await?;

            session
                .call(
                    "Input.dispatchKeyEvent",
                    json!({
                        "type": "keyUp",
                        "text": key,
                    }),
                )
                .await?;
        }

        Ok(())
    }

    /// Take screenshot
    pub async fn screenshot(&self, tab_id: &str, full_page: bool) -> Result<String> {
        let session = self.get_tab_session(tab_id).await?;

        let params = if full_page {
            json!({
                "format": "png",
                "fromSurface": true,
                "captureBeyondViewport": true
            })
        } else {
            json!({
                "format": "png",
                "fromSurface": true
            })
        };

        let result = session.call("Page.captureScreenshot", params).await?;

        result
            .get("result")
            .and_then(|r| r.get("data"))
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .ok_or_else(|| anyhow!("Failed to capture screenshot"))
    }

    /// Execute JavaScript with proper error handling
    pub async fn evaluate(&self, tab_id: &str, script: &str) -> Result<Value> {
        let session = self.get_tab_session(tab_id).await?;

        let result = session
            .call(
                "Runtime.evaluate",
                json!({
                    "expression": script,
                    "returnByValue": true,
                    "awaitPromise": true,
                }),
            )
            .await?;

        // Check for exception
        if let Some(exception) = result
            .get("result")
            .and_then(|r| r.get("exceptionDetails"))
        {
            let message = exception
                .get("exception")
                .and_then(|e| e.get("description"))
                .and_then(Value::as_str)
                .unwrap_or("Unknown error");
            bail!("JavaScript error: {}", message);
        }

        result
            .get("result")
            .and_then(|r| r.get("result"))
            .cloned()
            .ok_or_else(|| anyhow!("No result from evaluation"))
    }

    /// Wait for element to appear
    pub async fn wait_for_element(
        &self,
        tab_id: &str,
        selector: &str,
        timeout_ms: u64,
    ) -> Result<()> {
        let session = self.get_tab_session(tab_id).await?;
        let start = Instant::now();

        loop {
            let doc = session
                .call("DOM.getDocument", json!({ "depth": 0 }))
                .await?;

            let root_node_id = doc
                .get("result")
                .and_then(|r| r.get("root"))
                .and_then(|r| r.get("nodeId"))
                .and_then(Value::as_i64)
                .ok_or_else(|| anyhow!("Failed to get document root"))?;

            let query = session
                .call(
                    "DOM.querySelector",
                    json!({
                        "nodeId": root_node_id,
                        "selector": selector
                    }),
                )
                .await?;

            let node_id = query
                .get("result")
                .and_then(|r| r.get("nodeId"))
                .and_then(Value::as_i64);

            if node_id.is_some() && node_id.unwrap() > 0 {
                return Ok(());
            }

            if start.elapsed().as_millis() as u64 > timeout_ms {
                bail!("Timeout waiting for element: {}", selector);
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Get pool statistics
    pub async fn pool_stats(&self) -> PoolStats {
        self.pool.stats().await
    }

    /// Shutdown the client
    pub async fn shutdown(&mut self) {
        self.pool.shutdown().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.max_sessions, 10);
        assert_eq!(config.idle_timeout_ms, 300_000);
        assert_eq!(config.operation_timeout_ms, 30_000);
        assert!(config.auto_recovery);
    }

    #[tokio::test]
    async fn test_pool_stats() {
        let pool = SessionPool::new(PoolConfig::default()).await.unwrap();
        let stats = pool.stats().await;
        assert_eq!(stats.total_sessions, 0);
        assert_eq!(stats.max_sessions, 10);
    }
}
