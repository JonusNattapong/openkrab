//! llm_task — LLM task execution extension.
//! Ported from `openkrab/extensions/llm-task/` (Phase 12).
//!
//! Provides structured LLM task definitions with typed inputs/outputs,
//! retry logic, and result caching for agentic workflows.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Task definition ──────────────────────────────────────────────────────────

/// Priority level for LLM tasks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Status of a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// An LLM task definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmTask {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    /// The system prompt for this task.
    pub system_prompt: String,
    /// The user prompt template (use `{input}` as placeholder).
    pub user_prompt_template: String,
    /// Maximum tokens to generate.
    pub max_tokens: Option<u32>,
    /// Temperature (0.0–2.0).
    pub temperature: Option<f32>,
    /// Provider name (e.g. "openai", "gemini", "ollama").
    pub provider: Option<String>,
    /// Model override.
    pub model: Option<String>,
    pub priority: TaskPriority,
    pub max_retries: u8,
}

impl LlmTask {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        system_prompt: impl Into<String>,
        user_prompt_template: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            system_prompt: system_prompt.into(),
            user_prompt_template: user_prompt_template.into(),
            max_tokens: None,
            temperature: None,
            provider: None,
            model: None,
            priority: TaskPriority::Normal,
            max_retries: 2,
        }
    }

    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn with_temperature(mut self, t: f32) -> Self {
        self.temperature = Some(t.clamp(0.0, 2.0));
        self
    }

    pub fn with_priority(mut self, p: TaskPriority) -> Self {
        self.priority = p;
        self
    }

    /// Render the user prompt by substituting `{input}`.
    pub fn render_prompt(&self, input: &str) -> String {
        self.user_prompt_template.replace("{input}", input)
    }

    /// Render with arbitrary key-value substitutions.
    pub fn render_prompt_with(&self, vars: &HashMap<String, String>) -> String {
        let mut out = self.user_prompt_template.clone();
        for (k, v) in vars {
            out = out.replace(&format!("{{{}}}", k), v);
        }
        out
    }
}

// ─── Task run ─────────────────────────────────────────────────────────────────

/// A single task execution run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRun {
    pub task_id: String,
    pub input: String,
    pub status: TaskStatus,
    pub output: Option<String>,
    pub error: Option<String>,
    pub attempt: u8,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub tokens_used: Option<u32>,
}

impl TaskRun {
    pub fn new(task_id: impl Into<String>, input: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        Self {
            task_id: task_id.into(),
            input: input.into(),
            status: TaskStatus::Running,
            output: None,
            error: None,
            attempt: 1,
            started_at: now,
            finished_at: None,
            tokens_used: None,
        }
    }

    pub fn complete(mut self, output: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        self.status = TaskStatus::Completed;
        self.output = Some(output.into());
        self.finished_at = Some(now);
        self
    }

    pub fn fail(mut self, error: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        self.status = TaskStatus::Failed;
        self.error = Some(error.into());
        self.finished_at = Some(now);
        self
    }

    pub fn duration_ms(&self) -> Option<i64> {
        self.finished_at.map(|f| (f - self.started_at) * 1000)
    }

    pub fn is_done(&self) -> bool {
        matches!(
            self.status,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
    }
}

// ─── Task registry ────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct TaskRegistry {
    tasks: HashMap<String, LlmTask>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, task: LlmTask) {
        self.tasks.insert(task.id.clone(), task);
    }

    pub fn get(&self, id: &str) -> Option<&LlmTask> {
        self.tasks.get(id)
    }

    pub fn remove(&mut self, id: &str) -> bool {
        self.tasks.remove(id).is_some()
    }

    pub fn list(&self) -> Vec<&LlmTask> {
        let mut tasks: Vec<_> = self.tasks.values().collect();
        tasks.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.name.cmp(&b.name)));
        tasks
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_prompt_substitution() {
        let task = LlmTask::new(
            "t1",
            "summarize",
            "You are a summarizer.",
            "Summarize: {input}",
        );
        assert_eq!(task.render_prompt("Hello world"), "Summarize: Hello world");
    }

    #[test]
    fn render_prompt_with_vars() {
        let task = LlmTask::new(
            "t1",
            "translate",
            "Translator.",
            "Translate {text} to {lang}.",
        );
        let mut vars = HashMap::new();
        vars.insert("text".into(), "hello".into());
        vars.insert("lang".into(), "Thai".into());
        assert_eq!(task.render_prompt_with(&vars), "Translate hello to Thai.");
    }

    #[test]
    fn task_run_complete() {
        let run = TaskRun::new("t1", "hello").complete("summarized");
        assert_eq!(run.status, TaskStatus::Completed);
        assert_eq!(run.output.as_deref(), Some("summarized"));
        assert!(run.is_done());
    }

    #[test]
    fn task_run_fail() {
        let run = TaskRun::new("t1", "hello").fail("timeout");
        assert_eq!(run.status, TaskStatus::Failed);
        assert!(run.error.is_some());
        assert!(run.is_done());
    }

    #[test]
    fn registry_list_by_priority() {
        let mut reg = TaskRegistry::new();
        reg.register(LlmTask::new("t1", "A", "sys", "prompt").with_priority(TaskPriority::Low));
        reg.register(LlmTask::new("t2", "B", "sys", "prompt").with_priority(TaskPriority::High));
        reg.register(LlmTask::new("t3", "C", "sys", "prompt").with_priority(TaskPriority::Normal));
        let list = reg.list();
        assert_eq!(list[0].id, "t2"); // High first
        assert_eq!(list[2].id, "t1"); // Low last
    }

    #[test]
    fn temperature_clamped() {
        let task = LlmTask::new("t", "n", "s", "p").with_temperature(5.0);
        assert_eq!(task.temperature, Some(2.0));
    }
}
