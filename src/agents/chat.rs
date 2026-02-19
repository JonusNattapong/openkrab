use crate::agents::tool::{ToolDefinition, ToolCall};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    System { content: String },
    User { content: UserContent },
    Assistant { 
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    Tool { 
        tool_call_id: String,
        content: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[async_trait]
pub trait ChatProvider: Send + Sync {
    async fn complete(&self, messages: Vec<ChatMessage>, tools: Option<&[ToolDefinition]>) -> Result<ChatCompletionResponse>;
}

pub struct OpenAiChatProvider {
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenAiChatProvider {
    pub fn new(api_key: String, base_url: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            model: model.unwrap_or_else(|| "gpt-4o".to_string()),
        }
    }
}

#[async_trait]
impl ChatProvider for OpenAiChatProvider {
    async fn complete(&self, messages: Vec<ChatMessage>, tools: Option<&[ToolDefinition]>) -> Result<ChatCompletionResponse> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.base_url);

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
        });

        if let Some(t) = tools {
            if !t.is_empty() {
                body["tools"] = serde_json::json!(t.iter().map(|td| {
                    serde_json::json!({
                        "type": "function",
                        "function": td,
                    })
                }).collect::<Vec<_>>());
            }
        }

        let res = client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {} - {}", status, error_text));
        }

        let json: serde_json::Value = res.json().await?;
        let choice = &json["choices"][0];
        let message_val = &choice["message"];
        
        let role = message_val["role"].as_str().unwrap_or("assistant");
        let content = message_val["content"].as_str().map(|s| s.to_string());
        
        let tool_calls = if let Some(calls) = message_val["tool_calls"].as_array() {
            let mut parsed_calls = Vec::new();
            for call in calls {
                parsed_calls.push(ToolCall {
                    id: call["id"].as_str().unwrap_or_default().to_string(),
                    name: call["function"]["name"].as_str().unwrap_or_default().to_string(),
                    arguments: call["function"]["arguments"].as_str().unwrap_or_default().to_string(),
                });
            }
            Some(parsed_calls)
        } else {
            None
        };

        let finish_reason = choice["finish_reason"].as_str().unwrap_or("stop").to_string();

        let message = match role {
            "assistant" => ChatMessage::Assistant { content, tool_calls },
            _ => return Err(anyhow::anyhow!("Unexpected role from OpenAI: {}", role)),
        };

        Ok(ChatCompletionResponse {
            message,
            finish_reason,
        })
    }
}
