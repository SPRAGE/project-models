use std::{fs, path::Path};
use crate::config_models::{config::Config, defaults::DefaultConfig, error::ConfigError};

pub struct ConfigValidator;

impl ConfigValidator {
    /// Ensures `config.toml` exists, validates its contents, fixes missing parts if needed,
    /// and then loads the configuration into the global OnceCell.
    pub fn validate_and_fix<P: AsRef<Path>>(config_path: P) -> Result<(), ConfigError> {
        let path = config_path.as_ref();
        if !path.exists() {
            println!("⚠️ Config file not found. Creating a new one...");
            Self::create_default_config(path)?;
        }

        // Read and deserialize the config file into a temporary Config instance.
        let config_str = fs::read_to_string(path)
            .map_err(|e| ConfigError::FileError(format!("Failed to read config file: {}", e)))?;
        let temp_config: Config = toml::from_str(&config_str)
            .map_err(|e| ConfigError::ParseError(format!("Invalid TOML format: {}", e)))?;

        // Validate the temporary configuration.
        if let Err(e) = Self::validate(&temp_config) {
            println!("⚠️ Config is missing sections: {}. Fixing it...", e);
            Self::fix_missing_parts(&temp_config, path)?;
        }

        // Finally, load the (fixed) config into the global OnceCell.
        // Config::load(path)?;
        Ok(())
    }

    /// Validates that all required sections exist in the configuration.
    pub fn validate(config: &Config) -> Result<(), ConfigError> {
        if config.clickhouse.is_none() {
            return Err(ConfigError::MissingSection("Missing ClickHouse configuration".to_string()));
        }
        if config.redis.is_none() {
            return Err(ConfigError::MissingSection("Missing Redis configuration".to_string()));
        }
        if config.kafka.is_none() {
            return Err(ConfigError::MissingSection("Missing Kafka configuration".to_string()));
        }
        if config.zerodha.is_none() {
            return Err(ConfigError::MissingSection("Missing Zerodha configuration".to_string()));
        }
        if config.servers.is_none() {
            return Err(ConfigError::MissingSection("Missing Servers configuration".to_string()));
        }
        if config.ssl.is_none() {
            return Err(ConfigError::MissingSection("Missing SSL configuration".to_string()));
        }
        Ok(())
    }

    /// Creates a default `config.toml` using the default configuration values.
    fn create_default_config<P: AsRef<Path>>(config_path: P) -> Result<(), ConfigError> {
        let default_config = DefaultConfig::default();
        let toml_str = toml::to_string_pretty(&default_config)
            .map_err(|e| ConfigError::FileError(format!("Failed to serialize default config: {}", e)))?;
        fs::write(config_path, toml_str)
            .map_err(|e| ConfigError::FileError(format!("Failed to write default config: {}", e)))?;
        println!("✅ Default config.toml created!");
        Ok(())
    }

    /// Fixes missing sections by merging in default values and then writing the updated config to disk.
    fn fix_missing_parts<P: AsRef<Path>>(config: &Config, config_path: P) -> Result<(), ConfigError> {
        let mut fixed_config = config.clone();

        if fixed_config.clickhouse.is_none() {
            println!("⚠️ Adding missing ClickHouse section...");
            fixed_config.clickhouse = Some(DefaultConfig::default_clickhouse());
        }
        if fixed_config.redis.is_none() {
            println!("⚠️ Adding missing Redis section...");
            fixed_config.redis = Some(DefaultConfig::default_redis());
        }
        if fixed_config.kafka.is_none() {
            println!("⚠️ Adding missing Kafka section...");
            fixed_config.kafka = Some(DefaultConfig::default_kafka());
        }
        if fixed_config.zerodha.is_none() {
            println!("⚠️ Adding missing Zerodha section...");
            fixed_config.zerodha = Some(DefaultConfig::default_zerodha());
        }
        if fixed_config.servers.is_none() {
            println!("⚠️ Adding missing Servers section...");
            fixed_config.servers = Some(DefaultConfig::default_servers());
        }
        if fixed_config.ssl.is_none() {
            println!("⚠️ Adding missing SSL section...");
            fixed_config.ssl = Some(DefaultConfig::default_ssl());
        }

        let toml_str = toml::to_string_pretty(&fixed_config)
            .map_err(|e| ConfigError::FileError(format!("Failed to serialize fixed config: {}", e)))?;
        fs::write(config_path, toml_str)
            .map_err(|e| ConfigError::FileError(format!("Failed to update config: {}", e)))?;
        println!("✅ Config updated with missing sections!");
        Ok(())
    }
}
