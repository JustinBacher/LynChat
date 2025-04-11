# Lyn AI Assistant: Epics & Tickets (v1)

This document outlines the initial epics and associated tickets for the development of Lyn, the privacy-first local/hybrid AI assistant.

## Epic 1: Core Application & Local LLM Foundation

Goal: Establish the core application structure, async runtime, configuration, logging, error handling, and integrate initial local LLM support (e.g., Ollama).

### Tickets:

#### Ticket LYN-1: Project Setup & Core Dependencies

- **Background**: Initialize the Rust project (2024 edition), set up the build system, define the basic module structure, and integrate core dependencies like async-std (preferred over Tokio as per brainstorming notes), thiserror for error handling, and tracing for logging.

- **Acceptance Criteria**:
  - README.md is created with description of the project, goals, roadmap, and starboard.
  - Rust project compiles successfully.
  - async-std is integrated as the primary async runtime.
  - A basic prelude module exists with a global Result type and base Error enum.
  - tracing and tracing-subscriber are configured for basic console logging.
  - Basic module structure (e.g., core, llm, config) is created.

- **Tech Suggestions**: Rust 2024, Cargo, async-std, thiserror, tracing, tracing-subscriber, reqwest.

#### Ticket LYN-2: Configuration Management

- **Background**: Implement a system to load and manage application configuration (e.g., LLM settings, API keys - though handled server-side later, paths). Configuration should be loadable from a file.

- **Acceptance Criteria**:
  - Configuration can be loaded from a standard format file (e.g., TOML, YAML).
  - Default configuration values are provided.
  - Configuration values (like local LLM endpoint) are accessible throughout the application.
  - Sensitive information placeholders are handled appropriately (even if values aren't stored directly yet).

- **Tech Suggestions**: serde, config-rs or similar Rust crates, dirs (to locate config path os-independantly)

#### Ticket LYN-3: Local LLM Interface Definition

- **Background**: Define a common trait or interface for interacting with different LLM providers (local and web). This allows for modularity.

- **Acceptance Criteria**:
  - A Rust trait (LLMProvider?) is defined with methods for sending prompts and receiving responses asynchronously.
  - Input and output structures/types for LLM interactions are defined.
  - Error handling specific to LLM communication is incorporated into the trait/implementations.

- **Tech Suggestions**: Separate sub-module of main project (providers?).

#### Ticket LYN-4: Ollama Integration [RESOLVED]

- **Background**: Implement the LLMProvider trait for Ollama, allowing the application to communicate with a locally running Ollama instance.

- **Acceptance Criteria**:
  - The application can connect to a specified Ollama API endpoint (from config).
  - Prompts can be sent to a selected Ollama model.
  - Responses from Ollama are received and parsed correctly.
  - Connection errors and API errors are handled gracefully.

- **Tech Suggestions**: ollama-rs.

## Epic 2: Summarization Memory & Context Management

Goal: Implement the summarization-based memory system using Qdrant for storing and retrieving interaction context, avoiding traditional chat history.

### Tickets:

#### Ticket LYN-5: Qdrant Integration Setup

- **Background**: Integrate the Qdrant client library to connect to a local Qdrant instance (either embedded or separate process - TBD based on LYN-X research ticket). Set up the necessary collections.

- **Acceptance Criteria**:
  - The application can connect to a configured Qdrant instance.
  - A specific Qdrant collection for storing interaction summaries is created on startup if it doesn't exist.
  - Basic connection and setup errors are handled.

- **Tech Suggestions**: qdrant-client Rust crate.

#### Ticket LYN-6: Interaction Summarization Logic

- **Background**: Develop the core logic to take a user prompt and the LLM response and generate a concise summary using an LLM call (initially can use the configured local LLM).

- **Acceptance Criteria**:
  - A function exists that takes (user_prompt, llm_response) and returns a summarized string.
  - This function utilizes the configured LLMProvider to perform the summarization task.
  - The summarization prompt is configurable.
  - Handles potential errors during the summarization LLM call.

- **Tech Suggestions**: Reuse LLMProvider trait (LYN-3).

#### Ticket LYN-7: Storing Summaries in Qdrant

- **Background**: Implement the mechanism to take the generated summary (from LYN-6), potentially generate embeddings (initially simple or none, later potentially via LLM or dedicated model), and store it as a point in Qdrant with relevant metadata (e.g., timestamp, involved tools).

- **Acceptance Criteria**:
  - Generated summaries are stored in the designated Qdrant collection.
  - Each stored point includes the summary text and relevant metadata.
  - (Optional initial) Basic vector representation is stored (e.g., placeholder or simple TF-IDF if embedding model not ready).
  - Errors during storage are handled.

- **Tech Suggestions**: qdrant-client.

#### Ticket LYN-8: Retrieving Context from Qdrant

- **Background**: Implement logic to query Qdrant based on a new user prompt to find relevant past interaction summaries that can serve as context for the LLM.

- **Acceptance Criteria**:
  - A function takes a user prompt (and potentially its embedding later).
  - It queries Qdrant for semantically similar summaries.
  - Returns a ranked list of relevant summary texts.
  - The number of summaries to retrieve is configurable.
  - Handles errors during retrieval.

- **Tech Suggestions**: qdrant-client.

## Epic 3: Tools Framework & Basic Task Automation

Goal: Develop the extensible "tools" framework and implement initial tools for calculation, date/time retrieval, and basic web search.

### Tickets:

#### Ticket LYN-9: Tool Interface & Registration

- **Background**: Define a standard interface (Rust trait) for "tools" that the LLM can invoke. Create a registry system to manage available tools.

- **Acceptance Criteria**:
  - A new module is created for tools
  - A Tool trait is defined with methods like name(), description(), execute(args).
  - The description() should be suitable for inclusion in an LLM prompt.
  - The execute method runs asynchronously and returns a result (e.g., String or structured data).
  - A central registry allows tools to be added and listed.

- **Tech Suggestions**: Rust traits, (async-trait has been stabilized in rust 2024 which we are using so no need to use async-trait).

#### Ticket LYN-10: LLM Tool Invocation Logic

- **Background**: Implement the core logic that allows the LLM to decide which tool to use based on the user prompt and available tools. This involves formatting the prompt to include tool descriptions and parsing the LLM response to detect tool calls.

- **Acceptance Criteria**:
  - The main LLM interaction logic can format prompts including descriptions of registered tools.
  - It can parse LLM responses to identify requests to use a specific tool and extract arguments.
  - If a tool call is detected, the corresponding tool's execute method is called via the registry.
  - The tool's result is captured (and potentially fed back to the LLM or returned to the user).
  - Handles cases where the LLM hallucinates tool names or provides bad arguments.

- **Tech Suggestions**: Prompt engineering techniques, potentially JSON mode if supported by LLM.

#### Ticket LYN-11: Calculator Tool

- **Background**: Implement a basic calculator tool that conforms to the Tool interface.

- **Acceptance Criteria**:
  - A CalculatorTool struct implements the Tool trait.
  - The description accurately reflects its ability to perform calculations.
  - The execute method parses a mathematical expression string and returns the result.
  - Handles basic parsing errors and calculation errors.

- **Tech Suggestions**: Rust math capabilities, potentially a parsing library like meval.

#### Ticket LYN-12: Date/Time Tool

- **Background**: Implement a tool to retrieve the current date and/or time.

- **Acceptance Criteria**:
  - A DateTimeTool struct implements the Tool trait.
  - The description indicates it can provide the current date and time.
  - The execute method returns the current date/time in a user-friendly format.

- **Tech Suggestions**: chrono crate.

#### Ticket LYN-13: Basic Web Search Tool (API)

- **Background**: Implement a tool to perform web searches using a search engine API (e.g., DuckDuckGo, Google Search - requires API key handling research/setup). This is a basic version, perhaps just returning top links or snippets.

- **Acceptance Criteria**:
  - A WebSearchTool implements the Tool trait.
  - Takes a search query as an argument.
  - Calls an external search API.
  - Returns a formatted string containing search results (e.g., list of titles and URLs).
  - Handles API errors and network issues.
  - API key management needs consideration (likely loaded from config initially, but needs secure handling).

- **Tech Suggestions**: reqwest, search engine API client libraries (if available) or direct HTTP calls.

## Epic 4: Multi-Client Architecture & CLI Interface

Goal: Design the architecture to support multiple clients (GUI, Web, CLI) and implement the initial Command Line Interface (CLI/TUI).

### Tickets:

#### Ticket LYN-14: Core Engine/Client Separation

- **Background**: Refactor the core logic (LLM interaction, memory, tools) into a reusable library/crate (the "engine") separate from any specific UI implementation. Define clear API boundaries.

- **Acceptance Criteria**:
  - Core assistant logic resides in a distinct Rust crate/module.
  - A clear, asynchronous API is exposed by the engine for tasks like sending prompts, receiving responses, managing config.
  - The engine is UI-agnostic.

- **Tech Suggestions**: Rust workspace, API design principles.

#### Ticket LYN-15: CLI Application Scaffolding

- **Background**: Create a new binary crate for the CLI client that depends on the core engine crate. Set up argument parsing.

- **Acceptance Criteria**:
  - A new Rust binary crate exists for the CLI.
  - It includes the core engine as a dependency.
  - Basic command-line argument parsing is implemented (e.g., for passing an initial prompt).

- **Tech Suggestions**: clap crate.

#### Ticket LYN-16: Interactive CLI Loop

- **Background**: Implement the main interactive loop for the CLI client, allowing users to type prompts and see responses.

- **Acceptance Criteria**:
  - The CLI application presents a prompt for user input.
  - User input is captured.
  - The input is sent to the core engine via its API.
  - The response from the engine is displayed to the user.
  - The loop continues until the user exits (e.g., types "exit").
  - Basic errors from the engine are displayed appropriately.

- **Tech Suggestions**: Standard input/output, potentially rustyline for better line editing.

#### Ticket LYN-17: (Optional) TUI Enhancements

- **Background**: Explore enhancing the CLI with Text-based UI elements for a richer experience (e.g., separating input/output areas, status indicators).

- **Acceptance Criteria**:
  - Uses a TUI library to structure the display.
  - Clear separation between user input area and assistant response area.
  - (Optional) Status indicators (e.g., "Assistant thinking...", "Using tool X...").

- **Tech Suggestions**: ratatui, crossterm.

*(Epics 5 and 6 would follow, covering Web LLM Integration/Privacy and Advanced Tools like App Installation/File Management, respectively. Further epics for GUI, Web UI, i18n etc. would come after these foundational pieces are in place.)*
