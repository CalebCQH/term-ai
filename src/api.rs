use std::time::Duration;

use crate::model::chat_message::ChatMessage;
use crate::model::chat_request::ChatRequestParam;
use crate::model::chat_response::{ChatResponse, ChatStreamEnum};
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
) -> Result<ChatStreamEnum, AppError> {
    let max_token = if config.max_tokens == 0 {
        1024
    } else {
        config.max_tokens
    };
    let request = ChatRequestParam::new(true, &config.model, max_token, messages);
    let response = match CLIENT
        .post(format!("{}/v1/messages", &config.post_url))
        .header("x-api-key", &config.api_key)
        .json(&request)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err(AppError::Api(e.to_string())),
    };

    match response.json().await {
        Ok(data) => return Ok(data),
        Err(e) => return Err(AppError::Api(e.to_string())),
    };
}

/**
 * This method is Unstreamed send, use `send_message_stream` for streaming responses.
 */
pub async fn send_message(
    messages: &[ChatMessage],
    config: &config::api_config::ApiConfig,
) -> Result<ChatResponse, AppError> {
    let max_tokens = if config.max_tokens == 0 {
        1024
    } else {
        config.max_tokens
    };
    let request = ChatRequestParam::new(false, &config.model, max_tokens, messages);
    let response = match CLIENT
        .post(format!("{}/v1/messages", &config.post_url))
        .header("x-api-key", &config.api_key)
        .json(&request)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err(AppError::Api(e.to_string())),
    };

    // let body = response
    //     .text()
    //     .await
    //     .map_err(|e| AppError::Api(e.to_string()))?;
    // println!("响应体: {}", body);
    // return Err(AppError::Api("查看上方响应体".into()));
    match response.json().await {
        Ok(data) => return Ok(data),
        Err(e) => return Err(AppError::Api(e.to_string())),
    };
}
