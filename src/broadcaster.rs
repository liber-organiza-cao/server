use crate::*;

use std::collections;
use std::ops;
use std::sync::Arc;
use tokio::sync;

pub enum MessageBroadcast {
	Foo,
}

pub type Sender = sync::broadcast::Sender<MessageBroadcast>;
pub type Receiver = sync::broadcast::Receiver<MessageBroadcast>;
pub type ChannelBrodcasterData = collections::HashMap<String, sync::broadcast::Sender<MessageBroadcast>>;

const SENDER_CAPACITY: usize = 512;

#[derive(Debug, Clone)]
pub struct ChannelBroadcaster(Arc<ChannelBrodcasterData>);

impl ChannelBroadcaster {
	#[inline]
	pub fn init(config: &config::Config) -> Self {
		let mut channels = collections::HashMap::new();
		for channel in &config.channels {
			channels.insert(channel.clone(), Sender::new(SENDER_CAPACITY));
		}

		Self(Arc::new(channels))
	}

	pub fn subscribe(&self, channel: &str) -> error::Result<Receiver> {
		match self.get(channel) {
			Some(sender) => Ok(sender.subscribe()),
			_ => Err(error::Error::ChannelDoesNotExist),
		}
	}

	pub fn send(&self, channel: &str, msg: MessageBroadcast) -> error::Result<()> {
		match self.get(channel) {
			Some(sender) => {
				let _ = sender.send(msg);
				Ok(())
			}
			_ => Err(error::Error::ChannelDoesNotExist),
		}
	}
}

impl ops::Deref for ChannelBroadcaster {
	type Target = Arc<ChannelBrodcasterData>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
