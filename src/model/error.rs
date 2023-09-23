use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	EntityNotFound {
		entity: &'static str,
		field: &'static str,
		value: &'static str,
	},
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

	Store(store::Error),
}

impl From<sqlx::Error> for Error {
	fn from(val: sqlx::Error) -> Self {
		Self::Sqlx(val)
	}
}

impl From<store::Error> for Error {
	fn from(val: store::Error) -> Self {
		Self::Store(val)
	}
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
