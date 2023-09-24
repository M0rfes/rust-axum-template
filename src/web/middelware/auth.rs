use crate::crypt::token;
use crate::ctx::Ctx;
use crate::model::user::{UserForAuth, UserRepo};
use crate::model::ModelManager;
use crate::web::error::{Error, Result};
use crate::web::{set_token_cookie, AUTH_TOKEN};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
	TokenNotInCookie,
	TokenWrongFormat,

	UserNotFound,
	ModelAccessError(String),
	FailValidate,
	CannotSetTokenCookie,

	CtxNotInRequestExt,
	CtxCreateFail(String),
	SubMissingInJWT,
	InvaliedSub,
}

async fn ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
	// -- Get Token String
	let jwt = cookies
		.get(AUTH_TOKEN)
		.map(|c| c.value().to_string())
		.ok_or(CtxExtError::TokenNotInCookie)?;

	// -- Parse Token
	let jwt = token::verify(jwt).map_err(|_| CtxExtError::FailValidate)?;
	let id = jwt
		.get("sub")
		.ok_or(CtxExtError::SubMissingInJWT)?
		.parse::<i64>()
		.map_err(|_| CtxExtError::InvaliedSub)?;
	// -- Get UserForAuth
	let user = UserRepo::find_by_id::<UserForAuth>(&mm, id)
		.await
		.map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?;

	// -- Update Token
	set_token_cookie(cookies, &user)
		.map_err(|_| CtxExtError::CannotSetTokenCookie)?;

	// -- Create CtxExtResult
	Ctx::new(user.id).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!("{:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<CtxExtResult>()
			.ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
			.clone()
			.map_err(Error::CtxExt)
	}
}

pub async fn mw_ctx_resolve<B>(
	mm: State<ModelManager>,
	cookies: Cookies,
	mut req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

	let ctx_ext_result = ctx_resolve(mm, &cookies).await;

	if ctx_ext_result.is_err()
		&& !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie))
	{
		cookies.remove(Cookie::named(AUTH_TOKEN))
	}

	// Store the ctx_ext_result in the request extension
	// (for Ctx extractor).
	req.extensions_mut()
		.insert(ctx_ext_result);

	Ok(next.run(req).await)
}

pub async fn mw_ctx_require<B>(
	ctx: Result<Ctx>,
	req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}
