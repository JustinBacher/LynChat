mod config;

use std::pin::Pin;

use async_std::stream::StreamExt;
pub use config::OllamaProviderConfig;
use futures::Stream;

use crate::{
    llm::{LLMError, LLMProvider},
    prelude::*,
};
use rig::{
    completion::{CompletionModel, CompletionRequest},
    message::AssistantContent,
    providers::ollama::Client as OllamaClient,
    streaming::{StreamingChoice, StreamingCompletionModel},
};

pub struct OllamaProvider {
    client: OllamaClient,
    model: String,
}

impl OllamaProvider {
    pub fn new(_endpoint: &str, model: String) -> Result<Self> {
        let client = OllamaClient::new();

        Ok(Self { client, model })
    }
}

#[async_trait::async_trait]
impl LLMProvider for OllamaProvider {
    async fn generate(&self, request: CompletionRequest) -> Result<String> {
        self.client
            .completion_model(&self.model)
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
            .completion_model(&self.model)
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

    async fn get_models(&self) -> Result<Vec<String>> {
        reqwest::get("http://127.0.0.1:11434/v1/models")
            .await?
            .json::<Vec<String>>()
            .await
            .map_err(|e| Error::LLM(LLMError::Other(e.to_string())))
    }
}
