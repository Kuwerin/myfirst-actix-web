use config::ConfigError;
use deadpool_postgres::Config as Cfg;
use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct Config{
    pub server: ServerConfig,
    pub postgres: Cfg,
}

#[derive(Deserialize)]
pub struct ServerConfig{
    pub(crate) host: String,
    pub(crate) port: i32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError>{
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}