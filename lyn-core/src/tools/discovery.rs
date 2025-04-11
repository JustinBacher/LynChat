//! Defines the `discover_tool` used by the LLM to request tool capabilities.

use serde::{Deserialize, Serialize};
use serde_json::json;

pub const DISCOVER_TOOL_NAME: &str = "discover_tool";
pub const DISCOVER_TOOL_DESCRIPTION: &str = "This tool allows you to find out about all the different things you can do. \
    If you ever feel uncertain, even for a moment, about whether there's a better way to accomplish something or if you \
    lack the perfect information, use this tool to explore your options. To use this tool, provide a concise description \
    of what you need or are trying to achieve. Think of it as your go-to for uncovering any potential assistance. \
    This tool will then return a list of available tools that fit your description.";

/// Arguments for the discover_tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverToolArgs {
    /// A clear description of the capability or task you need a tool for.
    /// Example: "calculate the result of a mathematical expression" or "get the current date and time".
    pub capability_description: String,
}

/// The tool definition for discovering other tools.
#[derive(Debug, Clone, Default)]
pub struct DiscoverTool;

impl DiscoverTool {
    pub fn new() -> Self {
        Self
    }

    /// Provides the JSON schema for the discover_tool.
    /// This schema is sent to the LLM so it knows how to call the tool.
    pub fn tool_schema() -> serde_json::Value {
        json!({
            "name": DISCOVER_TOOL_NAME,
            "description": DISCOVER_TOOL_DESCRIPTION,
            "parameters": {
                "type": "object",
                "properties": {
                    "capability_description": {
                        "type": "string",
                        "description": "A clear description of the capability or task you need a tool for. Example: \"calculate the result of a mathematical expression\" or \"get the current date and time\"."
                    }
                },
                "required": ["capability_description"]
            }
        })
    }
}

// Note: This tool doesn't actually *execute* in the traditional sense within the core.
// Its "execution" is handled by the core logic that intercepts the call,
// performs the semantic search in the ToolRegistry, and formulates a response
// back to the LLM about whether a tool was found.
// Therefore, it doesn't need to implement the `Tool` trait's `execute` method directly.
// We might need a way to represent this "meta-tool" in the registry later,
// primarily for schema generation, or handle it as a special case in the core loop.
