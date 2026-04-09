use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::model::chat_request::{ChatRequestParam, ChatMessage};
use crate::result::ApiResponse;
use crate::config::Config;
use crate::result::api_response::ApiErrorResponse;

pub async fn send_message(message: &str, config: &Config) -> ApiResponse {
    let client = Client::new();

    let request = ChatRequestParam {
        model: config.model.clone(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: message.to_string(),
        }],
    };

    let response = match client.post(&config.post_url)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .json(&request)
        .send()
        .await {
            Ok(response) => response,
            Err(e) => return ApiResponse::Error(ApiErrorResponse {
                code: ErrorCode::Unknown(response.status().as_u16()),
                message: e.to_string(),
            }),
        };

    let chat_response = match response.json().await {
        Ok(data) => data,
        Err(e) => return ApiResponse::Error(e.to_string()),
    };
    ApiResponse::Success(chat_response)
}