use serde::Serialize;
use crate::config_models::{
    clickhouse_config::ClickHouseConfig, 
    redis_config::RedisConfigData, 
    kafka_config::KafkaConfig,
    zerodha_config::ZerodhaConfig,

};

#[derive(Debug, Serialize)]
pub struct DefaultConfig {
    pub clickhouse: Option<ClickHouseConfig>,
    pub redis: Option<RedisConfigData>,
    pub kafka: Option<KafkaConfig>,
    pub zerodha: Option<ZerodhaConfig>,

}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            clickhouse: Some(Self::default_clickhouse()),
            redis: Some(Self::default_redis()),
            kafka: Some(Self::default_kafka()),
            zerodha: Some(Self::default_zerodha()),


        }
    }
}

impl DefaultConfig {
    pub fn default_clickhouse() -> ClickHouseConfig {
        ClickHouseConfig {
            url: "127.0.0.1:9000".to_string(),
            user: "default_user".to_string(),
            password: Some("default_password".to_string()),
            database: "default_db".to_string(),
        }
    }

    pub fn default_redis() -> RedisConfigData {
        RedisConfigData {
            host: "127.0.0.1".to_string(),
            port: 6379,
            read_user: "readonly_user".to_string(),
            read_password: "readonlypass".to_string(),
            write_user: "write_user".to_string(),
            write_password: "writepass".to_string(),
            api_db: Some("1".to_string()),
            greeks_db: Some("2".to_string()),
            futures_db: Some("3".to_string()),
            index_db: Some("4".to_string()),
        }
    }
    pub fn default_kafka() -> KafkaConfig {
        KafkaConfig {
            broker: "kafka_broker".to_string(),
            tick_data_topic: "kafka_topic".to_string(),
        }
    }
    pub fn default_zerodha() -> ZerodhaConfig {
        ZerodhaConfig {
            api_key: "API_KEY".to_string(),
            api_secret: "API_SECRET".to_string(),
            user_name: "USER_NAME".to_string(),
        }
    }   

}

