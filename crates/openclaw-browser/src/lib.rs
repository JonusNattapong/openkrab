use fantoccini::wdcapabilities::Capabilities;
use fantoccini::{Client, ClientBuilder};
use std::time::Duration;
use std::sync::Arc;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BrowserConfig {
    pub webdriver_url: String,
    pub timeout_seconds: Option<u64>,
}

pub struct Browser {
    webdriver_url: String,
    timeout: Duration,
}

impl Browser {
    pub fn new(cfg: BrowserConfig) -> Self {
        let timeout = Duration::from_secs(cfg.timeout_seconds.unwrap_or(60));
        Self { webdriver_url: cfg.webdriver_url, timeout }
    }

    pub async fn navigate_and_title(&self, url: &str) -> anyhow::Result<String> {
        let caps = Capabilities::new();
        let client = Client::with_capabilities(&self.webdriver_url, caps).await?;
        client.goto(url).await?;
        let title = client.title().await?;
        client.close().await.ok();
        Ok(title)
    }
}
