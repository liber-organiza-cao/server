use crate::*;

use socketioxide::extract::*;
use socketioxide::*;

const MESSAGE_PAGE_SIZE: i64 = 32;

#[derive(Debug, Clone)]
struct Channel(i64);

pub async fn join_channel(socket: SocketRef, Data(channel_id): Data<i64>, ack: AckSender) {
	socket.leave_all();
	socket.join(channel_id.to_string());
	socket.extensions.insert(Channel(channel_id));
	let _ = ack.send(&true);
}

pub async fn on_send_message(io: SocketIo, socket: SocketRef, Data(content): Data<String>, State(app): State<app::AppState>) {
	let Some(Channel(channel_id)) = socket.extensions.get::<Channel>() else {
		return;
	};
	let Ok(message) = db::create_message(&app.db_pool, channel_id, &content).await else {
		return;
	};
	let _ = io.to(channel_id.to_string()).emit("messageReceived", &message).await;
}

pub async fn on_load_messages(State(app): State<app::AppState>, socket: SocketRef, Data(before_id): Data<Option<i64>>, ack: AckSender) {
	let Some(Channel(channel_id)) = socket.extensions.get::<Channel>() else {
		return;
	};
	let messages = db::get_messages(&app.db_pool, channel_id, before_id, MESSAGE_PAGE_SIZE).await.unwrap_or_default();

	let _ = ack.send(&messages);
}
