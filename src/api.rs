use crate::config;
use crate::model::chat_message::ChatMessage;
use crate::model::chat_request::ChatRequestParam;
use crate::result::ApiResponse;
use crate::result::api_response::{ApiErrorResponse, ErrorCode};
use reqwest::Client;

pub async fn send_message(message: &str, config: &config::api_config::Config) -> ApiResponse {
    let client = Client::new();

    let request = ChatRequestParam {
        model: config.model.clone(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: message.to_string(),
        }],
    };

    let response = match client
        .post(&config.post_url)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .json(&request)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            return ApiResponse::Error(ApiErrorResponse {
                code: ErrorCode::Unauthorized,
                message: e.to_string(),
            });
        }
    };
    let status = response.status().as_u16();

    let chat_response = match response.json().await {
        Ok(data) => data,
        Err(e) => {
            return ApiResponse::Error(ApiErrorResponse {
                code: ErrorCode::from_u16(status),
                message: e.to_string(),
            });
        }
    };
    ApiResponse::Success(chat_response)
}
