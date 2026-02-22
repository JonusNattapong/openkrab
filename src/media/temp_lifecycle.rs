//! temp_lifecycle — Enhanced temporary file lifecycle management.
//!
//! Provides:
//! - Scoped temp files with automatic cleanup
//! - Reference counting for shared temp resources
//! - Async cleanup with retry logic
//! - Cleanup scheduling and batching

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::sync::RwLock;
use tokio::time::interval;
use uuid::Uuid;

/// Default TTL for temp files (5 minutes)
pub const DEFAULT_TEMP_TTL: Duration = Duration::from_secs(300);

/// Cleanup interval for background task (1 minute)
pub const CLEANUP_INTERVAL: Duration = Duration::from_secs(60);

/// Maximum retry attempts for cleanup
const MAX_CLEANUP_RETRIES: u32 = 3;

/// Delay between cleanup retries
const CLEANUP_RETRY_DELAY: Duration = Duration::from_millis(100);

/// Unique handle for a temp file resource
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TempHandle(String);

impl TempHandle {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TempHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for TempHandle {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata for a managed temp file
#[derive(Debug, Clone)]
pub struct TempFileMeta {
    pub handle: TempHandle,
    pub path: PathBuf,
    pub created_at: Instant,
    pub expires_at: Instant,
    pub ref_count: usize,
    pub tags: Vec<String>,
}

/// A scoped temp file that auto-cleans when dropped
pub struct ScopedTempFile {
    handle: TempHandle,
    path: PathBuf,
    registry: Weak<RwLock<TempFileRegistryInner>>,
}

impl ScopedTempFile {
    /// Get the path to the temp file
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the handle for this temp file
    pub fn handle(&self) -> &TempHandle {
        &self.handle
    }

    /// Keep the file alive longer
    pub async fn extend_lifetime(&self, additional_duration: Duration) -> Result<()> {
        if let Some(registry) = self.registry.upgrade() {
            let mut inner = registry.write().await;
            if let Some(meta) = inner.files.get_mut(&self.handle) {
                meta.expires_at += additional_duration;
            }
        }
        Ok(())
    }

    /// Explicitly delete the file before drop
    pub async fn delete(self) -> Result<()> {
        self.cleanup().await;
        Ok(())
    }

    async fn cleanup(&self) {
        // Try to delete the file with retries
        for attempt in 0..MAX_CLEANUP_RETRIES {
            match fs::remove_file(&self.path).await {
                Ok(_) => break,
                Err(e) if attempt < MAX_CLEANUP_RETRIES - 1 => {
                    eprintln!(
                        "[temp_lifecycle] Failed to delete temp file (attempt {}): {}",
                        attempt + 1,
                        e
                    );
                    tokio::time::sleep(CLEANUP_RETRY_DELAY).await;
                }
                Err(e) => {
                    eprintln!(
                        "[temp_lifecycle] Failed to delete temp file after {} attempts: {}",
                        MAX_CLEANUP_RETRIES, e
                    );
                }
            }
        }

        // Remove from registry
        if let Some(registry) = self.registry.upgrade() {
            let mut inner = registry.write().await;
            inner.files.remove(&self.handle);
        }
    }
}

impl Drop for ScopedTempFile {
    fn drop(&mut self) {
        // Spawn cleanup task - we can't await in drop
        let path = self.path.clone();
        let handle = self.handle.clone();
        let registry = self.registry.clone();

        tokio::spawn(async move {
            // Try to delete the file with retries
            for attempt in 0..MAX_CLEANUP_RETRIES {
                match fs::remove_file(&path).await {
                    Ok(_) => break,
                    Err(e) if attempt < MAX_CLEANUP_RETRIES - 1 => {
                        eprintln!(
                            "[temp_lifecycle] Failed to delete temp file in drop (attempt {}): {}",
                            attempt + 1,
                            e
                        );
                        tokio::time::sleep(CLEANUP_RETRY_DELAY).await;
                    }
                    Err(e) => {
                        eprintln!("[temp_lifecycle] Failed to delete temp file in drop after {} attempts: {}", MAX_CLEANUP_RETRIES, e);
                    }
                }
            }

            // Remove from registry
            if let Some(registry) = registry.upgrade() {
                let mut inner = registry.write().await;
                inner.files.remove(&handle);
            }
        });
    }
}

/// Inner registry state
#[derive(Debug)]
struct TempFileRegistryInner {
    files: HashMap<TempHandle, TempFileMeta>,
    base_dir: PathBuf,
    next_id: AtomicUsize,
}

/// Registry for managing temp file lifecycle
pub struct TempFileRegistry {
    inner: Arc<RwLock<TempFileRegistryInner>>,
}

impl Default for TempFileRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TempFileRegistry {
    /// Create a new temp file registry
    pub fn new() -> Self {
        let base_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("openkrab")
            .join("temp");

        Self {
            inner: Arc::new(RwLock::new(TempFileRegistryInner {
                files: HashMap::new(),
                base_dir,
                next_id: AtomicUsize::new(1),
            })),
        }
    }

    /// Create with custom base directory
    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        Self {
            inner: Arc::new(RwLock::new(TempFileRegistryInner {
                files: HashMap::new(),
                base_dir,
                next_id: AtomicUsize::new(1),
            })),
        }
    }

    /// Initialize the temp directory
    pub async fn initialize(&self) -> Result<()> {
        let inner = self.inner.read().await;
        fs::create_dir_all(&inner.base_dir).await?;
        Ok(())
    }

    /// Create a new temp file with scoped lifetime
    pub async fn create_scoped(
        &self,
        extension: Option<&str>,
        ttl: Option<Duration>,
        tags: Vec<String>,
    ) -> Result<ScopedTempFile> {
        let handle = TempHandle::new();
        let ttl = ttl.unwrap_or(DEFAULT_TEMP_TTL);

        let mut inner = self.inner.write().await;

        // Ensure directory exists
        fs::create_dir_all(&inner.base_dir).await?;

        // Generate unique filename
        let id = inner.next_id.fetch_add(1, Ordering::Relaxed);
        let filename = if let Some(ext) = extension {
            format!(
                "temp_{:08}_{}.{}",
                id,
                handle.as_str(),
                ext.trim_start_matches('.')
            )
        } else {
            format!("temp_{:08}_{}", id, handle.as_str())
        };

        let path = inner.base_dir.join(&filename);

        // Create empty file
        fs::write(&path, &[]).await?;

        let now = Instant::now();
        let meta = TempFileMeta {
            handle: handle.clone(),
            path: path.clone(),
            created_at: now,
            expires_at: now + ttl,
            ref_count: 1,
            tags,
        };

        inner.files.insert(handle.clone(), meta);

        Ok(ScopedTempFile {
            handle,
            path,
            registry: Arc::downgrade(&self.inner),
        })
    }

    /// Write buffer to a new temp file
    pub async fn write_buffer(
        &self,
        buffer: &[u8],
        extension: Option<&str>,
        ttl: Option<Duration>,
        tags: Vec<String>,
    ) -> Result<ScopedTempFile> {
        let scoped = self.create_scoped(extension, ttl, tags).await?;
        fs::write(&scoped.path, buffer).await?;
        Ok(scoped)
    }

    /// Get metadata for a temp file
    pub async fn get_meta(&self, handle: &TempHandle) -> Option<TempFileMeta> {
        let inner = self.inner.read().await;
        inner.files.get(handle).cloned()
    }

    /// Extend lifetime of a temp file
    pub async fn extend_ttl(
        &self,
        handle: &TempHandle,
        additional_duration: Duration,
    ) -> Result<()> {
        let mut inner = self.inner.write().await;
        if let Some(meta) = inner.files.get_mut(handle) {
            meta.expires_at += additional_duration;
            Ok(())
        } else {
            Err(anyhow!("Temp file not found: {:?}", handle))
        }
    }

    /// Delete a temp file explicitly
    pub async fn delete(&self, handle: &TempHandle) -> Result<()> {
        let mut inner = self.inner.write().await;

        if let Some(meta) = inner.files.remove(handle) {
            // Try to delete with retries
            for attempt in 0..MAX_CLEANUP_RETRIES {
                match fs::remove_file(&meta.path).await {
                    Ok(_) => return Ok(()),
                    Err(e) if attempt < MAX_CLEANUP_RETRIES - 1 => {
                        eprintln!(
                            "[temp_lifecycle] Failed to delete temp file (attempt {}): {}",
                            attempt + 1,
                            e
                        );
                        tokio::time::sleep(CLEANUP_RETRY_DELAY).await;
                    }
                    Err(e) => return Err(anyhow!("Failed to delete temp file: {}", e)),
                }
            }
        }

        Ok(())
    }

    /// Clean up expired files
    pub async fn cleanup_expired(&self) -> CleanupResult {
        let now = Instant::now();
        let mut inner = self.inner.write().await;

        let mut expired_handles = Vec::new();

        for (handle, meta) in &inner.files {
            if meta.expires_at <= now {
                expired_handles.push(handle.clone());
            }
        }

        let mut deleted = 0;
        let mut failed = 0;

        for handle in expired_handles {
            if let Some(meta) = inner.files.remove(&handle) {
                match fs::remove_file(&meta.path).await {
                    Ok(_) => {
                        deleted += 1;
                        eprintln!(
                            "[temp_lifecycle] Cleaned up expired temp file: {:?}",
                            meta.path
                        );
                    }
                    Err(e) => {
                        failed += 1;
                        eprintln!(
                            "[temp_lifecycle] Failed to clean up temp file {:?}: {}",
                            meta.path, e
                        );
                        // Re-insert with extended TTL for retry later
                        let mut meta = meta;
                        meta.expires_at = now + Duration::from_secs(60);
                        inner.files.insert(handle, meta);
                    }
                }
            }
        }

        CleanupResult { deleted, failed }
    }

    /// Clean up all temp files (force cleanup)
    pub async fn cleanup_all(&self) -> CleanupResult {
        let mut inner = self.inner.write().await;

        let mut deleted = 0;
        let mut failed = 0;

        for (_handle, meta) in inner.files.drain() {
            match fs::remove_file(&meta.path).await {
                Ok(_) => {
                    deleted += 1;
                    eprintln!("[temp_lifecycle] Cleaned up temp file: {:?}", meta.path);
                }
                Err(e) => {
                    failed += 1;
                    eprintln!(
                        "[temp_lifecycle] Failed to clean up temp file {:?}: {}",
                        meta.path, e
                    );
                }
            }
        }

        CleanupResult { deleted, failed }
    }

    /// Clean up files by tag
    pub async fn cleanup_by_tag(&self, tag: &str) -> CleanupResult {
        let mut inner = self.inner.write().await;

        let mut handles_to_remove = Vec::new();

        for (handle, meta) in &inner.files {
            if meta.tags.contains(&tag.to_string()) {
                handles_to_remove.push(handle.clone());
            }
        }

        let mut deleted = 0;
        let mut failed = 0;

        for handle in handles_to_remove {
            if let Some(meta) = inner.files.remove(&handle) {
                match fs::remove_file(&meta.path).await {
                    Ok(_) => {
                        deleted += 1;
                        eprintln!(
                            "[temp_lifecycle] Cleaned up tagged temp file: {:?}",
                            meta.path
                        );
                    }
                    Err(e) => {
                        failed += 1;
                        eprintln!(
                            "[temp_lifecycle] Failed to clean up temp file {:?}: {}",
                            meta.path, e
                        );
                    }
                }
            }
        }

        CleanupResult { deleted, failed }
    }

    /// Get stats about managed temp files
    pub async fn stats(&self) -> TempRegistryStats {
        let inner = self.inner.read().await;
        let now = Instant::now();

        let total_files = inner.files.len();
        let expired_files = inner.files.values().filter(|m| m.expires_at <= now).count();
        let total_size: usize = inner
            .files
            .values()
            .filter_map(|m| {
                std::fs::metadata(&m.path)
                    .ok()
                    .map(|meta| meta.len() as usize)
            })
            .sum();

        TempRegistryStats {
            total_files,
            expired_files,
            total_size_bytes: total_size,
        }
    }

    /// Start background cleanup task
    pub fn start_background_cleanup(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = interval(CLEANUP_INTERVAL);

            loop {
                interval.tick().await;

                let result = self.cleanup_expired().await;
                if result.deleted > 0 || result.failed > 0 {
                    eprintln!(
                        "[temp_lifecycle] Background cleanup: {} deleted, {} failed",
                        result.deleted, result.failed
                    );
                }
            }
        })
    }

    /// Get the base directory
    pub async fn base_dir(&self) -> PathBuf {
        let inner = self.inner.read().await;
        inner.base_dir.clone()
    }
}

/// Result of a cleanup operation
#[derive(Debug, Clone, Copy)]
pub struct CleanupResult {
    pub deleted: usize,
    pub failed: usize,
}

/// Stats about the temp file registry
#[derive(Debug, Clone, Copy)]
pub struct TempRegistryStats {
    pub total_files: usize,
    pub expired_files: usize,
    pub total_size_bytes: usize,
}

/// Global temp file registry singleton
static GLOBAL_REGISTRY: once_cell::sync::Lazy<Arc<TempFileRegistry>> =
    once_cell::sync::Lazy::new(|| Arc::new(TempFileRegistry::new()));

/// Get the global temp file registry
pub fn global_registry() -> Arc<TempFileRegistry> {
    GLOBAL_REGISTRY.clone()
}

/// Initialize the global temp file registry
pub async fn init_global_registry() -> Result<()> {
    GLOBAL_REGISTRY.initialize().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_scoped_temp() {
        let temp_dir = TempDir::new().unwrap();
        let registry = TempFileRegistry::with_base_dir(temp_dir.path().to_path_buf());
        registry.initialize().await.unwrap();

        let scoped = registry
            .create_scoped(Some("txt"), None, vec!["test".to_string()])
            .await
            .unwrap();
        assert!(scoped.path().exists());
        assert_eq!(scoped.path().extension().unwrap(), "txt");

        // Clean up
        scoped.delete().await.unwrap();
    }

    #[tokio::test]
    async fn test_write_buffer() {
        let temp_dir = TempDir::new().unwrap();
        let registry = TempFileRegistry::with_base_dir(temp_dir.path().to_path_buf());
        registry.initialize().await.unwrap();

        let data = b"Hello, World!";
        let scoped = registry
            .write_buffer(data, Some("txt"), None, vec![])
            .await
            .unwrap();

        assert!(scoped.path().exists());
        let read_data = fs::read(scoped.path()).await.unwrap();
        assert_eq!(read_data, data);

        scoped.delete().await.unwrap();
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let temp_dir = TempDir::new().unwrap();
        let registry = TempFileRegistry::with_base_dir(temp_dir.path().to_path_buf());
        registry.initialize().await.unwrap();

        // Create temp file with very short TTL
        let scoped = registry
            .create_scoped(Some("txt"), Some(Duration::from_millis(10)), vec![])
            .await
            .unwrap();
        let path = scoped.path().to_path_buf();

        // Don't delete - let it expire
        drop(scoped);

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Run cleanup
        let result = registry.cleanup_expired().await;
        assert_eq!(result.deleted, 1);
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn test_cleanup_by_tag() {
        let temp_dir = TempDir::new().unwrap();
        let registry = TempFileRegistry::with_base_dir(temp_dir.path().to_path_buf());
        registry.initialize().await.unwrap();

        let scoped1 = registry
            .create_scoped(Some("txt"), None, vec!["tag1".to_string()])
            .await
            .unwrap();
        let scoped2 = registry
            .create_scoped(Some("txt"), None, vec!["tag2".to_string()])
            .await
            .unwrap();

        let path1 = scoped1.path().to_path_buf();
        let path2 = scoped2.path().to_path_buf();

        // Drop without deleting
        drop(scoped1);
        drop(scoped2);

        // Clean up by tag
        let result = registry.cleanup_by_tag("tag1").await;
        assert_eq!(result.deleted, 1);
        assert!(!path1.exists());
        assert!(path2.exists());

        // Clean up remaining
        let _ = registry.cleanup_all().await;
    }

    #[tokio::test]
    async fn test_stats() {
        let temp_dir = TempDir::new().unwrap();
        let registry = TempFileRegistry::with_base_dir(temp_dir.path().to_path_buf());
        registry.initialize().await.unwrap();

        let scoped = registry
            .write_buffer(b"test data", Some("txt"), None, vec![])
            .await
            .unwrap();

        let stats = registry.stats().await;
        assert_eq!(stats.total_files, 1);
        assert_eq!(stats.total_size_bytes, 9); // "test data"

        scoped.delete().await.unwrap();
    }
}
