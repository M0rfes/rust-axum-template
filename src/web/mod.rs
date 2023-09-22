use axum::Router;

mod error;
//mod middelware;
mod routes;

use routes::health_check;

pub fn routes() -> Router {
	Router::new().merge(health_check::routes())
}
