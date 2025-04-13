use crate::llm::LLMProvider;
use crate::memory::error::MemoryError;
use crate::prelude::*;
use rig::embedding::{EmbeddingModel, EmbeddingRequest, EmbeddingResponse};

/// Generates embeddings for text using the specified LLM provider.
pub async fn generate_embedding(text: &str, llm_provider: &dyn LLMProvider) -> Result<Vec<f32>> {
    debug!(
        "Generating embedding for text: '{}'",
        text.chars().take(30).collect::<String>()
    );

    let request = EmbeddingRequest {
        input: text.to_string(),
        model: None, // Use default model
    };

    match llm_provider.generate_embedding(request).await {
        Ok(EmbeddingResponse { embedding, .. }) => {
            debug!(
                "Successfully generated embedding with {} dimensions",
                embedding.len()
            );
            Ok(embedding)
        }
        Err(e) => {
            error!("Failed to generate embedding: {}", e);
            Err(Error::Memory(MemoryError::Embedding(
                "text embedding".to_string(),
                e.to_string(),
            )))
        }
    }
}

/// Utility function to compute cosine similarity between two vectors.
pub fn cosine_similarity(v1: &[f32], v2: &[f32]) -> Result<f32> {
    if v1.len() != v2.len() {
        return Err(Error::Memory(MemoryError::DataProcessing(format!(
            "Vectors must have the same length ({} != {})",
            v1.len(),
            v2.len()
        ))));
    }

    if v1.is_empty() {
        return Err(Error::Memory(MemoryError::DataProcessing(
            "Vectors cannot be empty".to_string(),
        )));
    }

    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm1 == 0.0 || norm2 == 0.0 {
        return Ok(0.0);
    }

    Ok((dot_product / (norm1 * norm2)).clamp(-1.0, 1.0))
}
