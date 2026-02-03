use crate::*;

use axum::extract::*;
use axum::routing::*;
use axum::*;

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new().route("/", any(handler))
}

pub async fn handler(app: State<app::AppState>, ws: WebSocketUpgrade) -> error::Result<response::Response> {
	Ok(ws.on_upgrade(move |socket| handle_socket(app, socket)))
}

async fn handle_socket(app: State<app::AppState>, mut _socket: ws::WebSocket) {
	let Ok(_channel) = app.channels.subscribe("foo") else {
		return;
	};
	app.channels.send("foo", broadcaster::MessageBroadcast::Foo).unwrap();
}
