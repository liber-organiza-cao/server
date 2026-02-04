mod icon;
mod info;
mod ws;

use axum::extract::*;

use crate::*;

const BODY_MAX_SIZE: usize = 1024 * 1024 * 5; // 5 MB

pub fn get_routes() -> axum::Router<app::AppState> {
	let cors = tower_http::cors::CorsLayer::new().allow_headers(tower_http::cors::Any).allow_origin(tower_http::cors::Any).allow_methods(tower_http::cors::Any);
	let body_limit = DefaultBodyLimit::max(BODY_MAX_SIZE);

	axum::Router::new().nest("/ws", ws::router()).nest("/info", info::router()).nest("/icon", icon::router()).layer(cors).layer(body_limit)
}
