use super::*;
use crate::*;

use socketioxide::extract::*;
use socketioxide::*;
use uuid::Uuid;

pub async fn create_channel(io: SocketIo, socket: SocketRef, State(app): State<app::AppState>, Data(name): Data<String>, ack: AckSender) {
	if !auth::is_admin(&socket) {
		let _ = ack.send(&false);
		return;
	}

	match db::create_channel(&app.db_pool, &name).await {
		Ok(channel) => {
			let _ = io.emit("channelCreated", &channel).await;
			let _ = ack.send(&true);
		}
		Err(error) => {
			log::warn!("Failed to create channel: {error:?}");
			let _ = ack.send(&false);
		}
	}
}

pub async fn delete_channel(io: SocketIo, socket: SocketRef, State(app): State<app::AppState>, Data(channel_id): Data<Uuid>, ack: AckSender) {
	if !auth::is_admin(&socket) {
		let _ = ack.send(&false);
		return;
	}

	match db::delete_channel(&app.db_pool, channel_id).await {
		Ok(true) => {
			let _ = io.emit("channelDeleted", &channel_id).await;
			let _ = ack.send(&true);
		}
		Ok(false) => {
			let _ = ack.send(&false);
		}
		Err(error) => {
			log::warn!("Failed to delete channel: {error:?}");
			let _ = ack.send(&false);
		}
	}
}
