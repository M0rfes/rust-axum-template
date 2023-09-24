use serde::Serialize;
use sqlb::{Fields, HasFields};
use sqlx::{postgres::PgRow, FromRow};
use vendor::gooleOauth2::GoogleUser;

use super::ModelManager;
use crate::model::error::{Error, Result};

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
	pub id: i64,
	pub email: Option<String>,
	pub verified_email: Option<bool>,
	pub name: Option<String>,
	pub given_name: Option<String>,
	pub family_name: Option<String>,
	pub picture: Option<String>,
	pub locale: Option<String>,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct UserForAuth {
	pub id: i64,
}

pub trait UserBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send + std::fmt::Debug
{
}

impl UserBy for User {}
impl UserBy for GoogleUser {}
impl UserBy for UserForAuth {}

pub struct UserRepo;

impl Into<UserForAuth> for User {
	fn into(self) -> UserForAuth {
		UserForAuth { id: self.id }
	}
}

impl UserRepo {
	pub async fn upsert_from_google_user<E>(
		mm: &ModelManager,
		google_user: GoogleUser,
	) -> Result<E>
	where
		E: UserBy,
	{
		let db = mm.db();
		let email = google_user.email.clone();
		let user_in_db = sqlb::select()
			.table("user")
			.columns(E::field_names())
			.and_where("email", "=", email)
			.fetch_optional::<_, E>(db)
			.await?;

		if let Some(user) = user_in_db {
			Ok(user)
		} else {
			let fields = google_user.not_none_fields();
			let user = sqlb::insert()
				.table("user")
				.data(fields)
				.returning(E::field_names())
				.fetch_one::<_, E>(db)
				.await?;
			Ok(user)
		}
	}

	pub async fn find_by_id<E>(mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		let db = mm.db();
		let id_copy = id.clone();
		let user_in_db = sqlb::select()
			.table("user")
			.columns(E::field_names())
			.and_where("id", "=", id)
			.fetch_optional::<_, E>(db)
			.await?
			.ok_or(Error::EntityNotFound {
				entity: "user".to_string(),
				field: "id".to_string(),
				value: format!("{}", id_copy),
			})?;

		Ok(user_in_db)
	}
}
