use crate::{Agent, AgentConfig};
use openclaw_core::SessionId;
use openclaw_errors::Result;
use openclaw_storage::Storage;
use openclaw_tools::ToolRegistry;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

/// Runtime for managing multiple agents
pub struct AgentRuntime {
    agents: RwLock<HashMap<String, Arc<Agent>>>,
    storage: Arc<dyn Storage>,
    tool_registry: Arc<ToolRegistry>,
    default_config: AgentConfig,
}

impl AgentRuntime {
    pub fn new(
        storage: Arc<dyn Storage>,
        tool_registry: Arc<ToolRegistry>,
        default_config: AgentConfig,
    ) -> Self {
        Self {
            agents: RwLock::new(HashMap::new()),
            storage,
            tool_registry,
            default_config,
        }
    }
    
    /// Register an agent with a specific ID
    pub async fn register_agent(&self, id: impl Into<String>, agent: Arc<Agent>) {
        let mut agents = self.agents.write().await;
        agents.insert(id.into(), agent);
        info!("Agent registered");
    }
    
    /// Get an agent by ID
    pub async fn get_agent(&self, id: &str) -> Option<Arc<Agent>> {
        let agents = self.agents.read().await;
        agents.get(id).cloned()
    }
    
    /// Process a message for a specific session
    pub async fn process_message(
        &self,
        session_id: SessionId,
        message: &openclaw_core::Message,
    ) -> Result<crate::AgentResponse> {
        // Get or create session
        let mut session = match self.storage.get_session(session_id).await? {
            Some(s) => s,
            None => {
                // Create new session
                let session = openclaw_core::Session::new(
                    openclaw_core::ChannelId::new(),
                    &message.sender.user.as_ref().map(|u| u.channel_user_id.clone()).unwrap_or_default(),
                    &message.chat.channel_chat_id,
                );
                self.storage.save_session(&session).await?;
                session
            }
        };
        
        // Get default agent (could be extended to route to specific agents)
        let agent = self.get_default_agent().await?;
        
        // Process message
        let response = agent.process_message(&mut session, message).await;
        
        // Save updated session
        if let Err(e) = self.storage.save_session(&session).await {
            error!("Failed to save session: {}", e);
        }
        
        response
    }
    
    /// Get default agent
    async fn get_default_agent(&self) -> Result<Arc<Agent>> {
        // For now, create a new agent each time (should cache this)
        // In production, you'd want to manage agent lifecycle properly
        use crate::llm::OpenAiClient;
        
        let llm_client: Arc<dyn crate::LlmClient> = Arc::new(
            OpenAiClient::new(std::env::var("OPENAI_API_KEY").unwrap_or_default())
        );
        
        let agent = Arc::new(Agent::new(
            self.default_config.clone(),
            llm_client,
            self.tool_registry.clone(),
        ));
        
        Ok(agent)
    }
    
    /// Shutdown all agents
    pub async fn shutdown(&self) {
        info!("Shutting down agent runtime");
        let mut agents = self.agents.write().await;
        agents.clear();
    }
}
