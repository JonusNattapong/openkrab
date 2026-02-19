use crate::agents::identity::AgentIdentity;
use crate::agents::chat::{ChatProvider, ChatMessage};
use crate::agents::tool::{Tool, ToolDefinition};
use crate::memory::MemoryManager;
use anyhow::Result;
use std::sync::Arc;

pub struct Agent {
    pub identity: AgentIdentity,
    pub provider: Box<dyn ChatProvider>,
    pub memory: Option<Arc<MemoryManager>>,
    pub tools: Vec<Box<dyn Tool>>,
}

impl Agent {
    pub fn new(identity: AgentIdentity, provider: Box<dyn ChatProvider>, memory: Option<Arc<MemoryManager>>, tools: Vec<Box<dyn Tool>>) -> Self {
        Self {
            identity,
            provider,
            memory,
            tools,
        }
    }

    pub async fn answer(&self, query: &str) -> Result<String> {
        let mut messages = Vec::new();
        
        // 1. System Prompt
        messages.push(ChatMessage::System {
            content: self.identity.build_system_prompt(),
        });

        // 2. Initial Context (Static RAG)
        if let Some(ref memory) = self.memory {
            let results = memory.search_hybrid(query, Default::default()).await?;
            if !results.is_empty() {
                let mut context = "Use the following context to help answer the user's question:\n\n".to_string();
                for (i, res) in results.iter().take(5).enumerate() {
                    context.push_str(&format!("--- Document {} ({}) ---\n{}\n\n", i+1, res.path, res.text));
                }
                messages.push(ChatMessage::System {
                    content: context,
                });
            }
        }

        // 3. User Query
        messages.push(ChatMessage::User {
            content: crate::agents::chat::UserContent::Text(query.to_string()),
        });

        // 4. Interaction Loop
        let tool_definitions: Vec<ToolDefinition> = self.tools.iter().map(|t| t.definition()).collect();
        
        loop {
            let response = self.provider.complete(messages.clone(), Some(&tool_definitions)).await?;
            messages.push(response.message.clone());

            match response.message {
                ChatMessage::Assistant { ref tool_calls, ref content } => {
                    if let Some(calls) = tool_calls {
                        for call in calls {
                            println!("Executing tool: {} with args: {}", call.name, call.arguments);
                            let tool = self.tools.iter().find(|t| t.definition().name == call.name)
                                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", call.name))?;
                            
                            let output = tool.call(&call.arguments).await?;
                            messages.push(ChatMessage::Tool {
                                tool_call_id: call.id.clone(),
                                content: output,
                            });
                        }
                        // Continue loop to let assistant process tool results
                    } else {
                        // No tool calls, we are done
                        return Ok(content.clone().unwrap_or_default());
                    }
                }
                _ => return Err(anyhow::anyhow!("Unexpected response message type")),
            }

            if response.finish_reason == "stop" {
                if let ChatMessage::Assistant { content, .. } = response.message {
                    return Ok(content.unwrap_or_default());
                }
            }
        }
    }
}
