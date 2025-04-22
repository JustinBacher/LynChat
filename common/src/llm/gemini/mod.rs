pub mod config;

use std::{pin::Pin, sync::Arc};

use async_std::stream::StreamExt;
use futures::Stream;

use rig::{
    completion::{CompletionModel, CompletionRequest},
    embeddings::EmbeddingsBuilder,
    message::AssistantContent,
    providers::gemini::Client as GeminiClient,
    streaming::{StreamingChoice, StreamingCompletionModel},
};

use super::EmbeddingType;
use crate::{
    config::AppConfig,
    llm::{LLMError, LLMProvider},
    prelude::*,
};
pub use config::GeminiProviderConfig;

pub struct GeminiProvider {
    client: GeminiClient,
    config: Arc<AppConfig>,
}

impl GeminiProvider {
    pub fn new(config: Arc<AppConfig>) -> Result<Self> {
        let client = GeminiClient::new(
            &config
                .provider_configs
                .gemini
                .as_ref()
                .ok_or_else(|| LLMError::Configuration(String::from("Missing Gemini Config")))?
                .api_key,
        );

        Ok(Self { client, config })
    }
}

#[async_trait::async_trait]
impl LLMProvider for GeminiProvider {
    fn model(&self) -> &str {
        self.config
            .provider_configs
            .gemini
            .as_ref()
            .map(|g| g.api_key.as_str())
            .unwrap_or("gemini-2.0-flash-lite")
    }

    fn embedding_model(&self) -> Option<&str> {
        Some(
            &self
                .config
                .embedding_provider_configs
                .gemini
                .as_ref()
                .map(|g| g.embedding_model.as_str())
                .unwrap_or("gemini-pro-embedding"),
        )
    }

    async fn generate(&self, request: CompletionRequest) -> Result<String> {
        self.client
            .completion_model(&self.model())
            .completion(request)
            .await
            .map(|response| match response.choice.first() {
                AssistantContent::Text(choice) => choice.text,
                AssistantContent::ToolCall(tool_call) => tool_call.function.name,
            })
            .map_err(|e| Error::LLM(LLMError::Api(e.to_string())))
        }

    async fn generate_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let stream = self
            .client
            .completion_model(&self.model())
            .stream(request)
            .await
            .map(|stream| {
                stream.map(|c| match c {
                    Ok(StreamingChoice::Message(choice)) => Ok(choice),
                    Ok(StreamingChoice::ToolCall(name, _id, _params)) => Ok(name),
                    Err(e) => Err(Error::LLM(LLMError::Response(e))),
                })
            })
            .map_err(|e| Error::LLM(LLMError::Api(e.to_string())))?;
        Ok(Box::pin(stream) as Pin<Box<dyn Stream<Item = Result<String>> + Send>>)
    }

    async fn generate_embedding(&self, to_embed: EmbeddingType) -> Result<()> {
        let model = self.client.embedding_model(&self.model());
        let embedder = EmbeddingsBuilder::new(model.clone());
        match to_embed {
            EmbeddingType::Text(text) => {
                if let Err(e) = embedder.document(text.to_string()) {
                    error!("Embedding error: {}", e);
                }
            }
            EmbeddingType::Document(document) => {
                if let Err(e) = embedder.document(document.to_string()) {
                    error!("Embedding error: {}", e);
                }
            }
        }
        Ok(())
    }

    async fn get_models(&self) -> Result<Vec<String>> {
        reqwest::get("http://127.0.0.1:11434/v1/models")
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(|e| Error::LLM(LLMError::Other(e.to_string())))
    }
}
