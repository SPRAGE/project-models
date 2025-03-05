// tests/shared_config_tests.rs

use project_models::config_models::{
    config::Config,
    validation::ConfigValidator,
    clickhouse_config::ClickHouseConfig,
    redis_config::{RedisConfig, RedisDBType, RedisConnType},
};
use project_models::config_models::error::ConfigError;
use std::{env, path::Path};
use ctor::ctor; // Add ctor = "0.1" (or latest) in Cargo.toml under [dev-dependencies]

// This initializer runs once per test binary.
#[ctor]
fn init_config() {
    // Assume the dummy config.toml is in the project root.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let config_path = Path::new(&manifest_dir).join("config.toml");

    // Load the configuration using your validator. If it's already loaded, ignore the error.
    match ConfigValidator::validate_and_fix(&config_path) {
        Ok(()) => {},
        Err(ConfigError::AlreadyLoaded(_)) => {},
        Err(e) => panic!("Failed to initialize config: {}", e),
    }
}

#[test]
fn test_clickhouse_fields() {
    let config = Config::get().expect("Config not loaded");
    let clickhouse_config = config.clickhouse.as_ref().expect("Missing [clickhouse] section");

    assert_eq!(clickhouse_config.url, "127.0.0.1:9000");
    assert_eq!(clickhouse_config.user, "admin");
    assert_eq!(clickhouse_config.password, Some("securepass".to_string()));
    assert_eq!(clickhouse_config.database, "test_db");
}

#[test]
fn test_clickhouse_connection_string() {
    let config = Config::get().expect("Config not loaded");
    let clickhouse_config = config.clickhouse.as_ref().expect("Missing [clickhouse] section");

    // For example, assuming your connection_string() method returns:
    // "tcp://admin:securepass@127.0.0.1:9000?database=test_db"
    let expected = "tcp://admin:securepass@127.0.0.1:9000?database=test_db";
    assert_eq!(clickhouse_config.connection_string(), expected);
}

#[test]
fn test_redis_read_config() {
    let config = Config::get().expect("Config not loaded");
    // Create a RedisConfig using the READ connection type.
    let redis_config = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Read)
        .expect("Failed to create RedisConfig (read)");

    assert_eq!(redis_config.host, "127.0.0.1");
    assert_eq!(redis_config.port, 6379);
    assert_eq!(redis_config.user, "readonly_user");
    assert_eq!(redis_config.password, "readonlypass");
    assert_eq!(redis_config.db_num, "1");
}

#[test]
fn test_redis_write_config() {
    let config = Config::get().expect("Config not loaded");
    // Create a RedisConfig using the WRITE connection type.
    let redis_config = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Write)
        .expect("Failed to create RedisConfig (write)");

    assert_eq!(redis_config.host, "127.0.0.1");
    assert_eq!(redis_config.port, 6379);
    assert_eq!(redis_config.user, "write_user");
    assert_eq!(redis_config.password, "writepass");
    assert_eq!(redis_config.db_num, "1");
}

#[test]
fn test_redis_connection_string() {
    let config = Config::get().expect("Config not loaded");
    // Use WRITE credentials for generating the connection string.
    let redis_config = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Write)
        .expect("Failed to create RedisConfig (write)");
    
    // For example, assuming the connection string format is:
    // "redis://write_user:writepass@127.0.0.1:6379/1"
    let expected = "redis://write_user:writepass@127.0.0.1:6379/1";
    assert_eq!(redis_config.connection_string(), expected);
}

