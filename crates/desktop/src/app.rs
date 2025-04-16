use crate::commands::{chat, settings};
use crate::error::AppError;
use lyn_core::core::Engine;
use tracing::{error, info};
use std::sync::Arc;
use tauri::{generate_context, Manager, WebviewUrl, WebviewWindowBuilder};

pub async fn run_app() -> Result<(), AppError> {
    // Initialize the core engine
    let engine = match Engine::new().await {
        Ok(engine) => {
            info!("Core engine initialized successfully.");
            Arc::new(engine)
        }
        Err(e) => {
            error!("Failed to initialize core engine: {}", e);
            return Err(AppError::EngineInitFailed(e.to_string()));
        }
    };

    let builder = tauri::Builder::default()
        .setup(move |app| {
            // Make engine available to command handlers
            app.manage(engine.clone());
            // Remove manual creation of the main window; Tauri creates it by default
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            chat::send_message,
            chat::send_prompt,
            settings::get_settings,
            settings::update_settings,
            settings::reset_settings,
        ]);

    builder.run(generate_context!())?;

    Ok(())
}
