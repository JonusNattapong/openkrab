use crate::memory::embeddings::EmbeddingProvider;
use crate::memory::store::MemoryStore;
use anyhow::Result;
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

use crate::memory::mmr::{apply_mmr_to_results, MMRConfig};
use crate::memory::temporal_decay::{apply_temporal_decay_to_results, TemporalDecayConfig};

#[derive(Debug, Clone)]
pub struct HybridSearchOptions {
    pub max_results: usize,
    pub min_score: f64,
    pub vector_weight: f64,
    pub text_weight: f64,
    pub temporal_decay: Option<TemporalDecayConfig>,
    pub mmr: Option<MMRConfig>,
    pub workspace_dir: PathBuf,
}

impl Default for HybridSearchOptions {
    fn default() -> Self {
        Self {
            max_results: 10,
            min_score: 0.1,
            vector_weight: 0.7,
            text_weight: 0.3,
            temporal_decay: None,
            mmr: None,
            workspace_dir: PathBuf::new(),
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
        let candidates = opts.max_results * 2;

        // 1. Keyword search (FTS5) - with query expansion (OR)
        let (_original, _keywords, expanded) = crate::memory::query_expansion::expand_query_for_fts(query);
        let search_terms = if expanded.is_empty() { query } else { &expanded };

        // Use an empty string for FTS model if we want to search all models, but for now we limit to current model
        let keyword_results = self
            .store
            .search_fts(search_terms, model, candidates)
            .unwrap_or_default();

        // 2. Vector search (using sqlite-vec)
        // If embedding fails, fallback to FTS only
        let (vector_results, has_vector) = match self.provider.embed_query(query).await {
            Ok(query_vec) => {
                let has_v = query_vec.iter().any(|&x| x != 0.0);
                let res = self
                    .store
                    .search_vector(&query_vec, candidates)
                    .unwrap_or_default();
                (res, has_v)
            }
            Err(e) => {
                eprintln!(
                    "Warning: Vector embedding failed, falling back to FTS only: {}",
                    e
                );
                (Vec::new(), false)
            }
        };

        // 3. Merge hybrid results
        let mut final_results: Vec<SearchResult> = if !has_vector {
            keyword_results
        } else {
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
            merged.into_values().collect()
        };

        final_results.truncate(opts.max_results * 5); // Keep more candidates for re-ranking

        // 4. Temporal Decay
        if let Some(decay_cfg) = &opts.temporal_decay {
            final_results = apply_temporal_decay_to_results(
                final_results,
                Some(decay_cfg.clone()),
                Some(&opts.workspace_dir),
                None,
            );
        }

        // 5. MMR Re-ranking
        if let Some(mmr_cfg) = &opts.mmr {
            if mmr_cfg.enabled {
                final_results = apply_mmr_to_results(final_results, Some(mmr_cfg.clone()));
            }
        }

        // 6. Final Sort and Threshold Filter (Improved Flow: filter AFTER decay/rerank)
        final_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        final_results.retain(|r| r.score >= opts.min_score);
        final_results.truncate(opts.max_results);
        
        Ok(final_results)
    }

    pub async fn index_file(&self, workspace_dir: &Path, abs_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(abs_path)?;
        let hash = hash_text(&content);
        
        let rel_path = abs_path
            .strip_prefix(workspace_dir)?
            .to_string_lossy()
            .replace("\\", "/");
            
        let source = "memory";
        let model = self.provider.model().to_string();

        // High-Performance Skip: Check if file hash + model + source matches
        if let Some(existing_hash) = self.store.get_file_hash(&rel_path, source) {
            if existing_hash == hash {
                // Check if we have chunks for this model
                if self.store.has_chunks_for_path(&rel_path, source, &model)? {
                    return Ok(());
                }
            }
        }

        let chunks = chunk_markdown(&content, 2000);
        
        // Clean up old entries
        self.store.delete_chunks_by_path(&rel_path, source, &model)?;

        // Batch Embedding: 10x faster than sequential (where supported)
        let chunk_texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
        let embeddings = self.provider.embed_batch(&chunk_texts).await?;

        for (chunk, embedding_vec) in chunks.into_iter().zip(embeddings.into_iter()) {
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

        // Update file entry with new hash
        self.store.update_file_info(&rel_path, source, &hash)?;

        Ok(())
    }

    pub async fn warm_session(&self, session: &crate::sessions::Session) -> Result<()> {
        let transcript_text = session
            .transcript
            .iter()
            .map(|t| format!("{}: {}", t.role, t.text))
            .collect::<Vec<String>>()
            .join("\n");

        if transcript_text.is_empty() {
            return Ok(());
        }

        let chunks = chunk_markdown(&transcript_text, 2000);
        let rel_path = format!("sessions/{}.md", session.id);
        let source = "session";
        let model = self.provider.model().to_string();

        self.store
            .delete_chunks_by_path(&rel_path, source, &model)?;

        for chunk in chunks {
            if let Ok(embedding_vec) = self.provider.embed_query(&chunk.text).await {
                let _ = self.store.ensure_vector_index(embedding_vec.len());
                let chunk_id = hash_text(&format!(
                    "{}:{}:{}:{}",
                    rel_path, chunk.start_line, chunk.end_line, model
                ));

                let _ = self.store.insert_chunk(
                    &chunk_id,
                    &rel_path,
                    source,
                    chunk.start_line,
                    chunk.end_line,
                    &chunk.hash,
                    &model,
                    &chunk.text,
                    &embedding_vec,
                );
            }
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
