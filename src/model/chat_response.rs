use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    model: String,
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize, Debug)]
pub struct ChatChoice {
    index: u32,
    pub message: ChatMessage,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    role: String,
    pub content: String,
}