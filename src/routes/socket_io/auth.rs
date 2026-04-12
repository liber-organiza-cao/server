use crate::*;

use socketioxide::extract::*;

#[derive(Debug, Clone)]
pub struct AuthenticatedData {
	#[allow(dead_code)]
	pub public_key: Box<[u8]>,
	pub is_admin: bool,
}

#[derive(Debug, Clone)]
pub struct AuthenticatingData {
	pub public_key: Box<[u8]>,
	pub secret: [u8; 32],
}

pub async fn request_auth_challenge(socket: SocketRef, Data(public_key): Data<Box<[u8]>>, ack: AckSender) {
	let secret = crypto::generate_secret();

	socket.extensions.insert(AuthenticatingData { public_key, secret });

	let _ = ack.send(&secret.to_vec());
}

pub async fn confirm_auth_challenge(State(app): State<app::AppState>, Data(signature): Data<Box<[u8]>>, socket: SocketRef, ack: AckSender) {
	let mut is_valid = false;
	let mut is_admin = false;

	if let Some(AuthenticatingData { public_key, secret }) = socket.extensions.get::<AuthenticatingData>() {
		is_valid = crypto::verify_signature(&public_key, secret, &signature).unwrap_or(false);

		if is_valid {
			is_admin = app.config.admin_public_keys.iter().any(|key| *key == *public_key);
			socket.extensions.insert(AuthenticatedData { public_key, is_admin });
		} else {
			socket.extensions.remove::<AuthenticatedData>();
		}
	}
	let _ = ack.send(&(is_valid, is_admin));
}

#[inline]
pub fn is_admin(socket: &SocketRef) -> bool {
	if let Some(AuthenticatedData { is_admin, .. }) = socket.extensions.get::<AuthenticatedData>() {
		is_admin
	} else {
		false
	}
}
