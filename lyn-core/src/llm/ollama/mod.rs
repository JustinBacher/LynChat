mod config;

use async_trait::async_trait;
use rig::{
    completion::{CompletionModel, CompletionRequest, CompletionResponse},
    providers::ollama::Client as OllamaClient,
    streaming::{StreamingCompletionModel, StreamingResult},
};

use crate::llm::{LLMError, LLMProvider};
use crate::prelude::*;
pub use config::OllamaProviderConfig;

pub struct OllamaProvider {
    client: OllamaClient,
    model: String,
}

impl OllamaProvider {
    pub async fn new(_endpoint: &str, model: String) -> Result<Self> {
        let client = OllamaClient::new();

        Ok(Self { client, model })
    }
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn generate(&self, prompt: &CompletionRequest) -> Result<CompletionResponse<String>> {
        self.client
            .completion_model(self.model())
            .completion(prompt)
            .await
    }

    async fn generate_stream(&self, prompt: &CompletionRequest) -> Result<StreamingResult> {
        self.client
            .completion_model(self.model())
            .stream(prompt)
            .await
    }

    async fn get_models(&self) -> Result<Vec<String>> {
        reqwest::get("http://127.0.0.1:11434/v1/models")
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(|e| LLMError::Other(e.to_string()))
    }
}
