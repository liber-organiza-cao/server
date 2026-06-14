use super::*;
use crate::*;

use uuid::Uuid;

use std::fmt;
use std::ops;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Channel {
	pub id: Uuid,
	pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateChannelParams {
	pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteChannelParams {
	pub channel_id: uuid::Uuid,
}

#[derive(Debug, serde::Deserialize)]
pub struct JoinChannelParams {
	pub channel_id: ChannelIdentifier,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ChannelIdentifier(pub Uuid);

pub async fn create_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<CreateChannelParams>) -> error::Result<()> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_admin(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let channel = db::create_channel(&state.db_pool, &params.name).await?;

	app.room("events").emit("channelCreated", (channel,))?;

	Ok(())
}

pub async fn delete_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<DeleteChannelParams>) -> error::Result<()> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_admin(&socket) {
		return Err(error::Error::Unauthorized);
	}

	db::delete_channel(&state.db_pool, params.channel_id).await?;

	app.room("events").emit("channelDeleted", (params.channel_id,))?;

	Ok(())
}

pub async fn list_channels(app: wspc::App) -> error::Result<Vec<Channel>> {
	let state = app.get_state::<app::AppState>().unwrap();

	let db_channels = db::get_channels(&state.db_pool).await?;
	let channels = db_channels.into_iter().map(|c| Channel { id: c.id, name: c.name }).collect();

	Ok(channels)
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
