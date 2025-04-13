use std::collections::HashMap;

use rig::tool::{Tool, ToolDyn, ToolSet as RigToolSet};
use serde::{Deserialize, Serialize};

use super::{Calculator, DateTime};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ToolCategory {
    Utilities,
    System,
    FileOperations,
    WebAccess,
    Custom(String),
}

/// Registry for managing and organizing tools
pub struct ToolRegistry {
    // Map from category to tools in that category
    categories: HashMap<ToolCategory, Vec<Box<dyn ToolDyn>>>,
    // Map from tool name to category for quick lookups
    tool_categories: HashMap<String, ToolCategory>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            tool_categories: HashMap::new(),
        }
    }

    /// Register a tool in a specific category
    pub fn register<T>(&mut self, tool: T, category: ToolCategory)
    where
        T: Tool + 'static,
    {
        let tool_name = T::NAME.to_string();
        let boxed_tool = Box::new(tool);

        // Add to category map
        self.categories
            .entry(category.clone())
            .or_default()
            .push(boxed_tool);

        // Add to tool category map for lookups
        self.tool_categories.insert(tool_name, category);
    }

    /// Get all tools in a specific category
    pub fn get_by_category(&self, category: &ToolCategory) -> Vec<&Box<dyn ToolDyn>> {
        self.categories
            .get(category)
            .map(|tools| tools.iter().collect())
            .unwrap_or_default()
    }

    /// Get all available tools
    pub fn get_all_tools(&self) -> Vec<&Box<dyn ToolDyn>> {
        self.categories
            .values()
            .flat_map(|tools| tools.iter())
            .collect()
    }

    /// Get tool definitions for all tools
    pub async fn get_tool_definitions(&self, prompt: &str) -> Vec<rig::completion::ToolDefinition> {
        let mut definitions = Vec::new();
        for tool in self.get_all_tools() {
            // Note: This requires downcasting which might not be ideal
            // You might need to adapt this based on rig's actual API
            let definition = tool.definition(prompt.to_string()).await;
            definitions.push(definition);
        }
        definitions
    }

    /// Convert to a rig ToolSet for LLM integration
    pub fn as_rig_toolset(&self) -> RigToolSet {
        let mut toolset = RigToolSet::default();

        for category in self.categories.keys() {
            match category {
                ToolCategory::Utilities => {
                    // Add utility tools
                    toolset.add_tool(Calculator);
                    toolset.add_tool(DateTime);
                }
                ToolCategory::System => {
                    // Add system tools
                    // toolset.add_tool(SystemInfo);
                }
                ToolCategory::FileOperations => {
                    // Add file operation tools
                    // toolset.add_tool(FileSearcher);
                }
                ToolCategory::WebAccess => {
                    // Add web access tools
                    // toolset.add_tool(WebSearch);
                }
                ToolCategory::Custom(_) => {
                    // Custom tools would need special handling
                }
            }
        }

        toolset
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
