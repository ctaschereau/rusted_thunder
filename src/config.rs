use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub global: GlobalConfig,
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub api_token: String,
    pub default_vehicle: Option<String>,
    pub logspec: Option<String>,
}
