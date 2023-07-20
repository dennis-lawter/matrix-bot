use std::{io, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	pub base_url: String,
    pub local_username: String,
	pub full_username: String,
    pub password: Option<String>,
    pub token: Option<String>,
}
impl Config {
	pub fn new() -> io::Result<Self> {
		let contents = fs::read_to_string("config.toml")?;
		let config: Config = toml::from_str(&contents)
			.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
		Ok(config)
	}
	pub fn save(&self) -> io::Result<()> {
		let toml = toml::to_string(self)
			.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
		fs::write("config.toml", toml)
	}
}
