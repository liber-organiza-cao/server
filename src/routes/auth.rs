use crate::*;

use std::time;

use axum::extract::*;
use axum::routing::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RequestAuthChallenge {
	public_key: crypto::PublicKey,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ResponseAuthChallenge {
	token: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RequestConfirmAuthChallenge {
	token: String,
	signature: crypto::Signature,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ResponseConfirmAuthChallenge {
	token: String,
	payload: AuthenticatedPayload,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ChallengePayload {
	public_key: crypto::PublicKey,
	nonce: [u8; 32],
	exp: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthenticatedPayload {
	pub public_key: crypto::PublicKey,
	pub is_admin: bool,
	pub exp: u64,
}

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new()
		.route("/challenge/request", post(request_challenge))
		.route("/challenge/confirm", post(confirm_challenge))
}

async fn request_challenge(app: State<app::AppState>, Json(payload): Json<RequestAuthChallenge>) -> error::Result<Json<ResponseAuthChallenge>> {
	let nonce = crypto::rand32();
	let public_key = payload.public_key;

	let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)?;
	let exp = now.as_secs() + app.env.jwt_challenge_expiration_seconds;

	let challenge_payload = ChallengePayload { public_key, nonce, exp };
	let token = crypto::encode_jwt(app.env.jwt_challenge_secret.as_bytes(), &challenge_payload)?;

	Ok(Json(ResponseAuthChallenge { token }))
}

async fn confirm_challenge(app: State<app::AppState>, Json(payload): Json<RequestConfirmAuthChallenge>) -> error::Result<Json<ResponseConfirmAuthChallenge>> {
	let token = payload.token;
	let signature = payload.signature;
	let token_hash = crypto::sha256(token.as_bytes());
	let challenge_payload = crypto::decode_jwt::<ChallengePayload>(app.env.jwt_challenge_secret.as_bytes(), &token)?;
	let public_key = challenge_payload.public_key;

	let valid = crypto::verify_ecdsa(public_key, signature, token_hash).unwrap_or(false);

	if !valid {
		return Err(error::Error::Unauthorized);
	}

	let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)?;
	let exp = now.as_secs() + app.env.jwt_expiration_seconds;

	let is_admin = false; // todo: check if public key is in admin list
	let payload = AuthenticatedPayload { public_key, is_admin, exp };
	let token = crypto::encode_jwt(app.env.jwt_secret.as_bytes(), &payload)?;

	Ok(Json(ResponseConfirmAuthChallenge { token, payload }))
}
