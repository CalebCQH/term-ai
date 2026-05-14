mod api;
mod config;
mod error;
mod model;
mod result;
mod ui;

use error::AppError;

async fn run(chat: String) -> Result<(), AppError> {
    let config = config::api_config::load_config().await?;
    match api::send_message(&chat, &config).await {
        result::ApiResponse::Success(response) => {
            let content = &response.choices[0].message.content;
            println!("{}", content);
            Ok(())
        }
        result::ApiResponse::Error(e) => Err(AppError::Api(format!("{:?}", e))),
    }
}

#[tokio::main]
async fn main() {
    let chat = {
        println!("请输入对话:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    if let Err(e) = run(chat).await {
        eprintln!("错误: {}", e);
    }
}
