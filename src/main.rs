pub mod env;
pub mod error;
pub mod routes;

#[tokio::main]
async fn main() -> error::Result<()> {
	let env = env::Env::init()?;

	let listener = tokio::net::TcpListener::bind(&format!("{}:{}", env.server_addr, env.server_port)).await?;
	let router = routes::get_routes();

	Ok(axum::serve(listener, router).await?)
}
