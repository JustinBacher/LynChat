//! Core application logic.

use crate::{
    config::{self, AppConfig},
    llm::{LLMError, LLMProvider, LLMProviders, Message, OllamaClient, Prompt}, // LLMProvider might be removed if only Ollama used
    memory::{MemoryClient, qdrant::QdrantMemoryClient, summarizer::summarize_interaction},
    prelude::*,
    // Import the tool functions directly
    tools::{calculator::Calculator, datetime::DateTime},
};
use futures_util::{Stream, StreamExt, TryStreamExt};
use ollama_rs::{
    coordinator::Coordinator,
    generation::chat::{ChatMessage, MessageRole}, // Removed tools::Tool import
};
use std::{pin::Pin, sync::Arc};

// Removed old constants and ToolCallRequest struct

#[derive(Clone)]
pub struct Engine {
    #[allow(dead_code)] // Keep config for potential future use
    config: Arc<AppConfig>,
    // Store OllamaClient directly for Coordinator usage
    ollama_client: Arc<OllamaClient>,
    #[allow(dead_code)] // TODO: Remove this once memory client is implemented
    memory_client: Arc<dyn MemoryClient>,
    // Removed tools field - tools will be added to Coordinator dynamically
}

impl Engine {
    /// Creates a new instance of the Engine.
    ///
    /// Loads configuration, initializes Ollama client, memory client,
    /// and collects tools compatible with ollama-rs.
    pub async fn new() -> Result<Self> {
        info!("Initializing Lyn Engine...");

        // --- Load Configuration ---
        let app_config = Arc::new(config::load_config()?);
        info!("Configuration loaded successfully.");
        debug!("Loaded config: {:?}", app_config);

        // --- Initialize Ollama Client ---
        // Assuming Ollama is the only provider for now
        if app_config.provider != LLMProviders::Ollama {
            // Or handle other providers if logic is added later
            return Err(Error::Config(crate::config::ConfigError::ValidationError(
                "Configuration Error: Only Ollama provider is currently supported for tool usage."
                    .to_string(),
            )));
        }
        let ollama_client = Arc::new(OllamaClient::new(&app_config.provider_configs.ollama)?);
        info!("Ollama Client initialized.");

        // --- Initialize Memory (Qdrant) Client ---
        let memory_client = Arc::new(QdrantMemoryClient::new(&app_config.vector_db).await?);
        info!("Memory client (Qdrant) initialized.");

        // Tools are no longer stored in Engine, they will be added to Coordinator in process_prompt

        Ok(Self {
            config: app_config,
            ollama_client,
            memory_client,
            // tools field removed
        })
    }

    /// Processes a user prompt, handling potential tool calls using Ollama Coordinator,
    /// and returning the final response.
    pub async fn process_prompt(&self, user_prompt: &str) -> Result<String> {
        trace!("Engine processing prompt: '{}'", user_prompt);

        // TODO: Implement proper chat history management for the Coordinator
        let history: Vec<ChatMessage> = vec![]; // Start with empty history for now

        // Prepare the user message
        let user_message = ChatMessage::user(user_prompt.to_string());

        // Get model name using the getter
        let model_name = self.ollama_client.default_model().to_string();

        // Instantiate Coordinator using the client getter
        let mut coordinator = Coordinator::new(self.ollama_client.client(), model_name, history);

        // Add tool functions directly (as shown in ollama-rs examples)
        coordinator = coordinator.add_tool(Calculator);
        coordinator = coordinator.add_tool(DateTime);

        // TODO: Add configuration for Coordinator options (e.g., num_ctx from ModelOptions)
        // coordinator = coordinator.options(...)

        // Run the chat interaction via the Coordinator
        debug!("Sending prompt to Coordinator with tools...");
        let response = coordinator
            .chat(vec![user_message]) // Send only the new user message
            .await
            .map_err(|e| Error::LLM(LLMError::Api(f!("Coordinator chat error: {}", e))))?; // Map error

        let response_content = response.message.content;
        debug!("Received final response from Coordinator: {}", response_content);

        // Call summarization (temporary location)
        self.generate_summary_internal(user_prompt, &response_content)
            .await?;

        Ok(response_content)
    }

    // --- Temporary Internal Helper Methods (to be refactored) ---

    /// Internal helper to generate summaries. (Adapted from previous free function)
    /// TODO: Move this logic to MemoryClient implementation (LYN-Refactor-Step-2)
    async fn generate_summary_internal( // Removed generic S
        &self,
        user_prompt: &str, // Use &str directly
        llm_response: &str, // Use &str directly
    ) -> Result<String> {
        trace!("Generating summary for interaction");
        // Pass OllamaClient as &dyn LLMProvider
        summarize_interaction(
            &*self.ollama_client, // This coerces to &dyn LLMProvider
            user_prompt,          // Pass &str directly
            llm_response,         // Pass &str directly
        )
        .await
    }

    /// Processes a user prompt, handling potential tool calls using Ollama Coordinator,
    /// and returning the final response as a stream.
    pub async fn process_prompt_stream(
        &self,
        user_prompt: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        trace!("Engine processing prompt (stream): '{}'", user_prompt);

        // TODO: Implement proper chat history management for the Coordinator
        let history: Vec<ChatMessage> = vec![]; // Start with empty history for now

        // Prepare the user message
        let user_message = ChatMessage::user(user_prompt.to_string());

        // Get model name using the getter
        let model_name = self.ollama_client.default_model().to_string();

        // Instantiate Coordinator using the client getter
        // Need mutable coordinator to manage history internally during streaming
        let mut coordinator = Coordinator::new(self.ollama_client.client(), model_name, history);

        // Add tool functions directly
        coordinator = coordinator.add_tool(Calculator);
        coordinator = coordinator.add_tool(DateTime);

        // TODO: Add configuration for Coordinator options
        // coordinator = coordinator.options(...)
        // TODO: Add debug flag from config
        // coordinator = coordinator.debug(true);

        // Call the new chat_stream method on the Coordinator
        debug!("Sending prompt to Coordinator for streaming with tools...");
        let stream = coordinator.chat_stream(vec![user_message]).await; // Pass new message

        // Map the stream from Result<ChatCompletionChunk> to Result<String>
        let result_stream = stream
            .map_err(|e| Error::LLM(LLMError::Api(format!("Coordinator stream error: {}", e))))
            .and_then(|result| async move {
                let chunk = result?;
                Ok(chunk.message.content)
            });

        // TODO: Implement summarization for streamed responses if needed

        Ok(Box::pin(result_stream))
    }
}
