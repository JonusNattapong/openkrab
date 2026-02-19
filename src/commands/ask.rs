use crate::agents::{Agent, AgentIdentity, OpenAiChatProvider};
use crate::memory::{MemoryConfig, MemoryManager, MemoryStore};
use anyhow::{anyhow, Result};
use std::sync::Arc;

pub async fn ask_command(query: &str, db_path: Option<&str>) -> Result<String> {
    let cfg = crate::config_io::load_config().ok();
    let _ = crate::plugins::loader::PluginManager::bootstrap_from_config(
        cfg.as_ref().and_then(|c| c.plugins.as_ref()),
    )
    .await?;

    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| anyhow!("Missing OPENAI_API_KEY environment variable"))?;

    let db_path = db_path.unwrap_or("memory.db");
    let store = MemoryStore::open(db_path)?;

    let mem_config = MemoryConfig::default();
    let memory_manager = Arc::new(MemoryManager::from_config(store, mem_config)?);

    let identity = AgentIdentity::default();
    let provider = Box::new(OpenAiChatProvider::new(api_key, None, None));

    let workspace_root = std::env::current_dir()?;
    let tools: Vec<Box<dyn crate::agents::Tool>> = vec![
        Box::new(crate::agents::SearchMemoryTool::new(memory_manager.clone())),
        Box::new(crate::agents::ReadFileTool::new(workspace_root.clone())),
        Box::new(crate::agents::ListFilesTool::new(workspace_root.clone())),
        Box::new(crate::agents::WriteFileTool::new(workspace_root.clone())),
        Box::new(crate::agents::ExecCommandTool::new(
            workspace_root.clone(),
            true,
        )), // Security: Require approval
        Box::new(crate::agents::RememberTool::new(
            memory_manager.clone(),
            workspace_root.clone(),
        )),
        Box::new(crate::agents::TaskTool::new(workspace_root.clone())),
        Box::new(crate::agents::SpeakTool::new()),
        Box::new(crate::agents::ScheduleTool::new(workspace_root.clone())),
        Box::new(crate::agents::BrowserTool::new()),
        Box::new(crate::agents::CodeInterpreterTool::new(
            workspace_root.clone(),
        )),
    ];

    let agent = Agent::new(identity, provider, Some(memory_manager), tools);

    let response = agent.answer(query).await?;

    Ok(response)
}
