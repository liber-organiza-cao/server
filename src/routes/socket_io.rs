use crate::*;

use socketioxide::extract::*;
use socketioxide::*;

const MESSAGE_PAGE_SIZE: i64 = 32;

pub fn router(app: &app::AppState) -> socketioxide::layer::SocketIoLayer {
	let (layer, io) = socketioxide::SocketIo::builder().with_state(app.clone()).build_layer();

	io.ns("/", on_connect);

	layer
}

async fn on_connect(socket: SocketRef) {
	log::info!("New connection: {}", socket.id);

	socket.on("sendMessage", on_send_message);
	socket.on("loadMessages", on_load_messages);
}

async fn on_send_message(io: SocketIo, Data(content): Data<String>, State(app): State<app::AppState>) {
	if let Ok(message) = db::create_message(&app.db_pool, &content).await {
		let _ = io.emit("messageReceived", &message).await;
	}
}

async fn on_load_messages(State(app): State<app::AppState>, Data(before_id): Data<Option<i64>>, ack: AckSender) {
	let messages = db::get_messages(&app.db_pool, before_id, MESSAGE_PAGE_SIZE).await.unwrap_or_default();

	let _ = ack.send(&messages);
}
