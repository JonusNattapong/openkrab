use super::ToolHandler;
use openclaw_core::{ParameterType, Tool, ToolParameter};
use openclaw_errors::{OpenClawError, Result};
use async_trait::async_trait;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, warn};

/// Bash tool for executing shell commands
pub struct BashTool {
    allowed_commands: Vec<String>,
    blocked_commands: Vec<String>,
    timeout_seconds: u64,
}

impl BashTool {
    pub fn new() -> Self {
        Self {
            allowed_commands: vec![], // Empty means allow all (for main session)
            blocked_commands: vec![
                "rm -rf /".to_string(),
                "mkfs".to_string(),
                "dd".to_string(),
            ],
            timeout_seconds: 60,
        }
    }
    
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
    
    /// Validate command for security
    fn validate_command(&self, command: &str) -> Result<()> {
        // Check blocked commands
        for blocked in &self.blocked_commands {
            if command.contains(blocked) {
                return Err(OpenClawError::Tool(format!(
                    "Command contains blocked pattern: {}",
                    blocked
                )));
            }
        }
        
        // If allowed list is not empty, check against it
        if !self.allowed_commands.is_empty() {
            let cmd = command.split_whitespace().next().unwrap_or("");
            if !self.allowed_commands.iter().any(|allowed| cmd == allowed) {
                return Err(OpenClawError::Tool(format!(
                    "Command not in allowed list: {}",
                    cmd
                )));
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl ToolHandler for BashTool {
    fn definition(&self) -> Tool {
        Tool {
            name: "bash".to_string(),
            description: "Execute bash shell commands".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "command".to_string(),
                    description: "The bash command to execute".to_string(),
                    param_type: ParameterType::String { enum_values: None },
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "timeout".to_string(),
                    description: "Timeout in seconds (default: 60)".to_string(),
                    param_type: ParameterType::Integer,
                    required: false,
                    default: Some(serde_json::json!(60)),
                },
            ],
            returns: openclaw_core::ToolReturn {
                description: "Command output".to_string(),
                return_type: ParameterType::Object {
                    properties: [
                        ("stdout".to_string(), ParameterType::String { enum_values: None }),
                        ("stderr".to_string(), ParameterType::String { enum_values: None }),
                        ("exit_code".to_string(), ParameterType::Integer),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            dangerous: true,
        }
    }
    
    async fn execute(&self, arguments: serde_json::Value) -> Result<serde_json::Value> {
        let command = arguments
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenClawError::Tool("Missing 'command' argument".to_string()))?;
        
        debug!("Executing bash command: {}", command);
        
        // Validate command
        self.validate_command(command)?;
        
        let timeout = arguments
            .get("timeout")
            .and_then(|v| v.as_u64())
            .unwrap_or(self.timeout_seconds);
        
        // Execute command
        let mut cmd = Command::new("bash")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| OpenClawError::Tool(format!("Failed to spawn process: {}", e)))?;
        
        // Wait with timeout
        let result = tokio::time::timeout(
            tokio::time::Duration::from_secs(timeout),
            cmd.wait_with_output(),
        )
        .await;
        
        match result {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code().unwrap_or(-1);
                
                Ok(serde_json::json!({
                    "stdout": stdout,
                    "stderr": stderr,
                    "exit_code": exit_code,
                    "success": output.status.success(),
                }))
            }
            Ok(Err(e)) => Err(OpenClawError::Tool(format!("Command execution failed: {}", e))),
            Err(_) => {
                warn!("Command timed out after {} seconds", timeout);
                let _ = cmd.kill().await;
                Err(OpenClawError::Tool(format!(
                    "Command timed out after {} seconds",
                    timeout
                )))
            }
        }
    }
}

impl Default for BashTool {
    fn default() -> Self {
        Self::new()
    }
}
