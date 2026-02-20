//! Text chunking utilities for splitting large text into manageable chunks

/// Default maximum chunk size (characters)
pub const DEFAULT_CHUNK_SIZE: usize = 4000;

/// Default chunk overlap (characters)
pub const DEFAULT_CHUNK_OVERLAP: usize = 200;

/// Chunk text into pieces of approximately `chunk_size` with `overlap` between chunks
pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    if text.len() <= chunk_size {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();
    let mut start = 0;

    while start < text.len() {
        let end = (start + chunk_size).min(text.len());
        let chunk = &text[start..end];
        chunks.push(chunk.to_string());

        if end == text.len() {
            break;
        }

        // Move start forward, accounting for overlap
        start = end.saturating_sub(overlap);

        // Prevent infinite loop if overlap is 0 or too small
        if start >= end {
            start = end;
        }
    }

    chunks
}

/// Chunk text using default settings
pub fn chunk_text_default(text: &str) -> Vec<String> {
    chunk_text(text, DEFAULT_CHUNK_SIZE, DEFAULT_CHUNK_OVERLAP)
}

/// Chunk text by paragraphs, respecting max chunk size
pub fn chunk_by_paragraphs(text: &str, max_chunk_size: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    let paragraphs: Vec<&str> = text
        .split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .collect();

    if paragraphs.is_empty() {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for paragraph in paragraphs {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() {
            continue;
        }

        // If single paragraph exceeds max size, chunk it
        if trimmed.len() > max_chunk_size {
            if !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
            }

            let sub_chunks = chunk_text(trimmed, max_chunk_size, DEFAULT_CHUNK_OVERLAP);
            chunks.extend(sub_chunks);
            continue;
        }

        // Check if adding this paragraph would exceed limit
        let new_len = current_chunk.len() + trimmed.len() + 2; // +2 for "\n\n"
        if new_len > max_chunk_size && !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
            current_chunk.clear();
        }

        if !current_chunk.is_empty() {
            current_chunk.push_str("\n\n");
        }
        current_chunk.push_str(trimmed);
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    chunks
}

/// Chunk text by sentences, respecting max chunk size
pub fn chunk_by_sentences(text: &str, max_chunk_size: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    // Simple sentence splitting (not perfect but works for most cases)
    let sentence_enders = ['.', '!', '?'];
    let mut sentences = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        current.push(ch);
        if sentence_enders.contains(&ch) {
            let trimmed = current.trim();
            if !trimmed.is_empty() {
                sentences.push(trimmed.to_string());
            }
            current.clear();
        }
    }

    // Add remaining text
    if !current.trim().is_empty() {
        sentences.push(current.trim().to_string());
    }

    if sentences.is_empty() {
        return vec![text.to_string()];
    }

    // Combine sentences into chunks
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for sentence in sentences {
        // If single sentence exceeds max size, chunk it by words
        if sentence.len() > max_chunk_size {
            if !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
            }

            let sub_chunks = chunk_text(&sentence, max_chunk_size, DEFAULT_CHUNK_OVERLAP);
            chunks.extend(sub_chunks);
            continue;
        }

        // Check if adding this sentence would exceed limit
        let new_len = current_chunk.len() + sentence.len() + 1; // +1 for space
        if new_len > max_chunk_size && !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
            current_chunk.clear();
        }

        if !current_chunk.is_empty() {
            current_chunk.push(' ');
        }
        current_chunk.push_str(&sentence);
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    chunks
}

/// Estimate token count (rough approximation: 1 token ≈ 4 characters)
pub fn estimate_token_count(text: &str) -> usize {
    text.len().div_ceil(4)
}

/// Chunk text by token count
pub fn chunk_by_tokens(text: &str, max_tokens: usize) -> Vec<String> {
    let max_chars = max_tokens * 4;
    chunk_text(text, max_chars, DEFAULT_CHUNK_OVERLAP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_text_empty() {
        assert!(chunk_text("", 100, 10).is_empty());
    }

    #[test]
    fn test_chunk_text_small() {
        let text = "Hello world";
        let chunks = chunk_text(text, 100, 10);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_chunk_text_large() {
        let text = "a".repeat(1000);
        let chunks = chunk_text(&text, 100, 10);
        assert!(chunks.len() > 1);
        // Check overlap
        for i in 1..chunks.len() {
            let prev = &chunks[i - 1];
            let curr = &chunks[i];
            let overlap_len = prev.len() + curr.len() - text.len().min(prev.len() + curr.len());
            assert!(overlap_len <= 10 || curr.len() <= 10);
        }
    }

    #[test]
    fn test_chunk_by_paragraphs() {
        let text = "Para 1.\n\nPara 2.\n\nPara 3.";
        let chunks = chunk_by_paragraphs(text, 1000);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_estimate_token_count() {
        assert_eq!(estimate_token_count(""), 0);
        assert_eq!(estimate_token_count("abcd"), 1);
        assert_eq!(estimate_token_count("abcdefghijklmnop"), 4);
    }
}
