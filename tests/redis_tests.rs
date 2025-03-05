#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;
    use project_models::config_models::{
        Config,
        redis_config::{RedisConfig, RedisDBType, RedisConnType}
    };
    use project_models::config_models::error::RedisError;

    /// Helper function to create a temporary config file and keep the directory alive
    fn create_temp_config(content: &str) -> (TempDir, std::path::PathBuf) {
        let temp_dir = tempfile::tempdir().unwrap();  // ✅ Keeps temp directory alive
        let config_path = temp_dir.path().join("config.toml");

        let mut file = File::create(&config_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        (temp_dir, config_path)  // ✅ Return both dir handle and file path
    }

    #[test]
    fn test_load_redis_config() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "readonly_user"
        read_password = "readonlypass"
        write_user = "write_user"
        write_password = "writepass"
        api_db = "1"
        "#);

        Config::load(&config_path).unwrap();
        let config = Config::get().unwrap();
        let redis_config = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Read).unwrap();

        assert_eq!(redis_config.host, "127.0.0.1");
        assert_eq!(redis_config.port, 6379);
        assert_eq!(redis_config.user, "readonly_user");
        assert_eq!(redis_config.password, "readonlypass");
        assert_eq!(redis_config.db_num, "1");
    }

    #[test]
    fn test_missing_redis_section() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [redis]
        host = ""  # ✅ Minimal valid field
        port = 0   # ✅ Prevents TOML parse error
        [clickhouse]
        url = "127.0.0.1:9000"
        "#);

        Config::load(&config_path).unwrap();
        let config = Config::get().unwrap();

        let result = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Read);
        assert!(matches!(result, Err(RedisError::SectionMissing)));
    }

    #[test]
    fn test_missing_db_number() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "readonly_user"
        read_password = "readonlypass"
        write_user = "write_user"
        write_password = "writepass"
        "#); // ✅ Still missing API DB

        Config::load(&config_path).unwrap();
        let config = Config::get().unwrap();

        let result = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Read);
        assert!(matches!(result, Err(RedisError::MissingDB(_))));
    }

    #[test]
    fn test_load_redis_config_read_user() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "readonly_user"
        read_password = "readonlypass"
        write_user = "write_user"
        write_password = "writepass"
        api_db = "1"
        "#);

        Config::load(&config_path).unwrap();
        let config = Config::get().unwrap();
        let redis_config = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Read).unwrap();

        assert_eq!(redis_config.host, "127.0.0.1");
        assert_eq!(redis_config.port, 6379);
        assert_eq!(redis_config.user, "readonly_user");
        assert_eq!(redis_config.password, "readonlypass");
        assert_eq!(redis_config.db_num, "1");
    }

    #[test]
    fn test_load_redis_config_write_user() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "readonly_user"
        read_password = "readonlypass"
        write_user = "write_user"
        write_password = "writepass"
        api_db = "1"
        "#);

        Config::load(&config_path).unwrap();
        let config = Config::get().unwrap();
        let redis_config = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Write).unwrap();

        assert_eq!(redis_config.host, "127.0.0.1");
        assert_eq!(redis_config.port, 6379);
        assert_eq!(redis_config.user, "write_user");
        assert_eq!(redis_config.password, "writepass");
        assert_eq!(redis_config.db_num, "1");
    }

    #[test]
    fn test_redis_connection_string() {
        let (_temp_dir, config_path) = create_temp_config(r#"
        [redis]
        host = "127.0.0.1"
        port = 6379
        read_user = "readonly_user"
        read_password = "readonlypass"
        write_user = "write_user"
        write_password = "writepass"
        api_db = "1"
        "#);

        Config::load(&config_path).unwrap();
        let config = Config::get().unwrap();
        let redis_config = RedisConfig::new(&config, RedisDBType::Api, RedisConnType::Write).unwrap();

        let expected_conn_str = "redis://write_user:writepass@127.0.0.1:6379/1";
        assert_eq!(redis_config.connection_string(), expected_conn_str);
    }
}

