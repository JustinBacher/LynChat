mod config;
mod error;
mod ollama;

use std::sync::Arc;

use rig::{
    completion::{CompletionRequest, CompletionResponse},
    streaming::StreamingResult,
};

use crate::prelude::*;
pub use config::{LLMConfig, LLMProviders};
pub use error::LLMError;
pub use ollama::{OllamaProvider, OllamaProviderConfig};

#[async_trait::async_trait]
pub trait LLMProvider: Send + Sync {
    /// Sends a prompt to the LLM and returns the generated response.
    async fn generate(&self, prompt: &CompletionRequest) -> Result<CompletionResponse<String>>;

    /// Sends a prompt to the LLM and returns a stream of response chunks.
    async fn generate_stream(&self, prompt: &CompletionRequest) -> Result<StreamingResult>;

    // TODO: May remove this later in favor of hard coding models
    async fn get_models(&self) -> Result<Vec<String>>;
}

pub fn create_llm_provider(
    provider: &LLMProviders,
    config: &LLMConfig,
) -> Result<Arc<dyn LLMProvider>> {
    match provider {
        LLMProviders::Ollama => Ok(Arc::new(OllamaProvider::new(
            &config.ollama.url,
            config.ollama.model,
        )?)),
    }
}
