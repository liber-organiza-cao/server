use crate::*;

use rand::TryRng;
use rand::rngs::SysRng;
use sha2::Digest;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PublicKey(ed25519_dalek::VerifyingKey);

#[derive(Clone, PartialEq, Eq)]
pub struct PrivateKey(ed25519_dalek::SigningKey);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature(ed25519_dalek::Signature);

pub fn rand32() -> [u8; 32] {
	let mut secret = [0u8; 32];

	SysRng.try_fill_bytes(&mut secret).unwrap();

	secret
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
	sha2::Sha256::digest(data).as_slice().try_into().unwrap()
}

pub fn encode_jwt<T: serde::ser::Serialize>(secret: &[u8], claims: &T) -> error::Result<String> {
	let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret);
	Ok(jsonwebtoken::encode(&jsonwebtoken::Header::default(), claims, &encoding_key)?)
}

pub fn decode_jwt<T: serde::de::DeserializeOwned>(secret: &[u8], token: &str) -> error::Result<T> {
	let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret);
	Ok(jsonwebtoken::decode::<T>(token, &decoding_key, &jsonwebtoken::Validation::default())?.claims)
}

impl PublicKey {
	#[inline(always)]
	pub fn verify(&self, data: [u8; 32], signature: Signature) -> bool {
		self.0.verify_strict(&data, &signature.0).is_ok()
	}
	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8; 32] {
		self.0.as_bytes()
	}
}

impl PrivateKey {
	#[inline(always)]
	pub fn from_bytes(bytes: [u8; 32]) -> Self {
		Self(ed25519_dalek::SigningKey::from_bytes(&bytes))
	}
	#[inline(always)]
	pub fn to_bytes(&self) -> [u8; 32] {
		self.0.to_bytes()
	}
	#[inline(always)]
	pub fn public_key(&self) -> PublicKey {
		PublicKey(self.0.verifying_key())
	}
}

impl serde::Serialize for PublicKey {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		hex::encode(self.0.as_bytes()).serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for PublicKey {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let hex = String::deserialize(deserializer)?;

		let bytes: [u8; 32] = hex::decode(&hex)
			.map_err(serde::de::Error::custom)?
			.try_into()
			.map_err(|_| serde::de::Error::custom("invalid public key"))?;

		Ok(Self(ed25519_dalek::VerifyingKey::from_bytes(&bytes).map_err(serde::de::Error::custom)?))
	}
}

impl serde::Serialize for Signature {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		hex::encode(self.0.to_bytes()).serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for Signature {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let hex = String::deserialize(deserializer)?;

		let bytes = hex::decode(&hex)
			.map_err(serde::de::Error::custom)?
			.try_into()
			.map_err(|_| serde::de::Error::custom("invalid signature"))?;

		Ok(Self(ed25519_dalek::Signature::from_bytes(&bytes)))
	}
}

impl std::fmt::Debug for PublicKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&base32::encode(base32::Alphabet::Z, &self.0.to_bytes()))
	}
}

impl std::fmt::Debug for PrivateKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&base32::encode(base32::Alphabet::Z, &self.0.to_bytes()))
	}
}

impl std::fmt::Debug for Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&base32::encode(base32::Alphabet::Z, &self.0.to_bytes()))
	}
}
