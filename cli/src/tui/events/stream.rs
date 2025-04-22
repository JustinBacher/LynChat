//! Stream event types and handling

/// Events that can occur during streaming responses from the engine
#[derive(Debug)]
pub enum StreamEvent {
    /// A piece of the response stream
    Chunk(String),
    /// Stream finished successfully
    End,
    /// An error occurred during streaming
    Error(String),
}
