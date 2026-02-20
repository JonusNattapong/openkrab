//! Wizard session — drives the step-by-step wizard via a push/pull model.
//! Ported from `openclaw/src/wizard/session.ts`
//!
//! The session allows the wizard logic (which calls `prompter.select()`, etc.)
//! to be driven externally — e.g. from a gateway protocol that sends steps
//! over WebSocket and receives answers back.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;

use super::prompts::{
    WizardCancelledError, WizardConfirmParams, WizardMultiSelectParams,
    WizardProgress, WizardPrompter, WizardSelectParams, WizardTextParams, NoopProgress,
};

/// Represents a single wizard step sent to the client.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WizardStep {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: WizardStepType,
    pub title: Option<String>,
    pub message: Option<String>,
    pub options: Option<Vec<WizardStepOption>>,
    pub initial_value: Option<serde_json::Value>,
    pub placeholder: Option<String>,
    pub sensitive: Option<bool>,
    pub executor: Option<String>,
}

/// Step types supported by the wizard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WizardStepType {
    Note,
    Select,
    Text,
    Confirm,
    Multiselect,
    Progress,
    Action,
}

/// Option within a select/multiselect step.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WizardStepOption {
    pub value: serde_json::Value,
    pub label: String,
    pub hint: Option<String>,
}

/// Session status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WizardSessionStatus {
    Running,
    Done,
    Cancelled,
    Error,
}

/// Result of calling `next()` on a session.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WizardNextResult {
    pub done: bool,
    pub step: Option<WizardStep>,
    pub status: WizardSessionStatus,
    pub error: Option<String>,
}

/// A wizard session that bridges async wizard logic with external step/answer protocol.
pub struct WizardSession {
    inner: Arc<Mutex<WizardSessionInner>>,
}

struct WizardSessionInner {
    current_step: Option<WizardStep>,
    status: WizardSessionStatus,
    error: Option<String>,
    answer_channels: HashMap<String, oneshot::Sender<serde_json::Value>>,
    step_notify: Option<tokio::sync::oneshot::Sender<Option<WizardStep>>>,
}

impl WizardSession {
    /// Create a new wizard session with a runner function.
    ///
    /// The runner is an async function that receives a `WizardPrompter` and
    /// drives the wizard logic (calling select, text, confirm, etc.).
    /// The session adapts those calls into the step/answer protocol.
    pub fn new<F, Fut>(runner: F) -> Self
    where
        F: FnOnce(Arc<dyn WizardPrompter>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
    {
        let inner = Arc::new(Mutex::new(WizardSessionInner {
            current_step: None,
            status: WizardSessionStatus::Running,
            error: None,
            answer_channels: HashMap::new(),
            step_notify: None,
        }));

        let inner_clone = inner.clone();
        let prompter: Arc<dyn WizardPrompter> = Arc::new(SessionPrompter {
            inner: inner_clone.clone(),
        });

        tokio::spawn(async move {
            let result = runner(prompter).await;
            let mut state = inner_clone.lock().await;
            match result {
                Ok(()) => {
                    state.status = WizardSessionStatus::Done;
                }
                Err(e) => {
                    if e.downcast_ref::<WizardCancelledError>().is_some() {
                        state.status = WizardSessionStatus::Cancelled;
                        state.error = Some("cancelled".to_string());
                    } else {
                        state.status = WizardSessionStatus::Error;
                        state.error = Some(e.to_string());
                    }
                }
            }
            // Notify any waiting `next()` call
            if let Some(tx) = state.step_notify.take() {
                let _ = tx.send(None);
            }
        });

        Self { inner }
    }

    /// Get the next wizard step. Blocks until a step is available or the session ends.
    pub async fn next(&self) -> WizardNextResult {
        let rx = {
            let mut state = self.inner.lock().await;
            if let Some(step) = state.current_step.clone() {
                return WizardNextResult {
                    done: false,
                    step: Some(step),
                    status: state.status,
                    error: state.error.clone(),
                };
            }
            if state.status != WizardSessionStatus::Running {
                return WizardNextResult {
                    done: true,
                    step: None,
                    status: state.status,
                    error: state.error.clone(),
                };
            }
            let (tx, rx) = oneshot::channel();
            state.step_notify = Some(tx);
            rx
        };

        match rx.await {
            Ok(Some(step)) => WizardNextResult {
                done: false,
                step: Some(step),
                status: WizardSessionStatus::Running,
                error: None,
            },
            _ => {
                let state = self.inner.lock().await;
                WizardNextResult {
                    done: true,
                    step: None,
                    status: state.status,
                    error: state.error.clone(),
                }
            }
        }
    }

    /// Submit an answer for a pending step.
    pub async fn answer(&self, step_id: &str, value: serde_json::Value) -> anyhow::Result<()> {
        let mut state = self.inner.lock().await;
        let tx = state.answer_channels.remove(step_id)
            .ok_or_else(|| anyhow::anyhow!("wizard: no pending step with id {}", step_id))?;
        state.current_step = None;
        let _ = tx.send(value);
        Ok(())
    }

    /// Cancel the wizard session.
    pub async fn cancel(&self) {
        let mut state = self.inner.lock().await;
        if state.status != WizardSessionStatus::Running {
            return;
        }
        state.status = WizardSessionStatus::Cancelled;
        state.error = Some("cancelled".to_string());
        state.current_step = None;
        // Reject all pending answer channels
        state.answer_channels.clear();
        if let Some(tx) = state.step_notify.take() {
            let _ = tx.send(None);
        }
    }

    /// Get the current session status.
    pub async fn get_status(&self) -> WizardSessionStatus {
        self.inner.lock().await.status
    }
}

/// Internal prompter that bridges WizardPrompter calls to the session protocol.
struct SessionPrompter {
    inner: Arc<Mutex<WizardSessionInner>>,
}

impl SessionPrompter {
    async fn push_step_and_await(&self, step: WizardStep) -> anyhow::Result<serde_json::Value> {
        let (tx, rx) = oneshot::channel();
        {
            let mut state = self.inner.lock().await;
            if state.status != WizardSessionStatus::Running {
                return Err(WizardCancelledError::new("wizard: session not running").into());
            }
            state.answer_channels.insert(step.id.clone(), tx);
            state.current_step = Some(step.clone());
            // Notify `next()` waiters
            if let Some(notify_tx) = state.step_notify.take() {
                let _ = notify_tx.send(Some(step));
            }
        }
        rx.await.map_err(|_| WizardCancelledError::default().into())
    }
}

#[async_trait::async_trait]
impl WizardPrompter for SessionPrompter {
    async fn intro(&self, title: &str) -> anyhow::Result<()> {
        let step = WizardStep {
            id: Uuid::new_v4().to_string(),
            step_type: WizardStepType::Note,
            title: Some(title.to_string()),
            message: Some(String::new()),
            options: None,
            initial_value: None,
            placeholder: None,
            sensitive: None,
            executor: Some("client".to_string()),
        };
        let _ = self.push_step_and_await(step).await?;
        Ok(())
    }

    async fn outro(&self, message: &str) -> anyhow::Result<()> {
        let step = WizardStep {
            id: Uuid::new_v4().to_string(),
            step_type: WizardStepType::Note,
            title: Some("Done".to_string()),
            message: Some(message.to_string()),
            options: None,
            initial_value: None,
            placeholder: None,
            sensitive: None,
            executor: Some("client".to_string()),
        };
        let _ = self.push_step_and_await(step).await?;
        Ok(())
    }

    async fn note(&self, message: &str, title: Option<&str>) -> anyhow::Result<()> {
        let step = WizardStep {
            id: Uuid::new_v4().to_string(),
            step_type: WizardStepType::Note,
            title: title.map(|s| s.to_string()),
            message: Some(message.to_string()),
            options: None,
            initial_value: None,
            placeholder: None,
            sensitive: None,
            executor: Some("client".to_string()),
        };
        let _ = self.push_step_and_await(step).await?;
        Ok(())
    }

    async fn select(&self, params: WizardSelectParams<String>) -> anyhow::Result<String> {
        let step = WizardStep {
            id: Uuid::new_v4().to_string(),
            step_type: WizardStepType::Select,
            title: None,
            message: Some(params.message),
            options: Some(
                params.options.into_iter().map(|opt| WizardStepOption {
                    value: serde_json::Value::String(opt.value),
                    label: opt.label,
                    hint: opt.hint,
                }).collect()
            ),
            initial_value: params.initial_value.map(serde_json::Value::String),
            placeholder: None,
            sensitive: None,
            executor: Some("client".to_string()),
        };
        let result = self.push_step_and_await(step).await?;
        Ok(result.as_str().unwrap_or("").to_string())
    }

    async fn multiselect(&self, params: WizardMultiSelectParams<String>) -> anyhow::Result<Vec<String>> {
        let step = WizardStep {
            id: Uuid::new_v4().to_string(),
            step_type: WizardStepType::Multiselect,
            title: None,
            message: Some(params.message),
            options: Some(
                params.options.into_iter().map(|opt| WizardStepOption {
                    value: serde_json::Value::String(opt.value),
                    label: opt.label,
                    hint: opt.hint,
                }).collect()
            ),
            initial_value: params.initial_values.map(|vs| serde_json::json!(vs)),
            placeholder: None,
            sensitive: None,
            executor: Some("client".to_string()),
        };
        let result = self.push_step_and_await(step).await?;
        let arr = result.as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();
        Ok(arr)
    }

    async fn text(&self, params: WizardTextParams) -> anyhow::Result<String> {
        let step = WizardStep {
            id: Uuid::new_v4().to_string(),
            step_type: WizardStepType::Text,
            title: None,
            message: Some(params.message),
            options: None,
            initial_value: params.initial_value.map(serde_json::Value::String),
            placeholder: params.placeholder,
            sensitive: None,
            executor: Some("client".to_string()),
        };
        let result = self.push_step_and_await(step).await?;
        Ok(result.as_str().unwrap_or("").to_string())
    }

    async fn confirm(&self, params: WizardConfirmParams) -> anyhow::Result<bool> {
        let step = WizardStep {
            id: Uuid::new_v4().to_string(),
            step_type: WizardStepType::Confirm,
            title: None,
            message: Some(params.message),
            options: None,
            initial_value: params.initial_value.map(serde_json::Value::Bool),
            placeholder: None,
            sensitive: None,
            executor: Some("client".to_string()),
        };
        let result = self.push_step_and_await(step).await?;
        Ok(result.as_bool().unwrap_or(false))
    }

    fn progress(&self, _label: &str) -> Box<dyn WizardProgress> {
        Box::new(NoopProgress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_type_serialization() {
        let json = serde_json::to_string(&WizardStepType::Select).unwrap();
        assert_eq!(json, "\"select\"");

        let json = serde_json::to_string(&WizardStepType::Confirm).unwrap();
        assert_eq!(json, "\"confirm\"");
    }

    #[test]
    fn session_status_serialization() {
        let json = serde_json::to_string(&WizardSessionStatus::Running).unwrap();
        assert_eq!(json, "\"running\"");

        let json = serde_json::to_string(&WizardSessionStatus::Done).unwrap();
        assert_eq!(json, "\"done\"");
    }
}
