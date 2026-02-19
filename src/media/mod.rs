//! media — Media type definitions and handling utilities.
//! Ported from `openclaw/src/media/` (Phase 5).
//!
//! Provides normalised representations for images, audio, video, files,
//! and documents that can flow through the agent pipeline.

pub mod audio;
pub mod fetch;
pub mod image_ops;
pub mod input_files;
pub mod mime;
pub mod parse;
pub mod store;
pub mod temp_lifecycle;

use serde::{Deserialize, Serialize};

// ─── Media kind ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaKind {
    Image,
    Audio,
    Video,
    Document,
    Sticker,
    Voice,
    Unknown,
}

impl MediaKind {
    pub fn from_mime(mime: &str) -> Self {
        let m = mime.trim().to_lowercase();
        if m.starts_with("image/") {
            MediaKind::Image
        } else if m.starts_with("audio/") {
            MediaKind::Audio
        } else if m.starts_with("video/") {
            MediaKind::Video
        } else if m.contains("pdf") || m.starts_with("application/") {
            MediaKind::Document
        } else {
            MediaKind::Unknown
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MediaKind::Image => "image",
            MediaKind::Audio => "audio",
            MediaKind::Video => "video",
            MediaKind::Document => "document",
            MediaKind::Sticker => "sticker",
            MediaKind::Voice => "voice",
            MediaKind::Unknown => "unknown",
        }
    }
}

// ─── Media item ───────────────────────────────────────────────────────────────

/// A normalised media attachment that can come from any connector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    /// Type of media.
    pub kind: MediaKind,
    /// URL or platform-specific file_id for retrieval.
    pub url: Option<String>,
    /// Raw bytes (if already downloaded).
    #[serde(skip)]
    pub data: Option<Vec<u8>>,
    /// MIME type string.
    pub mime_type: Option<String>,
    /// File name (if available from the connector).
    pub filename: Option<String>,
    /// File size in bytes (if known).
    pub size_bytes: Option<u64>,
    /// Caption / alt text supplied by the sender.
    pub caption: Option<String>,
    /// Duration in seconds (for audio/video).
    pub duration_secs: Option<f32>,
    /// Width × height for images/videos.
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl MediaItem {
    pub fn new(kind: MediaKind) -> Self {
        Self {
            kind,
            url: None,
            data: None,
            mime_type: None,
            filename: None,
            size_bytes: None,
            caption: None,
            duration_secs: None,
            width: None,
            height: None,
        }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn with_mime(mut self, mime: impl Into<String>) -> Self {
        let m = mime.into();
        self.kind = MediaKind::from_mime(&m);
        self.mime_type = Some(m);
        self
    }

    pub fn with_dimensions(mut self, w: u32, h: u32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    /// Returns `true` if this item has either a URL or raw data.
    pub fn has_content(&self) -> bool {
        self.url.is_some() || self.data.is_some()
    }
}

// ─── Media message ────────────────────────────────────────────────────────────

/// A message that may contain one or more media attachments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMessage {
    pub text: Option<String>,
    pub attachments: Vec<MediaItem>,
}

impl MediaMessage {
    pub fn text_only(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            attachments: Vec::new(),
        }
    }

    pub fn add_attachment(mut self, item: MediaItem) -> Self {
        self.attachments.push(item);
        self
    }

    pub fn is_media_only(&self) -> bool {
        self.text
            .as_deref()
            .map(|t| t.trim().is_empty())
            .unwrap_or(true)
            && !self.attachments.is_empty()
    }

    pub fn has_images(&self) -> bool {
        self.attachments.iter().any(|a| a.kind == MediaKind::Image)
    }

    pub fn has_audio(&self) -> bool {
        self.attachments
            .iter()
            .any(|a| matches!(a.kind, MediaKind::Audio | MediaKind::Voice))
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Normalise a file extension to a MIME type (best-effort).
pub fn ext_to_mime(ext: &str) -> &'static str {
    match ext.trim_start_matches('.').to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "mp3" => "audio/mpeg",
        "ogg" | "oga" => "audio/ogg",
        "wav" => "audio/wav",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "pdf" => "application/pdf",
        "txt" => "text/plain",
        _ => "application/octet-stream",
    }
}

/// Guess the `MediaKind` from a URL's file extension.
pub fn kind_from_url(url: &str) -> MediaKind {
    let ext = url
        .split('.')
        .last()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("");
    MediaKind::from_mime(ext_to_mime(ext))
}

// Re-export temp_lifecycle types
pub use temp_lifecycle::{
    global_registry, init_global_registry, CleanupResult, ScopedTempFile, TempFileMeta,
    TempFileRegistry, TempHandle, TempRegistryStats, DEFAULT_TEMP_TTL,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_kind_from_mime() {
        assert_eq!(MediaKind::from_mime("image/jpeg"), MediaKind::Image);
        assert_eq!(MediaKind::from_mime("audio/mpeg"), MediaKind::Audio);
        assert_eq!(MediaKind::from_mime("video/mp4"), MediaKind::Video);
        assert_eq!(MediaKind::from_mime("application/pdf"), MediaKind::Document);
        assert_eq!(MediaKind::from_mime("text/html"), MediaKind::Unknown);
    }

    #[test]
    fn ext_to_mime_examples() {
        assert_eq!(ext_to_mime("jpg"), "image/jpeg");
        assert_eq!(ext_to_mime("pdf"), "application/pdf");
        assert_eq!(ext_to_mime(".mp3"), "audio/mpeg");
    }

    #[test]
    fn media_item_builder() {
        let item = MediaItem::new(MediaKind::Image)
            .with_url("https://example.com/img.jpg")
            .with_caption("test")
            .with_dimensions(800, 600);
        assert!(item.has_content());
        assert_eq!(item.width, Some(800));
        assert_eq!(item.caption.as_deref(), Some("test"));
    }

    #[test]
    fn media_message_helpers() {
        let msg = MediaMessage::text_only("hello")
            .add_attachment(MediaItem::new(MediaKind::Image).with_url("http://img"));
        assert!(msg.has_images());
        assert!(!msg.has_audio());
        assert!(!msg.is_media_only());

        let media_only = MediaMessage {
            text: None,
            attachments: vec![MediaItem::new(MediaKind::Audio)],
        };
        assert!(media_only.is_media_only());
        assert!(media_only.has_audio());
    }
}
