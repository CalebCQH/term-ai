use serde::Serialize;

use crate::model::chat_message::ChatMessage;

#[derive(Serialize, Debug)]
pub struct ChatRequestParam<'a> {
    pub model: &'a str,
    pub messages: &'a [ChatMessage],
}
