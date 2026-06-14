use crate::*;

use axum::extract::*;
use axum::routing::*;

#[derive(Debug, Clone, serde::Serialize)]
struct ServerInfoResponse {
	title: String,
}

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new().route("/", get(get_info_route))
}

async fn get_info_route(app: State<app::AppState>) -> error::Result<Json<ServerInfoResponse>> {
	let title = app.config.title.clone();

	Ok(Json(ServerInfoResponse { title }))
}
