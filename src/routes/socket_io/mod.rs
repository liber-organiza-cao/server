mod admin;
mod auth;
mod message;

use crate::*;

use socketioxide::extract::*;

pub fn router(app: &app::AppState) -> socketioxide::layer::SocketIoLayer {
	let (layer, io) = socketioxide::SocketIo::builder().with_state(app.clone()).build_layer();

	io.ns("/", on_connect);

	layer
}

async fn on_connect(socket: SocketRef) {
	log::info!("New connection: {}", socket.id);

	socket.on("joinChannel", message::join_channel);
	socket.on("sendMessage", message::on_send_message);
	socket.on("loadMessages", message::on_load_messages);

	socket.on("createChannel", admin::create_channel);
	socket.on("deleteChannel", admin::delete_channel);

	socket.on("requestAuthChallenge", auth::request_auth_challenge);
	socket.on("confirmAuthChallenge", auth::confirm_auth_challenge);
}
