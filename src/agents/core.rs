use crate::agents::chat::{ChatMessage, ChatProvider};
use crate::agents::identity::AgentIdentity;
use crate::agents::tool::{Tool, ToolDefinition};
use crate::memory::MemoryManager;
use anyhow::Result;
use std::sync::Arc;

pub struct Agent {
    pub identity: AgentIdentity,
    pub provider: Box<dyn ChatProvider>,
    pub memory: Option<Arc<MemoryManager>>,
    pub tools: Vec<Box<dyn Tool>>,
}

impl std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("identity", &self.identity.name)
            .field("provider", &"...")
            .field("memory", &self.memory.is_some())
            .field("tools_count", &self.tools.len())
            .finish()
    }
}

impl Agent {
    pub fn new(
        identity: AgentIdentity,
        provider: Box<dyn ChatProvider>,
        memory: Option<Arc<MemoryManager>>,
        tools: Vec<Box<dyn Tool>>,
    ) -> Self {
        Self {
            identity,
            provider,
            memory,
            tools,
        }
    }

    pub async fn answer(&self, query: &str) -> Result<String> {
        let mut session = crate::sessions::Session::new("manual-session");
        session.append_transcript(crate::sessions::TranscriptEntry::user(query));
        self.answer_session(&mut session, None).await
    }

    pub async fn answer_session(
        &self,
        session: &mut crate::sessions::Session,
        stream_handler: Option<crate::agents::streaming::StreamHandler>,
    ) -> Result<String> {
        // Emit Start Hook
        let mut start_payload = crate::hooks::HookPayload::new();
        start_payload.set("session_id", session.id.clone());
        start_payload.set("agent_name", self.identity.name.clone());
        crate::hooks::emit(crate::hooks::events::AGENT_START, &start_payload);

        let res = self.do_answer_session(session, stream_handler).await;

        match &res {
            Ok(text) => {
                // Emit Complete Hook
                let mut complete_payload = crate::hooks::HookPayload::new();
                complete_payload.set("session_id", session.id.clone());
                complete_payload.set("agent_name", self.identity.name.clone());
                complete_payload.set("final_output", text.clone());
                crate::hooks::emit(crate::hooks::events::AGENT_COMPLETE, &complete_payload);
            }
            Err(e) => {
                // Emit Error Hook
                let mut error_payload = crate::hooks::HookPayload::new();
                error_payload.set("session_id", session.id.clone());
                error_payload.set("error", e.to_string());
                crate::hooks::emit(crate::hooks::events::AGENT_ERROR, &error_payload);
            }
        }

        res
    }

    async fn do_answer_session(
        &self,
        session: &mut crate::sessions::Session,
        stream_handler: Option<crate::agents::streaming::StreamHandler>,
    ) -> Result<String> {
        // 1. Convert transcript to ChatMessages
        let history: Vec<ChatMessage> = session
            .transcript
            .iter()
            .cloned()
            .map(Into::into)
            .collect();

        // 2. Apply Compaction to history
        let context_window = crate::agents::compaction::resolve_context_window_tokens(None);
        let compaction_msgs: Vec<crate::agents::compaction::CompactionMessage> =
            history.into_iter().map(Into::into).collect();

        let compacted =
            crate::agents::compaction::compact_transcript(&compaction_msgs, context_window, None);

        let mut messages: Vec<ChatMessage> = compacted.into_iter().map(Into::into).collect();

        // 3. Prepend System Prompt (if not already summarized in compaction)
        messages.insert(
            0,
            ChatMessage::System {
                content: self.identity.build_system_prompt(),
            },
        );

        // 4. RAG / Memory Context
        if let Some(ref memory) = self.memory {
            // Use the last user message as query for RAG
            let query = session
                .transcript
                .iter()
                .rev()
                .find(|e| e.role == "user")
                .map(|e| e.text.as_str())
                .unwrap_or("");

            if !query.is_empty() {
                let results = memory.search_hybrid(query, Default::default()).await?;
                if !results.is_empty() {
                    let mut context =
                        "Use the following context to help answer the user's question:\n\n"
                            .to_string();
                    for (i, res) in results.iter().take(5).enumerate() {
                        context.push_str(&format!(
                            "--- Document {} ({}) ---\n{}\n\n",
                            i + 1,
                            res.path,
                            res.text
                        ));
                    }
                    messages.push(ChatMessage::System { content: context });
                }
            }
        }

        // 5. Interaction Loop
        let tool_definitions: Vec<ToolDefinition> =
            self.tools.iter().map(|t| t.definition()).collect();

        loop {
            let response = if let Some(ref handler) = stream_handler {
                self.provider
                    .stream(messages.clone(), Some(&tool_definitions), handler.clone())
                    .await?
            } else {
                self.provider
                    .complete(messages.clone(), Some(&tool_definitions))
                    .await?
            };

            messages.push(response.message.clone());

            match response.message {
                ChatMessage::Assistant {
                    ref tool_calls,
                    ref content,
                } => {
                    if let Some(calls) = tool_calls {
                        for call in calls {
                            if let Some(ref handler) = stream_handler {
                                handler.start_tool_call(&call.id, &call.name)?;
                            }

                            let tool = self
                                .tools
                                .iter()
                                .find(|t| t.definition().name == call.name)
                                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", call.name))?;

                            let output = tool.call(&call.arguments).await?;

                            if let Some(ref handler) = stream_handler {
                                handler.tool_result(&call.id, &output, false)?;
                            }

                            messages.push(ChatMessage::Tool {
                                tool_call_id: call.id.clone(),
                                content: output,
                            });
                        }
                    } else {
                        // Final reply
                        let final_text = content.clone().unwrap_or_default();
                        session.append_transcript(crate::sessions::TranscriptEntry::assistant(
                            &final_text,
                        ));
                        return Ok(final_text);
                    }
                }
                _ => return Err(anyhow::anyhow!("Unexpected response message type")),
            }

            if response.finish_reason == "stop" {
                if let ChatMessage::Assistant { ref content, .. } = response.message {
                    let final_text = content.clone().unwrap_or_default();
                    session.append_transcript(crate::sessions::TranscriptEntry::assistant(
                        &final_text,
                    ));
                    session.last_active = chrono::Utc::now();
                    return Ok(final_text);
                }
            }
        }
    }
}
