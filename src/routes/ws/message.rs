use super::*;
use crate::*;

use std::fmt;
use std::ops;
use uuid::Uuid;

const MESSAGE_PAGE_SIZE: i64 = 32;

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ChannelIdentifier(Uuid);

#[derive(Debug, serde::Deserialize)]
pub struct JoinChannelParams {
	pub channel_id: ChannelIdentifier,
}

#[derive(Debug, serde::Deserialize)]
pub struct SendMessageParams {
	pub content: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoadMessagesParams {
	pub before_id: Option<Uuid>,
}

pub async fn join_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<JoinChannelParams>) -> error::Result<db::Channel> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_auth(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let channel = db::get_channel(&state.db_pool, *params.channel_id).await?;

	if let Some(channel) = socket.get_state::<ChannelIdentifier>() {
		socket.leave(channel)?;
	};

	socket.join(params.channel_id)?;
	socket.set_state(params.channel_id);

	Ok(channel)
}

pub async fn send_message(app: wspc::App, socket: wspc::Socket, params: wspc::Params<SendMessageParams>) -> error::Result<()> {
	if !auth::is_auth(&socket) {
		return Err(error::Error::Unauthorized);
	}
	let state = app.get_state::<app::AppState>().unwrap();

	let Some(channel) = socket.get_state::<ChannelIdentifier>() else {
		return Err(error::Error::NotInChannel);
	};

	let message = db::create_message(&state.db_pool, channel.0, &params.content).await?;
	app.room(channel).emit("messageReceived", (message,))?;

	Ok(())
}

pub async fn load_messages(app: wspc::App, socket: wspc::Socket, params: wspc::Params<LoadMessagesParams>) -> error::Result<Vec<db::Message>> {
	if !auth::is_auth(&socket) {
		return Err(error::Error::Unauthorized);
	}
	let state = app.get_state::<app::AppState>().unwrap();

	let Some(channel) = socket.get_state::<ChannelIdentifier>() else {
		return Err(error::Error::NotInChannel);
	};
	let messages = db::get_messages(&state.db_pool, channel.0, params.before_id, MESSAGE_PAGE_SIZE).await.unwrap_or_default();

	Ok(messages)
}

impl fmt::Display for ChannelIdentifier {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl ops::Deref for ChannelIdentifier {
	type Target = Uuid;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
