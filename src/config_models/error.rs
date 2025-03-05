use std::fmt;

/// **General Config Errors**
#[derive(Debug)]
pub enum ConfigError {
    FileError(String),
    ParseError(String),
    NotLoaded(String),
    AlreadyLoaded(String),
    MissingSection(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::FileError(msg) => write!(f, "File Error: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            ConfigError::NotLoaded(msg) => write!(f, "Config Not Loaded: {}", msg),
            ConfigError::AlreadyLoaded(msg) => write!(f, "Configuration already loaded: {}", msg),
            ConfigError::MissingSection(msg) => write!(f, "Missing Section: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

/// **ClickHouse-Specific Errors**
#[derive(Debug)]
pub enum ClickHouseError {
    ConfigNotLoaded,
    SectionMissing,
    InvalidConfig(String),
    MissingPassword,
}

impl fmt::Display for ClickHouseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClickHouseError::ConfigNotLoaded => write!(f, "ClickHouse config not loaded"),
            ClickHouseError::SectionMissing => write!(f, "ClickHouse section missing in config"),
            ClickHouseError::InvalidConfig(msg) => write!(f, "Invalid ClickHouse config: {}", msg),
            ClickHouseError::MissingPassword => write!(f, "Missing ClickHouse password"),
        }
    }
}

impl std::error::Error for ClickHouseError {}

/// **Kafka-Specific Errors**
#[derive(Debug)]
pub enum KafkaError {
    ConfigNotLoaded,
    SectionMissing,
    InvalidConfig(String),
}

impl fmt::Display for KafkaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KafkaError::ConfigNotLoaded => write!(f, "Kafka config not loaded"),
            KafkaError::SectionMissing => write!(f, "Kafka section missing in config"),
            KafkaError::InvalidConfig(msg) => write!(f, "Invalid Kafka config: {}", msg),
        }
    }
}

impl std::error::Error for KafkaError {}

/// **Redis-Specific Errors**
#[derive(Debug)]
pub enum RedisError {
    ConfigNotLoaded,
    SectionMissing,
    MissingDB(String),
    InvalidCredentials(String),
    ConnectionError(String),
    InvalidRedisPort(String),
}

impl fmt::Display for RedisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisError::ConfigNotLoaded => write!(f, "Redis config not loaded"),
            RedisError::SectionMissing => write!(f, "Redis section missing in config"),
            RedisError::MissingDB(msg) => write!(f, "Missing Redis database config: {}", msg),
            RedisError::InvalidCredentials(msg) => write!(f, "Invalid Redis credentials: {}", msg),
            RedisError::ConnectionError(msg) => write!(f, "Redis connection error: {}", msg),
            RedisError::InvalidRedisPort(msg) => write!(f, "Invalid Redis port: {}", msg),
        }
    }
}

impl std::error::Error for RedisError {}

#[derive(Debug)]
pub enum ZerodhaConfigError {
    MissingCredentials,
}

impl std::fmt::Display for ZerodhaConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ZerodhaConfigError::MissingCredentials => write!(f, "Zerodha credentials are missing"),
        }
    }
}

impl std::error::Error for ZerodhaConfigError {}

