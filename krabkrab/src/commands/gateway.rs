use crate::gateway::{start_gateway, GatewayState};
use crate::memory::{MemoryStore, MemoryManager, MemoryConfig};
use anyhow::Result;
use std::sync::Arc;

pub async fn gateway_start_command(db_path: Option<&str>) -> Result<()> {
    let db_path = db_path.unwrap_or("memory.db");
    
    // Load API Key
    let api_key = std::env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "sk-dummy".to_string()); // Don't panic, but might fail later if used

    let store = MemoryStore::open(db_path)?;
    
    // For now, dimensions should match your provider's default
    // text-embedding-3-small is 1536
    store.ensure_vector_index(1536)?;

    let config = MemoryConfig::default();
    let manager = Arc::new(MemoryManager::from_config(store, config)?);
    
    // Initialize Agent
    let identity = crate::agents::AgentIdentity::default();
    let provider = Box::new(crate::agents::OpenAiChatProvider::new(api_key, None, None));
    
    let workspace_root = std::env::current_dir()?;
    let tools: Vec<Box<dyn crate::agents::Tool>> = vec![
        Box::new(crate::agents::SearchMemoryTool::new(manager.clone())),
        Box::new(crate::agents::ReadFileTool::new(workspace_root.clone())),
        Box::new(crate::agents::ListFilesTool::new(workspace_root.clone())),
        Box::new(crate::agents::WriteFileTool::new(workspace_root.clone())),
        Box::new(crate::agents::ExecCommandTool::new(workspace_root.clone(), true)),
        Box::new(crate::agents::RememberTool::new(manager.clone(), workspace_root.clone())),
        Box::new(crate::agents::TaskTool::new(workspace_root.clone())),
        Box::new(crate::agents::SpeakTool::new()),
        Box::new(crate::agents::ScheduleTool::new(workspace_root.clone())),
        Box::new(crate::agents::BrowserTool::new()),
        Box::new(crate::agents::CodeInterpreterTool::new(workspace_root.clone())),
    ];
    
    let agent = Arc::new(crate::agents::Agent::new(identity, provider, Some(manager.clone()), tools));

    let state = Arc::new(GatewayState {
        memory: manager,
        agent: agent,
    });

    start_gateway(state).await?;
    
    Ok(())
}
