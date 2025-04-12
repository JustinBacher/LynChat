# Technical Spike Research: Enhancing the Lyn AI Assistant

## Project Context

Lyn is a privacy-focused AI assistant written in Rust that operates locally with support for both local and web API-based LLMs. The project emphasizes privacy, using intelligent summarization for memory instead of traditional chat history, and includes a tools framework for task automation.

Current implementation:
- Uses ollama-rs for LLM integration
- Has basic tool discovery using semantic similarity with embeddings
- Supports multiple client interfaces (GUI, Web, CLI/TUI)
- Implements calculator and date/time tools as examples

```rust
// Current tool discovery approach
pub fn find_tool_by_capability(
    &self,
    capability_embedding: &[f32],
    threshold: f32,
) -> Option<Arc<dyn Tool>> {
    self.tools
        .values()
        .filter_map(|rt| {
            match utils::cosine_similarity(capability_embedding, &rt.description_embedding) {
                Ok(similarity) if similarity >= threshold => {
                    Some((rt.tool.clone(), similarity))
                }
                Ok(_) => None,
                Err(e) => {
                    tracing::error!(
                        "Error calculating similarity for tool '{}': {}",
                        rt.tool.name(),
                        e
                    );
                    None
                }
            }
        })
        .max_by(|(_, sim1), (_, sim2)| sim1.partial_cmp(sim2).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(tool, _)| tool)
}
```

## Research Tasks

1. **LLM Integration Alternatives**
   - Evaluate the genai crate (https://docs.rs/genai) as a potential replacement for ollama-rs
   - Compare feature sets, performance, maintenance status, and community support
   - Assess how tool integration works in each solution
   - Determine if genai would simplify implementation of tool discovery and execution

2. **Tool Implementation Best Practices**
   - Research modern approaches to tool implementation in LLMs (particularly "function calling" APIs)
   - Compare our current approach (embedding-based discovery) with direct API-based tool calling
   - Investigate how popular LLM frameworks handle tools/plugins/extensions
   - Recommend a design for implementing advanced tools (script execution, file operations, application management)

3. **Voice Integration**
   - Research options for integrating Whisper for speech recognition in a Rust project
   - Explore approaches for vendoring Whisper models with the application
   - Investigate TTS (text-to-speech) options compatible with Rust and our architecture
   - Compare local vs. API-based voice processing solutions

4. **GUI Implementation with Tauri**
   - Evaluate Tauri's compatibility with our Rust backend architecture
   - Research best practices for Tauri applications with LLM backends
   - Investigate performance considerations for running local LLMs alongside a Tauri GUI
   - Explore examples of similar applications using Tauri

## Deliverables

1. A comprehensive technical report covering each research area
2. Code examples demonstrating recommended approaches
3. Comparison tables of technology alternatives
4. Architecture diagrams for proposed implementation
5. Risk assessment and mitigation strategies

Please focus on technical feasibility and implementation details rather than high-level planning. Include specific code samples, API documentation references, and benchmarks where available.
