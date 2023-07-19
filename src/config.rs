use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	base_url: String,
    local_username: String,
    password: String,
    token: Option<String>,
}
impl Config {
	pub fn new() -> io::Result(Self) {
		let contents = fs::read_to_string("config.toml")?;
		let config: Config = toml::from_str(&contents)
			.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
		Ok(config)
	}
	pub fn save(&self) -> io::Result(()) {
		let toml = toml::to_string(self)
			.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
		fs::write("config.toml", toml)
	}
}
