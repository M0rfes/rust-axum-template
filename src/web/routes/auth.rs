// TODO: write integration test later;
use crate::model::user;
use crate::model::user::User;
use crate::model::ModelManager;
use crate::web::error::Result;
use axum::extract::State;
use axum::{extract::Query, response::Redirect, routing::get, Json, Router};
use vendor::gooleOauth2::{GoogleUser, GooleOAuth2Clinet, GooleQueryParams};

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
	State(mm): State<ModelManager>,
	Query(code): Query<GooleQueryParams>,
) -> Result<Json<User>> {
	let code = code.code;
	let google_user = mm
		.google_oauth2_client
		.get_user(code)
		.await?;
	let user = user::UserRepo::upsert_from_google_user(&mm, google_user).await?;
	// TODO: signe jwt;
	// TODO: add jwt to cookie;
	// TODO: Redirect to ui;
	Ok(Json(user))
}
