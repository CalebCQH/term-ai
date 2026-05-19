use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("{0}")]
    Config(String),

    #[error("API错误: {0}")]
    Api(String),

    #[error("请求错误: {0}")]
    Request(String),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Config(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::Request(format!("请求错误: {}", value.to_string()))
    }
}
