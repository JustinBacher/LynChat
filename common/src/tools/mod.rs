//! Tool system for Lyn AI assistant based on rig's Tool capabilities.

mod calculator;
mod datetime;
mod error;
mod registry;

// Re-exports
pub use calculator::Calculator;
pub use datetime::DateTime;
pub use error::ToolError;
pub use registry::{ToolCategory, ToolRegistry};
