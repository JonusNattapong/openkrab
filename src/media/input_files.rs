use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct InputImageContent {
    pub data: String,
    pub mime_type: String,
}

#[derive(Debug, Clone)]
pub struct InputFileExtractResult {
    pub filename: String,
    pub text: Option<String>,
    pub images: Option<Vec<InputImageContent>>,
}

#[derive(Debug, Clone)]
pub struct InputPdfLimits {
    pub max_pages: usize,
    pub max_pixels: usize,
    pub min_text_chars: usize,
}

#[derive(Debug, Clone)]
pub struct InputFileLimits {
    pub allow_url: bool,
    pub url_allowlist: Option<Vec<String>>,
    pub allowed_mimes: HashSet<String>,
    pub max_bytes: usize,
    pub max_chars: usize,
    pub max_redirects: usize,
    pub timeout_ms: u64,
    pub pdf: InputPdfLimits,
}

#[derive(Debug, Clone)]
pub struct InputImageLimits {
    pub allow_url: bool,
    pub url_allowlist: Option<Vec<String>>,
    pub allowed_mimes: HashSet<String>,
    pub max_bytes: usize,
    pub max_redirects: usize,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone)]
pub struct InputImageSource {
    pub source_type: String,
    pub data: Option<String>,
    pub url: Option<String>,
    pub media_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InputFileSource {
    pub source_type: String,
    pub data: Option<String>,
    pub url: Option<String>,
    pub media_type: Option<String>,
    pub filename: Option<String>,
}

pub const DEFAULT_INPUT_IMAGE_MIMES: &[&str] = &["image/jpeg", "image/png", "image/gif", "image/webp"];
pub const DEFAULT_INPUT_FILE_MIMES: &[&str] = &[
    "text/plain",
    "text/markdown",
    "text/html",
    "text/csv",
    "application/json",
    "application/pdf",
];
pub const DEFAULT_INPUT_IMAGE_MAX_BYTES: usize = 10 * 1024 * 1024;
pub const DEFAULT_INPUT_FILE_MAX_BYTES: usize = 5 * 1024 * 1024;
pub const DEFAULT_INPUT_FILE_MAX_CHARS: usize = 200_000;
pub const DEFAULT_INPUT_MAX_REDIRECTS: usize = 3;
pub const DEFAULT_INPUT_TIMEOUT_MS: u64 = 10_000;
pub const DEFAULT_INPUT_PDF_MAX_PAGES: usize = 4;
pub const DEFAULT_INPUT_PDF_MAX_PIXELS: usize = 4_000_000;
pub const DEFAULT_INPUT_PDF_MIN_TEXT_CHARS: usize = 200;

pub fn normalize_mime_type(value: Option<&str>) -> Option<String> {
    value.and_then(|v| {
        let cleaned = v.split(';').next()?.trim().to_lowercase();
        if cleaned.is_empty() { None } else { Some(cleaned) }
    })
}

pub fn parse_content_type(value: Option<&str>) -> (Option<String>, Option<String>) {
    let value = match value {
        Some(v) => v,
        None => return (None, None),
    };

    let parts: Vec<&str> = value.split(';').map(|p| p.trim()).collect();
    let mime_type = normalize_mime_type(parts.first().copied());
    
    let charset = parts.iter().skip(1).find_map(|part| {
        let part = part.trim();
        if let Some(stripped) = part.strip_prefix("charset=") {
            Some(stripped.trim().to_string())
        } else {
            None
        }
    });

    (mime_type, charset)
}

pub fn normalize_mime_list(values: Option<&[String]>, fallback: &[&str]) -> HashSet<String> {
    let input = values.filter(|v| !v.is_empty()).map(|v| v as &[_]).unwrap_or(fallback);
    input.iter().filter_map(|v| normalize_mime_type(Some(v))).collect()
}

pub fn resolve_input_file_limits(
    allow_url: Option<bool>,
    allowed_mimes: Option<&[String]>,
    max_bytes: Option<usize>,
    max_chars: Option<usize>,
    max_redirects: Option<usize>,
    timeout_ms: Option<u64>,
    pdf_max_pages: Option<usize>,
    pdf_max_pixels: Option<usize>,
    pdf_min_text_chars: Option<usize>,
) -> InputFileLimits {
    InputFileLimits {
        allow_url: allow_url.unwrap_or(true),
        allowed_mimes: normalize_mime_list(allowed_mimes, DEFAULT_INPUT_FILE_MIMES),
        max_bytes: max_bytes.unwrap_or(DEFAULT_INPUT_FILE_MAX_BYTES),
        max_chars: max_chars.unwrap_or(DEFAULT_INPUT_FILE_MAX_CHARS),
        max_redirects: max_redirects.unwrap_or(DEFAULT_INPUT_MAX_REDIRECTS),
        timeout_ms: timeout_ms.unwrap_or(DEFAULT_INPUT_TIMEOUT_MS),
        pdf: InputPdfLimits {
            max_pages: pdf_max_pages.unwrap_or(DEFAULT_INPUT_PDF_MAX_PAGES),
            max_pixels: pdf_max_pixels.unwrap_or(DEFAULT_INPUT_PDF_MAX_PIXELS),
            min_text_chars: pdf_min_text_chars.unwrap_or(DEFAULT_INPUT_PDF_MIN_TEXT_CHARS),
        },
    }
}

fn decode_text_content(buffer: &[u8], charset: Option<&str>) -> String {
    let encoding = charset.unwrap_or("utf-8");
    
    match encoding.to_lowercase().as_str() {
        "utf-8" | "utf8" => String::from_utf8_lossy(buffer).to_string(),
        "utf-16" | "utf16" | "utf-16le" => {
            let (cow, _had_errors) = encoding_rs::UTF_16LE.decode(buffer);
            cow.to_string()
        }
        "utf-16be" => {
            let (cow, _had_errors) = encoding_rs::UTF_16BE.decode(buffer);
            cow.to_string()
        }
        "iso-8859-1" | "latin1" | "latin-1" => {
            let (cow, _had_errors) = encoding_rs::WINDOWS_1252.decode(buffer);
            cow.to_string()
        }
        _ => String::from_utf8_lossy(buffer).to_string(),
    }
}

fn clamp_text(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        text.to_string()
    } else {
        text.chars().take(max_chars).collect()
    }
}

pub async fn extract_file_content_from_source(
    source: &InputFileSource,
    limits: &InputFileLimits,
) -> Result<InputFileExtractResult> {
    let filename = source.filename.clone().unwrap_or_else(|| "file".to_string());

    let buffer: Vec<u8>;
    let mime_type: Option<String>;
    let charset: Option<String>;

    if source.source_type == "base64" {
        let data = source.data.as_ref().ok_or_else(|| anyhow!("Missing base64 data"))?;
        buffer = base64_decode(data)?;
        let (mt, cs) = parse_content_type(source.media_type.as_deref());
        mime_type = mt;
        charset = cs;
    } else if source.source_type == "url" {
        let url = source.url.as_ref().ok_or_else(|| anyhow!("Missing URL"))?;
        
        if !limits.allow_url {
            return Err(anyhow!("URL sources are disabled"));
        }

        let options = crate::media::fetch::FetchMediaOptions {
            url: url.clone(),
            file_path_hint: source.filename.clone(),
            max_bytes: Some(limits.max_bytes),
            max_redirects: Some(limits.max_redirects),
            timeout_ms: Some(limits.timeout_ms),
        };

        let result = crate::media::fetch::fetch_remote_media(options).await?;
        buffer = result.buffer;
        let (mt, cs) = parse_content_type(result.content_type.as_deref());
        mime_type = mt.or(result.content_type);
        charset = cs;
    } else {
        return Err(anyhow!("Source must have url or data"));
    }

    if buffer.len() > limits.max_bytes {
        return Err(anyhow!("File too large: {} bytes (limit: {})", buffer.len(), limits.max_bytes));
    }

    let final_mime = mime_type.ok_or_else(|| anyhow!("Missing media type"))?;
    
    if !limits.allowed_mimes.contains(&final_mime) {
        return Err(anyhow!("Unsupported MIME type: {}", final_mime));
    }

    if final_mime == "application/pdf" {
        let text = extract_pdf_text(&buffer, limits)?;
        return Ok(InputFileExtractResult {
            filename,
            text: Some(clamp_text(&text, limits.max_chars)),
            images: None,
        });
    }

    let text = clamp_text(&decode_text_content(&buffer, charset.as_deref()), limits.max_chars);
    Ok(InputFileExtractResult {
        filename,
        text: Some(text),
        images: None,
    })
}

fn extract_pdf_text(buffer: &[u8], limits: &InputFileLimits) -> Result<String> {
    Ok(format!("[PDF content: {} bytes, max {} pages]", buffer.len(), limits.pdf.max_pages))
}

fn base64_decode(data: &str) -> Result<Vec<u8>> {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD
        .decode(data)
        .map_err(|e| anyhow!("Base64 decode error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_mime_type() {
        assert_eq!(normalize_mime_type(Some("image/png")), Some("image/png".to_string()));
        assert_eq!(normalize_mime_type(Some("image/png; charset=utf-8")), Some("image/png".to_string()));
        assert_eq!(normalize_mime_type(None), None);
    }

    #[test]
    fn test_parse_content_type() {
        let (mime, charset) = parse_content_type(Some("text/html; charset=utf-8"));
        assert_eq!(mime, Some("text/html".to_string()));
        assert_eq!(charset, Some("utf-8".to_string()));
    }

    #[test]
    fn test_clamp_text() {
        let text = "Hello World";
        assert_eq!(clamp_text(text, 5), "Hello");
        assert_eq!(clamp_text(text, 100), text);
    }

    #[test]
    fn test_base64_decode() {
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"hello");
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, b"hello");
    }
}
