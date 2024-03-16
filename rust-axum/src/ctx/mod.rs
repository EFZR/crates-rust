mod error;

pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: u64,
}

// Constructor
impl Ctx {
    pub fn root_ctx() -> Self {
        Self { user_id: 0 }
    }

    pub fn new(user_id: u64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRoot)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
