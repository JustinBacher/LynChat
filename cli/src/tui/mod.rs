//! TUI Application Runner

pub mod app;
pub mod engine;
pub mod events;
pub mod settings;
pub mod terminal;
pub mod ui;

use async_channel::{Receiver, Sender, unbounded};
use common::{core::Engine, prelude::*};

use app::AppState;
use engine::spawn_engine_task;
use events::{handle_events, StreamEvent};
use terminal::{restore_terminal, setup_terminal};

/// Runs the TUI application
pub async fn run_tui(engine: &Engine) -> Result<()> {
    info!("Initializing TUI...");

    // --- Terminal Setup ---
    let mut terminal = setup_terminal()?;

    // --- Application State ---
    // Get the config from the engine to initialize settings
    let config = engine.get_config();
    let mut app_state = AppState::with_config(&config);
    let engine_clone = engine.clone(); // Clone engine for async task

    // --- Create channels for communication ---
    // TUI -> Engine Task (Send prompts)
    let (prompt_tx, prompt_rx): (Sender<String>, Receiver<String>) = unbounded();
    // Engine Task -> TUI (Send stream events)
    let (event_tx, event_rx): (Sender<StreamEvent>, Receiver<StreamEvent>) = unbounded();

    // --- Spawn the engine processing task ---
    let engine_task = spawn_engine_task(engine_clone, prompt_rx, event_tx);

    // --- Main Event Loop ---
    let run_result = handle_events(&mut terminal, &mut app_state, prompt_tx, event_rx).await;

    // --- Terminal Cleanup ---
    if let Err(e) = restore_terminal(terminal) {
        error!("Failed to restore terminal: {}", e);
    }

    // Save settings if they were modified
    if app_state.settings_modified {
        info!("Saving modified settings...");
        let mut config = engine.get_config();
        if let Err(e) = app_state.settings.update_app_config(&mut config) {
            error!("Failed to update config: {}", e);
        } else {
            // Save config to file
            if let Err(e) = common::config::save_config(&config) {
                error!("Failed to save config to file: {}", e);
            } else {
                info!("Settings saved successfully.");
            }
        }
    }

    // IMPORTANT: Wait for the engine task to finish
    // This ensures all resources are properly cleaned up before exiting
    engine_task.await;

    // Return the result from the event loop
    run_result
}
