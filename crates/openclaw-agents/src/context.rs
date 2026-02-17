use openclaw_core::{ContextMessage, Role, SessionContext};
use tracing::{debug, info};

/// Manages conversation context and handles summarization
pub struct ContextManager {
    max_tokens: usize,
    summarization_threshold: usize,
}

impl ContextManager {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            summarization_threshold: max_tokens * 3 / 4, // 75% threshold
        }
    }
    
    /// Build context for LLM request
    pub fn build_context(&self, session_context: &SessionContext) -> Vec<ContextMessage> {
        let messages = &session_context.messages;
        
        // Estimate current token count
        let token_count = self.estimate_tokens(messages);
        
        if token_count > self.max_tokens {
            info!(token_count, max_tokens = self.max_tokens, "Context limit exceeded, summarizing");
            self.summarize_and_trim(messages)
        } else {
            messages.clone()
        }
    }
    
    /// Estimate token count (rough approximation: 4 chars ≈ 1 token)
    fn estimate_tokens(&self, messages: &[ContextMessage]) -> usize {
        messages.iter()
            .map(|msg| {
                let base = msg.content.len() / 4;
                let overhead = 4; // Role marker, etc.
                base + overhead
            })
            .sum()
    }
    
    /// Summarize and trim old messages to fit within limits
    fn summarize_and_trim(&self, messages: &[ContextMessage]) -> Vec<ContextMessage> {
        // Keep system message and recent messages
        let mut result = Vec::new();
        
        // Add system message if exists
        if let Some(system) = messages.iter().find(|m| matches!(m.role, Role::System)) {
            result.push(system.clone());
        }
        
        // Add summary placeholder
        let old_messages = messages.iter()
            .filter(|m| !matches!(m.role, Role::System))
            .take(messages.len().saturating_sub(10)); // Keep last 10
        
        if old_messages.clone().count() > 5 {
            result.push(ContextMessage {
                role: Role::System,
                content: format!("[... {} older messages summarized ...]", old_messages.count()),
                timestamp: chrono::Utc::now(),
                metadata: None,
            });
        }
        
        // Add recent messages
        let recent: Vec<_> = messages.iter()
            .filter(|m| !matches!(m.role, Role::System))
            .rev()
            .take(10)
            .cloned()
            .collect();
        
        result.extend(recent.into_iter().rev());
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_estimate_tokens() {
        let manager = ContextManager::new(1000);
        
        let messages = vec![
            ContextMessage {
                role: Role::User,
                content: "Hello, world!".to_string(),
                timestamp: chrono::Utc::now(),
                metadata: None,
            },
        ];
        
        let tokens = manager.estimate_tokens(&messages);
        assert!(tokens > 0);
        assert!(tokens < 100);
    }
}
