use crate::config_models::error::RedisError;
use crate::config_models::Config;
use serde::{Deserialize,Serialize};

#[derive(Clone, Debug)]
pub enum RedisDBType {
    Api,
    Greeks,
    Futures,
    Index,
}

#[derive(Clone, Debug)]
pub enum RedisConnType {
    Read,
    Write,
}

#[derive(Clone, Debug,Serialize ,Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_num: String,
}

impl RedisConfig {
    /// Creates a new RedisConfig from `Config`
    pub fn new(config: &Config, db_type: RedisDBType, conn_type: RedisConnType) -> Result<Self, RedisError> {
        let redis = config.redis.as_ref().ok_or(RedisError::SectionMissing)?;

        Ok(Self {
            host: redis.host.clone(),
            port: redis.port,
            user: Self::get_user(redis, conn_type.clone())?,
            password: Self::get_password(redis, conn_type)?,
            db_num: Self::get_db_number(redis, db_type)?,
        })
    }

    /// Retrieves the correct DB number
    fn get_db_number(redis: &RedisConfigData, db_type: RedisDBType) -> Result<String, RedisError> {
        match db_type {
            RedisDBType::Api => redis.api_db.as_deref(),
            RedisDBType::Greeks => redis.greeks_db.as_deref(),
            RedisDBType::Futures => redis.futures_db.as_deref(),
            RedisDBType::Index => redis.index_db.as_deref(),
        }
        .map(ToString::to_string)
        .ok_or_else(|| RedisError::MissingDB(format!("{:?} DB not set", db_type)))
    }

    /// Retrieves the correct user based on connection type
    fn get_user(redis: &RedisConfigData, conn_type: RedisConnType) -> Result<String, RedisError> {
        match conn_type {
            RedisConnType::Read => Ok(redis.read_user.clone()),
            RedisConnType::Write => Ok(redis.write_user.clone()),
        }
    }

    /// Retrieves the correct password based on connection type
    fn get_password(redis: &RedisConfigData, conn_type: RedisConnType) -> Result<String, RedisError> {
        match conn_type {
            RedisConnType::Read => Ok(redis.read_password.clone()),
            RedisConnType::Write => Ok(redis.write_password.clone()),
        }
    }

    /// Generates a Redis connection string
    pub fn connection_string(&self) -> String {
        format!("redis://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.db_num)
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RedisConfigData {
    #[serde(default)]  // ✅ Allows default empty string if missing
    pub host: String,
    
    #[serde(default)]
    pub port: u16,

    #[serde(default)]
    pub read_user: String,
    
    #[serde(default)]
    pub read_password: String,
    
    #[serde(default)]
    pub write_user: String,
    
    #[serde(default)]
    pub write_password: String,

    #[serde(default)]
    pub api_db: Option<String>,

    #[serde(default)]
    pub greeks_db: Option<String>,

    #[serde(default)]
    pub futures_db: Option<String>,

    #[serde(default)]
    pub index_db: Option<String>,
}


