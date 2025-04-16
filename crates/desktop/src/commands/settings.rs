use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    theme: String,
    font_size: String,
    llm_provider: String,
    model_name: String,
    store_conversations: bool,
    allow_anonymized_data: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            font_size: "medium".to_string(),
            llm_provider: "local".to_string(),
            model_name: "llama3.2:1b".to_string(),
            store_conversations: true,
            allow_anonymized_data: false,
        }
    }
}

#[tauri::command]
pub async fn get_settings(app_handle: AppHandle) -> Result<Settings, String> {
    let settings_path = get_settings_path(&app_handle)
        .map_err(|e| format!("Failed to get settings path: {}", e))?;

    if !settings_path.exists() {
        return Ok(Settings::default());
    }

    let settings_str = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let settings: Settings = serde_json::from_value(Value::String(settings_str))
        .map_err(|e| format!("Failed to deserialize settings: {}", e))?;
    Ok(settings)
}

#[tauri::command]
pub async fn update_settings(app_handle: AppHandle, settings: Settings) -> Result<(), String> {
    let settings_path = get_settings_path(&app_handle)
        .map_err(|e| format!("Failed to get settings path: {}", e))?;

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }

    let settings_json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&settings_path, settings_json)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn reset_settings(app_handle: AppHandle) -> Result<Settings, String> {
    let settings = Settings::default();
    update_settings(app_handle.clone(), settings.clone()).await?;
    Ok(settings)
}

fn get_settings_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get config directory {}", e))
        .map(|p| p.join("settings.json"))
}
