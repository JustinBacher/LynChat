use serde::Deserialize;
use url::Url;

use super::OllamaProviderConfig;

#[derive(Debug, Deserialize, Clone, Default, PartialEq)] // Added PartialEq
#[serde(rename_all = "lowercase")]
pub enum LLMProviders {
    #[default]
    Ollama,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LLMConfig {
    pub ollama: OllamaProviderConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VectorDbConfig {
    #[serde(default = "default_qdrant_url")]
    pub url: Url,

    #[serde(default = "default_collection_name")]
    pub collection_name: String,

    #[serde(default = "default_vector_size")]
    pub vector_size: u64,
}

impl Default for VectorDbConfig {
    fn default() -> Self {
        Self {
            url: default_qdrant_url(),
            collection_name: default_collection_name(),
            vector_size: default_vector_size(),
        }
    }
}

fn default_qdrant_url() -> Url {
    Url::parse("http://127.0.0.1:6334").unwrap()
}

fn default_collection_name() -> String {
    "lyn".to_string()
}

fn default_vector_size() -> u64 {
    768 // Common embedding size
}
