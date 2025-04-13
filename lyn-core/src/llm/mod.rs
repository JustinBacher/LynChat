mod config;
mod error;
mod ollama;

use std::{pin::Pin, sync::Arc};

use crate::prelude::*;
pub use config::{LLMConfig, LLMProviders};
pub use error::LLMError;
use futures::Stream;
pub use ollama::{OllamaProvider, OllamaProviderConfig};
use rig::{completion::CompletionRequest, message::Message};

#[async_trait::async_trait]
pub trait LLMProvider: Send + Sync {
    /// Sends a prompt to the LLM and returns the generated response.
    async fn generate(&self, prompt: CompletionRequest) -> Result<String>;

    /// Sends a prompt to the LLM and returns a stream of response chunks.
    async fn generate_stream(
        &self,
        prompt: CompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>>;

    // TODO: May remove this later in favor of hard coding models
    async fn get_models(&self) -> Result<Vec<String>>;

    fn create_prompt(&self, prompt: Message) -> CompletionRequest {
        CompletionRequest {
            prompt,
            preamble: None,
            max_tokens: None,
            temperature: None,
            chat_history: vec![],
            documents: vec![],
            tools: vec![],
            additional_params: None,
        }
    }
}

pub fn create_llm_provider(
    provider: &LLMProviders,
    config: &LLMConfig,
) -> Result<Arc<dyn LLMProvider>> {
    match provider {
        LLMProviders::Ollama => Ok(Arc::new(OllamaProvider::new(
            &config.ollama.url,
            config.ollama.model.to_owned(),
        )?)),
    }
}
