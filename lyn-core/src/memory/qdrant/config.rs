use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize, Clone)]
pub struct VectorDbConfig {
    #[serde(default = "default_qdrant_url")]
    pub url: Url,
}

impl Default for VectorDbConfig {
    fn default() -> Self {
        Self {
            url: default_qdrant_url(),
        }
    }
}

fn default_qdrant_url() -> Url {
    Url::parse("http://127.0.0.1:6334").unwrap()
}
