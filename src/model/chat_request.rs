use serde::Serialize;

use crate::model::chat_message::ChatMessage;

#[derive(Serialize, Debug)]
pub struct ChatRequestParam<'a> {
    pub stream: bool,
    pub model: &'a str,
    pub max_tokens: usize,
    pub messages: &'a [ChatMessage],
}

impl<'a> ChatRequestParam<'a> {
    pub fn new(
        stream: bool,
        model: &'a str,
        max_tokens: usize,
        messages: &'a [ChatMessage],
    ) -> Self {
        Self {
            stream,
            model,
            max_tokens,
            messages,
        }
    }
}
