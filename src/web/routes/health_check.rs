use axum::{routing::any, Json, Router};
use serde_json::{json, Value};

use crate::web::error::Result;

pub fn routes() -> Router {
	Router::new().route("/health-check", any(health_check))
}

async fn health_check() -> Result<Json<Value>> {
	Ok(Json(json!({
		"status":"ok"
	})))
}
