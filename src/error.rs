use std::fmt;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
	Sqlx(sqlx::error::Error),
	MigrateError(sqlx::migrate::MigrateError),
	Dotenv(dotenvy::Error),
	Var(std::env::VarError),
	ParseIntError(std::num::ParseIntError),
	Io(std::io::Error),
	SystemTime(std::time::SystemTimeError),
	Jwt(jsonwebtoken::errors::Error),
	Toml(toml::de::Error),
	SerdeJson(serde_json::Error),
	Axum(axum::Error),
	Unauthorized,
	ChannelDoesNotExist,
	IconNotFound,
}

impl From<sqlx::error::Error> for Error {
	#[inline(always)]
	fn from(value: sqlx::error::Error) -> Self {
		Self::Sqlx(value)
	}
}

impl From<sqlx::migrate::MigrateError> for Error {
	#[inline(always)]
	fn from(value: sqlx::migrate::MigrateError) -> Self {
		Self::MigrateError(value)
	}
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

impl From<std::time::SystemTimeError> for Error {
	#[inline(always)]
	fn from(value: std::time::SystemTimeError) -> Self {
		Self::SystemTime(value)
	}
}

impl From<jsonwebtoken::errors::Error> for Error {
	#[inline(always)]
	fn from(value: jsonwebtoken::errors::Error) -> Self {
		Self::Jwt(value)
	}
}

impl From<toml::de::Error> for Error {
	#[inline(always)]
	fn from(value: toml::de::Error) -> Self {
		Self::Toml(value)
	}
}

impl From<serde_json::Error> for Error {
	#[inline(always)]
	fn from(value: serde_json::Error) -> Self {
		Self::SerdeJson(value)
	}
}

impl From<axum::Error> for Error {
	#[inline(always)]
	fn from(value: axum::Error) -> Self {
		Self::Axum(value)
	}
}

impl axum::response::IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		match self {
			Self::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized"),
			Self::IconNotFound => (axum::http::StatusCode::NOT_FOUND, "Icon Not Found"),
			_ => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
		}
		.into_response()
	}
}

impl fmt::Display for Error {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "{self:?}")
	}
}
