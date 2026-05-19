use clap::Parser;
use tokio::{fs, io::AsyncBufReadExt};

use crate::{config::api_config::BASE_CONFIG, result::AppError};

#[derive(Parser)]
#[command(name = "term-ai")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Send a chat message
    Chat {
        /// The message to send
        message: String,
    },
    /// Lisrt all available models
    Models,
    /// Show the details of a model
    ShowModel {
        /// The name of the model
        name: String,
    },
    /// Start the configuration wizard
    Config,
    /// Edit the current model
    EditModel {
        /// The name of the model
        name: String,
    },
    /// Remove a configured model
    RemoveConfig {
        /// The name of the model
        name: String,
    },
    /// Current Chat History
    CurrentHistory,
    /// Clear the chat history
    Clear,
    /// Exit the program
    Exit,
}

/**
 * Read a line from the terminal.
 */
pub async fn read_line(
    prompt: &str,
    reader: &mut (impl AsyncBufReadExt + Unpin),
) -> Result<String, AppError> {
    print!("{}", prompt);
    std::io::Write::flush(&mut std::io::stdout())?;
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    Ok(line.trim().to_string())
}

/**
 * Check yes or no
 */
pub async fn is_correct(
    prompt: &str,
    reader: &mut (impl AsyncBufReadExt + Unpin),
    correct_answer: &str,
) -> Result<bool, AppError> {
    let answer = read_line(prompt, reader).await?;
    Ok(answer == correct_answer)
}

/**
 * Exit the program and save the config.
 */
pub async fn exit() -> Result<(), AppError> {
    // Get the config path
    let config_path = match crate::config::api_config::get_user_config_path().await {
        Some(path) => path,
        None => {
            println!("再见！");
            return Ok(());
        }
    };
    // Get the config lock
    let config_guard = BASE_CONFIG.lock().await;
    let toml_string = toml::to_string(&*config_guard)
        .map_err(|e| AppError::Config(format!("序列化配置失败: {}", e)))?;
    drop(config_guard);
    // Async write the config to disk
    fs::write(&config_path, toml_string)
        .await
        .map_err(|e| AppError::Config(format!("写入配置文件失败: {}", e)))?;
    println!("再见！");
    Ok(())
}
