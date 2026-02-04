use std::fs;

use crate::*;

use axum::extract::*;
use axum::routing::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ServerInfoResponse {
	title: String,
}

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new().route("/", get(get_info_route)).route("/icon", get(get_icon_route))
}

async fn get_info_route(app: State<app::AppState>) -> Json<ServerInfoResponse> {
	Json(ServerInfoResponse { title: app.config.title.clone() })
}

async fn get_icon_route(app: State<app::AppState>) -> error::Result<axum::response::Response> {
	let icon_path = &app.config.icon_path;
	if !icon_path.exists() {
		return Err(error::Error::IconNotFound);
	}
	let mime_type = match mime_guess::from_path(icon_path).first_raw() {
		Some(mime) => mime,
		None => return Err(error::Error::IconNotFound),
	};
	let data = fs::read(&icon_path)?;
	let response = axum::response::Response::builder().header("Content-Type", mime_type).body(axum::body::Body::from(data)).unwrap();

	Ok(response)
}
