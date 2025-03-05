pub mod config;
pub mod validation;
pub mod defaults;
pub mod error;
pub mod clickhouse_config;
pub mod redis_config;
pub mod kafka_config;  // If needed

pub use config::Config;
pub use validation::ConfigValidator;
pub use error::{ConfigError, ClickHouseError, RedisError, KafkaError};
pub use clickhouse_config::ClickHouseConfig;
pub use redis_config::{RedisConfig, RedisConfigData, RedisDBType, RedisConnType};

