use serde::{Deserialize, Serialize};

pub struct SignalClient;

impl SignalClient {
    pub fn new() -> Self {
        Self
    }
}

/// SSE Event for Signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseEvent {
    pub event: String,
    pub data: String,
    pub id: Option<String>,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: Option<String>,
}

/// Perform health check on Signal daemon
pub async fn health_check(url: &str) -> anyhow::Result<HealthCheckResponse> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", url))
        .send()
        .await?;
    
    if response.status().is_success() {
        let health = response.json::<HealthCheckResponse>().await?;
        Ok(health)
    } else {
        Err(anyhow::anyhow!("Health check failed: {}", response.status()))
    }
}
