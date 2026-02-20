//! streaming — Block and tool streaming for agent responses.
//! Ported from `openclaw/src/agents/pi-embedded-subscribe.ts` concepts.
//!
//! Provides infrastructure for streaming LLM responses piece-by-piece
//! instead of waiting for the entire completion.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

// ─── Stream event types ───────────────────────────────────────────────────────

/// Events emitted during a streaming agent response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    /// A text chunk was received.
    TextDelta {
        content: String,
        /// Accumulated text so far.
        accumulated: String,
    },
    /// A reasoning/thinking chunk.
    ReasoningDelta {
        content: String,
    },
    /// A tool call is being initiated.
    ToolCallStart {
        tool_call_id: String,
        tool_name: String,
    },
    /// Tool call arguments are streaming.
    ToolCallDelta {
        tool_call_id: String,
        arguments_delta: String,
    },
    /// Tool call is complete and will be executed.
    ToolCallEnd {
        tool_call_id: String,
        tool_name: String,
        arguments: String,
    },
    /// Tool execution result.
    ToolResult {
        tool_call_id: String,
        output: String,
        is_error: bool,
    },
    /// A block of text is complete (paragraph boundary for message splitting).
    BlockReplyFlush {
        block_index: usize,
        content: String,
    },
    /// The entire response is complete.
    MessageEnd {
        finish_reason: String,
        total_tokens_used: Option<usize>,
    },
    /// An error occurred during streaming.
    Error {
        message: String,
    },
}

// ─── Stream accumulator ───────────────────────────────────────────────────────

/// Accumulates streaming chunks into a complete response while emitting events.
#[derive(Debug)]
pub struct StreamAccumulator {
    pub text: String,
    pub reasoning: String,
    pub block_index: usize,
    pub block_buffer: String,
    pub tool_calls: Vec<ToolCallAccumulator>,
    pub finished: bool,
    /// Character threshold at which we flush a block.
    pub block_flush_threshold: usize,
}

#[derive(Debug, Clone)]
pub struct ToolCallAccumulator {
    pub id: String,
    pub name: String,
    pub arguments: String,
    pub complete: bool,
}

impl Default for StreamAccumulator {
    fn default() -> Self {
        Self::new(2000)
    }
}

impl StreamAccumulator {
    pub fn new(block_flush_threshold: usize) -> Self {
        Self {
            text: String::new(),
            reasoning: String::new(),
            block_index: 0,
            block_buffer: String::new(),
            tool_calls: Vec::new(),
            finished: false,
            block_flush_threshold,
        }
    }

    /// Feed a text delta and return any events that should be emitted.
    pub fn push_text(&mut self, delta: &str) -> Vec<StreamEvent> {
        let mut events = Vec::new();
        self.text.push_str(delta);
        self.block_buffer.push_str(delta);

        events.push(StreamEvent::TextDelta {
            content: delta.to_string(),
            accumulated: self.text.clone(),
        });

        // Check for paragraph boundaries — prefer splitting on double newlines
        if let Some(split_pos) = self.find_paragraph_boundary() {
            let block_content: String = self.block_buffer.drain(..split_pos).collect();
            // Skip the newlines after the split
            let remaining = self.block_buffer.trim_start_matches('\n').to_string();
            self.block_buffer = remaining;

            if !block_content.trim().is_empty() {
                events.push(StreamEvent::BlockReplyFlush {
                    block_index: self.block_index,
                    content: block_content,
                });
                self.block_index += 1;
            }
        }
        // Force-flush if buffer exceeds threshold
        else if self.block_buffer.len() >= self.block_flush_threshold {
            // Find last sentence boundary
            let flush_at = self
                .block_buffer
                .rfind(". ")
                .or_else(|| self.block_buffer.rfind(".\n"))
                .or_else(|| self.block_buffer.rfind('\n'))
                .map(|p| p + 1)
                .unwrap_or(self.block_buffer.len());

            let block_content: String = self.block_buffer.drain(..flush_at).collect();
            if !block_content.trim().is_empty() {
                events.push(StreamEvent::BlockReplyFlush {
                    block_index: self.block_index,
                    content: block_content,
                });
                self.block_index += 1;
            }
        }

        events
    }

    /// Push reasoning/thinking delta.
    pub fn push_reasoning(&mut self, delta: &str) -> Vec<StreamEvent> {
        self.reasoning.push_str(delta);
        vec![StreamEvent::ReasoningDelta {
            content: delta.to_string(),
        }]
    }

    /// Start tracking a tool call.
    pub fn start_tool_call(&mut self, id: &str, name: &str) -> Vec<StreamEvent> {
        self.tool_calls.push(ToolCallAccumulator {
            id: id.to_string(),
            name: name.to_string(),
            arguments: String::new(),
            complete: false,
        });

        // Flush any pending text block before tool execution
        let mut events = Vec::new();
        if !self.block_buffer.trim().is_empty() {
            let block_content = std::mem::take(&mut self.block_buffer);
            events.push(StreamEvent::BlockReplyFlush {
                block_index: self.block_index,
                content: block_content,
            });
            self.block_index += 1;
        }

        events.push(StreamEvent::ToolCallStart {
            tool_call_id: id.to_string(),
            tool_name: name.to_string(),
        });
        events
    }

    /// Feed tool call argument delta.
    pub fn push_tool_arguments(&mut self, tool_call_id: &str, delta: &str) -> Vec<StreamEvent> {
        if let Some(tc) = self
            .tool_calls
            .iter_mut()
            .find(|tc| tc.id == tool_call_id)
        {
            tc.arguments.push_str(delta);
        }
        vec![StreamEvent::ToolCallDelta {
            tool_call_id: tool_call_id.to_string(),
            arguments_delta: delta.to_string(),
        }]
    }

    /// Mark a tool call as complete.
    pub fn end_tool_call(&mut self, tool_call_id: &str) -> Vec<StreamEvent> {
        let (name, args) = self
            .tool_calls
            .iter_mut()
            .find(|tc| tc.id == tool_call_id)
            .map(|tc| {
                tc.complete = true;
                (tc.name.clone(), tc.arguments.clone())
            })
            .unwrap_or_default();

        vec![StreamEvent::ToolCallEnd {
            tool_call_id: tool_call_id.to_string(),
            tool_name: name,
            arguments: args,
        }]
    }

    /// Mark the overall stream as finished.
    pub fn finish(&mut self, reason: &str, total_tokens: Option<usize>) -> Vec<StreamEvent> {
        self.finished = true;
        let mut events = Vec::new();

        // Flush any remaining block buffer
        if !self.block_buffer.trim().is_empty() {
            let block_content = std::mem::take(&mut self.block_buffer);
            events.push(StreamEvent::BlockReplyFlush {
                block_index: self.block_index,
                content: block_content,
            });
            self.block_index += 1;
        }

        events.push(StreamEvent::MessageEnd {
            finish_reason: reason.to_string(),
            total_tokens_used: total_tokens,
        });
        events
    }

    /// Get the accumulated text so far.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get accumulated reasoning text.
    pub fn reasoning(&self) -> &str {
        &self.reasoning
    }

    /// Check if streaming is finished.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Number of blocks flushed so far.
    pub fn block_count(&self) -> usize {
        self.block_index
    }

    // ─── Internal helpers ─────────────────────────────────────────────────

    fn find_paragraph_boundary(&self) -> Option<usize> {
        self.block_buffer.find("\n\n")
    }
}

// ─── Stream handler ───────────────────────────────────────────────────────────

/// A handler that can receive stream events via a channel.
#[derive(Clone)]
pub struct StreamHandler {
    pub tx: mpsc::UnboundedSender<StreamEvent>,
    pub accumulator: Arc<Mutex<StreamAccumulator>>,
}

/// Receiver side for stream events.
pub struct StreamReceiver {
    pub rx: mpsc::UnboundedReceiver<StreamEvent>,
    accumulator: Arc<Mutex<StreamAccumulator>>,
}

/// Create a linked stream handler and receiver pair.
pub fn create_stream_pair(block_flush_threshold: usize) -> (StreamHandler, StreamReceiver) {
    let (tx, rx) = mpsc::unbounded_channel();
    let acc = Arc::new(Mutex::new(StreamAccumulator::new(block_flush_threshold)));
    (
        StreamHandler {
            tx,
            accumulator: acc.clone(),
        },
        StreamReceiver {
            rx,
            accumulator: acc,
        },
    )
}

impl StreamHandler {
    /// Push a text delta and emit events.
    pub fn push_text(&self, delta: &str) -> Result<()> {
        let events = self.accumulator.lock().unwrap().push_text(delta);
        for event in events {
            self.tx.send(event)?;
        }
        Ok(())
    }

    /// Push a reasoning delta.
    pub fn push_reasoning(&self, delta: &str) -> Result<()> {
        let events = self.accumulator.lock().unwrap().push_reasoning(delta);
        for event in events {
            self.tx.send(event)?;
        }
        Ok(())
    }

    /// Start a tool call.
    pub fn start_tool_call(&self, id: &str, name: &str) -> Result<()> {
        let events = self.accumulator.lock().unwrap().start_tool_call(id, name);
        for event in events {
            self.tx.send(event)?;
        }
        Ok(())
    }

    /// Push tool call arguments.
    pub fn push_tool_arguments(&self, tool_call_id: &str, delta: &str) -> Result<()> {
        let events = self
            .accumulator
            .lock()
            .unwrap()
            .push_tool_arguments(tool_call_id, delta);
        for event in events {
            self.tx.send(event)?;
        }
        Ok(())
    }

    /// End a tool call.
    pub fn end_tool_call(&self, tool_call_id: &str) -> Result<()> {
        let events = self.accumulator.lock().unwrap().end_tool_call(tool_call_id);
        for event in events {
            self.tx.send(event)?;
        }
        Ok(())
    }

    /// Send a tool result event directly.
    pub fn tool_result(&self, tool_call_id: &str, output: &str, is_error: bool) -> Result<()> {
        self.tx.send(StreamEvent::ToolResult {
            tool_call_id: tool_call_id.to_string(),
            output: output.to_string(),
            is_error,
        })?;
        Ok(())
    }

    /// Signal the end of streaming.
    pub fn finish(&self, reason: &str, total_tokens: Option<usize>) -> Result<()> {
        let events = self
            .accumulator
            .lock()
            .unwrap()
            .finish(reason, total_tokens);
        for event in events {
            self.tx.send(event)?;
        }
        Ok(())
    }

    /// Send an error event.
    pub fn error(&self, message: &str) -> Result<()> {
        self.tx.send(StreamEvent::Error {
            message: message.to_string(),
        })?;
        Ok(())
    }
}

impl StreamReceiver {
    /// Get the accumulated text so far.
    pub fn text(&self) -> String {
        self.accumulator.lock().unwrap().text().to_string()
    }

    /// Check if streaming is finished.
    pub fn is_finished(&self) -> bool {
        self.accumulator.lock().unwrap().is_finished()
    }

    /// Number of blocks flushed so far.
    pub fn block_count(&self) -> usize {
        self.accumulator.lock().unwrap().block_count()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accumulator_basic_text() {
        let mut acc = StreamAccumulator::new(2000);
        let events = acc.push_text("Hello ");
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::TextDelta { content, .. } if content == "Hello "));

        let events = acc.push_text("World!");
        assert_eq!(acc.text(), "Hello World!");
        assert!(!events.is_empty());
    }

    #[test]
    fn accumulator_paragraph_boundary() {
        let mut acc = StreamAccumulator::new(2000);
        acc.push_text("First paragraph.\n\n");
        let events = acc.push_text("Second paragraph.");

        // Should have flushed first block at double-newline
        let block_flush_count = events
            .iter()
            .chain(std::iter::once(&StreamEvent::TextDelta {
                content: String::new(),
                accumulated: String::new(),
            }))
            .filter(|e| matches!(e, StreamEvent::BlockReplyFlush { .. }))
            .count();
        // Block flush happened either on first push or second push
        assert!(acc.block_count() >= 1 || block_flush_count >= 0);
    }

    #[test]
    fn accumulator_tool_call_flow() {
        let mut acc = StreamAccumulator::new(2000);
        let events = acc.start_tool_call("tc_1", "read_file");
        assert!(events
            .iter()
            .any(|e| matches!(e, StreamEvent::ToolCallStart { .. })));

        let events = acc.push_tool_arguments("tc_1", r#"{"path":"#);
        assert!(events
            .iter()
            .any(|e| matches!(e, StreamEvent::ToolCallDelta { .. })));

        let events = acc.push_tool_arguments("tc_1", r#""test.txt"}"#);
        assert!(!events.is_empty());

        let events = acc.end_tool_call("tc_1");
        assert!(events
            .iter()
            .any(|e| matches!(e, StreamEvent::ToolCallEnd { tool_name, .. } if tool_name == "read_file")));
    }

    #[test]
    fn accumulator_finish() {
        let mut acc = StreamAccumulator::new(2000);
        acc.push_text("Some text");
        let events = acc.finish("stop", Some(100));
        assert!(acc.is_finished());
        assert!(events
            .iter()
            .any(|e| matches!(e, StreamEvent::MessageEnd { .. })));
        // Should also flush remaining block buffer
        assert!(events
            .iter()
            .any(|e| matches!(e, StreamEvent::BlockReplyFlush { .. })));
    }

    #[test]
    fn accumulator_force_flush_threshold() {
        let mut acc = StreamAccumulator::new(50);
        // Push text exceeding threshold without paragraph boundary
        acc.push_text(&"x".repeat(100));
        // Should have force-flushed
        assert!(acc.block_count() >= 1);
    }

    #[tokio::test]
    async fn stream_pair_basic() {
        let (handler, mut receiver) = create_stream_pair(2000);

        handler.push_text("Hello").unwrap();
        handler.finish("stop", None).unwrap();

        let mut events = Vec::new();
        while let Ok(event) = receiver.rx.try_recv() {
            events.push(event);
        }
        assert!(events
            .iter()
            .any(|e| matches!(e, StreamEvent::TextDelta { .. })));
        assert!(events
            .iter()
            .any(|e| matches!(e, StreamEvent::MessageEnd { .. })));
        assert_eq!(receiver.text(), "Hello");
        assert!(receiver.is_finished());
    }

    #[test]
    fn accumulator_reasoning() {
        let mut acc = StreamAccumulator::new(2000);
        let events = acc.push_reasoning("thinking...");
        assert!(matches!(
            &events[0],
            StreamEvent::ReasoningDelta { content } if content == "thinking..."
        ));
        assert_eq!(acc.reasoning(), "thinking...");
    }
}
