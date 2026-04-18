use crate::*;

use socketioxide::extract::*;
use socketioxide::*;
use socketioxide_core::adapter::*;
use uuid::Uuid;

const MESSAGE_PAGE_SIZE: i64 = 32;

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ChannelIdentifier(Uuid);

impl RoomParam for ChannelIdentifier {
	type IntoIter = std::iter::Once<Room>;

	fn into_room_iter(self) -> Self::IntoIter {
		std::iter::once(Room::from(self.0.to_string()))
	}
}

pub async fn join_channel(socket: SocketRef, Data(channel): Data<ChannelIdentifier>, ack: AckSender) {
	if let Some(channel) = socket.extensions.get::<ChannelIdentifier>() {
		socket.leave(channel);
	};

	socket.join(channel);
	socket.extensions.insert(channel);

	let _ = ack.send(&true);
}

pub async fn send_message(io: SocketIo, socket: SocketRef, Data(content): Data<String>, State(app): State<app::AppState>) {
	let Some(channel) = socket.extensions.get::<ChannelIdentifier>() else {
		return;
	};
	let Ok(message) = db::create_message(&app.db_pool, channel.0, &content).await else {
		return;
	};

	let _ = io.to(channel).emit("messageReceived", &message).await;
}

pub async fn load_messages(State(app): State<app::AppState>, socket: SocketRef, Data(before_id): Data<Option<Uuid>>, ack: AckSender) {
	let Some(channel) = socket.extensions.get::<ChannelIdentifier>() else {
		return;
	};
	let messages = db::get_messages(&app.db_pool, channel.0, before_id, MESSAGE_PAGE_SIZE).await.unwrap_or_default();

	let _ = ack.send(&messages);
}
