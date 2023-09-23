use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, strum_macros::AsRefStr, Serialize)]
pub enum Error {
	// TODO: added http error info
	FailedToGetGooleToken,
	FailedToGetGooleUser,
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
