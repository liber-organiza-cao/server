use crate::*;

use std::fs;
use std::path;

const DEFAULT_CONFIG: &str = include_str!("../config.toml");

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
	pub title: String,
	pub icon_path: path::PathBuf,
	#[serde_as(as = "Vec<serde_with::hex::Hex>")]
	pub admin_public_keys: Vec<[u8; 33]>,
}

impl Config {
	pub fn init(env: &env::Env) -> error::Result<Self> {
		let config_path = path::PathBuf::from(&env.config_path);

		if !config_path.exists() {
			log::info!("Configuration file does not exist, creating one in {}", env.config_path);
			fs::write(&config_path, DEFAULT_CONFIG)?;
		}

		Ok(toml::from_str(&fs::read_to_string(&config_path)?)?)
	}
}
