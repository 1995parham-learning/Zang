use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    #[serde(rename = "listen-address")]
    pub listen_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
}

impl Settings {
    pub fn init() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .add_source(config::Environment::with_prefix("ZANG"))
            .build()?;

        let settings: Settings = config.try_deserialize()?;

        Ok(settings)
    }
}
