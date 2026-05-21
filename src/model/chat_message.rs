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

impl ChatContent {
    pub fn new(chat_type: &str, text: &str) -> Self {
        Self {
            chat_type: chat_type.to_string(),
            text: text.to_string(),
        }
    }
}

impl ChatMessage {
    pub fn new_once(role: &str, chat_type: &str, text: &str) -> Self {
        Self {
            role: role.to_string(),
            content: vec![ChatContent {
                chat_type: chat_type.to_string(),
                text: text.to_string(),
            }],
        }
    }

    pub fn new(role: &str, chat_content: Vec<ChatContent>) -> Self {
        Self {
            role: role.to_string(),
            content: chat_content,
        }
    }
}
