use crate::*;

use rand::TryRng;
use rand::rngs::SysRng;
use sha2::Digest;
use socketioxide::extract::*;

#[derive(Debug, Clone)]
enum AuthState {
	Authenticating { public_key: Box<[u8]>, secret: Box<[u8]> },
	Authenticated { _public_key: Box<[u8]> },
}

pub async fn request_auth_challenge(socket: SocketRef, Data(public_key): Data<Box<[u8]>>, ack: AckSender) {
	let secret = generate_secret();

	socket.extensions.insert(AuthState::Authenticating { public_key, secret: secret.clone() });

	let _ = ack.send(&secret);
}

pub async fn confirm_auth_challenge(Data(signature): Data<Box<[u8]>>, socket: SocketRef, ack: AckSender) {
	if let Some(AuthState::Authenticating { public_key, secret }) = socket.extensions.get::<AuthState>() {
		let valid = verify_signature(&public_key, &secret, &signature).unwrap_or(false);

		if valid {
			socket.extensions.insert(AuthState::Authenticated { _public_key: public_key });
		} else {
			socket.extensions.remove::<AuthState>();
		}

		let _ = ack.send(&valid);
	} else {
		let _ = ack.send(&false);
	}
}

fn generate_secret() -> Box<[u8]> {
	let mut secret = [0u8; 32];

	SysRng.try_fill_bytes(&mut secret).unwrap();

	sha2::Sha256::digest(&secret).as_slice().to_vec().into_boxed_slice()
}

fn verify_signature(public_key: &[u8], secret: &[u8], signature: &[u8]) -> error::Result<bool> {
	let secp = secp256k1::Secp256k1::verification_only();

	let pub_key = secp256k1::PublicKey::from_slice(&public_key)?;
	let signature = secp256k1::ecdsa::Signature::from_compact(&signature)?;
	let msg = secp256k1::Message::from_digest((*secret).try_into().unwrap());

	Ok(secp.verify_ecdsa(msg, &signature, &pub_key).is_ok())
}
