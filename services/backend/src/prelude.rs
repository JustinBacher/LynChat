pub use crate::error::Error;

pub use std::result::Result as StdResult;

#[allow(dead_code)]
pub type Result<T> = StdResult<T, Error>;
