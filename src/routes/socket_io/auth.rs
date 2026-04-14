use crate::*;

use socketioxide::extract::*;

#[derive(Debug, Clone)]
pub struct AuthenticatedData {
	#[allow(dead_code)]
	pub public_key: crypto::PublicKey,
	pub is_admin: bool,
}

#[inline]
pub fn is_admin(socket: &SocketRef) -> bool {
	if let Some(AuthenticatedData { is_admin, .. }) = socket.extensions.get::<AuthenticatedData>() {
		is_admin
	} else {
		false
	}
}

pub async fn auth_middleware(app: State<app::AppState>, socket: SocketRef, Data(token): Data<String>) -> error::Result<()> {
	let jwt_secret = app.env.jwt_secret.as_bytes();
	let data = crypto::decode_jwt::<routes::auth::AuthenticatedPayload>(jwt_secret, &token)?;

	let public_key = data.public_key;
	let is_admin = data.is_admin;

	socket.extensions.insert(AuthenticatedData { public_key, is_admin });

	Ok(())
}
