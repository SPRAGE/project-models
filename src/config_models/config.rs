use std::{fs, path::Path, sync::Mutex};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::config_models::{error::ConfigError, clickhouse_config::ClickHouseConfig, redis_config::RedisConfigData};

/// Global Lazy Config Instance (Thread-Safe)
static CONFIG: Lazy<Mutex<Option<Config>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub clickhouse: Option<ClickHouseConfig>,

    #[serde(default)]
    pub redis: Option<RedisConfigData>,
}

impl Config {
    /// Reads the config file **once** and stores it in memory.
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<(), ConfigError> {
        let config_str = fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::FileError(format!("Failed to read config file: {}", e)))?;

        let parsed_config: Config = toml::from_str(&config_str)
            .map_err(|e| ConfigError::ParseError(format!("Invalid TOML format: {}", e)))?;

        let mut config_lock = CONFIG.lock().unwrap();
        *config_lock = Some(parsed_config);

        Ok(())
    }

    /// Returns the entire loaded configuration.
    pub fn get() -> Result<Config, ConfigError> {
        let config_lock = CONFIG.lock().unwrap();
        config_lock.clone().ok_or_else(|| ConfigError::NotLoaded("Configuration not loaded".to_string()))
    }
}

