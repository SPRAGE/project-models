use serde::{Deserialize, Serialize};
use crate::config_models::error::KafkaError;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct KafkaConfig {
    pub broker: String,
    pub tick_data_topic: String,
}

impl KafkaConfig {
    /// Loads the Kafka configuration from the centralized `Config`.
    /// This assumes that the centralized configuration (e.g. from your TOML file)
    /// has an optional field for Kafka, which you can add to your `Config` struct.
    pub fn from_config(config: &crate::config_models::Config) -> Result<Self, KafkaError> {
        config.kafka.clone().ok_or(KafkaError::SectionMissing)
    }
}

