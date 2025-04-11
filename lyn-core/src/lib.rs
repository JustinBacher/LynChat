// Declare modules as public to expose them
pub mod config;
pub mod core;
pub mod llm;
pub mod memory;
pub mod prelude;
pub mod tools;
pub mod utils; // Added utils module

// Re-export key types or functions for easier use by clients if needed
// pub use prelude::Result;
// pub use core::Engine; // Example if an Engine struct is defined in core

// Note: The actual initialization logic (loading config, creating clients,
// registering tools, etc.) should be encapsulated within functions or structs
// exposed by the public modules (e.g., in `core` or `config`).
// The client application (e.g., lyn-cli) will call these functions.
