//! Handles application configuration loading and management.

mod error;

use std::{fs, path::PathBuf};

use crate::memory::qdrant::VectorDbConfig;
use crate::{
    llm::{LLMConfig, LLMProviders},
    prelude::*,
};
use serde::Deserialize;

pub use error::ConfigError;

/// The main application configuration structure.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub provider_configs: LLMConfig,

    #[serde(default)]
    pub vector_db: VectorDbConfig,

    #[serde(default)]
    pub provider: LLMProviders,
}

const CONFIG_DIR_NAME: &str = "lyn";
const CONFIG_FILE_NAME: &str = "config.toml";

/// Loads the application configuration.
pub fn load_config() -> Result<AppConfig> {
    // Return AppConfig
    let config_path = get_config_path()?;
    info!("Loading configuration from: {}", config_path.display());

    let settings = config::Config::builder()
        .add_source(config::File::from(config_path).required(false))
        .add_source(config::Environment::with_prefix("LYN").separator("_"))
        .build()
        .map_err(ConfigError::from)?;

    let app_config: AppConfig = settings.try_deserialize().map_err(ConfigError::from)?;

    Ok(app_config)
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or(ConfigError::DirectoryNotFound)?
        .join(CONFIG_DIR_NAME);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(ConfigError::ReadError)?;
        info!("Created configuration directory: {}", config_dir.display());
    }

    Ok(config_dir.join(CONFIG_FILE_NAME))
}
