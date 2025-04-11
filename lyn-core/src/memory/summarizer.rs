//! Logic for summarizing interactions using an LLM.

use crate::{
    llm::{LLMProvider, Message, Prompt},
    memory::error::MemoryError,
    prelude::*,
};

/// Summarizes a user prompt and the corresponding LLM response.
pub async fn summarize_interaction<L, P>(
    llm_provider: &L,
    user_prompt: &str,
    llm_response: &str,
) -> Result<String>
where
    L: AsRef<P>,
    P: LLMProvider + ?Sized,
{
    // TODO: Make this prompt template configurable (LYN-6 requirement)
    let summarization_prompt_content = f!(
        "Summarize the following interaction concisely:\n\nUser: {}\nAssistant: {}\n\nSummary:",
        user_prompt,
        llm_response
    );

    let summarization_prompt = Prompt {
        messages: vec![Message {
            role: "user".to_string(), // Or maybe a "system" role? Needs consideration.
            content: summarization_prompt_content,
        }],
        // Add other parameters like temperature if needed for summarization
    };

    info!("Requesting summarization from LLM...");
    debug!("Summarization prompt: {:?}", summarization_prompt);

    match llm_provider.as_ref().generate(&summarization_prompt).await {
        Ok(response) => {
            info!("Summarization successful.");
            debug!("Summary received: {}", response.content);
            Ok(response.content)
        }
        Err(llm_error) => {
            error!("LLM summarization failed: {}", llm_error);
            // Wrap the LLM error into our specific Summarization error
            Err(Error::Memory(MemoryError::Summarization(
                llm_error.to_string(),
            )))
        }
    }
}
