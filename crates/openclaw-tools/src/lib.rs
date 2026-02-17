use openclaw_core::{Tool, ToolCall, ToolResult};
use openclaw_errors::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{debug, error, info};

pub mod bash;
pub mod file;
pub mod search;

pub use bash::BashTool;
pub use file::{ReadFileTool, WriteFileTool};
pub use search::SearchTool;

/// Tool handler trait
#[async_trait]
pub trait ToolHandler: Send + Sync {
    /// Get tool definition
    fn definition(&self) -> Tool;

    /// Execute tool with arguments
    async fn execute(&self, arguments: serde_json::Value) -> Result<serde_json::Value>;
}

/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolHandler>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };

        // Register built-in tools
        registry.register(Box::new(BashTool::new()));
        registry.register(Box::new(ReadFileTool::new()));
        registry.register(Box::new(WriteFileTool::new()));
        registry.register(Box::new(SearchTool::new()));

        registry
    }

    /// Register a tool
    pub fn register(&mut self, handler: Box<dyn ToolHandler>) {
        let name = handler.definition().name.clone();
        self.tools.insert(name, handler);
        info!("Tool registered");
    }

    /// Get tool by name
    pub fn get(&self, name: &str) -> Option<&dyn ToolHandler> {
        self.tools.get(name).map(|h| h.as_ref())
    }

    /// List all registered tools
    pub fn list_tools(&self) -> Vec<Tool> {
        self.tools.values().map(|h| h.definition()).collect()
    }

    /// Execute a tool call
    pub async fn execute(&self, call: &ToolCall) -> Result<ToolResult> {
        let start = std::time::Instant::now();

        match self.get(&call.name) {
            Some(handler) => {
                debug!(tool_name = %call.name, "Executing tool");

                match handler.execute(call.arguments.clone()).await {
                    Ok(result) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        success: true,
                        result: Some(result),
                        error: None,
                        execution_time_ms: start.elapsed().as_millis() as u64,
                    }),
                    Err(e) => {
                        error!(tool_name = %call.name, error = %e, "Tool execution failed");
                        Ok(ToolResult {
                            call_id: call.id.clone(),
                            success: false,
                            result: None,
                            error: Some(e.to_string()),
                            execution_time_ms: start.elapsed().as_millis() as u64,
                        })
                    }
                }
            }
            None => Ok(ToolResult {
                call_id: call.id.clone(),
                success: false,
                result: None,
                error: Some(format!("Tool '{}' not found", call.name)),
                execution_time_ms: 0,
            }),
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}