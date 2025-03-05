use project_models::config_models::{
    config::Config,
    error::ClickHouseError,
};
use std::fs;
use tempfile::TempDir;

/// Helper function to create a temporary config file.
fn create_temp_config(content: &str) -> (TempDir, std::path::PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, content).unwrap();
    (temp_dir, config_path)
}

#[test]
fn test_missing_clickhouse_section() {
    // Create a TOML that omits the [clickhouse] section.
    let (_temp_dir, config_path) = create_temp_config(
        r#"
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

    // Load the config without auto‑fixing.
    Config::load(&config_path).unwrap();
    let config = Config::get().unwrap();
    // Since the [clickhouse] section is missing, we expect a SectionMissing error.
    let result = config.clickhouse.clone().ok_or(ClickHouseError::SectionMissing);
    assert!(matches!(result, Err(ClickHouseError::SectionMissing)));
}

