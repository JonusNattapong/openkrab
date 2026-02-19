use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UserId(pub String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub from: Option<UserId>,
}

impl Message {
    pub fn simple(text: &str) -> Self {
        Self { id: "msg-1".into(), text: text.to_string(), from: None }
    }
}
