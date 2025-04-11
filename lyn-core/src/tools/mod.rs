use serde_json::json;

use crate::{
    prelude::*,
    utils,
};
use std::{collections::HashMap, sync::Arc};

// Module declarations
pub mod calculator;
pub mod datetime;
pub mod discovery; // Added discovery module
pub mod error;

// Re-exports
pub use self::{
    calculator::CalculatorTool,
    datetime::DateTimeTool,
    discovery::{DiscoverTool, DiscoverToolArgs, DISCOVER_TOOL_NAME}, // Re-export discovery tool items
    error::ToolError,
};

/// Represents the arguments passed to a tool's execute method.
/// Using serde_json::Value allows for flexible argument structures.
pub type ToolArgs = serde_json::Value;

/// Represents the result returned by a tool's execute method.
/// Using String for simplicity initially, could be structured data later.
pub type ToolResult = String;

use async_trait::async_trait; // Add async_trait import

#[async_trait] // Add async_trait attribute for dyn Tool compatibility
pub trait Tool: Send + Sync + std::fmt::Debug {
    // Added Debug requirement
    fn name(&self) -> String;
    fn description(&self) -> String;
    /// Returns the JSON schema for the tool's arguments.
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, args: ToolArgs) -> Result<ToolResult>;
}

/// Holds a registered tool along with its pre-computed description embedding.
#[derive(Clone, Debug)] // Added Debug
struct RegisteredTool {
    tool: Arc<dyn Tool>,
    description_embedding: Vec<f32>,
}

/// Registry for available tools, enabling semantic search based on descriptions.
#[derive(Default, Clone, Debug)] // Added Debug
pub struct ToolRegistry {
    // Store RegisteredTool which includes the tool and its embedding
    tools: HashMap<String, RegisteredTool>,
    // TODO: Consider storing embedding model name used for consistency checks?
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Registers a tool along with its pre-computed description embedding.
    pub fn register(
        &mut self,
        tool: Arc<dyn Tool>,
        description_embedding: Vec<f32>,
    ) -> Result<()> {
        let name = tool.name();
        if self.tools.contains_key(&name) {
            Err(Error::Tool(ToolError::RegistrationFailed(f!(
                "Tool '{}' already registered",
                name.clone()
            ))))
        } else {
            let registered_tool = RegisteredTool {
                tool,
                description_embedding,
            };
            self.tools.insert(name.clone(), registered_tool);
            tracing::info!("Registered tool: {}", self.tools.get(&name).unwrap().tool.name()); // Log registered tool name
            Ok(())
        }
    }

    /// Retrieves a tool by its exact name.
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).map(|rt| rt.tool.clone())
    }

    /// Lists the names and descriptions of all registered tools.
    pub fn list_tools(&self) -> Vec<(String, String)> {
        self.tools
            .values()
            .map(|rt| (rt.tool.name(), rt.tool.description()))
            .collect()
    }

    /// Finds the best matching tool based on semantic similarity of the capability description.
    ///
    /// # Arguments
    /// * `capability_embedding` - The vector embedding of the requested capability description.
    /// * `threshold` - The minimum cosine similarity score required for a match (e.g., 0.75).
    ///
    /// # Returns
    /// * `Option<Arc<dyn Tool>>` - The best matching tool found above the threshold, or None.
    pub fn find_tool_by_capability(
        &self,
        capability_embedding: &[f32],
        threshold: f32,
    ) -> Option<Arc<dyn Tool>> {
        self.tools
            .values()
            // Calculate similarity for each tool
            .filter_map(|rt| {
                match utils::cosine_similarity(capability_embedding, &rt.description_embedding) {
                    Ok(similarity) if similarity >= threshold => {
                        Some((rt.tool.clone(), similarity)) // Keep tool and its score if above threshold
                    }
                    Ok(_) => None, // Below threshold
                    Err(e) => {
                        // Log the error but don't stop the search
                        tracing::error!(
                            "Error calculating similarity for tool '{}': {}",
                            rt.tool.name(),
                            e
                        );
                        None
                    }
                }
            })
            // Find the tool with the maximum similarity score
            .max_by(|(_, sim1), (_, sim2)| sim1.partial_cmp(sim2).unwrap_or(std::cmp::Ordering::Equal))
            // Return just the tool Arc
            .map(|(tool, _)| tool)
    }

     /// Returns the JSON schemas for all registered tools *except* discover_tool.
     /// This is useful for providing available tools to the LLM after discovery.
     pub fn get_tool_schemas(&self) -> Vec<serde_json::Value> {
         self.tools
             .values()
             .filter(|rt| rt.tool.name() != DISCOVER_TOOL_NAME) // Exclude discover_tool itself
             .map(|rt| {
                 // Construct the schema expected by Ollama (or similar LLMs)
                 json!({
                     "name": rt.tool.name(),
                     "description": rt.tool.description(),
                     "parameters": rt.tool.parameters_schema() // Assuming Tool trait provides this
                 })
             })
             .collect()
     }
}
