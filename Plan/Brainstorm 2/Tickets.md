# Lyn AI Assistant: Updated Implementation Epics & Tickets

## Epic 1: LLM Integration Modernization

**Goal**: Migrate from ollama-rs to rig for LLM integration, enabling more flexible provider support and implementing MCP servers support for enhanced AI capabilities, while also leveraging rig's vector store and embedding capabilities.

### Ticket LYN-20: Rig Integration Foundation

**Background**: While ollama-rs has served us well for Ollama integration, we've discovered the rig crate which provides a unified interface for multiple LLM providers. This aligns with our research findings suggesting a more flexible approach to LLM integration.

**Acceptance Criteria**:

- Add rig-core and necessary dependencies to the project
- Create an abstraction layer that implements our LLMProvider trait using rig
- Implement initial Ollama support through rig
- Update the configuration system to support rig's provider structure
- Ensure basic non-tool prompts work with the new integration
- Write unit tests to verify the integration works correctly

**Tech Suggestions**:

- rig-core for the main integration
- tracing for logging the migration process
- Consider implementing adapter pattern to minimize changes to existing code

### Ticket LYN-21: Tool System Adaptation for Rig

**Background**: Our current tool system was designed for ollama-rs. We need to adapt it to work with rig's approach to function/tool calling across different LLM providers.

**Acceptance Criteria**:

- Update the Tool trait to align with rig's function calling standards
- Modify ToolRegistry to work with rig's tool/function schema format
- Adapt the existing tool discovery mechanism to use rig's capabilities
- Ensure existing calculator and datetime tools work with the new system
- Update the prompt construction in core.rs to use rig's format
- Update tests to verify tools work correctly with the new integration

**Tech Suggestions**:

- Review rig documentation for function calling conventions
- Use serde_json for handling tool schemas
- Consider creating a transition layer if rig's approach differs significantly

### Ticket LYN-22: Vector Database Migration to Rig

**Background**: Currently, we're using qdrant-client directly for our vector database needs. The rig ecosystem provides a unified interface for vector stores via its rig-qdrant crate, which would align with our move to rig for LLM integration.

**Acceptance Criteria**:

- Add rig-qdrant dependency to the project
- Refactor the MemoryClient trait to leverage rig's VectorStoreIndex trait
- Implement the QdrantMemoryClient using rig-qdrant instead of direct qdrant-client usage
- Update configuration to support rig's vector store configuration pattern
- Migrate existing vector operations (insertion, retrieval, etc.) to use rig's API
- Ensure backward compatibility with existing stored data
- Add comprehensive tests for the new implementation
- Document the migration process for future reference

**Tech Suggestions**:

- rig-qdrant crate for Qdrant integration
- Consider using the adapter pattern to maintain backward compatibility
- Use rig's built-in error handling for vector store operations
- Leverage rig's document model for storage

### Ticket LYN-23: Embedding System Migration to Rig

**Background**: Our current embedding generation relies directly on LLM providers. Since we're migrating to rig, we should leverage its unified embedding model interface to make our embedding logic more flexible and provider-agnostic.

**Acceptance Criteria**:

- Refactor the embedding generation to use rig's EmbeddingModel trait
- Update the configuration to specify embedding models separate from completion models
- Implement provider-specific embedding models (e.g., for Ollama, OpenAI)
- Ensure tool descriptions and other embeddings are properly generated and stored
- Update semantic similarity calculation to use rig's vector operations if available
- Add performance benchmarks to compare with the previous implementation
- Write tests for embedding generation and similarity calculation
- Document the new embedding system

**Tech Suggestions**:

- Use rig's EmbeddingModel trait implementations
- Consider caching embeddings to improve performance
- Use rig's embedding batch processing if available
- Implement fallback mechanisms if a provider's embedding service is unavailable

## Epic 2: Multi-Provider Integration

**Goal**: Integrate additional LLM providers to expand the range of options available to the user.

### Ticket LYN-24: Google Gemini Integration

**Background**: With rig's multi-provider support, we can now integrate Google Gemini as an additional LLM option, providing users with more choices and capabilities.

**Acceptance Criteria**:

- Implement Google Gemini provider support through rig
- Add configuration options for Gemini API (keys, models, etc.)
- Ensure tool calling works correctly with Gemini
- Implement proper error handling for Gemini-specific issues
- Add testing for Gemini integration
- Document how to set up and use Gemini provider

**Tech Suggestions**:

- rig's Gemini provider components
- Create a secure API key management system
- Add feature flags to make Gemini support optional
- Consider token usage tracking for cost management

### Ticket LYN-25: OpenAI Integration

**Background**: OpenAI provides powerful models that many users may prefer. Adding OpenAI integration through rig will expand Lyn's capabilities.

**Acceptance Criteria**:

- Implement OpenAI provider support through rig
- Add configuration options for OpenAI API (keys, models, etc.)
- Ensure tool calling works correctly with OpenAI models
- Implement proper error handling for OpenAI-specific issues
- Add testing for OpenAI integration
- Document how to set up and use OpenAI provider
- Include information about costs and token usage

**Tech Suggestions**:

- rig's OpenAI provider components
- Implement token counting and budget management
- Add configuration for model parameters (temperature, etc.)
- Consider implementing response caching for efficiency

### Ticket LYN-26: MCP Server Support

**Background**: Model Context Protocol (MCP) servers allow AI assistants to access external tools and data sources. The rmcp crate provides a Rust implementation of this protocol that we can leverage to greatly enhance Lyn's capabilities.

**Acceptance Criteria**:

- Integrate the rmcp crate for MCP server client support
- Create a trait-based abstraction for MCP servers to allow different implementations
- Implement a configuration system for MCP servers (connection types, endpoints)
- Create a mechanism to discover and register available MCP tools at runtime
- Develop a unified mechanism for invoking tools from both local and MCP sources
- Add proper error handling and logging for MCP interactions
- Write unit and integration tests for MCP server support

**Tech Suggestions**:

- rmcp crate for the MCP protocol implementation
- tokio for async runtime support with MCP servers
- Implement registry pattern for MCP servers management
- Consider adding capability to spawn local MCP servers via child processes

## Epic 3: Tool System Implementation

**Goal**: Implement a robust, extensible, and secure tool system that allows the AI assistant to perform a wide range of useful operations.

### Ticket LYN-27: File Operations Tool Implementation

**Background**: One of the key capabilities requested is the ability to find, move, group, and rearrange files. This ticket implements a new tool for file operations using the new rig-based tool system.

**Acceptance Criteria**:

- Create a new FileOperationsTool module with capabilities for:
  - Finding files matching a query
  - Moving files from one location to another
  - Creating new directories
  - Listing files in a directory
- Implement proper permission checks and validation for all file operations
- Path traversal security measures to prevent accessing sensitive files
- Add comprehensive tests for all file operations
- Document the tool's capabilities and security measures

**Tech Suggestions**:

- std::fs for file operations
- path-clean or similar for path sanitization
- glob for file pattern matching
- Ensure compatibility with rig's function calling format

### Ticket LYN-28: Tool Security Framework

**Background**: With more powerful tools like file operations, we need a robust security framework to prevent misuse and protect user data.

**Acceptance Criteria**:

- Implement a permission system for tools with different risk levels
- Create a confirmation mechanism for potentially destructive operations
- Add path sanitization and validation for all file-related operations
- Create a sandboxing approach for script execution (future tool)
- Develop a logging system for tool usage for audit purposes
- Write documentation on the security measures and best practices

**Tech Suggestions**:

- Use Rust's type system to encode permission levels
- Consider capability-based security patterns
- Implement allow/deny lists for file paths
- tracing crate for enhanced logging of security-relevant actions

### Ticket LYN-29: Internet Search Tool

**Background**: Internet search is a critical capability for an AI assistant. We need to implement a flexible search tool that can work with different search providers.

**Acceptance Criteria**:

- Create a SearchTool trait defining the common interface for search providers
- Implement at least two search provider implementations:
  - Google Search (using a compatible API)
  - DuckDuckGo Search
- Add support for Perplexity search as an optional provider
- Implement configuration options for each search provider
- Add rate limiting and caching to prevent excessive API usage
- Create a results parsing system that standardizes output across providers
- Write comprehensive tests for the search functionality
- Document how to use and extend the search capabilities

**Tech Suggestions**:

- reqwest for HTTP requests
- duckduckgo_rs or duckduckgo_search for DuckDuckGo integration
- serpapi-search-rust for Google Search
- Consider implementing a local cache using sled or similar
- Use async-trait for the provider trait

### Ticket LYN-30: Popular MCP Server Integrations

**Background**: Many useful MCP servers exist that could enhance Lyn's capabilities. We should implement integrations with some of the most popular ones.

**Acceptance Criteria**:

- Create trait implementations for at least 3 popular MCP servers:
  - Filesystem MCP Server for enhanced file operations
  - Web Search MCP Server for internet search capabilities
  - GitHub MCP Server for code repository interactions
- Implement proper configuration and authentication for each MCP server
- Create a unified way to discover and use tools from these servers
- Add documentation on how to use each MCP server
- Write tests to verify correct MCP server connections and tool usage

**Tech Suggestions**:

- Use the rmcp crate's client capabilities
- Implement secure credential storage for MCP server authentication
- Consider adding a discovery mechanism for MCP servers on the local network

### Ticket LYN-31: Application Management Tools

**Background**: Users want to be able to discover, install and manage applications through the assistant. We need to implement these tools in a secure way.

**Acceptance Criteria**:

- Create a PackageManager trait for platform-specific implementations
- Implement platform-specific package managers:
  - apt for Linux
  - homebrew for macOS
  - winget for Windows
- Add application discovery capabilities (search for available packages)
- Implement secure installation, update, and removal operations
- Create a validation and verification system for package integrity
- Add logging for all package operations
- Document the security model and usage guidelines

**Tech Suggestions**:

- std::process for executing package manager commands
- Use command pattern for package operations
- Consider using tokio::process for async execution
- Implement robust output parsing for different package managers

## Epic 4: Voice Integration

**Goal**: Add voice capabilities to Lyn, including speech-to-text and text-to-speech, while maintaining the privacy-focused approach of processing everything locally.

### Ticket LYN-32: Speech-to-Text Integration (Whisper)

**Background**: Voice input would significantly enhance the user experience. Research indicates that whisper-rs or mutter would be appropriate libraries for integrating Whisper STT capabilities.

**Acceptance Criteria**:

- Integrate a Rust speech-to-text library (whisper-rs recommended)
- Create an abstraction layer for audio capture and processing
- Implement model loading and management system
- Handle on-demand downloading of Whisper models as needed
- Add a configuration option to enable/disable voice features
- Document the STT features and required dependencies

**Tech Suggestions**:

- whisper-rs or mutter for Whisper integration
- rodio or cpal for audio capture
- async-std for handling audio processing without blocking the main thread
- flume for handling audio stream processing

### Ticket LYN-33: Text-to-Speech Implementation

**Background**: To complement speech recognition, we need text-to-speech capabilities to allow for fully voice-based interaction. Research suggests tts-rs or sherpa-rs as viable options for local TTS.

**Acceptance Criteria**:

- Integrate a Rust text-to-speech library
- Create an abstraction layer for TTS services
- Implement voice selection and configuration options
- Add support for basic SSML or similar markup for speech control
- Ensure TTS operations are asynchronous to maintain UI responsiveness
- Document TTS features and voice options

**Tech Suggestions**:

- tts-rs for cross-platform TTS support
- sherpa-rs if higher quality voices are required
- async-std for non-blocking TTS processing
- Consider a fallback system if local TTS is not available

### Ticket LYN-34: Voice Interaction Flow

**Background**: With both STT and TTS implemented, we need to create a cohesive voice interaction flow that integrates with the core application logic.

**Acceptance Criteria**:

- Implement a voice activation mechanism (wake word or button)
- Create a state machine for voice interaction flow
- Add audio indicators for system state (listening, processing, responding)
- Integrate voice processing with the main Engine
- Handle errors and edge cases gracefully (no speech detected, unclear speech)
- Document the full voice interaction flow and configuration options

**Tech Suggestions**:

- state machine pattern for managing voice interaction states
- Consider a simple wake word implementation (optional)
- Audio visualization components for feedback
- Consider a queue-based architecture for voice processing

## Epic 5: TUI Enhancement

**Goal**: Enhance the existing Ratatui-based TUI with interactive confirmation buttons for system modifications and collapsible thought process visualization.

### Ticket LYN-35: Interactive Confirmation Buttons

**Background**: When the AI wants to perform potentially risky system modifications through tools, users need to explicitly confirm these actions. We need to add interactive buttons to the existing Ratatui implementation.

**Acceptance Criteria**:

- Add a ButtonWidget component to the existing TUI framework
- Implement hover and selection states for buttons
- Create a confirmation dialog system that can be triggered by tools
- Add support for both keyboard navigation and mouse interaction for buttons
- Implement a central registry for managing active confirmation requests
- Modify the tool execution flow to wait for confirmation when needed
- Ensure buttons are accessible with good visual contrast
- Update the event handling system to process button interactions

**Tech Suggestions**:

- Use Ratatui's custom widget capabilities to implement buttons
- Leverage crossterm's mouse event support
- Consider implementing a focus system for keyboard navigation between buttons
- Create an enum for different confirmation types with associated styling

### Ticket LYN-36: LLM Thought Process Visualization

**Background**: Users would benefit from seeing the LLM's reasoning process, but it should be discreet and not clutter the interface. We need to add collapsible sections to the existing TUI.

**Acceptance Criteria**:

- Create a CollapsiblePanel widget for the TUI framework
- Modify the LLM interaction flow to capture and display thinking processes
- Implement auto-collapse functionality when LLM output is complete
- Add visual indicators (like an animated spinner) during active thinking
- Ensure panels can be manually expanded/collapsed via keyboard or mouse
- Add scrolling support for long thought processes
- Create syntax highlighting for structured content in thoughts
- Add configuration options to control thought display behavior

**Tech Suggestions**:

- Use Ratatui's Layout system to create a reserved space for the thought panel
- Implement a buffer to capture streaming thought content
- Consider using Unicode block characters for collapse/expand indicators
- Use different text styles to distinguish thinking from final outputs

### Ticket LYN-37: Advanced Event Handling System

**Background**: To support the new interactive elements, we need to enhance the event handling system in our TUI to properly manage keyboard, mouse, and application events.

**Acceptance Criteria**:

- Refactor the existing event handling system to support component-specific events
- Create a focus management system for interactive elements
- Implement event bubbling or delegation for nested components
- Add support for custom keyboard shortcuts
- Create a consistent visual indicator for focused elements
- Ensure all interactive elements can be accessed via keyboard
- Modify the main event loop to dispatch events to the appropriate handlers
- Add proper error handling for event processing

**Tech Suggestions**:

- Consider implementing an observer pattern for event handling
- Use an event queue to manage multiple simultaneous events
- Create a mapping system for keyboard shortcuts
- Add visual highlighting for the currently focused element

### Ticket LYN-38: State Management Improvements

**Background**: With the addition of interactive elements and collapsible sections, we need a more robust state management system for the TUI.

**Acceptance Criteria**:

- Create a centralized state management system for the TUI
- Implement state transitions for interactive elements
- Add support for component-specific state
- Create a mechanism to persist relevant state between application runs
- Ensure state changes trigger appropriate UI updates
- Add logging for state transitions to aid debugging
- Create a clear architecture for state management
- Update existing TUI components to use the new state system

**Tech Suggestions**:

- Consider an approach inspired by the Elm architecture or Redux
- Use a variant of the observer pattern for state change notifications
- Store state in a structured format that can be easily serialized
- Consider using a state machine for complex interaction flows

## Epic 6: Context Management and Memory System

**Goal**: Enhance the memory and context management system to provide more relevant information to the LLM and create a more personalized experience.

### Ticket LYN-39: Context Management Enhancement

**Background**: The way we manage context for LLM interactions needs improvement to make better use of our memory system.

**Acceptance Criteria**:

- Implement more sophisticated context retrieval from the memory system
- Add relevance scoring for retrieved memories
- Create a dynamic context window management system
- Implement context compression for longer interactions
- Add configurable parameters for context management
- Document the enhanced context management approach

**Tech Suggestions**:

- Use rig's vector store capabilities for nuanced retrieval
- Consider semantic chunking for context compression
- Implement a priority queue for context elements
- Add logging of context decisions for debugging

### Ticket LYN-40: User Preference Learning

**Background**: To provide a more personalized experience, Lyn should learn and remember user preferences over time.

**Acceptance Criteria**:

- Implement a system to identify and extract user preferences from conversations
- Create a storage mechanism for preferences in the memory system
- Develop a way to apply preferences to future interactions
- Add explicit preference management commands
- Create a privacy-focused approach to preference handling
- Document the preference system and how users can manage it

**Tech Suggestions**:

- Use rig's vector store for preference storage
- Implement schema validation for preferences
- Consider using a simple rule-based system for preference extraction
- Add encryption for sensitive preference data

### Ticket LYN-41: Memory Visualization and Management

**Background**: Users should be able to view and manage what the assistant remembers about their interactions.

**Acceptance Criteria**:

- Create a TUI interface for viewing stored memories
- Implement search and filtering capabilities for memories
- Add the ability to delete or edit specific memories
- Create visualization for memory connections and relationships
- Implement memory export and import functionality
- Document the memory management capabilities

**Tech Suggestions**:

- Use Ratatui's custom widgets for memory visualization
- Implement pagination for large memory sets
- Consider using a faceted search interface
- Add confirmation for destructive memory operations

This implementation plan provides a comprehensive approach to enhancing the Lyn AI Assistant, now including the migration to rig's vector store and embedding capabilities. These changes will consolidate our dependencies while providing a more unified and flexible architecture.
