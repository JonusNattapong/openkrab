use super::ToolHandler;
use openclaw_core::{ParameterType, Tool, ToolParameter};
use openclaw_errors::{OpenClawError, Result};
use async_trait::async_trait;
use tokio::fs;
use tracing::debug;

/// Read file tool
pub struct ReadFileTool;

impl ReadFileTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ToolHandler for ReadFileTool {
    fn definition(&self) -> Tool {
        Tool {
            name: "read".to_string(),
            description: "Read contents of a file".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "path".to_string(),
                    description: "Path to the file to read".to_string(),
                    param_type: ParameterType::String { enum_values: None },
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "offset".to_string(),
                    description: "Byte offset to start reading from".to_string(),
                    param_type: ParameterType::Integer,
                    required: false,
                    default: Some(serde_json::json!(0)),
                },
                ToolParameter {
                    name: "limit".to_string(),
                    description: "Maximum bytes to read".to_string(),
                    param_type: ParameterType::Integer,
                    required: false,
                    default: Some(serde_json::json!(10000)),
                },
            ],
            returns: openclaw_core::ToolReturn {
                description: "File contents".to_string(),
                return_type: ParameterType::Object {
                    properties: [
                        ("content".to_string(), ParameterType::String { enum_values: None }),
                        ("size".to_string(), ParameterType::Integer),
                        ("truncated".to_string(), ParameterType::Boolean),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            dangerous: false,
        }
    }
    
    async fn execute(&self, arguments: serde_json::Value) -> Result<serde_json::Value> {
        let path = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenClawError::Tool("Missing 'path' argument".to_string()))?;
        
        debug!("Reading file: {}", path);
        
        let offset = arguments
            .get("offset")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        
        let limit = arguments
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(10000) as usize;
        
        // Read file
        let content = fs::read_to_string(path).await.map_err(|e| {
            OpenClawError::Tool(format!("Failed to read file '{}': {}", path, e))
        })?;
        
        let total_size = content.len();
        
        // Apply offset and limit
        let start = std::cmp::min(offset, content.len());
        let end = std::cmp::min(start + limit, content.len());
        let truncated = end < content.len();
        let content = content[start..end].to_string();
        
        Ok(serde_json::json!({
            "content": content,
            "size": total_size,
            "truncated": truncated,
            "offset": start,
        }))
    }
}

impl Default for ReadFileTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Write file tool
pub struct WriteFileTool;

impl WriteFileTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ToolHandler for WriteFileTool {
    fn definition(&self) -> Tool {
        Tool {
            name: "write".to_string(),
            description: "Write content to a file".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "path".to_string(),
                    description: "Path to the file to write".to_string(),
                    param_type: ParameterType::String { enum_values: None },
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "content".to_string(),
                    description: "Content to write".to_string(),
                    param_type: ParameterType::String { enum_values: None },
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "append".to_string(),
                    description: "Append to file instead of overwriting".to_string(),
                    param_type: ParameterType::Boolean,
                    required: false,
                    default: Some(serde_json::json!(false)),
                },
            ],
            returns: openclaw_core::ToolReturn {
                description: "Write result".to_string(),
                return_type: ParameterType::Object {
                    properties: [
                        ("bytes_written".to_string(), ParameterType::Integer),
                        ("path".to_string(), ParameterType::String { enum_values: None }),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            dangerous: true,
        }
    }
    
    async fn execute(&self, arguments: serde_json::Value) -> Result<serde_json::Value> {
        let path = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenClawError::Tool("Missing 'path' argument".to_string()))?;
        
        let content = arguments
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenClawError::Tool("Missing 'content' argument".to_string()))?;
        
        let append = arguments
            .get("append")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        debug!("Writing file: {} (append: {})", path, append);
        
        let bytes_written = if append {
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .await
                .map_err(|e| OpenClawError::Tool(format!("Failed to open file: {}", e)))?
                .write(content.as_bytes())
                .await
                .map_err(|e| OpenClawError::Tool(format!("Failed to write file: {}", e)))?
        } else {
            fs::write(path, content).await.map_err(|e| {
                OpenClawError::Tool(format!("Failed to write file '{}': {}", path, e))
            })?;
            content.len()
        };
        
        Ok(serde_json::json!({
            "bytes_written": bytes_written,
            "path": path,
        }))
    }
}

impl Default for WriteFileTool {
    fn default() -> Self {
        Self::new()
    }
}
