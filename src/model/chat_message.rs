use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: ChatType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatType {
    #[serde(rename = "type")]
    pub test_type: String,
    pub text: String,
}

pub enum ChatTypeEnum {
    Text,
}

impl ChatMessage {
    pub fn new(role: String, test_type: String, text: String) -> Self {
        let chat_type = ChatType { test_type, text };
        Self {
            role,
            content: chat_type,
        }
    }
}

impl ChatTypeEnum {
    pub fn display(&self) -> String {
        match self {
            ChatTypeEnum::Text => "text".to_string(),
        }
    }
}
