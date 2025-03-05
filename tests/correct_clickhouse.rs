use project_models::config_models::{
    config::Config,
    validation::ConfigValidator,
    clickhouse_config::ClickHouseConfig,
};
use project_models::config_models::error::ConfigError;
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
fn test_load_clickhouse_config() {
    let (_temp_dir, config_path) = create_temp_config(
        r#"
        [clickhouse]
        url = "127.0.0.1:9000"
        user = "admin"
        password = "securepass"
        database = "test_db"
        
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "ru"
        read_password = "rp"
        write_user = "wu"
        write_password = "wp"
        api_db = "1"
        greeks_db = "2"
        futures_db = "3"
        index_db = "4"
        
        [kafka]
        broker = "kafka_broker"
        tick_data_topic = "kafka_topic"
        
        [zerodha]
        api_key = "default_api_key"
        api_secret = "default_api_secret"
        user_name = "default_user"
        "#,
    );

    // Use the validator to auto–fix and load the config.
    match ConfigValidator::validate_and_fix(&config_path) {
        Ok(_) => {}
        Err(ConfigError::AlreadyLoaded(_)) => {} // already loaded is acceptable
        Err(e) => panic!("Unexpected error: {}", e),
    }
    let config = Config::get().unwrap();
    let clickhouse_config: ClickHouseConfig = config.clickhouse.clone().unwrap();

    assert_eq!(clickhouse_config.url, "127.0.0.1:9000");
    assert_eq!(clickhouse_config.user, "admin");
    assert_eq!(clickhouse_config.password, Some("securepass".to_string()));
    assert_eq!(clickhouse_config.database, "test_db");
}

#[test]
fn test_clickhouse_connection_string() {
    let (_temp_dir, config_path) = create_temp_config(
        r#"
        [clickhouse]
        url = "127.0.0.1:9000"
        user = "admin"
        password = "securepass"
        database = "test_db"
        
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "ru"
        read_password = "rp"
        write_user = "wu"
        write_password = "wp"
        api_db = "1"
        greeks_db = "2"
        futures_db = "3"
        index_db = "4"
        
        [kafka]
        broker = "kafka_broker"
        tick_data_topic = "kafka_topic"
        
        [zerodha]
        api_key = "default_api_key"
        api_secret = "default_api_secret"
        user_name = "default_user"
        "#,
    );
    //
    match ConfigValidator::validate_and_fix(&config_path) {
        Ok(_) => {}
        Err(ConfigError::AlreadyLoaded(_)) => {} // already loaded is acceptable
        Err(e) => panic!("Unexpected error: {}", e),
    }
    let config = Config::get().unwrap();
    let clickhouse_config: ClickHouseConfig = config.clickhouse.clone().unwrap();

    // Assuming your connection_string() generates a TCP URL like below.
    let expected_conn_str = "http://127.0.0.1:9000";
    assert_eq!(clickhouse_config.connection_string(), expected_conn_str);
}

