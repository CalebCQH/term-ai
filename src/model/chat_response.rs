use serde::{Deserialize};

use crate::model::chat_message::ChatMessage;

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub model: String,
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize, Debug)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
}