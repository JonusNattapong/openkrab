//! Network domain support for request interception and monitoring
//!
//! This module provides CDP Network domain functionality for:
//! - Request/response interception
//! - Network monitoring and logging
//! - Request modification and blocking
//! - HAR-style network recording

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::browser::pool::PooledSession;

/// Network interception patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterceptPattern {
    pub url_pattern: String,
    pub resource_type: Option<ResourceType>,
    pub interception_stage: Option<InterceptionStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceType {
    Document,
    Stylesheet,
    Image,
    Media,
    Font,
    Script,
    Xhr,
    Fetch,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InterceptionStage {
    Request,
    HeadersReceived,
    Response,
}

/// Network request data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub request_id: String,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub post_data: Option<String>,
    pub resource_type: String,
    pub timestamp: f64,
}

/// Network response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResponse {
    pub request_id: String,
    pub url: String,
    pub status: i64,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub mime_type: String,
    pub timestamp: f64,
}

/// Network event types
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    RequestWillBeSent(NetworkRequest),
    ResponseReceived(NetworkResponse),
    LoadingFinished { request_id: String, timestamp: f64 },
    LoadingFailed { request_id: String, error_text: String },
    RequestIntercepted(InterceptedRequest),
}

/// Intercepted request for modification
#[derive(Debug, Clone)]
pub struct InterceptedRequest {
    pub interception_id: String,
    pub request: NetworkRequest,
    pub response_status_code: Option<i64>,
    pub response_headers: Option<HashMap<String, String>>,
}

/// Network manager for a CDP session
pub struct NetworkManager {
    session: Arc<PooledSession>,
    requests: Arc<RwLock<HashMap<String, NetworkRequest>>>,
    responses: Arc<RwLock<HashMap<String, NetworkResponse>>>,
    event_tx: mpsc::UnboundedSender<NetworkEvent>,
    event_rx: Arc<RwLock<mpsc::UnboundedReceiver<NetworkEvent>>>,
    intercept_enabled: Arc<RwLock<bool>>,
}

impl NetworkManager {
    /// Create a new network manager for a session
    pub async fn new(session: Arc<PooledSession>) -> Result<Self> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let manager = Self {
            session,
            requests: Arc::new(RwLock::new(HashMap::new())),
            responses: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            event_rx: Arc::new(RwLock::new(event_rx)),
            intercept_enabled: Arc::new(RwLock::new(false)),
        };

        // Enable network domain
        manager.enable().await?;

        // Start event listeners
        manager.start_event_listeners().await?;

        Ok(manager)
    }

    /// Enable network domain
    async fn enable(&self) -> Result<()> {
        self.session
            .call("Network.enable", json!({}))
            .await
            .context("Failed to enable Network domain")?;
        Ok(())
    }

    /// Disable network domain
    pub async fn disable(&self) -> Result<()> {
        self.session
            .call("Network.disable", json!({}))
            .await
            .context("Failed to disable Network domain")?;
        Ok(())
    }

    /// Start listening for network events
    async fn start_event_listeners(&self) -> Result<()> {
        let requests = self.requests.clone();
        let responses = self.responses.clone();
        let event_tx = self.event_tx.clone();
        let session = self.session.clone();

        // Subscribe to Network.requestWillBeSent
        let mut req_rx = session.subscribe("Network.requestWillBeSent").await?;
        tokio::spawn(async move {
            while let Some(params) = req_rx.recv().await {
                let request = NetworkRequest {
                    request_id: params
                        .get("requestId")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    url: params
                        .get("request")
                        .and_then(|r| r.get("url"))
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    method: params
                        .get("request")
                        .and_then(|r| r.get("method"))
                        .and_then(Value::as_str)
                        .unwrap_or("GET")
                        .to_string(),
                    headers: params
                        .get("request")
                        .and_then(|r| r.get("headers"))
                        .and_then(Value::as_object)
                        .map(|h| {
                            h.iter()
                                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                                .collect()
                        })
                        .unwrap_or_default(),
                    post_data: params
                        .get("request")
                        .and_then(|r| r.get("postData"))
                        .and_then(Value::as_str)
                        .map(ToString::to_string),
                    resource_type: params
                        .get("type")
                        .and_then(Value::as_str)
                        .unwrap_or("Other")
                        .to_string(),
                    timestamp: params
                        .get("timestamp")
                        .and_then(Value::as_f64)
                        .unwrap_or(0.0),
                };

                let mut reqs = requests.write().await;
                reqs.insert(request.request_id.clone(), request.clone());

                let _ = event_tx.send(NetworkEvent::RequestWillBeSent(request));
            }
        });

        // Subscribe to Network.responseReceived
        let mut resp_rx = session.subscribe("Network.responseReceived").await?;
        let responses_clone = self.responses.clone();
        let event_tx_clone = self.event_tx.clone();
        tokio::spawn(async move {
            while let Some(params) = resp_rx.recv().await {
                let response = NetworkResponse {
                    request_id: params
                        .get("requestId")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    url: params
                        .get("response")
                        .and_then(|r| r.get("url"))
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    status: params
                        .get("response")
                        .and_then(|r| r.get("status"))
                        .and_then(Value::as_i64)
                        .unwrap_or(0),
                    status_text: params
                        .get("response")
                        .and_then(|r| r.get("statusText"))
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    headers: params
                        .get("response")
                        .and_then(|r| r.get("headers"))
                        .and_then(Value::as_object)
                        .map(|h| {
                            h.iter()
                                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                                .collect()
                        })
                        .unwrap_or_default(),
                    mime_type: params
                        .get("response")
                        .and_then(|r| r.get("mimeType"))
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    timestamp: params
                        .get("timestamp")
                        .and_then(Value::as_f64)
                        .unwrap_or(0.0),
                };

                let mut resps = responses_clone.write().await;
                resps.insert(response.request_id.clone(), response.clone());

                let _ = event_tx_clone.send(NetworkEvent::ResponseReceived(response));
            }
        });

        // Subscribe to Network.loadingFinished
        let mut finish_rx = session.subscribe("Network.loadingFinished").await?;
        let event_tx_clone = self.event_tx.clone();
        tokio::spawn(async move {
            while let Some(params) = finish_rx.recv().await {
                let request_id = params
                    .get("requestId")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                let timestamp = params
                    .get("timestamp")
                    .and_then(Value::as_f64)
                    .unwrap_or(0.0);

                let _ = event_tx_clone.send(NetworkEvent::LoadingFinished {
                    request_id,
                    timestamp,
                });
            }
        });

        // Subscribe to Network.loadingFailed
        let mut fail_rx = session.subscribe("Network.loadingFailed").await?;
        let event_tx_clone = self.event_tx.clone();
        tokio::spawn(async move {
            while let Some(params) = fail_rx.recv().await {
                let request_id = params
                    .get("requestId")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                let error_text = params
                    .get("errorText")
                    .and_then(Value::as_str)
                    .unwrap_or("Unknown error")
                    .to_string();

                let _ = event_tx_clone.send(NetworkEvent::LoadingFailed {
                    request_id,
                    error_text,
                });
            }
        });

        Ok(())
    }

    /// Set extra HTTP headers for all requests
    pub async fn set_extra_headers(&self, headers: HashMap<String, String>) -> Result<()> {
        self.session
            .call("Network.setExtraHTTPHeaders", json!({ "headers": headers }))
            .await
            .context("Failed to set extra headers")?;
        Ok(())
    }

    /// Clear browser cache
    pub async fn clear_cache(&self) -> Result<()> {
        self.session
            .call("Network.clearBrowserCache", json!({}))
            .await
            .context("Failed to clear cache")?;
        Ok(())
    }

    /// Set user agent override
    pub async fn set_user_agent(&self, user_agent: &str, accept_language: Option<&str>) -> Result<()> {
        let mut params = json!({ "userAgent": user_agent });
        if let Some(lang) = accept_language {
            params["acceptLanguage"] = json!(lang);
        }

        self.session
            .call("Network.setUserAgentOverride", params)
            .await
            .context("Failed to set user agent")?;
        Ok(())
    }

    /// Enable request interception
    pub async fn enable_interception(&self, patterns: Vec<InterceptPattern>) -> Result<()> {
        let patterns_json: Vec<Value> = patterns
            .into_iter()
            .map(|p| {
                let mut obj = json!({
                    "urlPattern": p.url_pattern,
                });
                if let Some(rt) = p.resource_type {
                    obj["resourceType"] = json!(rt);
                }
                if let Some(stage) = p.interception_stage {
                    obj["interceptionStage"] = json!(stage);
                }
                obj
            })
            .collect();

        self.session
            .call(
                "Fetch.enable",
                json!({
                    "patterns": patterns_json,
                    "handleAuthRequests": true
                }),
            )
            .await
            .context("Failed to enable request interception")?;

        *self.intercept_enabled.write().await = true;

        // Start handling intercepted requests
        self.start_interception_handler().await?;

        Ok(())
    }

    /// Start handling intercepted requests
    async fn start_interception_handler(&self) -> Result<()> {
        let mut rx = self.session.subscribe("Fetch.requestPaused").await?;
        let session = self.session.clone();
        let event_tx = self.event_tx.clone();

        tokio::spawn(async move {
            while let Some(params) = rx.recv().await {
                let interception_id = params
                    .get("requestId")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();

                let request = NetworkRequest {
                    request_id: interception_id.clone(),
                    url: params
                        .get("request")
                        .and_then(|r| r.get("url"))
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    method: params
                        .get("request")
                        .and_then(|r| r.get("method"))
                        .and_then(Value::as_str)
                        .unwrap_or("GET")
                        .to_string(),
                    headers: params
                        .get("request")
                        .and_then(|r| r.get("headers"))
                        .and_then(Value::as_object)
                        .map(|h| {
                            h.iter()
                                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                                .collect()
                        })
                        .unwrap_or_default(),
                    post_data: None,
                    resource_type: params
                        .get("resourceType")
                        .and_then(Value::as_str)
                        .unwrap_or("Other")
                        .to_string(),
                    timestamp: 0.0,
                };

                let intercepted = InterceptedRequest {
                    interception_id: interception_id.clone(),
                    request,
                    response_status_code: params
                        .get("responseStatusCode")
                        .and_then(Value::as_i64),
                    response_headers: None,
                };

                let _ = event_tx.send(NetworkEvent::RequestIntercepted(intercepted));

                // Default: continue the request
                let _ = session
                    .call(
                        "Fetch.continueRequest",
                        json!({ "requestId": interception_id }),
                    )
                    .await;
            }
        });

        Ok(())
    }

    /// Continue an intercepted request
    pub async fn continue_intercepted(
        &self,
        interception_id: &str,
        modifications: Option<RequestModifications>,
    ) -> Result<()> {
        let mut params = json!({ "requestId": interception_id });

        if let Some(mods) = modifications {
            if let Some(url) = mods.url {
                params["url"] = json!(url);
            }
            if let Some(method) = mods.method {
                params["method"] = json!(method);
            }
            if let Some(headers) = mods.headers {
                params["headers"] = json!(headers);
            }
            if let Some(post_data) = mods.post_data {
                params["postData"] = json!(post_data);
            }
        }

        self.session
            .call("Fetch.continueRequest", params)
            .await
            .context("Failed to continue intercepted request")?;

        Ok(())
    }

    /// Fulfill an intercepted request with a mock response
    pub async fn fulfill_intercepted(
        &self,
        interception_id: &str,
        response_code: i64,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<()> {
        let mut params = json!({
            "requestId": interception_id,
            "responseCode": response_code,
        });

        if let Some(body) = body {
            // Base64 encode body
            let encoded = base64::encode(body);
            params["body"] = json!(encoded);
        }

        if let Some(headers) = headers {
            let header_array: Vec<Value> = headers
                .into_iter()
                .map(|(name, value)| json!({ "name": name, "value": value }))
                .collect();
            params["responseHeaders"] = json!(header_array);
        }

        self.session
            .call("Fetch.fulfillRequest", params)
            .await
            .context("Failed to fulfill intercepted request")?;

        Ok(())
    }

    /// Fail an intercepted request
    pub async fn fail_intercepted(&self, interception_id: &str, error_reason: &str) -> Result<()> {
        self.session
            .call(
                "Fetch.failRequest",
                json!({
                    "requestId": interception_id,
                    "errorReason": error_reason
                }),
            )
            .await
            .context("Failed to fail intercepted request")?;

        Ok(())
    }

    /// Get response body for a request
    pub async fn get_response_body(&self, request_id: &str) -> Result<(String, bool)> {
        let result = self
            .session
            .call(
                "Network.getResponseBody",
                json!({ "requestId": request_id }),
            )
            .await
            .context("Failed to get response body")?;

        let body = result
            .get("result")
            .and_then(|r| r.get("body"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();

        let base64_encoded = result
            .get("result")
            .and_then(|r| r.get("base64Encoded"))
            .and_then(Value::as_bool)
            .unwrap_or(false);

        Ok((body, base64_encoded))
    }

    /// Get all recorded requests
    pub async fn get_requests(&self) -> Vec<NetworkRequest> {
        let requests = self.requests.read().await;
        requests.values().cloned().collect()
    }

    /// Get all recorded responses
    pub async fn get_responses(&self) -> Vec<NetworkResponse> {
        let responses = self.responses.read().await;
        responses.values().cloned().collect()
    }

    /// Get event receiver
    pub fn event_receiver(&self) -> Arc<RwLock<mpsc::UnboundedReceiver<NetworkEvent>>> {
        self.event_rx.clone()
    }

    /// Clear recorded network data
    pub async fn clear(&self) {
        let mut requests = self.requests.write().await;
        requests.clear();

        let mut responses = self.responses.write().await;
        responses.clear();
    }
}

/// Request modifications for interception
#[derive(Debug, Clone, Default)]
pub struct RequestModifications {
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: Option<Vec<HashMap<String, String>>>,
    pub post_data: Option<String>,
}

impl RequestModifications {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn with_post_data(mut self, data: impl Into<String>) -> Self {
        self.post_data = Some(data.into());
        self
    }
}

/// HAR-style network recording
pub struct HarRecorder {
    entries: Vec<HarEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarEntry {
    pub started_date_time: String,
    pub time: f64,
    pub request: HarRequest,
    pub response: Option<HarResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HarHeader>,
    pub post_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarResponse {
    pub status: i64,
    pub status_text: String,
    pub headers: Vec<HarHeader>,
    pub content: HarContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarContent {
    pub size: i64,
    pub mime_type: String,
    pub text: Option<String>,
}

impl HarRecorder {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn record_request(&mut self, request: &NetworkRequest) {
        let entry = HarEntry {
            started_date_time: chrono::Utc::now().to_rfc3339(),
            time: 0.0,
            request: HarRequest {
                method: request.method.clone(),
                url: request.url.clone(),
                headers: request
                    .headers
                    .iter()
                    .map(|(k, v)| HarHeader {
                        name: k.clone(),
                        value: v.clone(),
                    })
                    .collect(),
                post_data: request.post_data.clone(),
            },
            response: None,
        };
        self.entries.push(entry);
    }

    pub fn record_response(&mut self, request_id: &str, response: &NetworkResponse) {
        if let Some(entry) = self.entries.iter_mut().find(|e| {
            // Match by URL since we don't have request_id in HAR entry
            e.request.url == response.url
        }) {
            entry.response = Some(HarResponse {
                status: response.status,
                status_text: response.status_text.clone(),
                headers: response
                    .headers
                    .iter()
                    .map(|(k, v)| HarHeader {
                        name: k.clone(),
                        value: v.clone(),
                    })
                    .collect(),
                content: HarContent {
                    size: 0,
                    mime_type: response.mime_type.clone(),
                    text: None,
                },
            });
        }
    }

    pub fn to_har(&self) -> Value {
        json!({
            "log": {
                "version": "1.2",
                "creator": {
                    "name": "OpenKrab Browser",
                    "version": "1.0"
                },
                "entries": self.entries
            }
        })
    }
}

impl Default for HarRecorder {
    fn default() -> Self {
        Self::new()
    }
}

// Base64 encoding helper
mod base64 {
    pub fn encode(input: String) -> String {
        use std::io::Write;
        let mut encoder = ::base64::write::EncoderStringWriter::new(&::base64::engine::general_purpose::STANDARD);
        encoder.write_all(input.as_bytes()).unwrap();
        encoder.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_modifications_builder() {
        let mods = RequestModifications::new()
            .with_url("https://example.com")
            .with_method("POST")
            .with_post_data("test data");

        assert_eq!(mods.url, Some("https://example.com".to_string()));
        assert_eq!(mods.method, Some("POST".to_string()));
        assert_eq!(mods.post_data, Some("test data".to_string()));
    }

    #[test]
    fn test_har_recorder() {
        let mut recorder = HarRecorder::new();

        let request = NetworkRequest {
            request_id: "test-1".to_string(),
            url: "https://example.com".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            post_data: None,
            resource_type: "Document".to_string(),
            timestamp: 0.0,
        };

        recorder.record_request(&request);
        assert_eq!(recorder.entries.len(), 1);

        let har = recorder.to_har();
        assert!(har.get("log").is_some());
    }
}
