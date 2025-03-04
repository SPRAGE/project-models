use std::{fs, path::Path, sync::Mutex};
use once_cell::sync::Lazy;
use serde::Deserialize;

pub mod error;
pub mod clickhouse_config;
pub mod kafka_config;
pub mod redis_config;

use error::{ConfigError, ClickHouseError, KafkaError, RedisError};
use clickhouse_config::ClickHouseConfig;
// use kafka_config::KafkaConfig;
// use redis_config::RedisConfig;

/// Global Lazy Config Instance (Thread-Safe)
static CONFIG: Lazy<Mutex<Option<Config>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub clickhouse: Option<ClickHouseConfig>,
    // pub kafka: Option<KafkaConfig>,
    // pub redis: Option<RedisConfig>,
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

    /// Returns the ClickHouse configuration with its own error handling.
    pub fn get_clickhouse_config() -> Result<ClickHouseConfig, ClickHouseError> {
        Config::get()
            .map_err(|_| ClickHouseError::ConfigNotLoaded)?
            .clickhouse
            .ok_or(ClickHouseError::SectionMissing)
    }

    // Returns the Kafka configuration with its own error handling.
    // pub fn get_kafka_config() -> Result<KafkaConfig, KafkaError> {
    //     Config::get()
    //         .map_err(|_| KafkaError::ConfigNotLoaded)?
    //         .kafka
    //         .ok_or(KafkaError::SectionMissing)
    // }
    //
    // /// Returns the Redis configuration with its own error handling.
    // pub fn get_redis_config() -> Result<RedisConfig, RedisError> {
    //     Config::get()
    //         .map_err(|_| RedisError::ConfigNotLoaded)?
    //         .redis
    //         .ok_or(RedisError::SectionMissing)
    // }
}

