use clap::Parser;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::{
    commands::{Cli, exit},
    config::api_config::{BASE_CONFIG, configure_model, get_current_model},
    model::{
        chat_message::{ChatContent, ChatMessage},
        chat_response::{ChatResponse, ChatStreamEnum, ContentTypeEnum},
        chat_roles::ChatRoles,
        chat_type::ChatTypeEnum,
    },
    result::AppError,
};

mod api;
mod commands;
mod config;
mod model;
mod result;

async fn run<'a>(
    chat: &'a str,
    history: &'a [ChatMessage],
) -> Result<Vec<ChatMessage>, result::AppError> {
    let lock = BASE_CONFIG.lock().await;
    let current_model = &lock.current_model;
    let model_config = lock
        .models
        .get(current_model)
        .ok_or_else(|| AppError::Config(format!("当前模型 '{}' 未配置", current_model)))?;
    // Judge whether this history message is not empty.
    let mut message: Vec<ChatMessage> = Vec::with_capacity(history.len() + 1);
    message.extend(history.iter().cloned());
    let chat_message = ChatMessage::new_once(
        ChatRoles::User.display_name().as_str(),
        ChatTypeEnum::Text.display().as_str(),
        chat,
    );
    message.push(chat_message.clone());
    // Create an API request and get the response.
    let chat_messages = api::send_message_stream(message.as_slice(), model_config).await?;
    message.extend(chat_messages);
    Ok(message)
}

async fn init() -> Result<config::api_config::ApiConfig, result::AppError> {
    config::api_config::load_config().await?;
    let current_model = get_current_model().await;
    match &current_model {
        Ok(model) => println!("欢迎使用term-ai v0.0.1 - 当前模型: {}", model.model),
        Err(e) => println!("欢迎使用term-ai 错误: {:?}", e),
    }
    current_model
}

#[tokio::main]
async fn main() -> Result<(), result::AppError> {
    match init().await {
        Ok(_) => {
            print!("term-id> ");
            // The code is a temporary solution for chat history.
            // I`ll refer to Gemini to develop a complete solution.
            let mut history: Vec<ChatMessage> = vec![];
            loop {
                let stdin = io::stdin();
                let mut reader = BufReader::new(stdin).lines();
                match reader.next_line().await? {
                    Some(command) => {
                        let args = std::iter::once("term-ai").chain(command.split_whitespace());
                        let try_parse_from = Cli::try_parse_from(args);
                        match try_parse_from {
                            Ok(cli) => match &cli.command {
                                commands::Command::Chat { message } => {
                                    let chat_resp = run(&message, &history).await?;
                                    history.extend(chat_resp);
                                    println!("\nUser: ");
                                }
                                commands::Command::Models => {
                                    let base_config = BASE_CONFIG.lock().await;
                                    for model in base_config.models.values() {
                                        println!("{}", model.model);
                                    }
                                }
                                commands::Command::ShowModel { name } => {
                                    let base_config = BASE_CONFIG.lock().await;
                                    if let Some(model) = base_config.models.get(name) {
                                        println!("{:?}", model);
                                    } else {
                                        println!("未找到模型: {}", name);
                                    }
                                }
                                commands::Command::Config => {
                                    configure_model().await?;
                                }
                                commands::Command::EditModel { name } => {
                                    let mut lock = BASE_CONFIG.lock().await;
                                    if lock.models.contains_key(&name.to_string()) {
                                        lock.current_model = name.clone();
                                        println!("当前模型已更新为: {}", name)
                                    } else {
                                        println!("未找到模型: {}", name)
                                    }
                                }
                                commands::Command::RemoveConfig { name } => {
                                    match BASE_CONFIG.lock().await.models.remove(name) {
                                        Some(_) => println!("已移除模型: {}", name),
                                        None => println!("未找到模型: {}", name),
                                    }
                                }
                                commands::Command::CurrentHistory => {
                                    println!("历史记录：{:?}", history)
                                }
                                commands::Command::Clear => {
                                    history.clear();
                                    println!("当前对话历史已清空！");
                                }
                                commands::Command::Exit => {
                                    let _ = exit().await;
                                    return Ok(());
                                }
                            },
                            Err(e) => eprintln!("命令解析失败: {}", e),
                        }
                    }
                    None => {
                        let _ = exit().await;
                        // EOF (Ctrl+D 或 管道关闭)
                        return Ok(());
                    }
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    // if let Err(e) = run(chat).await {
    //     eprintln!("错误: {}", e);
    // }
}
