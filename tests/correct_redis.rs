// tests/correct_redis.rs

use project_models::config_models::{
    config::Config,
    redis_config::{RedisConfig, RedisDBType, RedisConnType},
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper function to create a temporary config file.
fn create_temp_config(content: &str) -> (TempDir, std::path::PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, content).unwrap();
    (temp_dir, config_path)
}

#[test]
fn test_load_redis_config_read_user() {
    let (_temp_dir, config_path) = create_temp_config(
        r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "readonly_user"
        read_password = "readonlypass"
        write_user = "write_user"
        write_password = "writepass"
        api_db = "1"
        greeks_db = "2"
        futures_db = "3"
        index_db = "4"
        
        [clickhouse]
        url = "127.0.0.1:9000"
        user = "admin"
        password = "securepass"
        database = "test_db"
        
        [kafka]
        broker = "kafka_broker"
        tick_data_topic = "kafka_topic"
        
        [zerodha]
        api_key = "default_api_key"
        api_secret = "default_api_secret"
        user_name = "default_user"
        "#,
    );

    // Load the configuration
    Config::load(&config_path).unwrap();
    let config = Config::get().unwrap();
    // Create a RedisConfig using the READ connection type.
    let redis_config =
        RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Read).unwrap();

    assert_eq!(redis_config.host, "127.0.0.1");
    assert_eq!(redis_config.port, 6379);
    assert_eq!(redis_config.user, "readonly_user");
    assert_eq!(redis_config.password, "readonlypass");
    assert_eq!(redis_config.db_num, "1");
}

#[test]
fn test_load_redis_config_write_user() {
    let (_temp_dir, config_path) = create_temp_config(
        r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "readonly_user"
        read_password = "readonlypass"
        write_user = "write_user"
        write_password = "writepass"
        api_db = "1"
        greeks_db = "2"
        futures_db = "3"
        index_db = "4"
        
        [clickhouse]
        url = "127.0.0.1:9000"
        user = "admin"
        password = "securepass"
        database = "test_db"
        
        [kafka]
        broker = "kafka_broker"
        tick_data_topic = "kafka_topic"
        
        [zerodha]
        api_key = "default_api_key"
        api_secret = "default_api_secret"
        user_name = "default_user"
        "#,
    );

    Config::load(&config_path).unwrap();
    let config = Config::get().unwrap();
    // Create a RedisConfig using the WRITE connection type.
    let redis_config =
        RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Write).unwrap();

    assert_eq!(redis_config.host, "127.0.0.1");
    assert_eq!(redis_config.port, 6379);
    assert_eq!(redis_config.user, "write_user");
    assert_eq!(redis_config.password, "writepass");
    assert_eq!(redis_config.db_num, "1");
}

#[test]
fn test_redis_connection_string() {
    let (_temp_dir, config_path) = create_temp_config(
        r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "readonly_user"
        read_password = "readonlypass"
        write_user = "write_user"
        write_password = "writepass"
        api_db = "1"
        greeks_db = "2"
        futures_db = "3"
        index_db = "4"
        
        [clickhouse]
        url = "127.0.0.1:9000"
        user = "admin"
        password = "securepass"
        database = "test_db"
        
        [kafka]
        broker = "kafka_broker"
        tick_data_topic = "kafka_topic"
        
        [zerodha]
        api_key = "default_api_key"
        api_secret = "default_api_secret"
        user_name = "default_user"
        "#,
    );

    Config::load(&config_path).unwrap();
    let config = Config::get().unwrap();
    // Use WRITE credentials for generating the connection string.
    let redis_config =
        RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Write).unwrap();
    // Assume the connection string format is: "redis://{user}:{password}@{host}:{port}/{db_num}"
    let expected_conn_str = "redis://write_user:writepass@127.0.0.1:6379/1";
    assert_eq!(redis_config.connection_string(), expected_conn_str);
}

