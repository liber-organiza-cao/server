use crate::*;

use std::collections;
use std::fs;
use std::path;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
	#[serde(default)]
	pub title: String,
	#[serde(default)]
	pub channels: collections::HashSet<String>,
}

impl Config {
	pub fn init(env: &env::Env) -> error::Result<Self> {
		let config_path = path::PathBuf::from(&env.config_path);

		if !config_path.exists() {
			let default_values = toml::to_string_pretty(&Config::default()).unwrap();

			log::info!("Configuration file does not exist, creating one in {}", env.config_path);
			fs::write(&config_path, default_values)?;
		}

		Ok(toml::from_str(&fs::read_to_string(&config_path)?)?)
	}
}
