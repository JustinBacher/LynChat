mod config;
mod error;
mod ollama;

use serde::{Deserialize, Serialize};
// Removed serde_json::Value import

use crate::prelude::*;
pub use config::{LLMConfig, LLMProviders};
pub use error::LLMError;
pub use ollama::{OllamaClient, OllamaConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub messages: Vec<Message>,
    // Add other potential fields like temperature, max_tokens, etc. if needed universally
    // pub temperature: Option<f32>,
    // pub max_tokens: Option<u32>,
}

/// Represents the response received from an LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
    // Add other potential fields like token usage, finish reason, etc.
    // pub token_usage: Option<TokenUsage>,
    // pub finish_reason: Option<String>,
}

// --- LLM Provider Trait ---

/// A trait defining the interface for interacting with different LLM providers.
///
/// Implementors of this trait provide the specific logic for communicating
use std::any::Any; // Import Any

/// with a particular LLM service (e.g., Ollama, OpenAI API).
///
/// Requires `Send + Sync` because it needs to be shareable across threads
/// (e.g., when used in the TUI's background task).
/// Requires `Any` to allow downcasting to concrete types.
#[async_trait::async_trait]
pub trait LLMProvider: Send + Sync + Any {
    /// Returns self as `Any` to allow downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Sends a prompt to the LLM and returns the generated response.
    async fn generate(&self, prompt: &Prompt) -> Result<LLMResponse>; // Reverted signature

    /// Sends a prompt to the LLM and returns a stream of response chunks.
    async fn generate_stream(
        &self,
        prompt: &Prompt,
    ) -> Result<std::pin::Pin<Box<dyn futures_util::Stream<Item = Result<String>> + Send>>>;

    async fn get_models(&self) -> Result<Vec<String>>;
}
