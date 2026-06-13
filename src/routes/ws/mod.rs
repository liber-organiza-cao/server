mod admin;
mod auth;
mod life_cycle;
mod message;

use crate::*;

use axum::routing::*;

pub fn router(state: &app::AppState) -> MethodRouter<app::AppState> {
	let (route, app) = wspc::App::build_route();

	app.set_state(state.clone());

	app.on("connect", life_cycle::connect);

	app.on("auth", auth::auth);
	app.on("requestChallenge", auth::request_challenge);
	app.on("confirmChallenge", auth::confirm_challenge);

	app.on("joinChannel", message::join_channel);
	app.on("sendMessage", message::send_message);
	app.on("loadMessages", message::load_messages);

	app.on("createChannel", admin::create_channel);
	app.on("deleteChannel", admin::delete_channel);

	route
}
