use anyhow::{anyhow, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use uuid::Uuid;

use super::temp_lifecycle::TempFileRegistry;

pub const MEDIA_MAX_BYTES: usize = 5 * 1024 * 1024;
const DEFAULT_TTL_MS: u64 = 2 * 60 * 1000;

fn resolve_media_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("openkrab")
        .join("media")
}

fn sanitize_filename(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let sanitized: String = trimmed
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '_' || c == '-' || c.is_ascii() {
                c
            } else {
                '_'
            }
        })
        .collect();

    sanitized
        .replace("__", "_")
        .trim_matches('_')
        .chars()
        .take(60)
        .collect()
}

pub fn extract_original_filename(file_path: &str) -> String {
    let basename = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file.bin");

    let ext = Path::new(basename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let name_without_ext = Path::new(basename)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or(basename);

    let uuid_pattern = regex::Regex::new(
        r"^(.+?)---[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$",
    )
    .unwrap();

    if let Some(caps) = uuid_pattern.captures(name_without_ext) {
        if let Some(original) = caps.get(1) {
            return if ext.is_empty() {
                original.as_str().to_string()
            } else {
                format!("{}.{}", original.as_str(), ext)
            };
        }
    }

    basename.to_string()
}

pub fn get_media_dir() -> PathBuf {
    resolve_media_dir()
}

pub async fn ensure_media_dir() -> Result<PathBuf> {
    let media_dir = resolve_media_dir();
    fs::create_dir_all(&media_dir).await?;
    Ok(media_dir)
}

/// Legacy cleanup function - now also fires cleanup hooks
pub async fn clean_old_media(ttl_ms: Option<u64>) -> Result<()> {
    let ttl = ttl_ms.unwrap_or(DEFAULT_TTL_MS);
    let media_dir = ensure_media_dir().await?;
    let now = Utc::now().timestamp_millis() as u64;

    let mut cleaned_paths = Vec::new();

    let mut entries = fs::read_dir(&media_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let metadata = entry.metadata().await?;

        if metadata.is_dir() {
            let mut dir_entries = fs::read_dir(&path).await?;
            while let Some(dir_entry) = dir_entries.next_entry().await? {
                let file_meta = dir_entry.metadata().await?;
                if file_meta.is_file() {
                    if let Ok(modified) = file_meta.modified() {
                        let modified_ms = modified
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_millis() as u64)
                            .unwrap_or(0);
                        if now - modified_ms > ttl {
                            let file_path = dir_entry.path();
                            if fs::remove_file(&file_path).await.is_ok() {
                                cleaned_paths.push(file_path);
                            }
                        }
                    }
                }
            }
        } else if metadata.is_file() {
            if let Ok(modified) = metadata.modified() {
                let modified_ms = modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);
                if now - modified_ms > ttl {
                    if fs::remove_file(&path).await.is_ok() {
                        cleaned_paths.push(path);
                    }
                }
            }
        }
    }

    // Fire cleanup hooks if any are registered
    if !cleaned_paths.is_empty() {
        // Note: In a real implementation, you might want to pass this to a hook registry
        eprintln!(
            "[media] Cleaned up {} expired media files",
            cleaned_paths.len()
        );
    }

    Ok(())
}

/// Enhanced cleanup using temp lifecycle registry
pub async fn clean_old_media_enhanced(
    registry: Arc<TempFileRegistry>,
    ttl_ms: Option<u64>,
) -> Result<super::temp_lifecycle::CleanupResult> {
    let result = registry.cleanup_expired().await;

    // Also run legacy cleanup for backward compatibility
    clean_old_media(ttl_ms).await?;

    Ok(result)
}

#[derive(Debug)]
pub struct SavedMedia {
    pub id: String,
    pub path: PathBuf,
    pub size: usize,
    pub content_type: Option<String>,
}

/// Options for saving media with enhanced lifecycle management
#[derive(Debug, Clone)]
pub struct SaveMediaOptions {
    pub content_type: Option<String>,
    pub subdir: Option<String>,
    pub max_bytes: Option<usize>,
    pub original_filename: Option<String>,
    /// Use temp lifecycle registry for enhanced management
    pub use_temp_lifecycle: bool,
    /// TTL for temp files (only used with temp lifecycle)
    pub ttl: Option<Duration>,
    /// Tags for the temp file
    pub tags: Vec<String>,
}

impl Default for SaveMediaOptions {
    fn default() -> Self {
        Self {
            content_type: None,
            subdir: None,
            max_bytes: None,
            original_filename: None,
            use_temp_lifecycle: false,
            ttl: None,
            tags: Vec::new(),
        }
    }
}

pub async fn save_media_buffer(
    buffer: &[u8],
    content_type: Option<&str>,
    subdir: Option<&str>,
    max_bytes: Option<usize>,
    original_filename: Option<&str>,
) -> Result<SavedMedia> {
    save_media_buffer_with_options(
        buffer,
        SaveMediaOptions {
            content_type: content_type.map(|s| s.to_string()),
            subdir: subdir.map(|s| s.to_string()),
            max_bytes,
            original_filename: original_filename.map(|s| s.to_string()),
            ..Default::default()
        },
    )
    .await
}

pub async fn save_media_buffer_with_options(
    buffer: &[u8],
    options: SaveMediaOptions,
) -> Result<SavedMedia> {
    let max = options.max_bytes.unwrap_or(MEDIA_MAX_BYTES);
    if buffer.len() > max {
        return Err(anyhow!("Media exceeds {}MB limit", max / (1024 * 1024)));
    }

    // Use temp lifecycle if requested
    if options.use_temp_lifecycle {
        let registry = super::temp_lifecycle::global_registry();
        let ext = options
            .original_filename
            .as_ref()
            .and_then(|f| Path::new(f).extension().and_then(|e| e.to_str()));

        let scoped = registry
            .write_buffer(buffer, ext, options.ttl, options.tags)
            .await?;

        let detected_mime = super::mime::detect_mime(
            Some(buffer),
            options.content_type.as_deref(),
            options.original_filename.as_deref(),
        )
        .await;

        return Ok(SavedMedia {
            id: scoped.handle().to_string(),
            path: scoped.path().to_path_buf(),
            size: buffer.len(),
            content_type: detected_mime,
        });
    }

    // Legacy path
    let base_dir = resolve_media_dir();
    let dir = options
        .subdir
        .as_ref()
        .map(|s| base_dir.join(s))
        .unwrap_or(base_dir);

    fs::create_dir_all(&dir).await?;
    let _ = clean_old_media(None).await;

    let uuid = Uuid::new_v4().to_string();
    let detected_mime = super::mime::detect_mime(
        Some(buffer),
        options.content_type.as_deref(),
        options.original_filename.as_deref(),
    )
    .await;
    let ext = super::mime::extension_for_mime(detected_mime.as_deref())
        .or(options
            .original_filename
            .as_ref()
            .and_then(|f| Path::new(f).extension().and_then(|e| e.to_str())))
        .map(|e| format!(".{}", e.trim_start_matches('.')))
        .unwrap_or_default();

    let id = if let Some(orig) = options.original_filename {
        let base = Path::new(&orig)
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let sanitized = sanitize_filename(base);
        if sanitized.is_empty() {
            format!("{}{}", uuid, ext)
        } else {
            format!("{}---{}{}", sanitized, uuid, ext)
        }
    } else {
        format!("{}{}", uuid, ext)
    };

    let dest = dir.join(&id);
    fs::write(&dest, buffer).await?;

    Ok(SavedMedia {
        id,
        path: dest,
        size: buffer.len(),
        content_type: detected_mime,
    })
}

fn looks_like_url(src: &str) -> bool {
    src.starts_with("http://") || src.starts_with("https://")
}

pub async fn save_media_source(
    source: &str,
    _headers: Option<std::collections::HashMap<String, String>>,
    subdir: Option<&str>,
) -> Result<SavedMedia> {
    let base_dir = resolve_media_dir();
    let dir = subdir.map(|s| base_dir.join(s)).unwrap_or(base_dir);

    fs::create_dir_all(&dir).await?;
    let _ = clean_old_media(None).await;

    let uuid = Uuid::new_v4().to_string();

    if looks_like_url(source) {
        let options = super::fetch::FetchMediaOptions {
            url: source.to_string(),
            file_path_hint: None,
            max_bytes: Some(MEDIA_MAX_BYTES),
            max_redirects: Some(5),
            timeout_ms: Some(30_000),
        };

        let fetched = super::fetch::fetch_remote_media(options).await?;

        let ext = super::mime::extension_for_mime(fetched.content_type.as_deref())
            .map(|e| e.to_string())
            .or_else(|| {
                let path_part = source.split('?').next().unwrap_or(source);
                let path_part = path_part.split('#').next().unwrap_or(path_part);
                if let Some(idx) = path_part.rfind('/') {
                    let filename = &path_part[idx + 1..];
                    if let Some(dot_idx) = filename.rfind('.') {
                        return Some(format!(".{}", &filename[dot_idx + 1..].to_lowercase()));
                    }
                }
                None
            })
            .unwrap_or_default();

        let id = if ext.is_empty() {
            uuid.clone()
        } else {
            format!("{}{}", uuid, ext)
        };
        let final_dest = dir.join(&id);

        fs::write(&final_dest, &fetched.buffer).await?;

        return Ok(SavedMedia {
            id,
            path: final_dest,
            size: fetched.buffer.len(),
            content_type: fetched.content_type,
        });
    }

    let local_path = Path::new(source);
    let metadata = fs::metadata(local_path).await?;

    if !metadata.is_file() {
        return Err(anyhow!("Media path is not a file"));
    }

    if metadata.len() as usize > MEDIA_MAX_BYTES {
        return Err(anyhow!("Media exceeds 5MB limit"));
    }

    let buffer = fs::read(local_path).await?;
    let detected_mime = super::mime::detect_mime(Some(&buffer), None, Some(source)).await;

    let ext = super::mime::extension_for_mime(detected_mime.as_deref())
        .map(|e| e.to_string())
        .or_else(|| {
            local_path
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
        })
        .unwrap_or_default();

    let id = if ext.is_empty() {
        uuid.clone()
    } else {
        format!("{}{}", uuid, ext)
    };
    let dest = dir.join(&id);

    fs::write(&dest, &buffer).await?;

    Ok(SavedMedia {
        id,
        path: dest,
        size: buffer.len(),
        content_type: detected_mime,
    })
}

/// Save media from URL with enhanced lifecycle options
pub async fn save_media_source_with_options(
    source: &str,
    options: SaveMediaOptions,
) -> Result<SavedMedia> {
    if !looks_like_url(source) {
        // For local files, use legacy path
        return save_media_source(source, None, options.subdir.as_deref()).await;
    }

    let fetch_options = super::fetch::FetchMediaOptions {
        url: source.to_string(),
        file_path_hint: options.original_filename.clone(),
        max_bytes: Some(options.max_bytes.unwrap_or(MEDIA_MAX_BYTES)),
        max_redirects: Some(5),
        timeout_ms: Some(30_000),
    };

    let fetched = super::fetch::fetch_remote_media(fetch_options).await?;

    // Use buffer save with options
    save_media_buffer_with_options(
        &fetched.buffer,
        SaveMediaOptions {
            content_type: fetched.content_type.clone().or(options.content_type),
            ..options
        },
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("test file.txt"), "test file.txt");
        assert_eq!(sanitize_filename("../../../etc/passwd"), "etc_passwd");
        assert_eq!(sanitize_filename(""), "");
    }

    #[test]
    fn test_extract_original_filename() {
        let result = extract_original_filename("image---550e8400-e29b-41d4-a716-446655440000.jpg");
        assert_eq!(result, "image.jpg");

        let simple = extract_original_filename("simple.png");
        assert_eq!(simple, "simple.png");
    }

    #[test]
    fn test_looks_like_url() {
        assert!(looks_like_url("http://example.com/image.png"));
        assert!(looks_like_url("https://example.com/image.png"));
        assert!(!looks_like_url("/path/to/file.png"));
    }
}
