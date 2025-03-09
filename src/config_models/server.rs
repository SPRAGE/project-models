use serde::{Deserialize,Serialize};
use crate::config_models::{Config, error::ConfigError};

#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerName {
    AuthServer,
    IngestionServer,
    AnalysisServer,
    WebSocketServer,
}

impl ServerConfig {
    /// **Creates a new `ServerConfig` from `config.toml`**
    pub fn new(name: ServerName) -> Result<Self, ConfigError> {
        let config = Config::get()?;

        let servers = config.servers.as_ref().ok_or(ConfigError::MissingSection("Missing [servers] section in config.toml".to_string()))?;

        match name {
            ServerName::AuthServer => servers.auth.clone().ok_or(ConfigError::MissingSection("Missing [servers.auth] section".to_string())),
            ServerName::IngestionServer => servers.ingestion.clone().ok_or(ConfigError::MissingSection("Missing [servers.ingestion] section".to_string())),
            ServerName::AnalysisServer => servers.analysis.clone().ok_or(ConfigError::MissingSection("Missing [servers.analysis] section".to_string())),
            ServerName::WebSocketServer => servers.websocket.clone().ok_or(ConfigError::MissingSection("Missing [servers.websocket] section".to_string())),
        }
    }
}

/// **Stores the `servers` section in `config.toml`**
#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct ServersConfig {
    pub auth: Option<ServerConfig>,
    pub ingestion: Option<ServerConfig>,
    pub analysis: Option<ServerConfig>,
    pub websocket: Option<ServerConfig>,
}

