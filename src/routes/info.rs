use crate::*;

use axum::extract::*;
use axum::routing::*;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize)]
struct ChannelInfo {
	id: Uuid,
	name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct ServerInfoResponse {
	title: String,
	channels: Vec<ChannelInfo>,
}

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new().route("/", get(get_info_route))
}

async fn get_info_route(app: State<app::AppState>) -> error::Result<Json<ServerInfoResponse>> {
	let title = app.config.title.clone();
	let channels = db::get_channels(&app.db_pool).await?;
	let channels = channels.into_iter().map(|c| ChannelInfo { id: c.id, name: c.name }).collect();

	Ok(Json(ServerInfoResponse { title, channels }))
}
