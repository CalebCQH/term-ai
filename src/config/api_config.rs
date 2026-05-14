use std::{fs, io::Read, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    // pub id: String,
    pub model: String,
    pub post_url: String,
    pub api_key: String,
}

pub fn load_config() -> ApiConfig {
    if let Some(path) = get_user_config_path() {
        if path.exists() {
            let user_config: ApiConfig = read_toml(path);
            println!("{:?}", user_config);
            return user_config;
        } else {
            println!("当前工作目录: {:?}", std::env::current_dir().unwrap());
            println!("用户配置路径: {:?}", get_user_config_path());
            return ApiConfig {
                model: "".to_string(),
                post_url: "".to_string(),
                api_key: "".to_string(),
            };
        }
    } else {
        return ApiConfig {
            model: "".to_string(),
            post_url: "".to_string(),
            api_key: "".to_string(),
        };
    }
}

fn read_toml(path: PathBuf) -> ApiConfig {
    let mut config = match fs::OpenOptions::new().append(true).create(true).open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("{:?}", e.to_string());
            return ApiConfig {
                model: "".to_string(),
                post_url: "".to_string(),
                api_key: "".to_string(),
            };
        }
    };
    let mut content = String::new();
    let _ = config.read_to_string(&mut content);
    let config_str = content.clone();
    let config = toml::from_str(&config_str).unwrap();
    config
}

fn get_user_config_path() -> Option<PathBuf> {
    // 获取用户主目录
    let home = dirs::home_dir()?;
    Some(home.join(".config/api-config.toml"))
}
