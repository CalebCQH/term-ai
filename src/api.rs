use std::time::Duration;

use crate::model::chat_message::ChatMessage;
use crate::model::chat_request::ChatRequestParam;
use crate::model::chat_response::ChatResponse;
use crate::{config, result::AppError};
use once_cell::sync::Lazy;
use reqwest::Client;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("创建 HTTP 客户端请求失败")
});

pub async fn send_message(
    messages: &[ChatMessage],
    config: &config::api_config::ApiConfig,
) -> Result<ChatResponse, AppError> {
    let request = ChatRequestParam {
        model: &config.model,
        messages: messages,
    };
    let response = match CLIENT
        .post(format!("{}/v1/messages", &config.post_url))
        .header("x-api-key", format!("Bearer {}", &config.api_key))
        .json(&request)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err(AppError::Api(e.to_string())),
    };
    match response.json().await {
        Ok(data) => {
            return Ok(data);
        }
        Err(e) => return Err(AppError::Api(e.to_string())),
    };
}
