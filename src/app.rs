use crate::*;

#[derive(Clone)]
pub struct AppState {
	pub env: env::Env,
	pub config: config::Config,
}

impl AppState {
	pub fn init() -> error::Result<Self> {
		let env = env::Env::init()?;
		let config = config::Config::init(&env)?;

		Ok(Self { env, config })
	}
}
