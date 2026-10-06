use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, File, Environment};
use std::env;
use crate::enums::transport_mode::TransportMode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub service_host: String,
    pub service_port: u16,
    pub app_prefix: String,
    pub request_timeout: u64,
    pub log_level: String,
    pub transport_mode: TransportMode,
    pub typesafe_api_url: String,
    pub typesafe_api_key: String,
    pub model: String,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
        let settings = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", app_env)))
            .add_source(Environment::with_prefix("APP"))
            .build()?
            .try_deserialize::<Self>()?;
        Ok(settings)
    }
}