#[derive(Deserialize)]
pub struct BaseConfig {
    pub models: HashMap<String, ApiConfig>,
    pub default_model: String,
}