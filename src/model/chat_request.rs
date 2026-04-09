use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatRequestParam {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}