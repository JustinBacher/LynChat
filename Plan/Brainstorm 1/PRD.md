# 🧾 Product Requirements Document (PRD) – Lyn: Privacy-First Local/Hybrid AI Assistant

## 1️⃣ Overview

- **Product Name**: Lyn
- **Category**: AI Personal Assistant
- **Target Platforms**: Desktop (GUI, Web, CLI/TUI)
- **Primary Audience**: Both daily users seeking ease of use and power users desiring advanced control and customization, united by a need for a privacy-respecting digital assistant.  
- **Core Value Proposition**: A primarily open-source, local-first assistant leveraging AI (supporting local and web LLMs) to automate digital tasks. It prioritizes user privacy and uses intelligent summarization for memory instead of traditional chat history.  

## 2️⃣ Problem Statement

Current digital assistants often rely heavily on cloud infrastructure, compromising user privacy and offering limited offline functionality. Users, especially technical ones, lack a programmable, performant assistant that respects privacy, operates effectively locally, and maintains contextual awareness without constant cloud dependency.  

## 3️⃣ Goals and Non-Goals

### ✅ Goals

- **Local-First Operation**: Design the application to operate primarily locally, maximizing offline functionality and data control.
- **Privacy & Anonymity**: Implement robust privacy measures, including local data storage by default, optional encrypted cloud backup, and server-mediated, anonymized access to web LLMs.
- **Hybrid LLM Support**: Integrate support for both locally run LLMs (e.g., via Ollama/LM Studio) and web API-based LLMs (e.g., OpenAI, Google).
- **Task Automation**: Enable users to automate tasks like web searches, getting news, installing applications, calculations, file finding/manipulation (moving, grouping), and retrieving dates.
- **Summarization Memory**: Implement a self-managed context system using intelligent summarization of past interactions stored locally (e.g., in Qdrant), avoiding reliance on LLM context windows.
- **Extensible "Tools" System**: Develop a modular "tools" system allowing the LLM to perform actions and retrieve information beyond its base capabilities, extensible by users/community.
- **Multi-Client Support**: Provide interfaces for GUI, Web, and CLI/TUI interactions [cite: 19-29, 90].
- **Performance & Async**: Build a performant, asynchronous application.
- **Open Source**: Maintain a primarily open-source codebase (exceptions minimal and transparent) under a permissive license (e.g., MIT or Apache 2.0).
- **Internationalization**: Support multiple languages (i18n).

### ❌ Non-Goals (for v1)

- Building a complex, automatic cloud synchronization system (focus on optional, user-initiated encrypted backup).
- Supporting every available LLM model or provider initially.
- Advanced multi-user collaboration features.

## 4️⃣ Functional Requirements

### 4.1 Core Capabilities

- **LLM Integration**: Ability to configure and interact with selected local and web LLMs.
- **Task Automation Engine**: Executes tasks requested by the user or LLM via the "Tools" system.
- **Privacy Framework**: Enforces data handling policies (local storage, server mediation for web APIs).
- **Memory Management**: Stores, retrieves, and utilizes conversation summaries and user preferences via Qdrant and summarization logic.
- **"Tools" System**: Provides specific functionalities (web search, file ops, etc.) callable by the LLM.

### 4.2 Features for Daily Users

- **Simple Information Retrieval**: Web lookups (like Google search), news gathering.
- **Application Management**: Discover and install applications via package managers.
- **Basic Utilities**: Calculator, date/time retrieval.
- **File Management**: Find, move, group, rearrange local files/photos.
- **Intuitive UI**: Accessible interface requiring minimal technical knowledge (GUI primary).
- **(Potential Future) Voice Control**: Hands-free interaction, potentially using local LLM for STT.

### 4.3 Features for Power Users

- **LLM Configuration**: Select and configure specific local or web LLMs and their parameters.
- **Advanced Tool Usage**: Utilize the "Tools" system for scripting complex automation workflows; ability to create custom tools.
- **Memory Control**: Fine-tune parameters for memory summarization and retrieval.
- **System Information Access**: Query system-level information (with security safeguards).
- **UI/Behavior Customization**: Adapt client behavior and interfaces (GUI, Web, CLI/TUI).

### 4.4 Non-Chat Interaction Model

- The system will not store traditional chat logs. Instead, it will generate summaries of interactions. These summaries, stored in Qdrant, form the basis of the assistant's memory and context. This reduces reliance on large LLM context windows and optimizes resource use.

## 5️⃣ LLM Integration

- **Local LLMs**: Support running LLMs locally for privacy and offline use. Aim to integrate with frameworks like Ollama or LM Studio to simplify setup. Provide clear documentation regarding hardware requirements (RAM, GPU, storage).
- **Web API LLMs**: Support integration with providers like OpenAI, Google, etc., via a server-mediated approach.
  - **Server Mediation**: Client requests are sent to our server, which securely manages API keys and forwards requests to the chosen LLM provider, relaying the response back to the client. This protects user API keys.
  - **Anonymization**: Investigate techniques to anonymize requests on the server before forwarding them to the LLM provider.
- **Self-Managed Context**: The application (not the LLM) manages context. Before querying the LLM, the application retrieves relevant summaries/preferences from Qdrant and constructs a prompt containing this context plus the user's query. Each LLM request gets a clean, explicitly provided context.

## 6️⃣ Privacy and Anonymity Framework

- **Local Data Storage**: All user data (summaries, preferences) stored locally by default.
- **Server-Mediated Web LLM Access**: As described in section 5, requests to web LLMs go through our server to avoid exposing user API keys on the client and enable anonymization. Secure, potentially pseudo-anonymous client-server communication is required.
- **Optional Encrypted Cloud Backup**: Users can optionally encrypt and back up their local data to a cloud service. Encryption methods must be robust.
- **Minimal Sensitive Data Handling**: Minimize processing/storage of user input on servers. Explore client-side redaction/anonymization before sending data to the relay server. Strict data retention policies for servers.

## 7️⃣ Self-Managed Context and Memory Implementation

- **Qdrant Integration**: Use Qdrant vector database to store conversation summaries and user preferences as vector embeddings [cite: 19-29, 155, 156, 157, 396, 397, 398].
- **Semantic Retrieval**: Query Qdrant using the embedding of the current prompt to find semantically similar past summaries/preferences for context building.
- **Key Information Recognition**: Employ mechanisms (potentially a local LLM) to analyze summaries and extract key entities, recurring themes, and user preferences (e.g., desired response style) [cite: 19-29, 162, 163, 403]. Store these recognized elements in Qdrant for enhanced personalization.

## 8️⃣ "Tools" System Architecture

- **Plugin-Based Architecture**: Design as a modular system where individual "tools" handle specific tasks.
- **Core Tools (v1)**: Web search (API/scraping), app installation (package managers), file system operations, calculator, Qdrant querying for memory/context.
- **LLM Interaction**: LLM can recognize when it needs a tool and formulate a request (ideally structured, e.g., JSON). The application executes the tool and provides the output back to the LLM to formulate its response.
- **Extensibility**: Allow users and the community to develop and add new tools.
- **Security**: Implement permission models for tools interacting with the user's system.

## 9️⃣ UI/UX

- **Multi-Client**: Provide GUI, Web, and CLI/TUI interfaces [cite: 19-29, 90].
- **Scalability**: Design UI to be intuitive for daily users while exposing advanced options for power users (progressive disclosure).
- **Customization**: Allow power users to customize UI/client behavior.

## 🔟 Technical Requirements

- **Language**: Rust (2024 Edition).
- **Async Runtime**: async-std (explicitly exclude Tokio).
- **Error Handling**: thiserror crate, use a global Result type defined in prelude.
- **Logging/Tracing**: tracing and tracing-subscriber crates (not log).
- **Code Style**: Merge imports by module, prefer small/specific modules, use traits effectively, avoid deep nesting (max 2-3 levels), follow Rust conventions (match, let Ok/Some, etc.).
- **Database**: Qdrant for vector storage/search [cite: 19-29, 155].
- **Internationalization (i18n)**: Must support i18n.

## 1️⃣1️⃣ Open-Source Strategy

- **Commitment**: Primarily open-source to foster trust, transparency, and community collaboration.
- **Licensing**: Target a permissive license like MIT or Apache 2.0. Selection requires evaluation.
- **Exceptions**: Closed-source components only if strictly necessary (e.g., proprietary dependencies), kept minimal, and communicated transparently. Core privacy features must remain open.

## 1️⃣2️⃣ Non-Functional Requirements

- **Performance**: Respond quickly to user requests, especially for local tasks. (Specific metric TBD, e.g., <500ms target mentioned in previous context). Async I/O is mandatory.
- **Resource Footprint**: Keep runtime dependencies minimal. Be mindful of resource usage, especially for local LLMs.
- **Security**: Secure client-server communication (encryption), protect API keys, implement permissions for tools.
- **Reliability**: Ensure stability across supported platforms.

## 1️⃣3️⃣ Open Questions

- Which local LLM framework(s) (Ollama, LM Studio, etc.) should be prioritized for initial integration?
- Will Qdrant be embedded within the application or run as a separate process?
- What specific algorithm/approach should be used for retrieving the most relevant context from Qdrant (e.g., simple similarity, more complex weighting)?
- Which GUI library (e.g., Slint, Tauri, egui) offers the best balance of features, performance, and cross-platform compatibility for our needs?
- What specific mechanisms will be used for secure/anonymous client-server communication (tokens, request signing)?
