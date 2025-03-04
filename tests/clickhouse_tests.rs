use project_models::config_models::{Config, clickhouse_config::{ClickHouseConfig, ClickHouseUserType}};
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

#[test]
fn test_load_clickhouse_config_read_user() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    let toml_content = r#"
    [clickhouse]
    url = "127.0.0.1:9000"
    database = "test_db"
    user = "readonly"
    password = "readonlypass"
    "#;

    let mut file = File::create(&config_path).unwrap();
    file.write_all(toml_content.as_bytes()).unwrap();

    Config::load(&config_path).unwrap();
    let config = Config::get().unwrap();
    let clickhouse_config = ClickHouseConfig::from_config(&config, ClickHouseUserType::Read).unwrap();

    assert_eq!(clickhouse_config.url, "127.0.0.1:9000");
    assert_eq!(clickhouse_config.database, "test_db");
    assert_eq!(clickhouse_config.user, "readonly");
    assert_eq!(clickhouse_config.password, None);
}

