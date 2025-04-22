use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiProviderConfig {
    pub api_key: String,
    pub model: String,
    pub safety_settings: Vec<SafetySetting>,
    pub generation_config: GenerationConfig,
    pub embedding_model: String,
    pub embedding: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetySetting {
    category: String,
    threshold: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub temperature: Option<f32>,
    pub max_output_tokens: Option<u32>,
}