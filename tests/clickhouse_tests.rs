
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;
    use project_models::config_models::{
        Config, clickhouse_config::ClickHouseConfig, error::ClickHouseError
    };
    use project_models::config_models::validation::ConfigValidator;

    /// ✅ Helper function to create a temporary config file.
    fn create_temp_config(content: &str) -> (TempDir, std::path::PathBuf) {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let mut file = File::create(&config_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        (temp_dir, config_path)
    }

    #[test]
    fn test_load_clickhouse_config() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [clickhouse]
        url = "127.0.0.1:9000"
        user = "admin"
        password = "securepass"
        database = "test_db"
        "#);

        ConfigValidator::validate_and_fix(&config_path).unwrap();
        Config::load(&config_path).unwrap();
        let config = Config::get().unwrap();
        let clickhouse_config = config.clickhouse.unwrap();

        assert_eq!(clickhouse_config.url, "127.0.0.1:9000");
        assert_eq!(clickhouse_config.user, "admin");
        assert_eq!(clickhouse_config.password, Some("securepass".to_string()));
        assert_eq!(clickhouse_config.database, "test_db");
    }

    #[test]
    fn test_missing_clickhouse_section() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        "#);

        ConfigValidator::validate_and_fix(&config_path).unwrap();
        Config::load(&config_path).unwrap();
        let result = Config::get_clickhouse_config();

        assert!(matches!(result, Err(ClickHouseError::SectionMissing)));
    }

    #[test]
    fn test_missing_required_fields() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [clickhouse]
        url = ""
        user = ""
        password = ""
        database = ""
        "#);

        ConfigValidator::validate_and_fix(&config_path).unwrap();
        Config::load(&config_path).unwrap();
        let result = Config::get_clickhouse_config();

        assert!(matches!(result, Err(ClickHouseError::InvalidConfig(_))));
    }

    #[test]
    fn test_clickhouse_connection_string() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [clickhouse]
        url = "127.0.0.1:9000"
        user = "admin"
        password = "securepass"
        database = "test_db"
        "#);

        ConfigValidator::validate_and_fix(&config_path).unwrap();
        Config::load(&config_path).unwrap();
        let config = Config::get().unwrap();
        let clickhouse_config = config.clickhouse.unwrap();

        let expected_conn_str = "tcp://admin:securepass@127.0.0.1:9000?database=test_db";
        assert_eq!(clickhouse_config.connection_string(), expected_conn_str);
    }
}
