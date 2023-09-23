mod error;
mod store;
pub mod user;
pub use self::error::{Error, Result};
use axum::extract::FromRef;
use store::{new_db_pool, Db};
use vendor::gooleOauth2::GooleOAuth2Clinet;

#[derive(Clone, FromRef)]
pub struct ModelManager {
	pub db: Db,
	pub google_oauth2_client: &'static GooleOAuth2Clinet,
}

impl ModelManager {
	/// Constructor
	pub async fn new() -> Result<Self> {
		let cfg = crate::config();
		let db = new_db_pool().await?;
		let google_oauth2_client = GooleOAuth2Clinet::new(
			cfg.GOOGLE_CLIENT_ID.to_owned(),
			cfg.GOOGLE_CLIENT_SECRET.to_owned(),
			cfg.GOOLE_REDIRECT.to_owned(),
		);

		Ok(ModelManager {
			db,
			google_oauth2_client,
		})
	}

	/// Returns the sqlx db pool reference.
	/// (Only for the model layer)
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
