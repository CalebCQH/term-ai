use std::{fs, path::PathBuf};

use serde::Deserialize;
use tokio::fs::File;

use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    // pub id: String,
    pub model: String,
    pub post_url: String,
    pub api_key: String,
}

pub async fn load_config() -> Result<ApiConfig, AppError> {
    let path = get_user_config_path()
        .await
        .ok_or_else(|| AppError::Config("无法获取用户配置路径".into()))?;

    if !path.exists() {
        let cwd = std::env::current_dir()?;
        return Err(AppError::Config(format!(
            "配置文件不存在: {:?}\n当前工作目录: {:?}",
            path, cwd
        )));
    }

    let user_config = read_toml(&path)
        .map_err(|e| AppError::Config(format!("读取配置文件失败: {:?}: {}", path, e)))?;

    println!("{:?}", user_config);
    Ok(user_config)
}

fn read_toml(path: &PathBuf) -> Result<ApiConfig, AppError> {
    let content = fs::read_to_string(path)?;
    toml::from_str::<ApiConfig>(&content)
        .map_err(|e| AppError::Config(format!("TOML解析失败: {}", e)))
}

async fn get_user_config_path() -> Option<PathBuf> {
    // 获取用户主目录
    let home_dir = dirs::home_dir()?.join(".config");
    let config_path = home_dir.join("api-config.toml");
    if !home_dir.exists() {
        let create_dir_all = fs::create_dir_all(&home_dir);
        println!("创建目录成功");
        if create_dir_all.is_err() {
            return None;
        }
    }

    if !config_path.exists() {
        println!("创建配置文件成功");
        File::create(&config_path).await.ok();
    }
    Some(config_path)
}
