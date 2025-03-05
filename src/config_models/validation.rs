use std::{fs, path::Path};
use crate::config_models::{config::Config, defaults::DefaultConfig, error::ConfigError};

pub struct ConfigValidator;

impl ConfigValidator {
    /// **Ensures `config.toml` exists & auto-fixes missing parts**
    pub fn validate_and_fix<P: AsRef<Path>>(config_path: P) -> Result<(), ConfigError> {
        if !config_path.as_ref().exists() {

            println!("⚠️ Config file not found. Creating a new one...");
            Self::create_default_config(&config_path)?;
        }

        Config::load(&config_path)?;

        let config = Config::get()?;
        if let Err(e) = Self::validate(&config) {
            println!("⚠️ Config is missing sections: {}. Fixing it...", e);
            Self::fix_missing_parts(&config, &config_path)?;
        }

        Ok(())
    }

    /// **Validates if all required sections exist in the loaded config**
    pub fn validate(config: &Config) -> Result<(), ConfigError> {
        if config.clickhouse.is_none() {
            return Err(ConfigError::MissingSection("Missing ClickHouse configuration".to_string()));
        }

        if config.redis.is_none() {
            return Err(ConfigError::MissingSection("Missing Redis configuration".to_string()));
        }

        Ok(())
    }

    /// **Creates a default `config.toml`**
    fn create_default_config<P: AsRef<Path>>(config_path: P) -> Result<(), ConfigError> {
        let default_config = DefaultConfig::default();

        let toml_str = toml::to_string_pretty(&default_config)
            .map_err(|e| ConfigError::FileError(format!("Failed to serialize default config: {}", e)))?;

        fs::write(config_path, toml_str)
            .map_err(|e| ConfigError::FileError(format!("Failed to write default config: {}", e)))?;

        println!("✅ Default config.toml created!");
        Ok(())
    }

    /// **Adds missing sections if they don’t exist**
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

        let toml_str = toml::to_string_pretty(&fixed_config)
            .map_err(|e| ConfigError::FileError(format!("Failed to serialize fixed config: {}", e)))?;

        fs::write(config_path, toml_str)
            .map_err(|e| ConfigError::FileError(format!("Failed to update config: {}", e)))?;

        println!("✅ Config updated with missing sections!");
        Ok(())
    }
}

