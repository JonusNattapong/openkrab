use crate::agents::chat::ChatMessage;
use crate::agents::tool::ToolCall;
use std::collections::HashSet;

pub struct ToolUseRepairReport {
    pub messages: Vec<ChatMessage>,
    pub added_count: usize,
    pub dropped_duplicate_count: usize,
    pub dropped_orphan_count: usize,
    pub moved: bool,
}

/// Anthropic (and Cloud Code Assist) reject transcripts where assistant tool calls are not
/// immediately followed by matching tool results. Session files can end up with results
/// displaced (e.g. after user turns) or duplicated. Repair by:
/// - moving matching toolResult messages directly after their assistant toolCall turn
/// - inserting synthetic error toolResults for missing ids
/// - dropping duplicate toolResults for the same id (anywhere in the transcript)
pub fn repair_tool_use_result_pairing(messages: &[ChatMessage]) -> ToolUseRepairReport {
    let mut out: Vec<ChatMessage> = Vec::new();
    let mut seen_tool_result_ids = HashSet::new();
    let mut added_count = 0;
    let mut dropped_duplicate_count = 0;
    let mut dropped_orphan_count = 0;
    let mut moved = false;
    let mut changed = false;

    let mut i = 0;
    while i < messages.len() {
        let msg = &messages[i];

        match msg {
            ChatMessage::Assistant { tool_calls, content } => {
                // If there are tool calls, we need to find their results
                if let Some(calls) = tool_calls {
                    if calls.is_empty() {
                        out.push(msg.clone());
                        i += 1;
                        continue;
                    }

                    let tool_call_ids: HashSet<String> = calls.iter().map(|c| c.id.clone()).collect();
                    let mut span_results: Vec<ChatMessage> = Vec::new();
                    let mut remainder: Vec<ChatMessage> = Vec::new();

                    let mut j = i + 1;
                    while j < messages.len() {
                        let next = &messages[j];
                        match next {
                            ChatMessage::Assistant { .. } => {
                                break;
                            }
                            ChatMessage::Tool { tool_call_id, .. } => {
                                if tool_call_ids.contains(tool_call_id) {
                                    if seen_tool_result_ids.contains(tool_call_id) {
                                        dropped_duplicate_count += 1;
                                        changed = true;
                                    } else {
                                        seen_tool_result_ids.insert(tool_call_id.clone());
                                        span_results.push(next.clone());
                                    }
                                } else {
                                    dropped_orphan_count += 1;
                                    changed = true;
                                }
                            }
                            _ => {
                                remainder.push(next.clone());
                            }
                        }
                        j += 1;
                    }

                    // Push assistant message
                    out.push(msg.clone());

                    if !span_results.is_empty() && !remainder.is_empty() {
                        moved = true;
                        changed = true;
                    }

                    // Append results for this assistant turn immediately
                    for call in calls {
                        if let Some(pos) = span_results.iter().position(|r| {
                            if let ChatMessage::Tool { tool_call_id, .. } = r {
                                tool_call_id == &call.id
                            } else {
                                false
                            }
                        }) {
                            out.push(span_results.remove(pos));
                        } else {
                            // Missing result - insert synthetic
                            added_count += 1;
                            changed = true;
                            let synthetic = ChatMessage::Tool {
                                tool_call_id: call.id.clone(),
                                content: format!(
                                    "[openkrab] missing tool result for {}; inserted synthetic error.",
                                    call.name
                                ),
                            };
                            seen_tool_result_ids.insert(call.id.clone());
                            out.push(synthetic);
                        }
                    }

                    // Re-append other messages (non-tool results)
                    for rem in remainder {
                        out.push(rem);
                    }

                    i = j;
                } else {
                    out.push(msg.clone());
                    i += 1;
                }
            }
            ChatMessage::Tool { tool_call_id, .. } => {
                // Orphaned tool result found outside an assistant turn context
                dropped_orphan_count += 1;
                changed = true;
                i += 1;
            }
            _ => {
                out.push(msg.clone());
                i += 1;
            }
        }
    }

    ToolUseRepairReport {
        messages: if changed || moved { out } else { messages.to_vec() },
        added_count,
        dropped_duplicate_count,
        dropped_orphan_count,
        moved: changed || moved,
    }
}

pub fn strip_tool_result_details(messages: &[ChatMessage]) -> Vec<ChatMessage> {
    // In OpenClaw, this strips the 'details' field from toolResult messages.
    // In OpenKrab, ChatMessage::Tool doesn't have a 'details' field currently,
    // but we can prepare it for future parity if we add it.
    messages.to_vec()
}
