use axum::Router;

mod error;
//mod middelware;
mod middelware;
mod routes;

use routes::health_check;

use crate::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.merge(health_check::routes())
		.nest("/api", routes::auth::routes(mm))
}
