pub mod app;
pub mod broadcaster;
pub mod config;
pub mod env;
pub mod error;
pub mod routes;

#[tokio::main]
async fn main() -> error::Result<()> {
	simple_logger::init_with_level(log::Level::Info).unwrap();

	let app = app::AppState::init()?;

	let listener = tokio::net::TcpListener::bind(&format!("{}:{}", app.env.server_addr, app.env.server_port)).await?;

	log::info!("Listening on {}:{}", app.env.server_addr, app.env.server_port);

	let router = routes::get_routes().with_state(app);

	Ok(axum::serve(listener, router).await?)
}
