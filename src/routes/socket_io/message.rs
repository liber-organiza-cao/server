use crate::*;

use socketioxide::extract::*;
use socketioxide::*;

const MESSAGE_PAGE_SIZE: i64 = 32;

pub async fn on_send_message(io: SocketIo, Data(content): Data<String>, State(app): State<app::AppState>) {
	if let Ok(message) = db::create_message(&app.db_pool, &content).await {
		let _ = io.emit("messageReceived", &message).await;
	}
}

pub async fn on_load_messages(State(app): State<app::AppState>, Data(before_id): Data<Option<i64>>, ack: AckSender) {
	let messages = db::get_messages(&app.db_pool, before_id, MESSAGE_PAGE_SIZE).await.unwrap_or_default();

	let _ = ack.send(&messages);
}
