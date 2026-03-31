use crate::*;

use socketioxide::extract::*;
use socketioxide::*;

pub fn router(app: &app::AppState) -> socketioxide::layer::SocketIoLayer {
	let (layer, io) = socketioxide::SocketIo::builder().with_state(app.clone()).build_layer();

	io.ns("/", async |socket: SocketRef, State(app): State<app::AppState>| {
		log::info!("New connection: {}", socket.id);

		socket.on("sendMessage", on_send_message);

		if let Ok(msgs) = db::get_messages(&app.db_pool).await {
			let msgs = msgs.into_iter().map(|m| m.content).collect::<Vec<_>>();
			let _ = socket.emit("updateMessages", &msgs);
		}
	});

	layer
}

async fn on_send_message(io: SocketIo, Data(content): Data<String>, State(app): State<app::AppState>) {
	let _ = db::create_message(&app.db_pool, &content).await;
	let _ = io.emit("messageReceived", &content).await;
}
