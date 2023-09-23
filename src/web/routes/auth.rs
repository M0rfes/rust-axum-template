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
) -> Result<Json<GoogleUser>> {
	let code = code.code;
	let google_user = mm
		.google_oauth2_client
		.get_user(code)
		.await?;
	// TODO: save user in db;
	// TODO: signe jwt;
	// TODO: add jwt to header;
	// TODO: Redirect to ui;
	Ok(Json(google_user))
}
