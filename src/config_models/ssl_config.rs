use serde::{Deserialize,Serialize};
use std::path::Path;
use crate::config_models::{Config, error::ConfigError};

#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct SslConfig {
    pub cert_path: String,
    pub key_path: String,
}

impl SslConfig {
    /// **Loads SSL configuration from `config.toml`**
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::get()?;

        let ssl_config = config.ssl.as_ref()
            .ok_or_else(|| ConfigError::MissingSection("Missing [ssl] section in config.toml".to_string()))?;

        // Validate paths exist
        if !Path::new(&ssl_config.cert_path).exists() {
            return Err(ConfigError::FileError(format!(
                "Certificate file not found at {}",
                ssl_config.cert_path
            )));
        }

        if !Path::new(&ssl_config.key_path).exists() {
            return Err(ConfigError::FileError(format!(
                "Private key file not found at {}",
                ssl_config.key_path
            )));
        }

        Ok(ssl_config.clone())
    }
}

