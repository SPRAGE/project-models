use serde::{Deserialize,Serialize};
use crate::config_models::error::ClickHouseError;

#[derive(Debug, Deserialize, Clone,Serialize)]
pub struct ClickHouseConfig {
    pub url: String,
    pub user: String,
    pub password: Option<String>,
    pub database: String,
}

#[derive(Clone, Debug)]
pub enum ClickHouseUserType {
    Read,
    Write,
}

impl ClickHouseConfig {
    /// Load ClickHouse configuration from the centralized `Config`
    pub fn from_config(config: &crate::config_models::Config, user_type: ClickHouseUserType) -> Result<Self, ClickHouseError> {
        let clickhouse_config = config.clickhouse.clone().ok_or(ClickHouseError::SectionMissing)?;

        let (user, password) = match user_type {
            ClickHouseUserType::Read => (clickhouse_config.user, None), // Read user doesn't need a password
            ClickHouseUserType::Write => (
                clickhouse_config.user,
                Some(clickhouse_config.password.clone().ok_or(ClickHouseError::MissingPassword)?),
            ),
        };

        Ok(Self {
            url: clickhouse_config.url,
            user,
            password,
            database: clickhouse_config.database,
        })
    }

    /// Generates a ClickHouse connection string
    pub fn connection_string(&self) -> String {
        let password = self.password.as_deref().unwrap_or("");
        format!(
            "tcp://{}:{}@{}?database={}",
            self.user, password, self.url, self.database
        )
    }
}

