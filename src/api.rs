use std::io::{self, Write};
use std::time::Duration;

use crate::model::chat_message::{ChatContent, ChatMessage};
use crate::model::chat_request::ChatRequestParam;
use crate::model::chat_response::{ChatStreamEnum, ContentTypeEnum};
use crate::model::chat_roles::ChatRoles;
use crate::model::chat_type::ChatTypeEnum;
use crate::result;
use crate::{config, result::AppError};
use once_cell::sync::Lazy;
use reqwest::Client;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(600))
        .build()
        .expect("创建 HTTP 客户端请求失败")
});

/**
 * This method is Streamed send, use `send_message` for unstreamed responses.
 */
pub async fn send_message_stream(
    messages: &[ChatMessage],
    config: &config::api_config::ApiConfig,
) -> Result<Vec<ChatMessage>, result::AppError> {
    let max_token = if config.max_tokens == 0 {
        1024
    } else {
        config.max_tokens
    };
    let request = ChatRequestParam::new(true, &config.model, max_token, messages);
    let mut response = match CLIENT
        .post(format!("{}/v1/messages", &config.post_url))
        .header("x-api-key", &config.api_key)
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            println!("{:?}", response);
            response
        }
        Err(e) => return Err(AppError::Api(e.to_string())),
    };

    let mut buffer = String::new();
    let mut full_text = String::new();
    let mut is_text_start: bool = true;
    let mut is_thinking_start: bool = true;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|e| AppError::Api(e.to_string()))?
    {
        buffer.push_str(&String::from_utf8_lossy(&chunk));
        // Get a line from the buffer
        while let Some(nl_pos) = buffer.find('\n') {
            let line = buffer[..nl_pos].to_string();
            buffer = buffer[nl_pos + 1..].to_string();
            // Handle lines that start with "data: "
            let Some(data) = line.strip_prefix("data: ") else {
                continue;
            };
            // Deserialize the line into a ChatStreamEnum.
            let Ok(event) = serde_json::from_str::<ChatStreamEnum>(data) else {
                continue;
            };
            match event {
                ChatStreamEnum::ContentBlockDelta { delta, .. } => match delta {
                    ContentTypeEnum::TextDelta { text } => {
                        if is_text_start {
                            print!("\n[Text]: ");
                            is_text_start = false;
                        }
                        print!("{}", text);
                        let _ = io::stdout().flush().unwrap();
                        full_text.push_str(&text);
                    }
                    ContentTypeEnum::ThinkingDelta { thinking } => {
                        if is_thinking_start {
                            print!("\n[Thinking]: ");
                            is_thinking_start = false;
                        }
                        print!("{}", thinking);
                        let _ = io::stdout().flush().unwrap();
                    }
                    _ => {} // signature ignore
                },
                ChatStreamEnum::MessageStop => {
                    let text_content =
                        ChatContent::new(ChatTypeEnum::Text.display().as_str(), full_text.as_str());
                    println!();
                    // Merge the thinking content and text content into a single ChatMessage
                    return Ok(vec![ChatMessage::new(
                        ChatRoles::Assistant.display_name().as_str(),
                        vec![text_content],
                    )]);
                }
                _ => {}
            }
        }
    }
    return Err(AppError::Request("未知错误".to_string()));
}
