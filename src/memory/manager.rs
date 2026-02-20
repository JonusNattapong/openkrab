use crate::memory::embeddings::EmbeddingProvider;
use crate::memory::store::MemoryStore;
use anyhow::Result;
use futures_util::StreamExt;
use notify::{Config, Event, RecursiveMode, Watcher};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct MemoryChunk {
    pub start_line: i32,
    pub end_line: i32,
    pub text: String,
    pub hash: String,
}

pub fn hash_text(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    hex::encode(hasher.finalize())
}

pub fn chunk_markdown(content: &str, max_chars: usize) -> Vec<MemoryChunk> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut current_lines = Vec::new();
    let mut current_chars = 0;
    let mut start_line = 1;

    for (i, line) in lines.iter().enumerate() {
        let line_len = line.len() + 1; // +1 for newline

        if current_chars + line_len > max_chars && !current_lines.is_empty() {
            let text = current_lines.join("\n");
            chunks.push(MemoryChunk {
                start_line: start_line as i32,
                end_line: (i) as i32,
                hash: hash_text(&text),
                text,
            });
            current_lines.clear();
            current_chars = 0;
            start_line = i + 1;
        }

        current_lines.push(line.to_string());
        current_chars += line_len;
    }

    if !current_lines.is_empty() {
        let text = current_lines.join("\n");
        chunks.push(MemoryChunk {
            start_line: start_line as i32,
            end_line: lines.len() as i32,
            hash: hash_text(&text),
            text,
        });
    }

    chunks
}

pub struct MemoryManager {
    pub store: MemoryStore,
    pub provider: Box<dyn EmbeddingProvider>,
}

impl std::fmt::Debug for MemoryManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryManager")
            .field("store", &"...")
            .field("provider", &"...")
            .finish()
    }
}

use crate::memory::store::SearchResult;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct HybridSearchOptions {
    pub max_results: usize,
    pub vector_weight: f64,
    pub text_weight: f64,
}

impl Default for HybridSearchOptions {
    fn default() -> Self {
        Self {
            max_results: 10,
            vector_weight: 0.7,
            text_weight: 0.3,
        }
    }
}

impl MemoryManager {
    pub fn new(store: MemoryStore, provider: Box<dyn EmbeddingProvider>) -> Self {
        Self { store, provider }
    }

    pub fn from_config(
        store: MemoryStore,
        config: crate::memory::config::MemoryConfig,
    ) -> Result<Self> {
        let provider = config.create_provider()?;
        Ok(Self { store, provider })
    }

    pub async fn search_hybrid(
        &self,
        query: &str,
        opts: HybridSearchOptions,
    ) -> Result<Vec<SearchResult>> {
        let model = self.provider.model();

        // 1. Keyword search (FTS5)
        let keyword_results = self.store.search_fts(query, model, opts.max_results * 2)?;

        // 2. Vector search (using sqlite-vec)
        let query_vec = self.provider.embed_query(query).await?;
        let vector_results = self.store.search_vector(&query_vec, opts.max_results * 2)?;

        // 3. Merge hybrid results
        let mut merged: HashMap<String, SearchResult> = HashMap::new();

        for r in vector_results {
            let mut entry = r;
            entry.score *= opts.vector_weight;
            merged.insert(entry.id.clone(), entry);
        }

        for r in keyword_results {
            if let Some(existing) = merged.get_mut(&r.id) {
                existing.score += r.score * opts.text_weight;
            } else {
                let mut entry = r;
                entry.score *= opts.text_weight;
                merged.insert(entry.id.clone(), entry);
            }
        }

        let mut final_results: Vec<SearchResult> = merged.into_values().collect();
        final_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        final_results.truncate(opts.max_results);

        Ok(final_results)
    }

    pub async fn index_file(&self, workspace_dir: &Path, abs_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(abs_path)?;
        let chunks = chunk_markdown(&content, 2000); // 2000 chars roughly 500 tokens
        let rel_path = abs_path
            .strip_prefix(workspace_dir)?
            .to_string_lossy()
            .replace("\\", "/");

        let source = "memory";
        let model = self.provider.model().to_string();

        // Clean up old entries
        self.store
            .delete_chunks_by_path(&rel_path, source, &model)?;

        for chunk in chunks {
            let embedding_vec = self.provider.embed_query(&chunk.text).await?;

            // Ensure vector index exists with these dimensions
            self.store.ensure_vector_index(embedding_vec.len())?;

            let chunk_id = hash_text(&format!(
                "{}:{}:{}:{}",
                rel_path, chunk.start_line, chunk.end_line, model
            ));

            self.store.insert_chunk(
                &chunk_id,
                &rel_path,
                source,
                chunk.start_line,
                chunk.end_line,
                &chunk.hash,
                &model,
                &chunk.text,
                &embedding_vec,
            )?;
        }

        Ok(())
    }

    pub async fn sync_workspace(&self, workspace_dir: &Path) -> Result<()> {
        let memory_dir = workspace_dir.join("memory");
        let memory_md = workspace_dir.join("MEMORY.md");

        let mut files_to_index = Vec::new();

        if memory_md.exists() {
            files_to_index.push(memory_md);
        }

        if memory_dir.exists() && memory_dir.is_dir() {
            for entry in WalkDir::new(memory_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
            {
                files_to_index.push(entry.path().to_path_buf());
            }
        }

        for file_path in files_to_index {
            println!("Indexing: {:?}", file_path);
            if let Err(e) = self.index_file(workspace_dir, &file_path).await {
                eprintln!("Failed to index {:?}: {}", file_path, e);
            }
        }

        Ok(())
    }

    pub async fn watch_workspace(self: Arc<Self>, workspace_dir: PathBuf) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(100);

        let mut watcher = notify::RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(event) = res {
                    if event.kind.is_modify() || event.kind.is_create() {
                        for path in event.paths {
                            if path.extension().map_or(false, |ext| ext == "md") {
                                let _ = tx.blocking_send(path);
                            }
                        }
                    }
                }
            },
            Config::default(),
        )?;

        watcher.watch(&workspace_dir, RecursiveMode::Recursive)?;
        println!("Watching for changes in: {:?}", workspace_dir);

        // Background task to handle events
        let manager = self.clone();
        let ws_dir = workspace_dir.clone();

        tokio::spawn(async move {
            // Keep watcher alive by moving it into the task
            let _watcher = watcher;

            while let Some(path) = rx.recv().await {
                println!("File changed: {:?}", path);
                // Simple debounce/delay to let file write finish
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                if let Err(e) = manager.index_file(&ws_dir, &path).await {
                    eprintln!("Auto-index failed for {:?}: {}", path, e);
                } else {
                    println!("Successfully re-indexed: {:?}", path);
                }
            }
        });

        Ok(())
    }
}
