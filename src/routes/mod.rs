mod icon;
mod info;
mod socket_io;

use axum::extract::*;

use crate::*;

const BODY_MAX_SIZE: usize = 1024 * 1024 * 5; // 5 MB

pub fn get_routes(app: &app::AppState) -> axum::Router<app::AppState> {
	let cors = tower_http::cors::CorsLayer::new().allow_headers(tower_http::cors::Any).allow_origin(tower_http::cors::Any).allow_methods(tower_http::cors::Any);
	let body_limit = DefaultBodyLimit::max(BODY_MAX_SIZE);

	axum::Router::new().layer(socket_io::router(app)).nest("/info", info::router()).nest("/icon", icon::router()).layer(cors).layer(body_limit)
}
