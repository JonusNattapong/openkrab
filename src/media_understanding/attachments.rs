use std::path::PathBuf;
use futures_util::FutureExt;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct MediaAttachment {
    pub index: usize,
    pub path: Option<String>,
    pub url: Option<String>,
    pub mime: Option<String>,
    pub already_transcribed: bool,
}

impl Default for MediaAttachment {
    fn default() -> Self {
        Self {
            index: 0,
            path: None,
            url: None,
            mime: None,
            already_transcribed: false,
        }
    }
}

pub fn resolve_attachment_kind(attachment: &MediaAttachment) -> &'static str {
    if let Some(mime) = &attachment.mime {
        let kind = crate::media::mime::kind_from_mime(Some(mime));
        match kind {
            crate::media::mime::MediaKind::Image => return "image",
            crate::media::mime::MediaKind::Audio => return "audio",
            crate::media::mime::MediaKind::Video => return "video",
            _ => {}
        }
    }

    let ext = crate::media::mime::get_file_extension(
        attachment.path.as_deref().or(attachment.url.as_deref()),
    );

    if let Some(e) = ext {
        if [".mp4", ".mov", ".mkv", ".webm", ".avi", ".m4v"].contains(&e) {
            return "video";
        }
        if crate::media::mime::is_audio_file_name(
            attachment.path.as_deref().or(attachment.url.as_deref()),
        ) {
            return "audio";
        }
        if [
            ".png", ".jpg", ".jpeg", ".webp", ".gif", ".bmp", ".tiff", ".tif",
        ]
        .contains(&e)
        {
            return "image";
        }
    }

    "unknown"
}

pub fn is_video_attachment(attachment: &MediaAttachment) -> bool {
    resolve_attachment_kind(attachment) == "video"
}

pub fn is_audio_attachment(attachment: &MediaAttachment) -> bool {
    resolve_attachment_kind(attachment) == "audio"
}

pub fn is_image_attachment(attachment: &MediaAttachment) -> bool {
    resolve_attachment_kind(attachment) == "image"
}

#[derive(Debug)]
pub struct MediaBufferResult {
    pub buffer: Vec<u8>,
    pub mime: Option<String>,
    pub file_name: String,
    pub size: usize,
}

pub struct MediaPathResult {
    pub path: PathBuf,
    pub cleanup: Option<Box<dyn std::future::Future<Output = ()> + Send>>,
}

impl std::fmt::Debug for MediaPathResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MediaPathResult")
            .field("path", &self.path)
            .field("cleanup", &"...")
            .finish()
    }
}

pub struct MediaAttachmentCache {
    attachments: Vec<MediaAttachment>,
}

impl MediaAttachmentCache {
    pub fn new(attachments: Vec<MediaAttachment>) -> Self {
        Self { attachments }
    }

    pub async fn get_buffer(
        &self,
        attachment_index: usize,
        max_bytes: usize,
        timeout_ms: u64,
    ) -> Result<MediaBufferResult, MediaAttachmentError> {
        let attachment = self
            .attachments
            .get(attachment_index)
            .ok_or_else(|| MediaAttachmentError::NotFound(attachment_index))?;

        if let Some(path) = &attachment.path {
            let path = PathBuf::from(path);
            let metadata = fs::metadata(&path)
                .await
                .map_err(|e| MediaAttachmentError::Io(e.to_string()))?;

            if metadata.len() as usize > max_bytes {
                return Err(MediaAttachmentError::TooLarge(max_bytes));
            }

            let buffer = fs::read(&path)
                .await
                .map_err(|e| MediaAttachmentError::Io(e.to_string()))?;

            let mime = attachment.mime.clone().or_else(|| {
                crate::media::mime::detect_mime(
                    Some(&buffer),
                    None,
                    Some(path.to_str().unwrap_or("")),
                )
                .now_or_never()
                .flatten()
            });

            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&format!("media-{}", attachment_index + 1))
                .to_string();

            return Ok(MediaBufferResult {
                size: buffer.len(),
                buffer,
                mime,
                file_name,
            });
        }

        if let Some(url) = &attachment.url {
            let options = crate::media::fetch::FetchMediaOptions {
                url: url.clone(),
                file_path_hint: None,
                max_bytes: Some(max_bytes),
                max_redirects: Some(3),
                timeout_ms: Some(timeout_ms),
            };

            let result = crate::media::fetch::fetch_remote_media(options)
                .await
                .map_err(|e| MediaAttachmentError::Fetch(e.to_string()))?;

            return Ok(MediaBufferResult {
                size: result.buffer.len(),
                buffer: result.buffer,
                mime: result.content_type,
                file_name: result
                    .file_name
                    .unwrap_or_else(|| format!("media-{}", attachment_index + 1)),
            });
        }

        Err(MediaAttachmentError::NoContent(attachment_index))
    }

    pub fn attachments(&self) -> &[MediaAttachment] {
        &self.attachments
    }
}

#[derive(Debug)]
pub enum MediaAttachmentError {
    NotFound(usize),
    NoContent(usize),
    TooLarge(usize),
    Io(String),
    Fetch(String),
}

impl std::fmt::Display for MediaAttachmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaAttachmentError::NotFound(idx) => write!(f, "Attachment {} not found", idx),
            MediaAttachmentError::NoContent(idx) => write!(f, "Attachment {} has no content", idx),
            MediaAttachmentError::TooLarge(max) => write!(f, "Attachment exceeds {} bytes", max),
            MediaAttachmentError::Io(e) => write!(f, "IO error: {}", e),
            MediaAttachmentError::Fetch(e) => write!(f, "Fetch error: {}", e),
        }
    }
}

impl std::error::Error for MediaAttachmentError {}

pub fn select_attachments(
    capability: &str,
    attachments: &[MediaAttachment],
    max_attachments: usize,
) -> Vec<MediaAttachment> {
    let matches: Vec<MediaAttachment> = attachments
        .iter()
        .filter(|att| {
            if capability == "audio" && att.already_transcribed {
                return false;
            }
            match capability {
                "image" => is_image_attachment(att),
                "audio" => is_audio_attachment(att),
                "video" => is_video_attachment(att),
                _ => false,
            }
        })
        .cloned()
        .collect();

    matches.into_iter().take(max_attachments.max(1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_attachment_kind() {
        let img = MediaAttachment {
            mime: Some("image/png".to_string()),
            ..Default::default()
        };
        assert_eq!(resolve_attachment_kind(&img), "image");

        let audio = MediaAttachment {
            mime: Some("audio/mpeg".to_string()),
            ..Default::default()
        };
        assert_eq!(resolve_attachment_kind(&audio), "audio");
    }

    #[test]
    fn test_is_video_attachment() {
        let video = MediaAttachment {
            mime: Some("video/mp4".to_string()),
            ..Default::default()
        };
        assert!(is_video_attachment(&video));

        let img = MediaAttachment {
            mime: Some("image/png".to_string()),
            ..Default::default()
        };
        assert!(!is_video_attachment(&img));
    }

    #[test]
    fn test_select_attachments() {
        let attachments = vec![
            MediaAttachment {
                mime: Some("image/png".to_string()),
                index: 0,
                ..Default::default()
            },
            MediaAttachment {
                mime: Some("audio/mpeg".to_string()),
                index: 1,
                ..Default::default()
            },
        ];

        let selected = select_attachments("image", &attachments, 1);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].index, 0);
    }
}
