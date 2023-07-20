use serde::{Deserialize, Serialize};
use std::{fs, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub base_url: String,
    pub local_username: String,
    pub full_username: String,
    pub password: Option<String>,
    pub token: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let contents = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
    pub fn save(&self) -> Result<(), ConfigError> {
        let toml = toml::to_string(self)?;
        fs::write("config.toml", toml)?;
        Ok(())
    }
}
