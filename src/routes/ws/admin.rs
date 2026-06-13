use super::*;
use crate::*;

#[derive(Debug, serde::Deserialize)]
pub struct CreateChannelParams {
	pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteChannelParams {
	pub channel_id: uuid::Uuid,
}

pub async fn create_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<CreateChannelParams>) -> error::Result<()> {
	if !auth::is_admin(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let state = app.get_state::<app::AppState>().unwrap();
	let channel = db::create_channel(&state.db_pool, &params.name).await?;
	app.room("events").emit("channelCreated", (channel,))?;

	Ok(())
}

pub async fn delete_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<DeleteChannelParams>) -> error::Result<()> {
	if !auth::is_admin(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let state = app.get_state::<app::AppState>().unwrap();

	db::delete_channel(&state.db_pool, params.channel_id).await?;
	app.room("events").emit("channelDeleted", (params.channel_id,))?;

	Ok(())
}
