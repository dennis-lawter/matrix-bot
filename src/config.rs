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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub base_url: String,
    pub local_username: String,
    pub full_username: String,
    pub password: Option<String>,
    pub token: Option<String>,
}

impl Config {
    pub fn load(config_filename: &str) -> Result<Self, ConfigError> {
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

    pub(crate) fn get_join_url(&self, room: &str) -> String {
        build_join_url(self.base_url.as_str(), room)
    }

    pub(crate) fn get_send_message_url(&self, room: &str) -> String {
        build_send_message_url(self.base_url.as_str(), room)
    }
}

pub fn build_profile_url(base_url: &str, username: &str) -> String {
    format!("{}/_matrix/client/r0/profile/{}", base_url, username)
}

pub fn build_login_url(base_url: &str) -> String {
    format!("{}/_matrix/client/r0/login", base_url)
}

pub fn build_join_url(base_url: &str, room: &str) -> String {
    format!("{}/_matrix/client/r0/rooms/{}/join", base_url, room,)
}

pub fn build_send_message_url(base_url: &str, room: &str) -> String {
    format!(
        "{}/_matrix/client/r0/rooms/{}/send/m.room.message",
        base_url, room,
    )
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;
    use std::{fs, io::Write};
    use tempfile::NamedTempFile;

    use crate::config::{Config, ConfigError};

    const FULL_CONFIG_CONTENTS: &str = r#"
base_url = "https://example.org"
local_username = "matrix-bot"
full_username = "@matrix-bot:example.org"
password = "Plaintext password"
token = "access_token from previous api calls"
"#;
    #[tokio::test]
    async fn test_full_config_load() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        write!(temp_file, "{}", FULL_CONFIG_CONTENTS).expect("Failed to write to temporary file");

        let loaded_config = Config::load(temp_file.path().to_str().unwrap()).unwrap();

        let expected_config = Config {
            base_url: "https://example.org".to_string(),
            local_username: "matrix-bot".to_string(),
            full_username: "@matrix-bot:example.org".to_string(),
            password: Some("Plaintext password".to_string()),
            token: Some("access_token from previous api calls".to_string()),
        };

        assert_eq!(loaded_config, expected_config);
    }

    const NO_TOKEN_CONFIG_CONTENTS: &str = r#"
base_url = "https://example.org"
local_username = "matrix-bot"
full_username = "@matrix-bot:example.org"
password = "Plaintext password"
"#;
    #[tokio::test]
    async fn test_no_token_config_load() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        write!(temp_file, "{}", NO_TOKEN_CONFIG_CONTENTS)
            .expect("Failed to write to temporary file");

        let loaded_config = Config::load(temp_file.path().to_str().unwrap()).unwrap();

        let expected_config = Config {
            base_url: "https://example.org".to_string(),
            local_username: "matrix-bot".to_string(),
            full_username: "@matrix-bot:example.org".to_string(),
            password: Some("Plaintext password".to_string()),
            token: None,
        };

        assert_eq!(loaded_config, expected_config);
    }

    const NO_PASSWORD_CONFIG_CONTENTS: &str = r#"
base_url = "https://example.org"
local_username = "matrix-bot"
full_username = "@matrix-bot:example.org"
token = "access_token from previous api calls"
"#;
    #[tokio::test]
    async fn test_no_password_config_load() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        write!(temp_file, "{}", NO_PASSWORD_CONFIG_CONTENTS)
            .expect("Failed to write to temporary file");

        let loaded_config = Config::load(temp_file.path().to_str().unwrap()).unwrap();

        let expected_config = Config {
            base_url: "https://example.org".to_string(),
            local_username: "matrix-bot".to_string(),
            full_username: "@matrix-bot:example.org".to_string(),
            password: None,
            token: Some("access_token from previous api calls".to_string()),
        };

        assert_eq!(loaded_config, expected_config);
    }

    const NO_BASE_URL_CONFIG_CONTENTS: &str = r#"
local_username = "matrix-bot"
full_username = "@matrix-bot:example.org"
password = "Plaintext password"
token = "access_token from previous api calls"
"#;
    #[tokio::test]
    async fn test_fail_config_load_without_base_url() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        write!(temp_file, "{}", NO_BASE_URL_CONFIG_CONTENTS)
            .expect("Failed to write to temporary file");

        let loaded_config = Config::load(temp_file.path().to_str().unwrap());

        assert!(loaded_config.is_err());
        assert_matches!(loaded_config.unwrap_err(), ConfigError::TomlDeserialize(_));
    }

    const NO_LOCAL_USERNAME_CONFIG_CONTENTS: &str = r#"
base_url = "https://example.org"
full_username = "@matrix-bot:example.org"
password = "Plaintext password"
token = "access_token from previous api calls"
"#;
    #[tokio::test]
    async fn test_fail_config_load_without_local_username() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        write!(temp_file, "{}", NO_LOCAL_USERNAME_CONFIG_CONTENTS)
            .expect("Failed to write to temporary file");

        let loaded_config = Config::load(temp_file.path().to_str().unwrap());

        assert!(loaded_config.is_err());
        assert_matches!(loaded_config.unwrap_err(), ConfigError::TomlDeserialize(_));
    }

    const NO_FULL_USERNAME_CONFIG_CONTENTS: &str = r#"
base_url = "https://example.org"
local_username = "matrix-bot"
password = "Plaintext password"
token = "access_token from previous api calls"
"#;
    #[tokio::test]
    async fn test_fail_config_load_without_full_username() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        write!(temp_file, "{}", NO_FULL_USERNAME_CONFIG_CONTENTS)
            .expect("Failed to write to temporary file");

        let loaded_config = Config::load(temp_file.path().to_str().unwrap());

        assert!(loaded_config.is_err());
        assert_matches!(loaded_config.unwrap_err(), ConfigError::TomlDeserialize(_));
    }

    #[tokio::test]
    async fn test_config_save() {
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");

        let config = Config {
            base_url: "https://example.org".to_string(),
            local_username: "matrix-bot".to_string(),
            full_username: "@matrix-bot:example.org".to_string(),
            password: Some("Plaintext password".to_string()),
            token: Some("access_token from previous api calls".to_string()),
        };
        let save_result = config.save(temp_file.path().to_str().unwrap());
        assert!(save_result.is_ok());
        let metadata_result = fs::metadata(temp_file.path().to_str().unwrap());
        assert!(metadata_result.is_ok());
        let metadata = metadata_result.unwrap();
        assert!(metadata.is_file());
        assert!(metadata.len() > 0);
    }
}
