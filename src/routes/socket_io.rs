use socketioxide::extract::*;

pub fn router() -> socketioxide::layer::SocketIoLayer {
	let (layer, io) = socketioxide::SocketIo::new_layer();

	io.ns("/", async |socket: SocketRef| {
		log::info!("New connection: {}", socket.id);
		socket.on("sendMessage", on_send_message);
	});

	layer
}

async fn on_send_message(s: SocketRef, Data(content): Data<String>) {
	let _ = s.emit("messageReceived", &content);
}
