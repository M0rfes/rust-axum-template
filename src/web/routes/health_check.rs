use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::Value;

pub fn routes() -> Router {
	Router::new().route("/health-check", get(health_check).post(echo))
}

async fn health_check() -> impl IntoResponse {
	"health_check"
}

async fn echo(Json(payload): Json<Value>) -> Json<Value> {
	Json(payload)
}
