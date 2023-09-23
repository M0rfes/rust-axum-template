use crate::{Error, Result};
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct Config {
	pub GOOGLE_CLIENT_ID: String,
	pub GOOGLE_CLIENT_SECRET: String,
	pub GOOLE_REDIRECT: String,
	pub PORT: u16,
	pub DB_URL: String,
}
impl Config {
	fn load_from_env() -> Result<Config> {
		Ok(Config {
			GOOGLE_CLIENT_ID: get_env("GOOGLE_CLIENT_ID")?,
			GOOGLE_CLIENT_SECRET: get_env("GOOGLE_CLIENT_SECRET")?,
			GOOLE_REDIRECT: get_env("GOOLE_REDIRECT")?,
			PORT: get_env_parse("PORT")?,
			DB_URL: get_env("DATABASE_URL")?,
		})
	}
}

fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	let val = get_env(name)?;
	val.parse::<T>()
		.map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
	base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}
