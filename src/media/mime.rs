use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

static EXT_BY_MIME: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("image/heic", ".heic");
    map.insert("image/heif", ".heif");
    map.insert("image/jpeg", ".jpg");
    map.insert("image/png", ".png");
    map.insert("image/webp", ".webp");
    map.insert("image/gif", ".gif");
    map.insert("audio/ogg", ".ogg");
    map.insert("audio/mpeg", ".mp3");
    map.insert("audio/x-m4a", ".m4a");
    map.insert("audio/mp4", ".m4a");
    map.insert("video/mp4", ".mp4");
    map.insert("video/quicktime", ".mov");
    map.insert("application/pdf", ".pdf");
    map.insert("application/json", ".json");
    map.insert("application/zip", ".zip");
    map.insert("application/gzip", ".gz");
    map.insert("application/x-tar", ".tar");
    map.insert("application/x-7z-compressed", ".7z");
    map.insert("application/vnd.rar", ".rar");
    map.insert("application/msword", ".doc");
    map.insert("application/vnd.ms-excel", ".xls");
    map.insert("application/vnd.ms-powerpoint", ".ppt");
    map.insert(
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ".docx",
    );
    map.insert(
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ".xlsx",
    );
    map.insert(
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        ".pptx",
    );
    map.insert("text/csv", ".csv");
    map.insert("text/plain", ".txt");
    map.insert("text/markdown", ".md");
    map
});

static MIME_BY_EXT: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (mime, ext) in EXT_BY_MIME.iter() {
        map.insert(*ext, *mime);
    }
    map.insert(".jpeg", "image/jpeg");
    map
});

static AUDIO_FILE_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert(".aac");
    set.insert(".caf");
    set.insert(".flac");
    set.insert(".m4a");
    set.insert(".mp3");
    set.insert(".oga");
    set.insert(".ogg");
    set.insert(".opus");
    set.insert(".wav");
    set
});

pub fn normalize_mime(mime: Option<&str>) -> Option<&str> {
    mime.and_then(|m| {
        let cleaned = m.split(';').next()?.trim().to_lowercase();
        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned.leak() as &'static str)
        }
    })
}

pub fn get_file_extension(file_path: Option<&str>) -> Option<&'static str> {
    let path = file_path?;

    if path.starts_with("http://") || path.starts_with("https://") {
        let path_part = path.split('?').next().unwrap_or(path);
        let path_part = path_part.split('#').next().unwrap_or(path_part);
        if let Some(idx) = path_part.rfind('/') {
            let filename = &path_part[idx + 1..];
            if let Some(dot_idx) = filename.rfind('.') {
                let ext = &filename[dot_idx..];
                let ext_lower = ext.to_lowercase();
                return Some(ext_lower.leak() as &'static str);
            }
        }
        return None;
    }

    std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e.to_lowercase()))
        .map(|e| e.leak() as &'static str)
}

pub fn is_audio_file_name(file_name: Option<&str>) -> bool {
    let ext = get_file_extension(file_name);
    ext.map_or(false, |e| AUDIO_FILE_EXTENSIONS.contains(e))
}

fn is_generic_mime(mime: Option<&str>) -> bool {
    match mime {
        None => true,
        Some(m) => m == "application/octet-stream" || m == "application/zip",
    }
}

pub async fn detect_mime(
    buffer: Option<&[u8]>,
    header_mime: Option<&str>,
    file_path: Option<&str>,
) -> Option<String> {
    let ext = get_file_extension(file_path);
    let ext_mime = ext.and_then(|e| MIME_BY_EXT.get(e)).copied();

    let header_mime = normalize_mime(header_mime);
    let sniffed = sniff_mime(buffer).await;

    if let Some(ref s) = sniffed {
        if !is_generic_mime(Some(s)) || ext_mime.is_none() {
            return Some(s.clone());
        }
    }

    if let Some(em) = ext_mime {
        return Some(em.to_string());
    }

    if let Some(h) = header_mime {
        if !is_generic_mime(Some(h)) {
            return Some(h.to_string());
        }
    }

    if let Some(s) = sniffed {
        return Some(s);
    }

    header_mime.map(|h| h.to_string())
}

async fn sniff_mime(buffer: Option<&[u8]>) -> Option<String> {
    let buf = buffer?;

    if buf.len() < 4 {
        return None;
    }

    if &buf[0..4] == b"\x89PNG" {
        return Some("image/png".to_string());
    }
    if &buf[0..2] == b"\xff\xd8" {
        return Some("image/jpeg".to_string());
    }
    if &buf[0..4] == b"GIF8" {
        return Some("image/gif".to_string());
    }
    if &buf[0..4] == b"RIFF" && buf.len() >= 12 && &buf[8..12] == b"WEBP" {
        return Some("image/webp".to_string());
    }
    if &buf[4..8] == b"ftyp" {
        return Some("video/mp4".to_string());
    }
    if &buf[0..4] == b"\x1a\x45\xdf\xa3" {
        return Some("video/webm".to_string());
    }
    if &buf[0..5] == b"%PDF-" {
        return Some("application/pdf".to_string());
    }
    if buf.starts_with(b"OggS") {
        return Some("audio/ogg".to_string());
    }
    if buf.starts_with(b"ID3") || (buf.len() >= 3 && &buf[0..3] == b"\xff\xfb") {
        return Some("audio/mpeg".to_string());
    }

    None
}

pub fn extension_for_mime(mime: Option<&str>) -> Option<&'static str> {
    let normalized = normalize_mime(mime)?;
    EXT_BY_MIME.get(normalized).copied()
}

pub fn is_gif_media(content_type: Option<&str>, file_name: Option<&str>) -> bool {
    if content_type
        .map(|c| c.to_lowercase() == "image/gif")
        .unwrap_or(false)
    {
        return true;
    }
    get_file_extension(file_name) == Some(".gif")
}

pub fn image_mime_from_format(format: Option<&str>) -> Option<&'static str> {
    match format?.to_lowercase().as_str() {
        "jpg" | "jpeg" => Some("image/jpeg"),
        "heic" => Some("image/heic"),
        "heif" => Some("image/heif"),
        "png" => Some("image/png"),
        "webp" => Some("image/webp"),
        "gif" => Some("image/gif"),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Image,
    Audio,
    Video,
    Document,
    Sticker,
    Voice,
    Unknown,
}

pub fn kind_from_mime(mime: Option<&str>) -> MediaKind {
    let normalized = normalize_mime(mime);
    match normalized {
        Some(m) if m.starts_with("image/") => MediaKind::Image,
        Some(m) if m.starts_with("audio/") => MediaKind::Audio,
        Some(m) if m.starts_with("video/") => MediaKind::Video,
        Some(m) if m.contains("pdf") || m.starts_with("application/") => MediaKind::Document,
        _ => MediaKind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_mime() {
        assert_eq!(normalize_mime(Some("image/png")), Some("image/png"));
        assert_eq!(
            normalize_mime(Some("image/png; charset=utf-8")),
            Some("image/png")
        );
        assert_eq!(normalize_mime(None), None);
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension(Some("image.png")), Some(".png"));
        assert_eq!(get_file_extension(Some("path/to/file.jpg")), Some(".jpg"));
    }

    #[test]
    fn test_is_audio_file_name() {
        assert!(is_audio_file_name(Some("audio.mp3")));
        assert!(is_audio_file_name(Some("voice.oga")));
        assert!(!is_audio_file_name(Some("image.png")));
    }

    #[test]
    fn test_kind_from_mime() {
        assert_eq!(kind_from_mime(Some("image/jpeg")), MediaKind::Image);
        assert_eq!(kind_from_mime(Some("audio/mpeg")), MediaKind::Audio);
        assert_eq!(kind_from_mime(Some("video/mp4")), MediaKind::Video);
        assert_eq!(kind_from_mime(Some("application/pdf")), MediaKind::Document);
        assert_eq!(kind_from_mime(None), MediaKind::Unknown);
    }
}
