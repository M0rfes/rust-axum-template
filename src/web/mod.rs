use std::collections::BTreeMap;

use axum::Router;

pub mod error;
mod middelware;
mod routes;

pub use middelware::{auth::mw_ctx_resolve, res_map::mw_reponse_map};
use routes::health_check;
use tower_cookies::{Cookie, Cookies};

use self::error::Result;
use crate::{
	crypt::token,
	model::{user::UserForAuth, ModelManager},
};
pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.merge(health_check::routes())
		.nest(
			"/api",
			routes::auth::routes(mm.clone()).merge(routes::user::routes(mm)),
		)
}

pub const AUTH_TOKEN: &str = "jwt";

fn set_token_cookie(cookies: &Cookies, user: &UserForAuth) -> Result<()> {
	let mut claims = BTreeMap::new();
	claims.insert("sub".to_string(), user.id.to_string());
	let token = token::sign(claims)?;
	let mut cookie = Cookie::new(AUTH_TOKEN, token);
	cookie.set_http_only(true);
	cookie.set_path("/");
	cookies.add(cookie);

	Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
	let mut cookie = Cookie::named(AUTH_TOKEN);
	cookie.set_path("/");

	cookies.remove(cookie);

	Ok(())
}
