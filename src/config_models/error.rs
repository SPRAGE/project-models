#[derive(Debug)]
pub enum ConfigError {
    FileError(String),
    ParseError(String),
    NotLoaded(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileError(msg) => write!(f, "File error: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::NotLoaded(msg) => write!(f, "Config not loaded: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}
#[derive(Debug)]
pub enum ClickHouseError {
    ConfigNotLoaded,
    SectionMissing,
    MissingPassword,
}

impl std::fmt::Display for ClickHouseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickHouseError::ConfigNotLoaded => write!(f, "ClickHouse config not loaded"),
            ClickHouseError::SectionMissing => write!(f, "ClickHouse section missing in config file"),
            ClickHouseError::MissingPassword => write!(f, "Write user requires a password"),
        }
    }
}

impl std::error::Error for ClickHouseError {}


/// Kafka-specific errors
#[derive(Debug)]
pub enum KafkaError {
    ConfigNotLoaded,
    SectionMissing,
}

impl std::fmt::Display for KafkaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KafkaError::ConfigNotLoaded => write!(f, "Kafka config not loaded"),
            KafkaError::SectionMissing => write!(f, "Kafka section missing in config file"),
        }
    }
}

impl std::error::Error for KafkaError {}

/// Redis-specific errors
#[derive(Debug)]
pub enum RedisError {
    ConfigNotLoaded,
    SectionMissing,
}

impl std::fmt::Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisError::ConfigNotLoaded => write!(f, "Redis config not loaded"),
            RedisError::SectionMissing => write!(f, "Redis section missing in config file"),
        }
    }
}

impl std::error::Error for RedisError {}

