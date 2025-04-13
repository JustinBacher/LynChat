//! Core application logic.

use std::{pin::Pin, sync::Arc};

use futures::Stream;
use rig::{
    OneOrMany,
    completion::{
        CompletionRequest, Message,
        message::{Text, UserContent},
    },
};

use crate::{
    config::{self, AppConfig},
    llm::{LLMError, LLMProvider, LLMProviders, create_llm_provider},
    memory::{MemoryClient, qdrant::QdrantMemoryClient, summarizer::summarize_interaction},
    prelude::*,
    tools::{Calculator, DateTime, ToolCategory, ToolRegistry},
};

// Removed old constants and ToolCallRequest struct

#[derive(Clone)]
pub struct Engine {
    #[allow(dead_code)] // Keep config for potential future use
    config: Arc<AppConfig>,
    // Store OllamaClient directly for Coordinator usage
    llm_client: Arc<dyn LLMProvider>,
    #[allow(dead_code)] // TODO: Remove this once memory client is implemented
    memory_client: Arc<dyn MemoryClient>,
    // Removed tools field - tools will be added to Coordinator dynamically
    // tool_registry: ToolRegistry,
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
        let llm_client = create_llm_provider(&app_config.provider, &app_config.provider_configs)?;

        // --- Initialize Memory (Qdrant) Client ---
        let memory_client = Arc::new(QdrantMemoryClient::new(&app_config.vector_db).await?);
        info!("Memory client (Qdrant) initialized.");

        // Tools are no longer stored in Engine, they will be added to Coordinator in process_prompt
        let mut tool_registry = ToolRegistry::new();

        // Register tools with their categories
        tool_registry.register(Calculator, ToolCategory::Utilities);
        tool_registry.register(DateTime, ToolCategory::Utilities);

        Ok(Self {
            config: app_config,
            llm_client,
            memory_client,
            // tools field removed
            // tool_registry,
        })
    }

    /// Processes a user prompt, handling potential tool calls using Ollama Coordinator,
    /// and returning the final response.
    pub async fn process_prompt(&self, user_prompt: &str) -> Result<String> {
        trace!("Engine processing prompt: '{}'", user_prompt);

        // let _toolset = self.tool_registry.as_rig_toolset();

        // TODO: Implement proper chat history management for the Coordinator
        let user_message = self.llm_client.create_prompt(user_prompt.into());

        // Run the chat interaction via the Coordinator
        debug!("Sending prompt to Coordinator with tools...");
        let response = self
            .llm_client
            .generate(user_message)
            .await
            .map_err(|e| Error::LLM(LLMError::Api(f!("Coordinator chat error: {}", e))))?; // Map error

        let response_content = response;

        summarize_interaction(
            &*self.llm_client, // This coerces to &dyn LLMProvider
            user_prompt,
            &response_content, // Pass &str directly
        )
        .await?;

        Ok(response_content)
    }

    /// Processes a user prompt, handling potential tool calls using Ollama Coordinator,
    /// and returning the final response as a stream.
    pub async fn process_prompt_stream(
        &self,
        user_prompt: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        trace!("Engine processing prompt (stream): '{}'", user_prompt);

        // TODO: Implement proper chat history management for the Coordinator
        let user_message = CompletionRequest {
            prompt: Message::User {
                content: OneOrMany::one(UserContent::Text(Text {
                    text: user_prompt.to_string(),
                })),
            },
            preamble: None,
            chat_history: vec![],
            documents: vec![],
            tools: vec![],
            temperature: None,
            max_tokens: None,
            additional_params: None,
        };

        let result_stream = self
            .llm_client
            .generate_stream(user_message)
            .await
            .map_err(|e| Error::LLM(LLMError::Api(f!("Coordinator chat error: {}", e))))?;

        // TODO: Implement summarization for streamed responses if needed

        Ok(result_stream)
    }
}
