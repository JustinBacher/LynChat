use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default, PartialEq)] // Added PartialEq
#[serde(rename_all = "lowercase")]
pub enum LLMProviders {
    #[default]
    Ollama,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OllamaProviderConfig {
    #[serde(default = "default_ollama_url")]
    pub url: String,

    #[serde(default = "default_ollama_port")]
    pub port: u16,

    #[serde(default = "default_ollama_model")]
    pub model: String,

    #[serde(default = "default_ollama_embedding_model")]
    pub embedding_model: String,
}

impl Default for OllamaProviderConfig {
    fn default() -> Self {
        Self {
            url: default_ollama_url(),
            port: default_ollama_port(),
            model: default_ollama_model(),
            embedding_model: default_ollama_embedding_model(),
        }
    }
}

fn default_ollama_url() -> String {
    String::from("http://127.0.0.1")
}

fn default_ollama_port() -> u16 {
    11434
}

fn default_ollama_model() -> String {
    String::from("llama3.2:1b")
}

fn default_ollama_embedding_model() -> String {
    String::from("llama3.2:1b")
}
