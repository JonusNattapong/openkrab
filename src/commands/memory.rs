use crate::memory::{MemoryConfig, MemoryManager, MemoryStore};
use anyhow::Result;
use std::path::Path;

pub async fn memory_sync_command(
    workspace_dir: &str,
    db_path: Option<&str>,
    watch: bool,
) -> Result<String> {
    let db_path = db_path.unwrap_or("memory.db");
    let store = MemoryStore::open(db_path)?;

    // For simplicity, using default config or env vars
    let config = MemoryConfig::default();
    let manager = std::sync::Arc::new(MemoryManager::from_config(store, config)?);

    let ws_path = Path::new(workspace_dir).to_path_buf();
    manager.sync_workspace(&ws_path).await?;

    if watch {
        manager.watch_workspace(ws_path).await?;
        Ok(format!(
            "Successfully synced and now watching workspace: {}",
            workspace_dir
        ))
    } else {
        Ok(format!("Successfully synced workspace: {}", workspace_dir))
    }
}

pub async fn memory_search_command(query: &str, db_path: Option<&str>) -> Result<String> {
    let db_path = db_path.unwrap_or("memory.db");
    let store = MemoryStore::open(db_path)?;

    let config = MemoryConfig::default();
    let manager = MemoryManager::from_config(store, config)?;

    let results = manager.search_hybrid(query, Default::default()).await?;

    if results.is_empty() {
        return Ok("No results found.".to_string());
    }

    let mut out = format!("Search results for '{}':\n", query);
    for (i, res) in results.iter().enumerate() {
        out.push_str(&format!(
            "{}. [{:.4}] {} (L{}-L{})\n   {}\n\n",
            i + 1,
            res.score,
            res.path,
            res.start_line,
            res.end_line,
            res.text.lines().next().unwrap_or("")
        ));
    }

    Ok(out)
}
