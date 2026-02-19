use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentIdentity {
    pub name: String,
    pub emoji: String,
    pub personality: String,
    pub system_prompt: Option<String>,
}

impl Default for AgentIdentity {
    fn default() -> Self {
        Self {
            name: "krabkrab".to_string(),
            emoji: "🦀".to_string(),
            personality: "A helpful and precise AI assistant ported to Rust.".to_string(),
            system_prompt: None,
        }
    }
}

impl AgentIdentity {
    pub fn build_system_prompt(&self) -> String {
        if let Some(ref prompt) = self.system_prompt {
            return prompt.clone();
        }

        format!(
            "Your name is {}. {}. \
            Always identify yourself with the emoji {}. \
            Be concise, accurate, and efficient.",
            self.name, self.personality, self.emoji
        )
    }
}
