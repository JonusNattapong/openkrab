use crate::common::Message;
use crate::connectors::telegram_client;
use crate::common::UserId;
use std::time::Duration;
use tokio::time::sleep;

pub fn normalize_inbound(text: &str, chat_id: i64, user_id: i64) -> Message {
    Message {
        id: format!("tg:{chat_id}:{user_id}"),
        text: text.to_string(),
        from: Some(UserId(format!("tg:{user_id}"))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[telegram] {text}")
}

/// Simple long-polling loop for Telegram updates.
/// This runs indefinitely until an error occurs or the process stops.
pub async fn monitor(state: std::sync::Arc<crate::gateway::GatewayState>, token: String) {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(40))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
        
    let mut offset: Option<i64> = None;
    println!("[telegram] Starting monitor loop...");

    loop {
        match telegram_client::get_updates(&client, &token, offset, Some(30)).await {
            Ok(val) => {
                if let Some(result) = val.get("result").and_then(|r| r.as_array()) {
                    for update in result {
                        // Update offset to potential next update_id
                        if let Some(upd_id) = update.get("update_id").and_then(|v| v.as_i64()) {
                            offset = Some(upd_id + 1);
                        }
                        
                        // Extract message text if present
                        if let Some(msg) = update.get("message") {
                             if let Some(text) = msg.get("text").and_then(|t| t.as_str()) {
                                 let chat_id = msg.get("chat").and_then(|c| c.get("id")).and_then(|id| id.as_i64()).unwrap_or(0);
                                 let user_id = msg.get("from").and_then(|f| f.get("id")).and_then(|id| id.as_i64()).unwrap_or(0);
                                 
                                 let normalized = normalize_inbound(text, chat_id, user_id);
                                 println!("Received telegram msg: {:?}", normalized);
                                 
                                 // Simple echo for testing (optional, maybe too dangerous for auto-loop)
                                 // let _ = telegram_client::send_message(&client, &token, &chat_id.to_string(), &format!("Echo: {}", text), None).await;

                                 // Dispatch to Agent
                                 let state_clone = state.clone();
                                 let token_clone = token.clone();
                                 let client_clone = client.clone();
                                 let text_owned = text.to_string();
                                 let chat_id_str = chat_id.to_string();

                                 tokio::spawn(async move {
                                     println!("[telegram] Processing with agent: {}", text_owned);
                                     match state_clone.agent.answer(&text_owned).await {
                                         Ok(answer) => {
                                             if let Err(e) = telegram_client::send_message(&client_clone, &token_clone, &chat_id_str, &answer, None).await {
                                                 eprintln!("[telegram] Failed to send reply: {}", e);
                                             }
                                         },
                                         Err(e) => {
                                             eprintln!("[telegram] Agent error: {}", e);
                                             let _ = telegram_client::send_message(&client_clone, &token_clone, &chat_id_str, &format!("unavailable: {}", e), None).await;
                                         }
                                     }
                                 });
                             }
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("[telegram] polling error: {}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_outbound() {
        assert_eq!(format_outbound("hi"), "[telegram] hi".to_string());
    }
}
