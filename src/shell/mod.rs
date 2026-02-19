//! Interactive shell - simple REPL over gateway WebSocket.

use anyhow::{anyhow, Context};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Clone)]
pub struct ShellConfig {
    pub url: String,
    pub token: Option<String>,
    pub session: String,
}

pub async fn run_interactive_shell(config: ShellConfig) -> anyhow::Result<()> {
    let ws_url = to_ws_url(&config.url)?;
    let (mut ws, _) = connect_async(&ws_url)
        .await
        .with_context(|| format!("failed to connect to gateway websocket: {ws_url}"))?;

    println!("krabkrab Shell");
    println!("==============");
    println!("URL: {ws_url}");
    println!("Session: {}", config.session);
    if config.token.is_some() {
        println!("Token: provided");
    }
    println!("Type text to run memory/search, /exit to quit.");
    println!();

    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    while let Some(line) = lines.next_line().await? {
        let query = line.trim();
        if query.is_empty() {
            continue;
        }
        if query.eq_ignore_ascii_case("/exit") || query.eq_ignore_ascii_case("/quit") {
            break;
        }

        let req = GatewaySearchRequest {
            msg_type: "memory/search".to_string(),
            query: query.to_string(),
        };
        let payload = serde_json::to_string(&req)?;
        ws.send(Message::Text(payload)).await?;

        match ws.next().await {
            Some(Ok(Message::Text(text))) => render_response(&text),
            Some(Ok(_)) => println!("[gateway] unexpected non-text response"),
            Some(Err(err)) => return Err(anyhow!("gateway websocket error: {err}")),
            None => return Err(anyhow!("gateway websocket closed")),
        }
    }

    let _ = ws.close(None).await;
    Ok(())
}

fn to_ws_url(input: &str) -> anyhow::Result<String> {
    let base = input.trim();
    if base.is_empty() {
        return Err(anyhow!("shell url is required"));
    }

    let has_scheme = base.contains("://");
    let normalized = if has_scheme {
        base.to_string()
    } else {
        format!("http://{base}")
    };

    let mut url = reqwest::Url::parse(&normalized)
        .with_context(|| format!("invalid shell url: {base}"))?;

    match url.scheme() {
        "http" => {
            let _ = url.set_scheme("ws");
        }
        "https" => {
            let _ = url.set_scheme("wss");
        }
        "ws" | "wss" => {}
        other => return Err(anyhow!("unsupported url scheme: {other}")),
    }

    if url.path().is_empty() || url.path() == "/" {
        url.set_path("/ws");
    }

    Ok(url.to_string())
}

fn render_response(text: &str) {
    let parsed: Result<GatewaySearchResponse, _> = serde_json::from_str(text);
    match parsed {
        Ok(GatewaySearchResponse::MemoryResults { results }) => {
            if results.is_empty() {
                println!("(no results)");
                return;
            }
            for (idx, item) in results.iter().enumerate() {
                println!(
                    "{}. {}:{}-{} score={:.3}\n   {}",
                    idx + 1,
                    item.path,
                    item.start_line,
                    item.end_line,
                    item.score,
                    single_line_preview(&item.text)
                );
            }
        }
        Ok(GatewaySearchResponse::Error { message }) => {
            println!("[gateway error] {message}");
        }
        Err(_) => {
            println!("[gateway raw] {text}");
        }
    }
}

fn single_line_preview(input: &str) -> String {
    let mut out = input.lines().next().unwrap_or_default().trim().to_string();
    if out.len() > 180 {
        out.truncate(180);
        out.push_str("...");
    }
    out
}

#[derive(Debug, Serialize)]
struct GatewaySearchRequest {
    #[serde(rename = "type")]
    msg_type: String,
    query: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum GatewaySearchResponse {
    #[serde(rename = "memory/results")]
    MemoryResults { results: Vec<SearchItem> },
    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    path: String,
    start_line: i32,
    end_line: i32,
    text: String,
    score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_config_defaults() {
        let config = ShellConfig {
            url: "http://localhost:18789".to_string(),
            token: None,
            session: "main".to_string(),
        };
        assert_eq!(config.session, "main");
    }

    #[test]
    fn ws_url_defaults_path() {
        let u = to_ws_url("http://localhost:3000").expect("url");
        assert_eq!(u, "ws://localhost:3000/ws");
    }

    #[test]
    fn ws_url_preserves_ws_scheme() {
        let u = to_ws_url("wss://example.com/custom").expect("url");
        assert_eq!(u, "wss://example.com/custom");
    }

    #[test]
    fn ws_url_rejects_unsupported_scheme() {
        let err = to_ws_url("ftp://example.com").expect_err("must fail");
        assert!(err.to_string().contains("unsupported url scheme"));
    }
}
