pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
	Dotenv(dotenvy::Error),
	Var(std::env::VarError),
	ParseIntError(std::num::ParseIntError),
	Io(std::io::Error),
	Toml(toml::de::Error),
	ChannelDoesNotExist,
}

impl From<dotenvy::Error> for Error {
	#[inline(always)]
	fn from(value: dotenvy::Error) -> Self {
		Self::Dotenv(value)
	}
}

impl From<std::env::VarError> for Error {
	#[inline(always)]
	fn from(value: std::env::VarError) -> Self {
		Self::Var(value)
	}
}

impl From<std::num::ParseIntError> for Error {
	#[inline(always)]
	fn from(value: std::num::ParseIntError) -> Self {
		Self::ParseIntError(value)
	}
}

impl From<std::io::Error> for Error {
	#[inline(always)]
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}

impl From<toml::de::Error> for Error {
	#[inline(always)]
	fn from(value: toml::de::Error) -> Self {
		Self::Toml(value)
	}
}

impl axum::response::IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		match self {
			_ => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
		}
		.into_response()
	}
}
