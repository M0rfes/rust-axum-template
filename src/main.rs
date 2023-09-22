mod config;
mod ctx;
mod error;
mod model;
mod web;

pub use self::error::{Error, Result};

use config::config;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time() // For early local development.
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let port = config().PORT;

	Ok(())
}

fn _add() {
	print!("add called");
}
