use axum::extract::State;
use axum::routing::get;
use axum::{middleware, Json, Router};

use crate::model::user::{User, UserRepo};
use crate::web::{middelware::auth, Result};
use crate::{ctx::Ctx, model::ModelManager};

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/user/me", get(me))
		.layer(middleware::from_fn(auth::mw_ctx_require))
		.with_state(mm)
}

async fn me(ctx: Result<Ctx>, State(mm): State<ModelManager>) -> Result<Json<User>> {
	let id = ctx?.user_id();
	let user = UserRepo::find_by_id(&mm, id).await?;
	Ok(Json(user))
}
