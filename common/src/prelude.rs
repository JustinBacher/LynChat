pub use crate::error::Error;
pub use std::result::Result as StdResult;
pub use std::io::Result as IoResult;

// Generic Result type using the application's Error type
pub type Result<T> = StdResult<T, Error>;

// Geneneric Wrapper type
#[derive(Debug, Clone)]
pub struct W<T>(pub T);

// Common standard library rewordings
pub use std::eprintln as eprint;
pub use std::format as f;
pub use std::println as print;

// Common tracing functions
pub use tracing::{
    Level, debug, debug_span, error, error_span, info, info_span, span, trace, warn,
};
