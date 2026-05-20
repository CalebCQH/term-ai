use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub chat_type: String,
    pub role: String,
    pub model: String,
    pub content: Vec<ContentTypeEnum>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ChatStreamEnum {
    #[serde(rename = "message_start")]
    MessageStart,
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: ContentTypeEnum,
    },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        index: usize,
        delta: ContentTypeEnum,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: usize },
    #[serde(rename = "message_delta")]
    MessageDelta,
    #[serde(rename = "message_stop")]
    MessageStop,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ContentTypeEnum {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String, signature: String },
    #[serde(rename = "thinking_delta")]
    ThinkingDelta { thinking: String },
    #[serde(rename = "signature")]
    Signature { signature: String },
}
