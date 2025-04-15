mod config;
mod error;
mod ollama;

use std::{pin::Pin, sync::Arc};

use crate::{config::AppConfig, prelude::*};
pub use config::{LLMConfig, LLMProviders, VectorDbConfig};
pub use error::LLMError;
use futures::Stream;
pub use ollama::{OllamaProvider, OllamaProviderConfig};
use rig::{
    completion::{CompletionRequest, Document},
    message::Message,
};

pub enum EmbeddingType {
    Text(String),
    Document(Document),
}

#[async_trait::async_trait]
pub trait LLMProvider: Send + Sync {
    fn model(&self) -> &str;

    fn embedding_model(&self) -> Option<&str>;

    /// Sends a prompt to the LLM and returns the generated response.
    async fn generate(&self, prompt: CompletionRequest) -> Result<String>;

    /// Sends a prompt to the LLM and returns a stream of response chunks.
    async fn generate_stream(
        &self,
        prompt: CompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>>;

    // TODO: May remove this later in favor of hard coding models
    async fn get_models(&self) -> Result<Vec<String>>;

    async fn generate_embedding(&self, to_embed: EmbeddingType) -> Result<()>;

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

pub fn create_llm_provider(config: Arc<AppConfig>) -> Result<Arc<dyn LLMProvider>> {
    match config.provider {
        LLMProviders::Ollama => Ok(Arc::new(OllamaProvider::new(config)?)),
    }
}
