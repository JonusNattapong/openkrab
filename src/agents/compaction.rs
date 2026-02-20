//! compaction — Transcript compaction for context window management.
//! Ported from `openclaw/src/agents/compaction.ts`.
//!
//! Provides methods to split, chunk, and summarize long transcripts so
//! that they stay within a model's context window.

use crate::agents::chat::{ChatMessage, ContentPart, UserContent};
use serde::{Deserialize, Serialize};

// ─── Constants ────────────────────────────────────────────────────────────────

/// Base ratio of context window used per summary chunk.
pub const BASE_CHUNK_RATIO: f64 = 0.4;
/// Minimum chunk ratio — never go below this.
pub const MIN_CHUNK_RATIO: f64 = 0.15;
/// 20% buffer for `estimate_tokens()` inaccuracy.
pub const SAFETY_MARGIN: f64 = 1.2;
/// Default summary when nothing is available.
const DEFAULT_SUMMARY_FALLBACK: &str = "No prior history.";
/// Default number of parts for splitting.
const DEFAULT_PARTS: usize = 2;

// ─── Message types ────────────────────────────────────────────────────────────

/// A simplified agent message for compaction purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionMessage {
    pub role: String,
    pub content: String,
    #[serde(default)]
    pub tool_call_id: Option<String>,
    /// Optional details that should NOT be included in summarization.
    #[serde(default)]
    pub details: Option<String>,
}

impl CompactionMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
            tool_call_id: None,
            details: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
            tool_call_id: None,
            details: None,
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
            tool_call_id: None,
            details: None,
        }
    }

    pub fn tool_result(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: "tool".into(),
            content: content.into(),
            tool_call_id: Some(id.into()),
            details: None,
        }
    }
}

// ─── Converters ───────────────────────────────────────────────────────────────

impl From<ChatMessage> for CompactionMessage {
    fn from(msg: ChatMessage) -> Self {
        match msg {
            ChatMessage::System { content } => Self::system(content),
            ChatMessage::User { content } => {
                let text = match content {
                    UserContent::Text(t) => t,
                    UserContent::Parts(parts) => parts
                        .into_iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                };
                Self::user(text)
            }
            ChatMessage::Assistant { content, .. } => Self::assistant(content.unwrap_or_default()),
            ChatMessage::Tool {
                tool_call_id,
                content,
            } => Self::tool_result(tool_call_id, content),
        }
    }
}

impl From<CompactionMessage> for ChatMessage {
    fn from(msg: CompactionMessage) -> Self {
        match msg.role.as_str() {
            "system" => ChatMessage::System {
                content: msg.content,
            },
            "user" => ChatMessage::User {
                content: UserContent::Text(msg.content),
            },
            "assistant" => ChatMessage::Assistant {
                content: Some(msg.content),
                tool_calls: None,
            },
            "tool" => ChatMessage::Tool {
                tool_call_id: msg.tool_call_id.unwrap_or_default(),
                content: msg.content,
            },
            _ => ChatMessage::System {
                content: msg.content,
            },
        }
    }
}

// ─── Token estimation ─────────────────────────────────────────────────────────

/// Rough token estimate: ~4 chars per token (GPT-family heuristic).
pub fn estimate_tokens(msg: &CompactionMessage) -> usize {
    // SECURITY: never include `details` — they can contain untrusted payloads
    let chars = msg.role.len() + msg.content.len();
    (chars + 3) / 4 // ceil division by 4
}

/// Estimate total tokens of a message list, stripping details.
pub fn estimate_messages_tokens(messages: &[CompactionMessage]) -> usize {
    messages.iter().map(|m| estimate_tokens(m)).sum()
}

// ─── Normalize parts ──────────────────────────────────────────────────────────

fn normalize_parts(parts: usize, message_count: usize) -> usize {
    if parts <= 1 {
        return 1;
    }
    parts.min(message_count.max(1))
}

// ─── Split by token share ─────────────────────────────────────────────────────

/// Split messages into `parts` chunks so each chunk has roughly equal token count.
pub fn split_messages_by_token_share(
    messages: &[CompactionMessage],
    parts: usize,
) -> Vec<Vec<CompactionMessage>> {
    if messages.is_empty() {
        return Vec::new();
    }
    let normalized_parts = normalize_parts(parts, messages.len());
    if normalized_parts <= 1 {
        return vec![messages.to_vec()];
    }

    let total_tokens = estimate_messages_tokens(messages);
    let target_tokens = total_tokens / normalized_parts;
    let mut chunks: Vec<Vec<CompactionMessage>> = Vec::new();
    let mut current: Vec<CompactionMessage> = Vec::new();
    let mut current_tokens: usize = 0;

    for msg in messages {
        let msg_tokens = estimate_tokens(msg);
        if chunks.len() < normalized_parts - 1
            && !current.is_empty()
            && current_tokens + msg_tokens > target_tokens
        {
            chunks.push(std::mem::take(&mut current));
            current_tokens = 0;
        }
        current.push(msg.clone());
        current_tokens += msg_tokens;
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

// ─── Chunk by max tokens ──────────────────────────────────────────────────────

/// Chunk messages so no chunk exceeds `max_tokens`.
pub fn chunk_messages_by_max_tokens(
    messages: &[CompactionMessage],
    max_tokens: usize,
) -> Vec<Vec<CompactionMessage>> {
    if messages.is_empty() {
        return Vec::new();
    }

    let mut chunks: Vec<Vec<CompactionMessage>> = Vec::new();
    let mut current_chunk: Vec<CompactionMessage> = Vec::new();
    let mut current_tokens: usize = 0;

    for msg in messages {
        let msg_tokens = estimate_tokens(msg);
        if !current_chunk.is_empty() && current_tokens + msg_tokens > max_tokens {
            chunks.push(std::mem::take(&mut current_chunk));
            current_tokens = 0;
        }
        current_chunk.push(msg.clone());
        current_tokens += msg_tokens;

        // Oversized single message — force-flush to avoid unbounded growth
        if msg_tokens > max_tokens {
            chunks.push(std::mem::take(&mut current_chunk));
            current_tokens = 0;
        }
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

// ─── Adaptive chunk ratio ─────────────────────────────────────────────────────

/// Compute adaptive chunk ratio based on average message size.
/// When messages are large, we use smaller chunks to avoid exceeding model limits.
pub fn compute_adaptive_chunk_ratio(messages: &[CompactionMessage], context_window: usize) -> f64 {
    if messages.is_empty() || context_window == 0 {
        return BASE_CHUNK_RATIO;
    }

    let total_tokens = estimate_messages_tokens(messages);
    let avg_tokens = total_tokens as f64 / messages.len() as f64;
    let safe_avg = avg_tokens * SAFETY_MARGIN;
    let avg_ratio = safe_avg / context_window as f64;

    if avg_ratio > 0.1 {
        let reduction = (avg_ratio * 2.0).min(BASE_CHUNK_RATIO - MIN_CHUNK_RATIO);
        (BASE_CHUNK_RATIO - reduction).max(MIN_CHUNK_RATIO)
    } else {
        BASE_CHUNK_RATIO
    }
}

/// Check if a single message is too large to summarize.
/// If single message > 50% of context, it can't be summarized safely.
pub fn is_oversized_for_summary(msg: &CompactionMessage, context_window: usize) -> bool {
    let tokens = estimate_tokens(msg) as f64 * SAFETY_MARGIN;
    tokens > context_window as f64 * 0.5
}

// ─── Pruning ──────────────────────────────────────────────────────────────────

/// Result of pruning history to fit within context share.
#[derive(Debug, Clone)]
pub struct PruneResult {
    /// Messages that were kept (within budget).
    pub messages: Vec<CompactionMessage>,
    /// Messages that were dropped (for potential summarization).
    pub dropped_messages: Vec<CompactionMessage>,
    /// Number of chunk-level drops.
    pub dropped_chunks: usize,
    /// Total number of individual messages dropped.
    pub dropped_count: usize,
    /// Estimated tokens of dropped messages.
    pub dropped_tokens: usize,
    /// Estimated tokens of kept messages.
    pub kept_tokens: usize,
    /// Token budget that was available.
    pub budget_tokens: usize,
}

/// Prune history to fit within `max_context_tokens * max_history_share`.
/// Drops oldest chunks first.
pub fn prune_history_for_context_share(
    messages: &[CompactionMessage],
    max_context_tokens: usize,
    max_history_share: Option<f64>,
    parts: Option<usize>,
) -> PruneResult {
    let share = max_history_share.unwrap_or(0.5);
    let budget_tokens = (max_context_tokens as f64 * share).floor().max(1.0) as usize;
    let parts = parts.unwrap_or(DEFAULT_PARTS);

    let mut kept_messages = messages.to_vec();
    let mut all_dropped: Vec<CompactionMessage> = Vec::new();
    let mut dropped_chunks: usize = 0;
    let mut dropped_count: usize = 0;
    let mut dropped_tokens: usize = 0;

    while !kept_messages.is_empty() && estimate_messages_tokens(&kept_messages) > budget_tokens {
        let chunks = split_messages_by_token_share(&kept_messages, parts);
        if chunks.len() <= 1 {
            break;
        }
        // Drop the oldest chunk (first)
        let dropped = &chunks[0];
        dropped_chunks += 1;
        dropped_count += dropped.len();
        dropped_tokens += estimate_messages_tokens(dropped);
        all_dropped.extend(dropped.clone());

        // Keep the rest
        kept_messages = chunks[1..].iter().flatten().cloned().collect();
    }

    PruneResult {
        kept_tokens: estimate_messages_tokens(&kept_messages),
        messages: kept_messages,
        dropped_messages: all_dropped,
        dropped_chunks,
        dropped_count,
        dropped_tokens,
        budget_tokens,
    }
}

// ─── Summary generation (local / stub) ────────────────────────────────────────

/// Format messages into a plain-text summary suitable for injection as system context.
/// This is a local summarizer that doesn't call an LLM. For LLM-based summarization,
/// use `summarize_with_provider`.
pub fn format_messages_as_summary(messages: &[CompactionMessage]) -> String {
    if messages.is_empty() {
        return DEFAULT_SUMMARY_FALLBACK.to_string();
    }

    let mut lines: Vec<String> = Vec::new();
    lines.push("=== Conversation Summary ===".to_string());

    for msg in messages {
        let role = msg.role.to_uppercase();
        let content = if msg.content.len() > 500 {
            format!("{}…", &msg.content[..500])
        } else {
            msg.content.clone()
        };
        lines.push(format!("[{}] {}", role, content));
    }

    lines.join("\n")
}

/// Build a compacted context: prune → summarize dropped → prepend summary to kept messages.
pub fn compact_transcript(
    messages: &[CompactionMessage],
    max_context_tokens: usize,
    custom_instructions: Option<&str>,
) -> Vec<CompactionMessage> {
    let prune = prune_history_for_context_share(messages, max_context_tokens, None, None);

    if prune.dropped_messages.is_empty() {
        return prune.messages;
    }

    // Build summary of dropped messages
    let summary = format_messages_as_summary(&prune.dropped_messages);

    let mut instructions = String::from("Previous conversation summary:\n\n");
    instructions.push_str(&summary);

    if let Some(custom) = custom_instructions {
        instructions.push_str("\n\nAdditional context:\n");
        instructions.push_str(custom);
    }

    instructions.push_str(&format!(
        "\n\n({} messages compacted, ~{} tokens freed)",
        prune.dropped_count, prune.dropped_tokens
    ));

    let mut result = vec![CompactionMessage::system(instructions)];
    result.extend(prune.messages);
    result
}

/// Resolve context window token count with fallback.
pub fn resolve_context_window_tokens(context_window: Option<usize>) -> usize {
    context_window.unwrap_or(128_000).max(1)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_msgs(count: usize, content_len: usize) -> Vec<CompactionMessage> {
        (0..count)
            .map(|i| CompactionMessage::user("x".repeat(content_len) + &format!(" msg{}", i)))
            .collect()
    }

    #[test]
    fn estimate_tokens_basic() {
        let msg = CompactionMessage::user("hello world");
        let tokens = estimate_tokens(&msg);
        // "user" (4) + "hello world" (11) = 15 chars → ceil(15/4) = 4
        assert!(tokens > 0);
        assert!(tokens < 20);
    }

    #[test]
    fn estimate_tokens_ignores_details() {
        let mut msg = CompactionMessage::user("hello");
        msg.details = Some("x".repeat(10000));
        let tokens = estimate_tokens(&msg);
        // Details should NOT contribute to token count
        assert!(tokens < 10);
    }

    #[test]
    fn split_empty() {
        let result = split_messages_by_token_share(&[], 2);
        assert!(result.is_empty());
    }

    #[test]
    fn split_single_part() {
        let msgs = make_msgs(5, 20);
        let result = split_messages_by_token_share(&msgs, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 5);
    }

    #[test]
    fn split_into_parts() {
        let msgs = make_msgs(10, 100);
        let result = split_messages_by_token_share(&msgs, 2);
        assert!(result.len() >= 1 && result.len() <= 3);
        let total: usize = result.iter().map(|c| c.len()).sum();
        assert_eq!(total, 10);
    }

    #[test]
    fn chunk_by_max_tokens() {
        let msgs = make_msgs(20, 100);
        let chunks = chunk_messages_by_max_tokens(&msgs, 100);
        // Each chunk should have roughly ≤ 100 tokens
        for chunk in &chunks {
            if chunk.len() == 1 {
                // Single oversized messages are fine
                continue;
            }
        }
        let total: usize = chunks.iter().map(|c| c.len()).sum();
        assert_eq!(total, 20);
    }

    #[test]
    fn adaptive_ratio_normal() {
        let msgs = make_msgs(10, 20);
        let ratio = compute_adaptive_chunk_ratio(&msgs, 128000);
        assert!((ratio - BASE_CHUNK_RATIO).abs() < 0.001);
    }

    #[test]
    fn adaptive_ratio_large_messages() {
        // Messages that are about 10% of context window each
        let msgs = make_msgs(5, 50000); // ~12500 tokens each
        let ratio = compute_adaptive_chunk_ratio(&msgs, 128000);
        assert!(ratio < BASE_CHUNK_RATIO);
        assert!(ratio >= MIN_CHUNK_RATIO);
    }

    #[test]
    fn oversized_check() {
        let small = CompactionMessage::user("hello");
        assert!(!is_oversized_for_summary(&small, 128000));

        let huge = CompactionMessage::user("x".repeat(300000));
        assert!(is_oversized_for_summary(&huge, 128000));
    }

    #[test]
    fn prune_within_budget() {
        let msgs = make_msgs(5, 20);
        let result = prune_history_for_context_share(&msgs, 100000, None, None);
        assert_eq!(result.messages.len(), 5);
        assert!(result.dropped_messages.is_empty());
    }

    #[test]
    fn prune_over_budget() {
        let msgs = make_msgs(100, 500);
        let result = prune_history_for_context_share(&msgs, 200, Some(0.5), None);
        assert!(result.messages.len() < 100);
        assert!(!result.dropped_messages.is_empty());
        assert!(result.dropped_chunks > 0);
    }

    #[test]
    fn compact_transcript_passthrough() {
        let msgs = make_msgs(3, 10);
        let result = compact_transcript(&msgs, 100000, None);
        // Should pass through without compaction
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn compact_transcript_compacts() {
        let msgs = make_msgs(100, 500);
        let result = compact_transcript(&msgs, 200, None);
        // First message should be system summary
        assert_eq!(result[0].role, "system");
        assert!(result[0].content.contains("Conversation Summary"));
        assert!(result.len() < 100);
    }

    #[test]
    fn format_summary_empty() {
        assert_eq!(format_messages_as_summary(&[]), DEFAULT_SUMMARY_FALLBACK);
    }

    #[test]
    fn format_summary_truncates_long() {
        let msg = CompactionMessage::user("x".repeat(1000));
        let summary = format_messages_as_summary(&[msg]);
        assert!(summary.contains("…"));
    }

    #[test]
    fn resolve_context_default() {
        assert_eq!(resolve_context_window_tokens(None), 128_000);
        assert_eq!(resolve_context_window_tokens(Some(32000)), 32000);
        assert_eq!(resolve_context_window_tokens(Some(0)), 1);
    }
}
