# Lyn AI Assistant

[![Rust](https://github.com/your-repo/lyn/actions/workflows/rust.yml/badge.svg)](https://github.com/your-repo/lyn/actions/workflows/rust.yml) <!-- TODO: Update badge URL -->
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Star History](https://api.star-history.com/svg?repos=your-repo/lyn&type=Date)](https://star-history.com/#your-repo/lyn&Date) <!-- TODO: Update Star History URL -->

Lyn is a privacy-first, local/hybrid AI assistant designed for both daily users and power users. It prioritizes local operation, data privacy, and user control.

## Goals

*   **Local-First**: Operate primarily locally.
*   **Privacy**: Strong focus on user data privacy and anonymity.
*   **Hybrid LLM**: Support local and web-based LLMs.
*   **Task Automation**: Automate various digital tasks.
*   **Summarization Memory**: Use summarization for context, not chat history.
*   **Extensible Tools**: Allow adding new capabilities via a tool system.
*   **Multi-Client**: Offer GUI, Web, and CLI/TUI interfaces.
*   **Open Source**: Primarily open-source codebase.

## Roadmap (High-Level)

1.  **Core Foundation**: Project setup, async runtime, config, logging, local LLM interface (Ollama).
2.  **Memory System**: Qdrant integration, summarization logic, context retrieval.
3.  **Tools Framework**: Basic tools (calculator, date, web search).
4.  **CLI Interface**: Initial command-line client.
5.  **Web LLM Integration**: Server-mediated, anonymized access.
6.  **Advanced Tools**: File management, application installation.
7.  **GUI/Web Clients**: Develop graphical and web interfaces.
8.  **Internationalization**: Add multi-language support.

*(More detailed roadmap tracked via project issues/epics)*

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to the project.

## License

Lyn is licensed under either of the MIT License or the Apache License, Version 2.0 (See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE)) at your option.
