use fantoccini::{Client, ClientBuilder};
use std::time::Duration;

pub struct Browser {
    webdriver_url: String,
}

impl Browser {
    pub fn new(webdriver_url: impl Into<String>) -> Self {
        Self { webdriver_url: webdriver_url.into() }
    }

    pub async fn open(&self, url: &str) -> anyhow::Result<String> {
        let client = Client::with_capabilities(&self.webdriver_url, serde_json::json!({})).await?;
        client.goto(url).await?;
        let title = client.title().await?;
        client.close().await.ok();
        Ok(title)
    }
}
