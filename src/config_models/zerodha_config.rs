use serde::{Deserialize, Serialize};
use crate::config_models::error::ZerodhaConfigError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ZerodhaConfig {
    pub api_key: String,
    pub api_secret: String,
    pub user_name: String,
}

impl ZerodhaConfig {
    /// Loads the Zerodha configuration from the centralized `Config`.
    pub fn from_config(config: &crate::config_models::Config) -> Result<Self, ZerodhaConfigError> {
        config.zerodha.clone().ok_or(ZerodhaConfigError::MissingCredentials)
    }
}

