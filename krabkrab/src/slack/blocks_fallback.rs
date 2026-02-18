use serde_json::Value;

fn clean_candidate(value: Option<&str>) -> Option<String> {
    value.map(|v| v.replace(|c: char| c.is_whitespace(), " ").trim().to_string()).and_then(|s| if s.is_empty() { None } else { Some(s) })
}

fn read_section_text(block: &Value) -> Option<String> {
    block.get("text").and_then(|t| t.get("text")).and_then(|v| v.as_str()).and_then(|s| clean_candidate(Some(s)))
}

fn read_header_text(block: &Value) -> Option<String> { read_section_text(block) }

fn read_image_text(block: &Value) -> Option<String> {
    clean_candidate(block.get("alt_text").and_then(|v| v.as_str()))
        .or_else(|| block.get("title").and_then(|t| t.get("text")).and_then(|v| v.as_str()).and_then(|s| clean_candidate(Some(s))))
}

fn read_video_text(block: &Value) -> Option<String> {
    block.get("title").and_then(|t| t.get("text")).and_then(|v| v.as_str()).and_then(|s| clean_candidate(Some(s)))
        .or_else(|| clean_candidate(block.get("alt_text").and_then(|v| v.as_str())))
}

fn read_context_text(block: &Value) -> Option<String> {
    block.get("elements").and_then(|e| e.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|el| el.get("text").and_then(|v| v.as_str()).and_then(|s| clean_candidate(Some(s))))
            .collect::<Vec<_>>()
            .join(" ")
    }).and_then(|s| if s.is_empty() { None } else { Some(s) })
}

pub fn build_slack_blocks_fallback_text(blocks: &[Value]) -> String {
    for block in blocks {
        if let Some(t) = block.get("type").and_then(|v| v.as_str()) {
            match t {
                "header" => {
                    if let Some(text) = read_header_text(block) { return text; }
                }
                "section" => {
                    if let Some(text) = read_section_text(block) { return text; }
                }
                "image" => {
                    if let Some(text) = read_image_text(block) { return text; }
                    return "Shared an image".to_string();
                }
                "video" => {
                    if let Some(text) = read_video_text(block) { return text; }
                    return "Shared a video".to_string();
                }
                "file" => return "Shared a file".to_string(),
                "context" => {
                    if let Some(text) = read_context_text(block) { return text; }
                }
                _ => {}
            }
        }
    }
    "Shared a Block Kit message".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn fallback_from_section() {
        let blocks = vec![json!({"type":"section","text":{"type":"mrkdwn","text":"Hello"}})];
        assert_eq!(build_slack_blocks_fallback_text(&blocks), "Hello".to_string());
    }

    #[test]
    fn fallback_image_default() {
        let blocks = vec![json!({"type":"image"})];
        assert_eq!(build_slack_blocks_fallback_text(&blocks), "Shared an image".to_string());
    }
}
