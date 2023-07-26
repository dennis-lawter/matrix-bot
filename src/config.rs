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
    pub fn new(config_filename: &str) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(config_filename)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
    pub fn save(&self, config_filename: &str) -> Result<(), ConfigError> {
        let toml = toml::to_string(self)?;
        fs::write(config_filename, toml)?;
        Ok(())
    }

    pub fn get_profile_url(&self) -> String {
        build_profile_url(self.base_url.as_str(), self.full_username.as_str())
    }

    pub fn get_login_url(&self) -> String {
        build_login_url(self.base_url.as_str())
    }
}

pub fn build_profile_url(base_url: &str, username: &str) -> String {
    format!("{}/_matrix/client/r0/profile/{}", base_url, username).to_owned()
}

pub fn build_login_url(base_url: &str) -> String {
    format!("{}/_matrix/client/r0/login", base_url).to_owned()
}
