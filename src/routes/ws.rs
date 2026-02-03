use crate::*;

use axum::extract::*;
use axum::routing::*;
use axum::*;

pub fn router() -> axum::Router<()> {
	axum::Router::new().route("/", any(handler))
}

pub async fn handler(ws: WebSocketUpgrade) -> error::Result<response::Response> {
	Ok(ws.on_upgrade(move |socket| handle_socket(socket)))
}

async fn handle_socket(mut _socket: ws::WebSocket) {}
