use crate::*;

use rand::TryRng;
use rand::rngs::SysRng;
use sha2::Digest;

pub type PublicKey = secp256k1::PublicKey;
pub type Signature = secp256k1::ecdsa::Signature;

pub fn rand32() -> [u8; 32] {
	let mut secret = [0u8; 32];

	SysRng.try_fill_bytes(&mut secret).unwrap();

	secret
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
	sha2::Sha256::digest(data).as_slice().try_into().unwrap()
}

pub fn verify_ecdsa(public_key: PublicKey, signature: Signature, sha256: [u8; 32]) -> error::Result<bool> {
	let secp = secp256k1::Secp256k1::verification_only();
	let msg = secp256k1::Message::from_digest(sha256);

	Ok(secp.verify_ecdsa(msg, &signature, &public_key).is_ok())
}

pub fn encode_jwt<T: serde::ser::Serialize>(secret: &[u8], claims: &T) -> error::Result<String> {
	let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret);
	Ok(jsonwebtoken::encode(&jsonwebtoken::Header::default(), claims, &encoding_key)?)
}

pub fn decode_jwt<T: serde::de::DeserializeOwned>(secret: &[u8], token: &str) -> error::Result<T> {
	let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret);
	Ok(jsonwebtoken::decode::<T>(token, &decoding_key, &jsonwebtoken::Validation::default())?.claims)
}
