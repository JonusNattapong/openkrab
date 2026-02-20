use anyhow::{anyhow, Result};
use reqwest::Response;
use std::path::Path;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFetchErrorCode {
    MaxBytes,
    HttpError,
    FetchFailed,
}

#[derive(Debug)]
pub struct MediaFetchError {
    pub code: MediaFetchErrorCode,
    pub message: String,
}

impl std::fmt::Display for MediaFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code_name(), self.message)
    }
}

impl std::error::Error for MediaFetchError {}

impl MediaFetchError {
    fn code_name(&self) -> &'static str {
        match self.code {
            MediaFetchErrorCode::MaxBytes => "max_bytes",
            MediaFetchErrorCode::HttpError => "http_error",
            MediaFetchErrorCode::FetchFailed => "fetch_failed",
        }
    }
}

#[derive(Debug)]
pub struct FetchMediaResult {
    pub buffer: Vec<u8>,
    pub content_type: Option<String>,
    pub file_name: Option<String>,
}

pub struct FetchMediaOptions {
    pub url: String,
    pub file_path_hint: Option<String>,
    pub max_bytes: Option<usize>,
    pub max_redirects: Option<usize>,
    pub timeout_ms: Option<u64>,
}

fn strip_quotes(value: &str) -> String {
    value.trim_matches(|c| c == '"' || c == '\'').to_string()
}

fn parse_content_disposition_filename(header: Option<&str>) -> Option<String> {
    let header = header?;

    let star_match = regex::Regex::new(r"filename\*\s*=\s*([^;]+)")
        .ok()?
        .captures(header)?;

    if let Some(matched) = star_match.get(1) {
        let cleaned = strip_quotes(matched.as_str().trim());
        let encoded = cleaned.split("''").skip(1).collect::<Vec<_>>().join("''");
        let to_decode = if encoded.is_empty() {
            &cleaned
        } else {
            &encoded
        };
        let decoded = urlencoding::decode(to_decode).unwrap_or_default();
        return Some(
            Path::new(decoded.as_ref())
                .file_name()?
                .to_string_lossy()
                .to_string(),
        );
    }

    let match_re = regex::Regex::new(r"filename\s*=\s*([^;]+)").ok()?;
    let caps = match_re.captures(header)?;
    let matched = caps.get(1)?;
    Some(
        Path::new(strip_quotes(matched.as_str().trim()).as_str())
            .file_name()?
            .to_string_lossy()
            .to_string(),
    )
}

async fn read_error_body_snippet(res: Response, max_chars: usize) -> Option<String> {
    let bytes = res.bytes().await.ok()?;
    let text = String::from_utf8_lossy(&bytes);
    if text.is_empty() {
        return None;
    }
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        return None;
    }
    if collapsed.len() <= max_chars {
        return Some(collapsed);
    }
    Some(format!("{}…", &collapsed[..max_chars]))
}

pub async fn fetch_remote_media(options: FetchMediaOptions) -> Result<FetchMediaResult> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(
            options.timeout_ms.unwrap_or(30_000),
        ))
        .redirect(reqwest::redirect::Policy::limited(
            options.max_redirects.unwrap_or(5),
        ))
        .build()?;

    let res = client
        .get(&options.url)
        .send()
        .await
        .map_err(|e| MediaFetchError {
            code: MediaFetchErrorCode::FetchFailed,
            message: format!("Failed to fetch media from {}: {}", options.url, e),
        })?;

    if !res.status().is_success() {
        let status = res.status();
        let mut detail = format!("HTTP {}", status);
        if let Some(snippet) = read_error_body_snippet(res, 200).await {
            detail = format!("{}; body: {}", detail, snippet);
        }
        return Err(MediaFetchError {
            code: MediaFetchErrorCode::HttpError,
            message: format!("Failed to fetch media from {}: {}", options.url, detail),
        }
        .into());
    }

    let content_length = res.content_length();
    let final_url = res.url().clone();
    let content_disposition = res
        .headers()
        .get("content-disposition")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let header_mime = res
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    // Now consume the response for the body
    let buffer = if let Some(max_bytes) = options.max_bytes {
        let limited = res.bytes().await.map_err(|e| MediaFetchError {
            code: MediaFetchErrorCode::FetchFailed,
            message: format!("Failed to read response: {}", e),
        })?;
        if limited.len() > max_bytes {
            return Err(MediaFetchError {
                code: MediaFetchErrorCode::MaxBytes,
                message: format!("Payload {} exceeds maxBytes {}", limited.len(), max_bytes),
            }
            .into());
        }
        limited.to_vec()
    } else {
        res.bytes().await?.to_vec()
    };

    let file_name_from_url = final_url
        .path_segments()
        .and_then(|s| s.last().map(|p| p.to_string()));

    let header_file_name = parse_content_disposition_filename(content_disposition.as_deref());
    let file_path_hint_name = options
        .file_path_hint
        .as_ref()
        .and_then(|p| Path::new(p).file_name())
        .map(|n| n.to_string_lossy().to_string());

    let file_name = header_file_name
        .or(file_name_from_url)
        .or(file_path_hint_name);

    let content_type =
        super::mime::detect_mime(Some(&buffer), header_mime.as_deref(), file_name.as_deref()).await;

    Ok(FetchMediaResult {
        buffer,
        content_type,
        file_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_content_disposition_filename() {
        let result = parse_content_disposition_filename(Some("attachment; filename=\"image.png\""));
        assert_eq!(result, Some("image.png".to_string()));
    }
}
