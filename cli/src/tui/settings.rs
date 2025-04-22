//! Settings management for the TUI

use std::collections::HashMap;

use common::{
    config::{self, AppConfig},
    llm::{LLMProviders, GeminiProviderConfig, OllamaProviderConfig},
    prelude::*,
};

/// Represents a setting that can be modified in the settings dialog
#[derive(Debug, Clone)]
pub enum SettingValue {
    /// A string value
    String(String),
    /// A boolean value
    Bool(bool),
    /// An enum value with possible options
    Enum {
        /// Current value
        current: String,
        /// Available options
        options: Vec<String>,
    },
    /// A dropdown value with possible options
    Dropdown {
        /// Current selected index
        selected: usize,
        /// Available options
        options: Vec<String>,
        /// Whether the dropdown is expanded
        expanded: bool,
    },
}

/// Represents a single setting in the settings dialog
#[derive(Debug, Clone)]
pub struct Setting {
    /// The key for this setting
    pub key: String,
    /// The display name for this setting
    pub name: String,
    /// The current value of this setting
    pub value: SettingValue,
    /// Description of this setting
    pub description: String,
}

/// Represents a provider configuration
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    /// The provider name
    pub name: String,
    /// The provider settings
    pub settings: Vec<Setting>,
    /// Whether this provider is currently selected
    pub is_selected: bool,
}

/// Represents the settings for the TUI application
#[derive(Debug, Clone)]
pub struct Settings {
    /// All available settings
    pub settings: Vec<Setting>,
    /// Provider-specific settings
    pub providers: Vec<ProviderConfig>,
    /// The currently selected setting index
    pub selected_index: usize,
    /// Whether the settings panel is visible
    pub visible: bool,
    /// Whether we're in provider selection mode
    pub provider_selection_mode: bool,
    /// Whether the settings panel is collapsed
    pub collapsed: bool,
}

impl Settings {
    /// Creates a new instance of Settings from the app config
    pub fn from_app_config(config: &AppConfig) -> Self {
        let mut general_settings = Vec::new();
        let mut providers = Vec::new();

        // Provider selection
        general_settings.push(Setting {
            key: "provider".to_string(),
            name: "LLM Provider".to_string(),
            value: SettingValue::Dropdown {
                selected: match config.provider {
                    LLMProviders::Ollama => 0,
                    LLMProviders::Gemini => 1,
                },
                options: vec!["Ollama".to_string(), "Gemini".to_string()],
                expanded: false,
            },
            description: "The LLM provider to use for generating responses".to_string(),
        });

        // Create Ollama provider config
        let mut ollama_settings = Vec::new();

        // Add URL setting
        ollama_settings.push(Setting {
            key: "ollama.url".to_string(),
            name: "URL".to_string(),
            value: SettingValue::String(config.provider_configs.ollama.url.clone()),
            description: "The URL of the Ollama server".to_string(),
        });

        // Add Port setting
        ollama_settings.push(Setting {
            key: "ollama.port".to_string(),
            name: "Port".to_string(),
            value: SettingValue::String(config.provider_configs.ollama.port.to_string()),
            description: "The port of the Ollama server".to_string(),
        });

        // Add Model setting
        ollama_settings.push(Setting {
            key: "ollama.model".to_string(),
            name: "Model".to_string(),
            value: SettingValue::String(config.provider_configs.ollama.model.clone()),
            description: "The model to use with Ollama".to_string(),
        });

        // Add Embedding Model setting
        ollama_settings.push(Setting {
            key: "ollama.embedding_model".to_string(),
            name: "Embedding Model".to_string(),
            value: SettingValue::String(config.provider_configs.ollama.embedding_model.clone()),
            description: "The model to use for embeddings with Ollama".to_string(),
        });

        providers.push(ProviderConfig {
            name: "Ollama".to_string(),
            settings: ollama_settings,
            is_selected: config.provider == LLMProviders::Ollama,
        });

        // Create Gemini provider config
        let mut gemini_settings = Vec::new();

        // Add API Key setting
        gemini_settings.push(Setting {
            key: "gemini.api_key".to_string(),
            name: "API Key".to_string(),
            value: SettingValue::String(
                config.provider_configs.gemini
                    .as_ref()
                    .map(|g| g.api_key.clone())
                    .unwrap_or_default()
            ),
            description: "The API key for Gemini".to_string(),
        });

        // Add Model setting
        gemini_settings.push(Setting {
            key: "gemini.model".to_string(),
            name: "Model".to_string(),
            value: SettingValue::String(
                config.provider_configs.gemini
                    .as_ref()
                    .map(|g| g.model.clone())
                    .unwrap_or_else(|| "gemini-pro".to_string())
            ),
            description: "The model to use with Gemini".to_string(),
        });

        // Add Embedding Model setting
        gemini_settings.push(Setting {
            key: "gemini.embedding_model".to_string(),
            name: "Embedding Model".to_string(),
            value: SettingValue::String(
                config.provider_configs.gemini
                    .as_ref()
                    .map(|g| g.embedding_model.clone())
                    .unwrap_or_else(|| "gemini-pro-embedding".to_string())
            ),
            description: "The model to use for embeddings with Gemini".to_string(),
        });

        providers.push(ProviderConfig {
            name: "Gemini".to_string(),
            settings: gemini_settings,
            is_selected: config.provider == LLMProviders::Gemini,
        });

        Self {
            settings: general_settings,
            providers,
            selected_index: 0,
            visible: false,
            provider_selection_mode: false,
            collapsed: false,
        }
    }

    /// Updates the app config with the current settings
    pub fn update_app_config(&self, config: &mut AppConfig) -> Result<()> {
        // Update general settings
        for setting in &self.settings {
            match setting.key.as_str() {
                "provider" => {
                    if let SettingValue::Dropdown { selected, options, .. } = &setting.value {
                        match options[*selected].as_str() {
                            "Ollama" => config.provider = LLMProviders::Ollama,
                            "Gemini" => config.provider = LLMProviders::Gemini,
                            _ => return Err(Error::Config(config::ConfigError::ValidationError(
                                format!("Invalid provider: {}", options[*selected])
                            ))),
                        }
                    }
                }
                _ => {}
            }
        }

        // Update provider-specific settings
        for provider in &self.providers {
            match provider.name.as_str() {
                "Ollama" => {
                    for setting in &provider.settings {
                        match setting.key.as_str() {
                            "ollama.url" => {
                                if let SettingValue::String(value) = &setting.value {
                                    config.provider_configs.ollama.url = value.clone();
                                }
                            }
                            "ollama.port" => {
                                if let SettingValue::String(value) = &setting.value {
                                    if let Ok(port) = value.parse::<u16>() {
                                        config.provider_configs.ollama.port = port;
                                    } else {
                                        return Err(Error::Config(config::ConfigError::ValidationError(
                                            format!("Invalid port: {}", value)
                                        )));
                                    }
                                }
                            }
                            "ollama.model" => {
                                if let SettingValue::String(value) = &setting.value {
                                    config.provider_configs.ollama.model = value.clone();
                                }
                            }
                            "ollama.embedding_model" => {
                                if let SettingValue::String(value) = &setting.value {
                                    config.provider_configs.ollama.embedding_model = value.clone();
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "Gemini" => {
                    // Collect Gemini settings
                    let mut api_key = String::new();
                    let mut model = String::new();
                    let mut embedding_model = String::new();

                    for setting in &provider.settings {
                        match setting.key.as_str() {
                            "gemini.api_key" => {
                                if let SettingValue::String(value) = &setting.value {
                                    api_key = value.clone();
                                }
                            }
                            "gemini.model" => {
                                if let SettingValue::String(value) = &setting.value {
                                    model = value.clone();
                                }
                            }
                            "gemini.embedding_model" => {
                                if let SettingValue::String(value) = &setting.value {
                                    embedding_model = value.clone();
                                }
                            }
                            _ => {}
                        }
                    }

                    // Create or update Gemini config
                    if config.provider_configs.gemini.is_none() && !api_key.is_empty() {
                        // Create a new Gemini config with default values
                        config.provider_configs.gemini = Some(GeminiProviderConfig {
                            api_key,
                            model: model.clone(),
                            safety_settings: Vec::new(),
                            generation_config: common::llm::gemini::config::GenerationConfig {
                                temperature: Some(0.7),
                                max_output_tokens: Some(1024),
                            },
                            embedding_model,
                            embedding: true,
                        });
                    } else if let Some(gemini_config) = &mut config.provider_configs.gemini {
                        // Update existing Gemini config
                        if !api_key.is_empty() {
                            gemini_config.api_key = api_key;
                        }
                        if !model.is_empty() {
                            gemini_config.model = model;
                        }
                        if !embedding_model.is_empty() {
                            gemini_config.embedding_model = embedding_model;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Moves the selection cursor up
    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Moves the selection cursor down
    pub fn select_next(&mut self) {
        let max_index = if self.provider_selection_mode {
            self.selected_provider()
                .map(|p| p.settings.len().saturating_sub(1))
                .unwrap_or(0)
        } else {
            self.settings.len().saturating_sub(1)
        };

        if self.selected_index < max_index {
            self.selected_index += 1;
        }
    }

    /// Gets the currently selected setting
    pub fn selected_setting(&self) -> Option<&Setting> {
        if self.provider_selection_mode {
            // Get the selected provider's selected setting
            self.providers.iter()
                .find(|p| p.is_selected)
                .and_then(|p| p.settings.get(self.selected_index))
        } else {
            // Get the general setting
            self.settings.get(self.selected_index)
        }
    }

    /// Gets a mutable reference to the currently selected setting
    pub fn selected_setting_mut(&mut self) -> Option<&mut Setting> {
        if self.provider_selection_mode {
            // Get the selected provider's selected setting
            self.providers.iter_mut()
                .find(|p| p.is_selected)
                .and_then(|p| p.settings.get_mut(self.selected_index))
        } else {
            // Get the general setting
            self.settings.get_mut(self.selected_index)
        }
    }

    /// Gets the currently selected provider
    pub fn selected_provider(&self) -> Option<&ProviderConfig> {
        self.providers.iter().find(|p| p.is_selected)
    }

    /// Gets a mutable reference to the currently selected provider
    pub fn selected_provider_mut(&mut self) -> Option<&mut ProviderConfig> {
        self.providers.iter_mut().find(|p| p.is_selected)
    }

    /// Toggle between general settings and provider-specific settings
    pub fn toggle_provider_selection_mode(&mut self) {
        self.provider_selection_mode = !self.provider_selection_mode;
        self.selected_index = 0; // Reset selection when switching modes
    }

    /// Cycles through the options for an enum setting
    pub fn cycle_enum_value(&mut self) -> Result<()> {
        if let Some(setting) = self.selected_setting_mut() {
            if let SettingValue::Enum { current, options } = &mut setting.value {
                let current_index = options.iter().position(|o| o == current);
                if let Some(index) = current_index {
                    let next_index = (index + 1) % options.len();
                    *current = options[next_index].clone();
                }
            } else if let SettingValue::Dropdown { selected, options, .. } = &mut setting.value {
                *selected = (*selected + 1) % options.len();

                // Update provider selection
                if setting.key == "provider" {
                    let selected_value = *selected;
                    self.update_selected_provider(selected_value);
                }
            }
        }
        Ok(())
    }

    /// Updates the selected provider based on dropdown selection
    pub fn update_selected_provider(&mut self, selected_index: usize) {
        for (i, provider) in self.providers.iter_mut().enumerate() {
            provider.is_selected = i == selected_index;
        }
    }

    /// Toggles the dropdown expansion state
    pub fn toggle_dropdown(&mut self) -> Result<()> {
        if let Some(setting) = self.selected_setting_mut() {
            if let SettingValue::Dropdown { expanded, .. } = &mut setting.value {
                *expanded = !*expanded;
            }
        }
        Ok(())
    }

    /// Selects a dropdown option
    pub fn select_dropdown_option(&mut self, option_index: usize) -> Result<()> {
        if let Some(setting) = self.selected_setting_mut() {
            if let SettingValue::Dropdown { selected, options, expanded } = &mut setting.value {
                if option_index < options.len() {
                    *selected = option_index;
                    *expanded = false;

                    // Update provider selection
                    if setting.key == "provider" {
                        let selected_idx = *selected;
                        self.update_selected_provider(selected_idx);
                    }
                }
            }
        }
        Ok(())
    }

    /// Toggles a boolean setting
    pub fn toggle_bool_value(&mut self) -> Result<()> {
        if let Some(setting) = self.selected_setting_mut() {
            if let SettingValue::Bool(value) = &mut setting.value {
                *value = !*value;
            }
        }
        Ok(())
    }

    /// Updates a string setting
    pub fn update_string_value(&mut self, new_value: String) -> Result<()> {
        if let Some(setting) = self.selected_setting_mut() {
            if let SettingValue::String(_) = &setting.value {
                setting.value = SettingValue::String(new_value);
            }
        }
        Ok(())
    }

    pub fn handle_click(&mut self, x: u16, y: u16) -> Result<()> {
        // Check if click is on provider dropdown
        if let Some(setting) = self.selected_setting() {
            if setting.key == "provider" {
                let selected_idx = self.selected_index;
                if let SettingValue::Dropdown { expanded, options, selected } = &mut self.selected_setting_mut().unwrap().value {
                    if *expanded {
                        // Calculate which option was clicked
                        let option_index = (y as usize).saturating_sub(selected_idx + 1);
                        if option_index < options.len() {
                            *selected = option_index;
                            *expanded = false;
                            return Ok(());
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
