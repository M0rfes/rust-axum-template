// TODO: write integration test later;
use crate::model::user::{self, UserForAuth};
use crate::model::ModelManager;
use crate::web::error::Result;
use crate::web::set_token_cookie;
use axum::extract::State;
use axum::{extract::Query, response::Redirect, routing::get, Json, Router};
use tower_cookies::Cookies;
use vendor::gooleOauth2::{GooleOAuth2Clinet, GooleQueryParams};

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/auth/google", get(go_to_google))
		.route("/auth/google-login", get(google_cb))
		.with_state(mm)
}

async fn go_to_google(
	State(google_client): State<&'static GooleOAuth2Clinet>,
) -> Redirect {
	let auth_url = google_client.get_auth_url();
	Redirect::to(auth_url.as_str())
}

async fn google_cb(
	cookies: Cookies,
	State(mm): State<ModelManager>,
	Query(code): Query<GooleQueryParams>,
) -> Result<Redirect> {
	let code = code.code;
	let google_user = mm
		.google_oauth2_client
		.get_user(code)
		.await?;
	let user =
		user::UserRepo::upsert_from_google_user::<UserForAuth>(&mm, google_user)
			.await?;
	set_token_cookie(&cookies, &user)?;
	Ok(Redirect::to("/api/user/me"))
}
