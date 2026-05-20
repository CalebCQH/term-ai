use std::{collections::HashMap, fs, path::PathBuf};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::BufReader, sync::Mutex};

use crate::{
    commands::{is_correct, read_line},
    result::AppError,
};

pub static BASE_CONFIG: Lazy<Mutex<BaseConfig>> = Lazy::new(|| {
    Mutex::new(BaseConfig {
        models: HashMap::new(),
        current_model: String::new(),
    })
});

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseConfig {
    pub models: HashMap<String, ApiConfig>,
    pub current_model: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ApiConfig {
    // pub id: String,
    pub max_tokens: usize,
    pub model: String,
    pub post_url: String,
    pub api_key: String,
}

impl BaseConfig {
    fn new(models: HashMap<String, ApiConfig>, current_model: String) -> Self {
        Self {
            models: models,
            current_model: current_model,
        }
    }
}

impl ApiConfig {
    fn new(max_tokens: usize, model: String, post_url: String, api_key: String) -> Self {
        Self {
            max_tokens: max_tokens,
            model: model,
            post_url: post_url,
            api_key: api_key,
        }
    }
}

pub async fn load_config() -> Result<(), AppError> {
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
        .await
        .map_err(|e| AppError::Config(format!("读取配置文件失败: {:?}: {}", path, e)))?;
    let mut mutex_guard = BASE_CONFIG.lock().await;
    *mutex_guard = user_config;
    Ok(())
}

pub async fn get_current_model() -> Result<ApiConfig, AppError> {
    let base_config = BASE_CONFIG.lock().await;
    match base_config.models.get(&base_config.current_model) {
        Some(model_config) => Ok(model_config.clone()),
        None => {
            return Err(AppError::Config("当前模型未配置".into()));
        }
    }
}

pub async fn configure_model() -> Result<ApiConfig, AppError> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let max_tokens: usize = match read_line("请配置最大令牌数：", &mut reader).await?.parse()
    {
        Ok(it) => it,
        Err(err) => return Err(AppError::Config(err.to_string())),
    };
    let model_name = read_line("请配置模型名称：", &mut reader).await?;
    let model_url = read_line("请配置模型URL：", &mut reader).await?;
    let api_key = read_line("请配置API密钥：", &mut reader).await?;
    // 配置
    let api_config = ApiConfig::new(max_tokens, model_name, model_url, api_key);
    let mut lock = BASE_CONFIG.lock().await;
    let exists = lock.models.contains_key(&api_config.model);
    if exists {
        let is_correct = is_correct("该模型已存在，是否需要替换?", &mut reader, "y").await?;
        if is_correct {
            lock.models
                .insert(api_config.model.clone(), api_config.clone());
            println!("添加成功！")
        }
    } else {
        lock.models
            .insert(api_config.model.clone(), api_config.clone());
        println!("添加成功！")
    }
    Ok(api_config)
}

pub async fn get_user_config_path() -> Option<PathBuf> {
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

async fn read_toml(path: &PathBuf) -> Result<BaseConfig, AppError> {
    let content = fs::read_to_string(path)
        .map_err(|e| AppError::Config(format!("读取配置文件失败: {}", e)))?;
    if content.is_empty() {
        println!("配置为空, 开始初始配置！");
        let config = configure_model().await?;
        // 构造 BaseConfig
        let mut models = HashMap::new();
        models.insert(config.model.clone(), config.clone());
        let base_config = BaseConfig::new(models, config.model.clone());
        let contents = toml::to_string(&base_config).unwrap();
        fs::write(path, contents).map_err(|e| AppError::Config(format!("配置错误：{}", e)))?;
        return Ok(base_config);
    }
    toml::from_str::<BaseConfig>(&content)
        .map_err(|e| AppError::Config(format!("TOML解析失败: {}", e)))
}
