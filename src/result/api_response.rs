use serde::{Deserialize};
use crate::model::chat_response::ChatResponse;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(ChatResponse),
    Error(ApiErrorResponse),
}

#[derive(Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum ErrorCode {
    Success,
    Unauthorized,
    Limit,
    ServerError,
    Unknown(u16),
}

impl ErrorCode {
    pub fn from_u16(code: u16) -> Self {
        match code {
            200..=299 => ErrorCode::Success,
            401 => ErrorCode::Unauthorized,
            429 => ErrorCode::Limit,
            500..=599 => ErrorCode::ServerError,
            _ => ErrorCode::Unknown(code),
        }
    }
}