//! Event handling for the TUI

pub mod handler;
pub mod stream;

pub use handler::handle_events;
pub use stream::StreamEvent;
