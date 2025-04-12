//! Implementation of the LLMProvider trait for Ollama.
mod config;

use futures_util::{Stream, StreamExt};
use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage, MessageRole}, // Removed ToolCall import
        embeddings::request::GenerateEmbeddingsRequest,
    },
    Ollama,
};
use std::{any::Any, pin::Pin};
use url::Url;

use crate::{
    llm::{LLMError, LLMProvider, LLMResponse, Prompt},
    prelude::*,
};
pub use config::OllamaConfig;
// Ensure serde_json::Value is removed if not used elsewhere

/// Client for interacting with an Ollama instance.
#[derive(Debug, Clone)]
pub struct OllamaClient {
    client: Ollama,
    default_model: String,
}

impl OllamaClient {
    pub fn new(config: &OllamaConfig) -> Result<Self> {
        let url = &config.url;

        let parsed_url: Url = url.parse().map_err(|e| {
            LLMError::Configuration(f!("Invalid Ollama base URL format for {}: {}", url, e))
        })?;

        let client = Ollama::new(parsed_url.to_string(), config.port);

        let default_model = config.model.to_owned();

        Ok(Self {
            client,
            default_model,
        })
    }

    /// Returns a clone of the underlying Ollama client instance.
    pub fn client(&self) -> Ollama {
        self.client.clone()
    }

    /// Returns the default model name configured for this client.
    pub fn default_model(&self) -> &str {
        &self.default_model
    }

    /// Generates embeddings for the given text using the specified model.
    pub async fn generate_embedding(&self, text: &str, model: &str) -> Result<Vec<f32>> {
        let request = GenerateEmbeddingsRequest::new(
            model.to_string(),
            ollama_rs::generation::embeddings::request::EmbeddingsInput::Single(text.to_string()),
        );
        let response = self
            .client
            .generate_embeddings(request)
            .await
            .map_err(|e| LLMError::Api(f!("Ollama embedding API error: {}", e)))?;

        // Extract the first embedding vector (assuming Vec<f64>) and convert its elements to f32
        let embeddings = response.embeddings; // Assumed Vec<Vec<f64>>

        if let Some(first_embedding_f32) = embeddings.into_iter().next() {
            Ok(first_embedding_f32)
        } else {
            // Handle the case where the embeddings vector was empty
            Err(LLMError::Api("Ollama returned no embeddings".to_string()).into())
        }
    }

    /// Converts internal Message role to Ollama MessageRole.
    fn map_role(role: &str) -> Result<MessageRole> {
        // Return local Result
        match role.to_lowercase().as_str() {
            "system" => Ok(MessageRole::System),
            "user" => Ok(MessageRole::User),
            "assistant" => Ok(MessageRole::Assistant),
            _ => Err(LLMError::Parsing(f!("Unknown message role: {}", role)).into()),
        }
    }
}

#[async_trait::async_trait]
impl LLMProvider for OllamaClient {
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Sends a prompt (chat history) to the Ollama API and returns the response.
    async fn generate(&self, prompt: &Prompt) -> Result<LLMResponse> {
        if prompt.messages.is_empty() {
            return Err(LLMError::Configuration(
                "Prompt must contain at least one message".to_string(),
            )
            .into());
        }

        // Get the model name from the client's default or return error
        let model = &self.default_model;

        // Map internal messages to Ollama chat messages
        // Use try_collect or a loop with ? for cleaner error handling if mapping fails
        let messages: Vec<ChatMessage> = prompt
            .messages
            .iter()
            .map(|msg| {
                let role = Self::map_role(&msg.role)?;
                Ok(ChatMessage::new(role, msg.content.clone()))
            })
            .collect::<Result<Vec<_>>>()?; // Collect into Result<Vec<_>, LLMError>

        let chat_req = ChatMessageRequest::new(model.clone(), messages);

        // Send request to Ollama
        let response = self
            .client
            .send_chat_messages(chat_req)
            .await
            .map_err(|e| LLMError::Api(f!("Ollama API error: {}", e)))?;

        // Extract the response content - assuming response.message is the ChatMessage
        let content = response.message.content; // Directly access content

        Ok(LLMResponse { content })
    }

    /// Sends a prompt to Ollama and returns a stream of response chunks.
    /// Note: Streaming with tool calls might require specific handling depending on the provider.
    async fn generate_stream(
        &self,
        prompt: &Prompt,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        // Explicit type for clarity
        if prompt.messages.is_empty() {
            return Err(LLMError::Configuration(
                "Prompt must contain at least one message".to_string(),
            )
            .into());
        }

        let model = &self.default_model;

        let messages: Vec<ChatMessage> = prompt
            .messages
            .iter()
            .map(|msg| {
                let role = Self::map_role(&msg.role)?;
                let mut message = ChatMessage::new(role, msg.content.clone());
                Ok(message)
            })
            .collect::<Result<Vec<_>>>()?;

        let chat_req = ChatMessageRequest::new(model.clone(), messages);

        // Get the stream from the ollama-rs client
        let stream = self
            .client
            .send_chat_messages_stream(chat_req)
            .await
            .map_err(|e| LLMError::Api(f!("Ollama API stream error: {}", e)))?;

        // Map the stream items to Result<String>
        let mapped_stream = stream.map(|res| match res {
            Ok(chat_response) => {
                // Extract the content chunk from the response message
                // Assuming chat_response.message contains the chunk
                Ok(chat_response.message.content)
            }
            Err(e) => {
                // Format the error message separately using Debug format
                let error_message = format!("Ollama stream item error: {:?}", e);
                Err(LLMError::Api(error_message).into())
            }
        });

        // Box and pin the stream, explicitly typing the stream
        let output_stream: Pin<Box<dyn Stream<Item = Result<String>> + Send>> =
            Box::pin(mapped_stream);
        Ok(output_stream)
    }

    async fn get_models(&self) -> Result<Vec<String>> {
        unimplemented!()
    }
}
