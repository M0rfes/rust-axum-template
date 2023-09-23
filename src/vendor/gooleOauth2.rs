use std::sync::OnceLock;

use super::error::{Error, Result};

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct GooleQueryParams {
	pub code: String,
}
pub struct GooleOAuth2Clinet {
	client_id: String,
	client_secret: String,
	redirect_uri: String,
	// TODO: add scope later
}

impl GooleOAuth2Clinet {
	pub fn new(
		client_id: String,
		client_secret: String,
		redirect_uri: String,
	) -> &'static Self {
		static INSTANCE: OnceLock<GooleOAuth2Clinet> = OnceLock::new();
		INSTANCE.get_or_init(|| GooleOAuth2Clinet {
			client_id,
			client_secret,
			redirect_uri,
		})
	}
}

#[derive(Deserialize, Debug)]
struct GoogleResponseToken {
	access_token: String,
	id_token: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GoogleUser {
	pub id: String,
	pub email: String,
	pub verified_email: bool,
	pub name: String,
	pub given_name: String,
	pub family_name: String,
	pub picture: String,
	pub locale: String,
}

impl GooleOAuth2Clinet {
	pub fn get_auth_url(&self) -> String {
		let mut url =
			Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
		url.query_pairs_mut()
			.append_pair("client_id", self.client_id.as_str());
		url.query_pairs_mut()
			.append_pair("redirect_uri", self.redirect_uri.as_str());
		url.query_pairs_mut()
			.append_pair("response_type", "code");
		url.query_pairs_mut()
			.append_pair("include_granted_scopes", "true");
		url.query_pairs_mut()
			.append_pair("access_type", "online");
		url.query_pairs_mut()
			.append_pair("scope", "https://www.googleapis.com/auth/userinfo.email");
		url.as_str().to_owned()
	}

	async fn get_token(&self, code: String) -> Result<GoogleResponseToken> {
		let client = Client::new();

		let token_response = client
			.post("https://oauth2.googleapis.com/token")
			.form(&[
				("grant_type", "authorization_code"),
				(
					"redirect_uri",
					"http://localhost:3000/api/auth/google-login",
				),
				("client_id", self.client_id.as_str()),
				("client_secret", self.client_secret.as_str()),
				("code", code.as_str()),
			])
			.send()
			.await
			.map_err(|r| {
				println!("{:?}", r);
				Error::FailedToGetGooleToken
			})?
			.json::<GoogleResponseToken>()
			.await
			.map_err(|r| {
				println!("{:?}", r);
				Error::FailedToGetGooleToken
			})?;

		Ok(token_response)
	}

	pub async fn get_user(&self, code: String) -> Result<GoogleUser> {
		let token = self.get_token(code).await?;
		let client = Client::new();
		let mut url =
			Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
		url.query_pairs_mut()
			.append_pair("alt", "json");
		url.query_pairs_mut()
			.append_pair("access_token", token.access_token.as_str());

		let response = client
			.get(url)
			.bearer_auth(token.id_token.as_str())
			.send()
			.await
			.map_err(|r| {
				println!("{:?}", r);
				Error::FailedToGetGooleUser
			})?
			.json::<GoogleUser>()
			.await
			.map_err(|r| {
				println!("{:?}", r);
				Error::FailedToGetGooleUser
			})?;

		Ok(response)
	}
}
