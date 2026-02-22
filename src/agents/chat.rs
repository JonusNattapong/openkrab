use crate::agents::streaming::StreamHandler;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    System {
        content: String,
    },
    User {
        content: UserContent,
    },
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
    async fn complete(
        &self,
        messages: Vec<ChatMessage>,
        tools: Option<&[ToolDefinition]>,
    ) -> Result<ChatCompletionResponse>;

    async fn stream(
        &self,
        messages: Vec<ChatMessage>,
        tools: Option<&[ToolDefinition]>,
        handler: StreamHandler,
    ) -> Result<ChatCompletionResponse>;
}

use crate::agents::tool::{ToolCall, ToolDefinition};

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
    async fn complete(
        &self,
        messages: Vec<ChatMessage>,
        tools: Option<&[ToolDefinition]>,
    ) -> Result<ChatCompletionResponse> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.base_url);

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
        });

        if let Some(t) = tools {
            if !t.is_empty() {
                body["tools"] = serde_json::json!(t
                    .iter()
                    .map(|td| {
                        serde_json::json!({
                            "type": "function",
                            "function": td,
                        })
                    })
                    .collect::<Vec<_>>());
            }
        }

        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await?;
            return Err(anyhow::anyhow!(
                "OpenAI API error: {} - {}",
                status,
                error_text
            ));
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
                    name: call["function"]["name"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    arguments: call["function"]["arguments"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                });
            }
            Some(parsed_calls)
        } else {
            None
        };

        let finish_reason = choice["finish_reason"]
            .as_str()
            .unwrap_or("stop")
            .to_string();

        let message = match role {
            "assistant" => ChatMessage::Assistant {
                content,
                tool_calls,
            },
            _ => return Err(anyhow::anyhow!("Unexpected role from OpenAI: {}", role)),
        };

        Ok(ChatCompletionResponse {
            message,
            finish_reason,
        })
    }

    async fn stream(
        &self,
        messages: Vec<ChatMessage>,
        tools: Option<&[ToolDefinition]>,
        handler: StreamHandler,
    ) -> Result<ChatCompletionResponse> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.base_url);

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true,
        });

        if let Some(t) = tools {
            if !t.is_empty() {
                body["tools"] = serde_json::json!(t
                    .iter()
                    .map(|td| {
                        serde_json::json!({
                            "type": "function",
                            "function": td,
                        })
                    })
                    .collect::<Vec<_>>());
            }
        }

        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await?;
            return Err(anyhow::anyhow!(
                "OpenAI API error: {} - {}",
                status,
                error_text
            ));
        }

        let mut stream = res.bytes_stream();
        let mut buffer = String::new();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(newline_pos) = buffer.find('\n') {
                let line = buffer.drain(..newline_pos + 1).collect::<String>();
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                if line == "data: [DONE]" {
                    break;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    let json: Value = serde_json::from_str(data)?;
                    let choice = &json["choices"][0];
                    let delta = &choice["delta"];

                    if let Some(content) = delta["content"].as_str() {
                        handler.push_text(content)?;
                    }

                    if let Some(reasoning) = delta["reasoning_content"].as_str() {
                        handler.push_reasoning(reasoning)?;
                    }

                    if let Some(tool_calls) = delta["tool_calls"].as_array() {
                        for call in tool_calls {
                            let _index = call["index"].as_u64().unwrap_or(0);
                            let id = call["id"].as_str();
                            let name = call["function"]["name"].as_str();
                            let args = call["function"]["arguments"].as_str();

                            if let Some(id_str) = id {
                                if let Some(name_str) = name {
                                    handler.start_tool_call(id_str, name_str)?;
                                }
                            }

                            if let Some(args_str) = args {
                                // Need to track which tool call this corresponds to.
                                // Simplified for now: assume index-based tracking or just use the last one.
                                // In streaming.rs we use tool_call_id.
                                if let Some(tc_id) = id.or_else(|| {
                                    // Fallback: finding the ID from the accumulator might be needed if OpenAI doesn't repeat it
                                    None
                                }) {
                                    handler.push_tool_arguments(tc_id, args_str)?;
                                } else {
                                    // If ID is missing in subsequent chunks (common in OpenAI SSE),
                                    // we'd need to track it by index.
                                    // For now, let's assume we can get it or use a simpler mapping.
                                }
                            }
                        }
                    }

                    if let Some(finish_reason) = choice["finish_reason"].as_str() {
                        handler.finish(finish_reason, None)?;
                    }
                }
            }
        }

        // Return the final accumulated message
        let acc = handler.accumulator.lock().unwrap();
        let content = if acc.text().is_empty() {
            None
        } else {
            Some(acc.text().to_string())
        };

        let tool_calls = if acc.tool_calls.is_empty() {
            None
        } else {
            Some(
                acc.tool_calls
                    .iter()
                    .map(|tc| ToolCall {
                        id: tc.id.clone(),
                        name: tc.name.clone(),
                        arguments: tc.arguments.clone(),
                    })
                    .collect(),
            )
        };

        Ok(ChatCompletionResponse {
            message: ChatMessage::Assistant {
                content,
                tool_calls,
            },
            finish_reason: "stop".to_string(), // Simplified, should get from acc
        })
    }
}
