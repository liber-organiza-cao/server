use super::*;
use crate::*;

use uuid::Uuid;

const MESSAGE_PAGE_SIZE: i64 = 32;

#[derive(Debug, serde::Deserialize)]
pub struct SendMessageParams {
	pub content: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoadMessagesParams {
	pub before_id: Option<Uuid>,
}

pub async fn send_message(app: wspc::App, socket: wspc::Socket, params: wspc::Params<SendMessageParams>) -> error::Result<()> {
	if !auth::is_auth(&socket) {
		return Err(error::Error::Unauthorized);
	}
	let state = app.get_state::<app::AppState>().unwrap();

	let Some(channel) = socket.get_state::<channel::ChannelIdentifier>() else {
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

	let Some(channel) = socket.get_state::<channel::ChannelIdentifier>() else {
		return Err(error::Error::NotInChannel);
	};
	let messages = db::get_messages(&state.db_pool, channel.0, params.before_id, MESSAGE_PAGE_SIZE).await.unwrap_or_default();

	Ok(messages)
}
