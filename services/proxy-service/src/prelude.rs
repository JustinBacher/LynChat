pub use crate::error::Error;

pub use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;
