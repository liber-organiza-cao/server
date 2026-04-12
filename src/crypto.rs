use crate::*;

use rand::TryRng;
use rand::rngs::SysRng;
use sha2::Digest;

pub fn generate_secret() -> [u8; 32] {
	let mut secret = [0u8; 32];

	SysRng.try_fill_bytes(&mut secret).unwrap();

	sha2::Sha256::digest(&secret).as_slice().try_into().unwrap()
}

pub fn verify_signature(public_key: &[u8], secret: [u8; 32], signature: &[u8]) -> error::Result<bool> {
	let secp = secp256k1::Secp256k1::verification_only();

	let pub_key = secp256k1::PublicKey::from_slice(&public_key)?;
	let signature = secp256k1::ecdsa::Signature::from_compact(&signature)?;
	let msg = secp256k1::Message::from_digest(secret);

	Ok(secp.verify_ecdsa(msg, &signature, &pub_key).is_ok())
}
