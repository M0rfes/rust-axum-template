mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod web;

use std::net::SocketAddr;

use axum::{middleware, Router};
pub use error::{Error, Result};
use tower_cookies::{CookieManager, CookieManagerLayer};
use tracing::info;
use web::routes;

use config::config;
use tracing_subscriber::EnvFilter;

use crate::web::{mw_ctx_resolve, mw_reponse_map};

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		//.without_time() // For early local development.
		//.with_target(false)
		//.with_env_filter(EnvFilter::from_default_env())
		.init();

	let port = config().PORT;

	let addr = SocketAddr::from(([127, 0, 0, 1], port));
	info!("{:<12} - http://{addr}\n", "LISTENING");
	let mm = model::ModelManager::new().await?;
	let route = Router::new()
		.merge(routes(mm.clone()))
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new());
	axum::Server::bind(&addr)
		.serve(route.into_make_service())
		.await
		.unwrap();

	Ok(())
}

fn _add() {
	print!("add called");
}
