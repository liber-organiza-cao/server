use crate::*;

use std::env;

#[derive(Debug, Clone)]
pub struct Env {
	pub server_addr: String,
	pub server_port: u16,
	pub config_path: String,
}

impl Env {
	pub fn init() -> error::Result<Self> {
		let _ = dotenvy::dotenv();

		let server_addr = env::var("SERVER_ADDR")?;
		let server_port = env::var("SERVER_PORT")?.parse()?;

		let config_path = env::var("CONFIG_PATH")?;

		Ok(Self { server_addr, server_port, config_path })
	}
}
