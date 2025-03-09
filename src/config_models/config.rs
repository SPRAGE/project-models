use std::{fs, path::Path, sync::Arc};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::config_models::{
    error::ConfigError,
    clickhouse_config::ClickHouseConfig,
    redis_config::RedisConfigData,
    kafka_config::KafkaConfig,
    zerodha_config::ZerodhaConfig,
    server::ServersConfig,
};

/// Global OnceCell holding the configuration wrapped in an Arc for shared access.
static CONFIG: OnceCell<Arc<Config>> = OnceCell::new();



#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub clickhouse: Option<ClickHouseConfig>,
    pub redis: Option<RedisConfigData>,
    pub kafka: Option<KafkaConfig>,
    pub zerodha: Option<ZerodhaConfig>,
    pub servers: Option<ServersConfig>,

}

impl Config {
    /// Reads the config file **once** and stores it in memory.
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<(), ConfigError> {
        let config_str = fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::FileError(format!("Failed to read config file: {}", e)))?;

        let parsed_config: Config = toml::from_str(&config_str)
            .map_err(|e| ConfigError::ParseError(format!("Invalid TOML format: {}", e)))?;

        // Try to set the global CONFIG. If already set, return an error.
        CONFIG
            .set(Arc::new(parsed_config))
            .map_err(|_| ConfigError::AlreadyLoaded("Configuration is already loaded".to_string()))
    }

    /// Returns a shared reference to the loaded configuration.
    pub fn get() -> Result<Arc<Config>, ConfigError> {
        CONFIG
            .get()
            .cloned()
            .ok_or_else(|| ConfigError::NotLoaded("Configuration not loaded".to_string()))
    }
}

