use clap::Parser;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::{
    commands::{Cli, exit},
    config::api_config::{BASE_CONFIG, configure_model, get_current_model},
    model::{
        chat_message::{ChatMessage, ChatTypeEnum},
        chat_rules::ChatRules,
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
        .ok_or_else(|| AppError::Config(format!("当前模型 '{}' 未配置", current_model,)))?;
    // Judge whether this history message is not empty.
    let mut message: Vec<ChatMessage> = Vec::with_capacity(history.len() + 1);
    message.extend(history.iter().cloned());
    let chat_message = ChatMessage::new(
        ChatRules::User.display_name(),
        ChatTypeEnum::Text.display(),
        chat.to_string(),
    );
    message.push(chat_message.clone());
    let response = api::send_message(message.as_slice(), model_config).await?;
    println!("{:?}", response.choices[0].message.content);
    let chat_resp = ChatMessage::new(
        ChatRules::Assistant.display_name(),
        ChatTypeEnum::Text.display(),
        response.choices[0].message.content.clone(),
    );
    Ok(vec![chat_message, chat_resp])
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
                                        println!("{}", model.model);
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
