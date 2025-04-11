//! Core application logic.

use crate::{
    config::{self, AppConfig},
    llm::{LLMError, LLMProvider, LLMProviders, Message, OllamaClient, Prompt}, // Consolidated
    memory::{MemoryClient, qdrant::QdrantMemoryClient, summarizer::summarize_interaction}, // Consolidated
    prelude::*,
    tools::{
        CalculatorTool, DISCOVER_TOOL_NAME, DateTimeTool, DiscoverTool, DiscoverToolArgs, Tool,
        ToolArgs, ToolRegistry,
    },
};
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{pin::Pin, sync::Arc}; // Added Any

// TODO: Define these constants more centrally, perhaps in config
const TOOL_CALL_MARKER: &str = "@@TOOL_CALL@@"; // Keep marker
const EMBEDDING_MODEL: &str = "nomic-embed-text"; // Or get from config
const SIMILARITY_THRESHOLD: f32 = 0.75; // Or get from config

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ToolCallRequest {
    tool_name: String,
    arguments: ToolArgs, // ToolArgs is serde_json::Value
}

#[derive(Clone)] // Clone might be useful if passing engine state around
pub struct Engine {
    #[allow(dead_code)] // TODO: Remove this once memory client is implemented
    config: Arc<AppConfig>,
    llm_provider: Arc<dyn LLMProvider>,
    #[allow(dead_code)] // TODO: Remove this once memory client is implemented
    memory_client: Arc<dyn MemoryClient>,
    tool_registry: ToolRegistry,
}

impl Engine {
    /// Creates a new instance of the Engine.
    ///
    /// Loads configuration, initializes LLM provider, memory client,
    /// and tool registry based on the loaded configuration.
    pub async fn new() -> Result<Self> {
        info!("Initializing Lyn Engine...");

        // --- Load Configuration ---
        let app_config = Arc::new(config::load_config()?);
        info!("Configuration loaded successfully.");
        debug!("Loaded config: {:?}", app_config);

        // --- Initialize LLM Provider ---
        // TODO: Make LLM provider selection configurable (Epic 1/LYN-3)
        // For now, hardcode Ollama based on LYN-4 being resolved.
        let llm_provider = Arc::new(match app_config.provider {
            LLMProviders::Ollama => OllamaClient::new(&app_config.provider_configs.ollama)?,
        });
        info!("LLM Provider (Ollama) initialized.");

        // --- Initialize Memory (Qdrant) Client ---
        let memory_client = Arc::new(QdrantMemoryClient::new(&app_config.vector_db).await?);
        info!("Memory client (Qdrant) initialized and collection checked/created.");

        // --- Initialize Tool Registry & Generate Embeddings ---
        let mut tool_registry = ToolRegistry::new();
        let tools_to_register: Vec<Arc<dyn Tool>> = vec![
            Arc::new(CalculatorTool::new()),
            Arc::new(DateTimeTool::new()),
            // Note: DiscoverTool is handled separately, not registered here
        ];

        info!(
            "Registering {} tools and generating description embeddings using '{}'...",
            tools_to_register.len(),
            EMBEDDING_MODEL
        );

        for tool in tools_to_register {
            let description = tool.description();
            debug!("Generating embedding for tool: '{}'", tool.name());
            let embedding = llm_provider
                .generate_embedding(&description, EMBEDDING_MODEL)
                .await
                .map_err(|e| {
                    error!(
                        "Failed to generate embedding for tool '{}': {}",
                        tool.name(),
                        e
                    );
                })
                .map_err(|_| LLMError::Embedding(description, String::from("")))?;
            debug!(
                "Embedding generated for tool '{}', size: {}",
                tool.name(),
                embedding.len()
            );
            tool_registry.register(tool.clone(), embedding)?; // Pass embedding
        }

        info!(
            "Tool registry initialized with {} tools.",
            tool_registry.list_tools().len() // list_tools still works
        );

        Ok(Self {
            config: app_config,
            llm_provider,
            memory_client,
            tool_registry,
        })
    }

    /// Processes a user prompt, handling potential tool calls and returning the final response.
    ///
    /// This is the main entry point for interacting with the engine.
    pub async fn process_prompt(&self, user_prompt: &str) -> Result<String> {
        trace!("Engine processing prompt: '{}'", user_prompt);

        // TODO: Add context retrieval from memory (LYN-8) before calling LLM

        // Call the internal function/method to handle LLM interaction and tool calls
        let response_content = self.process_prompt_with_tools_internal(user_prompt).await?;

        self.generate_summary_internal(user_prompt, &response_content)
            .await?;

        Ok(response_content)
    }

    /// Processes a user prompt and returns a stream of response chunks.
    ///
    /// NOTE: This currently bypasses tool checks and summarization.
    pub async fn process_prompt_stream(
        &self,
        user_prompt: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        trace!("Engine processing prompt stream: '{}'", user_prompt);

        // TODO: Add context retrieval from memory (LYN-8) before calling LLM
        // TODO: Consider how/if tools should interact with streaming

        // Format a basic prompt (similar to non-streaming, but without tool instructions initially)
        let system_prompt = "You are a helpful assistant.".to_string(); // Simplified system prompt for streaming

        let prompt = Prompt {
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
        };

        debug!("Sending stream prompt to LLM: {:?}", prompt);

        // Call the provider's stream method
        self.llm_provider.generate_stream(&prompt).await
    }

    // --- Internal Helper Methods ---

    /// Internal helper to generate summaries. (Adapted from previous free function)
    async fn generate_summary_internal<S: AsRef<str>>(
        &self,
        user_prompt: S,
        llm_response: S,
    ) -> Result<String> {
        trace!("Generating summary for interaction");

        summarize_interaction(
            &self.llm_provider,
            user_prompt.as_ref(),
            llm_response.as_ref(),
        )
        .await
    }

    /// Internal helper to process prompts, handling the tool discovery and execution flow.
    async fn process_prompt_with_tools_internal(&self, user_prompt: &str) -> Result<String> {
        trace!("Processing prompt with tool discovery: '{}'", user_prompt);

        // --- Initial LLM Call with Discover Tool Only ---

        // 1. Prepare initial prompt with only discover_tool
        let initial_tool_schemas = vec![DiscoverTool::tool_schema()]; // Only discover tool initially
        let system_prompt_1 = self.construct_system_prompt(&initial_tool_schemas);

        let prompt_1 = Prompt {
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt_1,
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
        };

        debug!(
            "Sending initial prompt to LLM (discover only): {:?}",
            prompt_1
        );
        // TODO: Update generate call if it needs to accept tool schemas explicitly
        let response_1 = self.llm_provider.generate(&prompt_1).await?;
        debug!("Received initial LLM response: {:?}", response_1.content);

        // 2. Check response for tool call marker
        if let Some(tool_call_json) = response_1.content.strip_prefix(TOOL_CALL_MARKER) {
            trace!("Detected potential tool call marker in first response.");
            let tool_call_request: ToolCallRequest = self.parse_tool_call(tool_call_json)?;
            debug!("Parsed tool call request: {:?}", tool_call_request);

            // 3. Handle the tool call (Discovery or Specific Tool - though specific shouldn't happen here)
            if tool_call_request.tool_name == DISCOVER_TOOL_NAME {
                // --- Handle Tool Discovery ---
                trace!("Handling discover_tool call.");
                let discover_args: DiscoverToolArgs =
                    serde_json::from_value(tool_call_request.arguments).map_err(|e| {
                        Error::ToolCallParseFailed(f!(
                            "Invalid arguments for discover_tool: {}. Args: '{}'",
                            e,
                            tool_call_json // Show original JSON on error
                        ))
                    })?;

                // Generate embedding for the capability description
                let ollama_client = self.get_ollama_client()?; // Helper to get typed client
                let capability_embedding = ollama_client
                    .generate_embedding(&discover_args.capability_description, EMBEDDING_MODEL)
                    .await?;

                // Find the best matching tool
                let found_tool = self
                    .tool_registry
                    .find_tool_by_capability(&capability_embedding, SIMILARITY_THRESHOLD);

                // --- Second LLM Call with Discovery Result & Potential Tool ---
                let (system_message_2, tool_schemas_2) = match found_tool {
                    Some(ref tool) => {
                        info!(
                            "Found matching tool '{}' for capability '{}'",
                            tool.name(),
                            discover_args.capability_description
                        );
                        let message = f!(
                            "Okay, I found the '{}' tool that might help with '{}'. You can use it now, or ask to discover another tool.",
                            tool.name(),
                            discover_args.capability_description
                        );
                        let schemas = vec![
                            DiscoverTool::tool_schema(),
                            self.construct_tool_schema(tool)?,
                        ];
                        (message, schemas)
                    }
                    None => {
                        info!(
                            "No suitable tool found for capability '{}'",
                            discover_args.capability_description
                        );
                        let message = f!(
                            "Sorry, I couldn't find a tool for '{}'. You can try describing the capability differently using 'discover_tool'.",
                            discover_args.capability_description
                        );
                        let schemas = vec![DiscoverTool::tool_schema()]; // Only discover tool again
                        (message, schemas)
                    }
                };

                let system_prompt_2 = self.construct_system_prompt(&tool_schemas_2);
                let prompt_2 = Prompt {
                    messages: vec![
                        // System prompt explaining available tools for this turn
                        Message {
                            role: "system".to_string(),
                            content: system_prompt_2,
                        },
                        // Original user query for context
                        Message {
                            role: "user".to_string(),
                            content: user_prompt.to_string(),
                        },
                        // Assistant's previous action (calling discover_tool) - crucial for context
                        Message {
                            role: "assistant".to_string(),
                            content: response_1.content.clone(),
                        },
                        // System message informing user about discovery result
                        Message {
                            role: "system".to_string(),
                            content: system_message_2,
                        },
                    ],
                };

                debug!(
                    "Sending second prompt to LLM (post-discovery): {:?}",
                    prompt_2
                );
                let response_2 = self.llm_provider.generate(&prompt_2).await?;
                debug!("Received second LLM response: {:?}", response_2.content);

                // 4. Process the *second* response (could be another tool call or direct answer)
                if let Some(tool_call_json_2) = response_2.content.strip_prefix(TOOL_CALL_MARKER) {
                    trace!("Detected potential tool call marker in second response.");
                    let tool_call_request_2 = self.parse_tool_call(tool_call_json_2)?;
                    debug!("Parsed second tool call request: {:?}", tool_call_request_2);

                    // Execute the specific tool found (or discover_tool again)
                    return self.execute_tool(tool_call_request_2).await;
                } else {
                    // No tool call in second response, return content
                    trace!("No tool call in second response. Returning direct LLM response.");
                    Ok(response_2.content)
                }
            } else {
                // LLM called a specific tool in the *first* response, which shouldn't happen
                // with this flow as only discover_tool was provided initially.
                warn!(
                    "LLM called specific tool '{}' unexpectedly in the first response.",
                    tool_call_request.tool_name
                );
                // We could try executing it anyway, or return an error/message.
                // Let's try executing it for now.
                return self.execute_tool(tool_call_request).await;
                // OR: return Err(anyhow!("LLM called an unavailable tool initially"));
            }
        } else {
            // No tool call in the first response, return content directly
            trace!("No tool call detected in first response. Returning direct LLM response.");
            Ok(response_1.content)
        }
    }

    /// Helper to parse tool call JSON from LLM response string.
    fn parse_tool_call(&self, tool_call_json: &str) -> Result<ToolCallRequest> {
        serde_json::from_str(tool_call_json.trim()).map_err(|e| {
            warn!(
                "Failed to parse tool call JSON: '{}'. Error: {}",
                tool_call_json.trim(),
                e
            );
            Error::ToolCallParseFailed(f!(
                "Invalid JSON format for tool call: {}. JSON: '{}'",
                e,
                tool_call_json.trim()
            ))
        })
    }

    /// Helper to execute a parsed tool call request.
    async fn execute_tool(&self, request: ToolCallRequest) -> Result<String> {
        let tool = self
            .tool_registry
            .get_tool(&request.tool_name)
            .ok_or_else(|| {
                warn!("Requested tool not found: {}", request.tool_name);
                Error::ToolNotFound(request.tool_name.clone())
            })?;

        trace!(
            "Executing tool '{}' with args: {:?}",
            request.tool_name, request.arguments
        );
        let tool_result = tool.execute(request.arguments).await?; // Propagates ToolError -> prelude::Error
        debug!(
            "Tool '{}' executed successfully. Result: {}",
            request.tool_name, tool_result
        );
        Ok(tool_result) // Return the tool's result string
    }

    /// Helper to construct the system prompt detailing available tools.
    fn construct_system_prompt(&self, tool_schemas: &[serde_json::Value]) -> String {
        let tool_list_str = if tool_schemas.is_empty() {
            "No tools available for this turn.".to_string()
        } else {
            tool_schemas
                .iter()
                .map(|schema| {
                    let name = schema["name"].as_str().unwrap_or("unknown");
                    let desc = schema["description"].as_str().unwrap_or("no description");
                    f!("- {}: {}", name, desc) // Simple list for now
                    // TODO: Could include parameter details if needed
                })
                .collect::<Vec<_>>()
                .join("\n")
        };

        // TODO: Refine prompt engineering.
        f!(
            "You are a helpful assistant. You have access to the following tools for this specific turn:\n{}\n\
             If you need to use one of these tools, respond ONLY with the following JSON structure prefixed by '{}':\n\
             ```json\n\
             {{\n  \"tool_name\": \"<name_of_tool>\",\n  \"arguments\": {{ <arguments_json_object> }}\n}}\n\
             ```\n\
             Pay close attention to the required arguments for the tool based on its description.\n\
             If you need a capability not listed, use the 'discover_tool'.\n\
             Otherwise, respond directly to the user.",
            tool_list_str,
            TOOL_CALL_MARKER
        )
        // TODO: Add explicit mention of tool schemas if the LLM provider needs them passed separately.
        // For Ollama with JSON mode/tool calling, the schema might be implicitly handled or need specific formatting.
        // Assuming for now the description in the prompt is sufficient guidance.
    }

    /// Helper to construct the JSON schema for a single tool in the format expected by LLM/Ollama.
    fn construct_tool_schema(&self, tool: &Arc<dyn Tool>) -> Result<serde_json::Value> {
        Ok(json!({
            "name": tool.name(),
            "description": tool.description(),
            "parameters": tool.parameters_schema()
        }))
    }

    /// Helper to safely downcast the LLMProvider to OllamaClient.
    fn get_ollama_client(&self) -> Result<&OllamaClient> {
        self.llm_provider
            .as_any() // Use the as_any method from the trait
            .downcast_ref::<OllamaClient>()
            .ok_or(Error::Other(String::from("Unable to downcast to ollama")))
    }
}
