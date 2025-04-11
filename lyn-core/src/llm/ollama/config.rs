use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct OllamaConfig {
    #[serde(default = "default_ollama_url")]
    pub url: String,
    #[serde(default = "default_ollama_port")]
    pub port: u16,
    #[serde(default = "default_ollama_model")]
    pub model: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            url: default_ollama_url(),
            port: default_ollama_port(),
            model: default_ollama_model(),
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
