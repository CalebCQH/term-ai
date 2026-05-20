use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: Vec<ChatContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContent {
    #[serde(rename = "type")]
    pub chat_type: String,
    pub text: String,
}

impl ChatMessage {
    pub fn new(role: &str, chat_type: &str, text: &str) -> Self {
        Self {
            role: role.to_string(),
            content: vec![ChatContent {
                chat_type: chat_type.to_string(),
                text: text.to_string(),
            }],
        }
    }
}
