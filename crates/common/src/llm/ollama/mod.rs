mod config;

use std::{pin::Pin, sync::Arc};

use async_std::stream::StreamExt;
use futures::Stream;

use rig::{
    completion::{CompletionModel, CompletionRequest},
    embeddings::EmbeddingsBuilder,
    message::AssistantContent,
    providers::ollama::Client as OllamaClient,
    streaming::{StreamingChoice, StreamingCompletionModel},
};

use super::EmbeddingType;
use crate::{
    config::AppConfig,
    llm::{LLMError, LLMProvider},
    prelude::*,
};
pub use config::OllamaProviderConfig;

pub struct OllamaProvider {
    client: OllamaClient,
    config: Arc<AppConfig>,
}

impl OllamaProvider {
    pub fn new(config: Arc<AppConfig>) -> Result<Self> {
        let client = OllamaClient::new();

        Ok(Self { client, config })
    }
}

#[async_trait::async_trait]
impl LLMProvider for OllamaProvider {
    fn model(&self) -> &str {
        &self.config.provider_configs.ollama.model
    }

    fn embedding_model(&self) -> Option<&str> {
        Some(
            &self
                .config
                .embedding_provider_configs
                .ollama
                .embedding_model,
        )
    }

    async fn generate(&self, request: CompletionRequest) -> Result<String> {
        self.client
            .completion_model(&self.model())
            .completion(request)
            .await
            .map_err(|e| Error::LLM(LLMError::Api(e.to_string())))
            .map(|response| match response.choice.first() {
                AssistantContent::Text(choice) => choice.text,
                // TODO: I'm just spitting out the tool name for now, need to figure out how to
                // handle this better
                AssistantContent::ToolCall(tool_call) => tool_call.function.name,
            })
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
                    // TODO: I'm just spitting out the tool name for now, need to figure out how to
                    // handle this better
                    Ok(StreamingChoice::ToolCall(name, _id, _params)) => Ok(name),
                    Err(e) => Err(Error::LLM(LLMError::Response(e))),
                })
            })
            .map_err(|e| Error::LLM(LLMError::Api(e.to_string())))?;
        Ok(Box::pin(stream) as Pin<Box<dyn Stream<Item = Result<String>> + Send>>)
    }

    async fn generate_embedding(&self, to_embed: EmbeddingType) -> Result<()> {
        let model = self.client.embedding_model(&self.model());
        match to_embed {
            EmbeddingType::Text(text) => {
                EmbeddingsBuilder::new(model.clone()).document(text.to_string());
            }
            EmbeddingType::Document(document) => {
                EmbeddingsBuilder::new(model.clone()).document(document.to_string());
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
