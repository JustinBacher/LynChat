use serde::Deserialize;

use super::OllamaConfig;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LLMConfig {
    pub ollama: OllamaConfig,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum LLMProviders {
    #[default]
    Ollama,
}
