pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
	Dotenv(dotenvy::Error),
	Var(std::env::VarError),
	ParseIntError(std::num::ParseIntError),
	Io(std::io::Error),
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
