use crate::{LlmClient, LlmRequest, LlmResponse, LlmRole, ToolCall, TokenUsage, ToolDefinition};
use openclaw_errors::{OpenClawError, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

/// OpenAI-compatible LLM client
pub struct OpenAiClient {
    api_key: String,
    base_url: String,
    http_client: reqwest::Client,
}

impl OpenAiClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.openai.com/v1".to_string(),
            http_client: reqwest::Client::new(),
        }
    }
    
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
}

#[async_trait::async_trait]
impl LlmClient for OpenAiClient {
    async fn complete(&self, request: LlmRequest) -> Result<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .map_err(|e| OpenClawError::Config(format!("Invalid API key: {}", e)))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        let body = OpenAiRequest {
            model: request.model,
            messages: request.messages.into_iter().map(|m| OpenAiMessage {
                role: match m.role {
                    LlmRole::System => "system",
                    LlmRole::User => "user",
                    LlmRole::Assistant => "assistant",
                    LlmRole::Tool => "tool",
                }.to_string(),
                content: m.content,
            }).collect(),
            temperature: request.temperature,
            max_tokens: request.max_tokens as i32,
            tools: request.tools.map(|tools| {
                tools.into_iter().map(|t| OpenAiTool {
                    tool_type: "function".to_string(),
                    function: OpenAiFunction {
                        name: t.name,
                        description: t.description,
                        parameters: t.parameters,
                    },
                }).collect()
            }),
        };
        
        debug!("Sending request to OpenAI API");
        
        let response = self.http_client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| OpenClawError::Http(e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("OpenAI API error: {}", error_text);
            return Err(OpenClawError::Agent(format!("OpenAI API error: {}", error_text)));
        }
        
        let openai_response: OpenAiResponse = response.json()
            .await
            .map_err(|e| OpenClawError::Serialization(e))?;
        
        let choice = openai_response.choices.into_iter()
            .next()
            .ok_or_else(|| OpenClawError::Agent("No response from LLM".to_string()))?;
        
        let tool_calls = choice.message.tool_calls.map(|calls| {
            calls.into_iter().map(|call| ToolCall {
                id: call.id,
                name: call.function.name,
                arguments: serde_json::from_str(&call.function.arguments).unwrap_or_default(),
            }).collect()
        }).unwrap_or_default();
        
        Ok(LlmResponse {
            content: choice.message.content.unwrap_or_default(),
            tool_calls,
            usage: TokenUsage {
                prompt_tokens: openai_response.usage.prompt_tokens as usize,
                completion_tokens: openai_response.usage.completion_tokens as usize,
                total_tokens: openai_response.usage.total_tokens as usize,
            },
        })
    }
    
    async fn stream(&self, _request: LlmRequest) -> Result<tokio::sync::mpsc::Receiver<String>> {
        // TODO: Implement streaming
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let _ = tx.send("Streaming not yet implemented".to_string()).await;
        Ok(rx)
    }
}

// OpenAI API types
#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAiTool>>,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAiTool {
    #[serde(rename = "type")]
    tool_type: String,
    function: OpenAiFunction,
}

#[derive(Serialize)]
struct OpenAiFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
    usage: OpenAiUsage,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiResponseMessage,
}

#[derive(Deserialize)]
struct OpenAiResponseMessage {
    content: Option<String>,
    tool_calls: Option<Vec<OpenAiToolCall>>,
}

#[derive(Deserialize)]
struct OpenAiToolCall {
    id: String,
    function: OpenAiFunctionCall,
}

#[derive(Deserialize)]
struct OpenAiFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Deserialize)]
struct OpenAiUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

/// Anthropic Claude client
pub struct ClaudeClient {
    api_key: String,
    base_url: String,
    http_client: reqwest::Client,
}

impl ClaudeClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.anthropic.com/v1".to_string(),
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmClient for ClaudeClient {
    async fn complete(&self, request: LlmRequest) -> Result<LlmResponse> {
        let url = format!("{}/messages", self.base_url);
        
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&self.api_key)
                .map_err(|e| OpenClawError::Config(format!("Invalid API key: {}", e)))?,
        );
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        // Convert messages to Claude format
        let system = request.messages.iter()
            .find(|m| matches!(m.role, LlmRole::System))
            .map(|m| m.content.clone());
        
        let messages: Vec<_> = request.messages.into_iter()
            .filter(|m| !matches!(m.role, LlmRole::System))
            .map(|m| ClaudeMessage {
                role: match m.role {
                    LlmRole::User => "user",
                    LlmRole::Assistant => "assistant",
                    _ => "user",
                }.to_string(),
                content: m.content,
            })
            .collect();
        
        let body = ClaudeRequest {
            model: request.model,
            max_tokens: request.max_tokens as i32,
            messages,
            system,
            temperature: request.temperature,
        };
        
        debug!("Sending request to Claude API");
        
        let response = self.http_client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| OpenClawError::Http(e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("Claude API error: {}", error_text);
            return Err(OpenClawError::Agent(format!("Claude API error: {}", error_text)));
        }
        
        let claude_response: ClaudeResponse = response.json()
            .await
            .map_err(|e| OpenClawError::Serialization(e))?;
        
        let content = claude_response.content.into_iter()
            .map(|c| c.text)
            .collect::<Vec<_>>()
            .join("");
        
        Ok(LlmResponse {
            content,
            tool_calls: vec![], // Claude tools not yet implemented
            usage: TokenUsage {
                prompt_tokens: claude_response.usage.input_tokens as usize,
                completion_tokens: claude_response.usage.output_tokens as usize,
                total_tokens: (claude_response.usage.input_tokens + claude_response.usage.output_tokens) as usize,
            },
        })
    }
    
    async fn stream(&self, _request: LlmRequest) -> Result<tokio::sync::mpsc::Receiver<String>> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let _ = tx.send("Streaming not yet implemented".to_string()).await;
        Ok(rx)
    }
}

// Claude API types
#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: i32,
    messages: Vec<ClaudeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    temperature: f32,
}

#[derive(Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
    usage: ClaudeUsage,
}

#[derive(Deserialize)]
struct ClaudeContent {
    text: String,
}

#[derive(Deserialize)]
struct ClaudeUsage {
    input_tokens: i32,
    output_tokens: i32,
}
