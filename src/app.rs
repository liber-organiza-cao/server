use crate::*;

#[derive(Clone)]
pub struct AppState {
	pub env: env::Env,
	pub config: config::Config,
	pub channels: broadcaster::ChannelBroadcaster,
}

impl AppState {
	pub fn init() -> error::Result<Self> {
		let env = env::Env::init()?;
		let config = config::Config::init(&env)?;
		let channels = broadcaster::ChannelBroadcaster::init(&config);

		Ok(Self { env, config, channels })
	}
}
