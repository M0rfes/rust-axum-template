use std::collections::BTreeMap;

use crate::{
	config::config,
	crypt::error::{Error, Result},
};
use hmac::{Hmac, Mac};
use jwt::{self, SignWithKey, VerifyWithKey};
use sha2::Sha256;

pub fn sign(claims: BTreeMap<String, String>) -> Result<String> {
	let key: Hmac<Sha256> = Hmac::new_from_slice(config().JWT_SECRET.as_bytes())
		.map_err(|_| Error::FailedToInitiliaze)?;
	let token_str = claims
		.sign_with_key(&key)
		.map_err(|_| Error::FailedToSign)?;
	Ok(token_str)
}

pub fn verify(token: String) -> Result<BTreeMap<String, String>> {
	let key: Hmac<Sha256> = Hmac::new_from_slice(config().JWT_SECRET.as_bytes())
		.map_err(|_| Error::FailedToInitiliaze)?;
	let claims: BTreeMap<String, String> = token
		.verify_with_key(&key)
		.map_err(|_| Error::FailedToVerify)?;
	Ok(claims)
}
