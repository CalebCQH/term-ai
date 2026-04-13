use serde::{Serialize};

use crate::model::chat_message::ChatMessage;

#[derive(Serialize)]
pub struct ChatRequestParam {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}