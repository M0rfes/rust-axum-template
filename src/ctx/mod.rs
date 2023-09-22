mod error;
pub use self::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64,
}

impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx { user_id: 0 }
	}

	pub fn new(user_id: i64) -> Result<Self> {
		Ok(Self { user_id })
	}
}

impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}
}