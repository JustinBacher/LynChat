//! Event handling logic for the TUI

use std::time::Duration;

use async_channel::{Receiver, Sender};
use common::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use ratatui::{Terminal, backend::Backend};

use crate::tui::app::{AppMode, AppState};
use crate::tui::events::stream::StreamEvent;
use crate::tui::ui::render::draw_ui;

/// Handles a single input event
pub async fn handle_key_event(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut AppState,
    prompt_tx: &Sender<String>,
) -> Result<bool> {
    // Debug log to see what modifiers are being detected - now goes to stderr
    debug!(
        "Key Event - code: {:?}, modifiers: {:?}, kind: {:?}, state: {:?}",
        key_event.code, key_event.modifiers, key_event.kind, key_event.state
    );

    // Handle both modern and legacy modifier detection
    match key_event.code {
        // Modern way - explicit modifier key detection
        KeyCode::Modifier(modifier_code) => {
            debug!("Modifier key detected: {:?}", modifier_code);
            return Ok(false);
        }

        // Legacy way - check for control characters and modifiers
        KeyCode::Char(c) => {
            let is_control = key_event.modifiers.contains(KeyModifiers::CONTROL) ||
                           (c as u8 <= 26); // ASCII control characters

            if is_control {
                match c.to_ascii_lowercase() {
                    'c' => return Ok(true), // Ctrl+C
                    'd' => return Ok(true), // Ctrl+D
                    '.' => {
                        app_state.toggle_settings();
                        return Ok(false);
                    }
                    _ => {
                        debug!("Unhandled control combination: Ctrl+{}", c);
                        return Ok(false);
                    }
                }
            }
        }
        _ => {}
    }

    // If we have any modifier keys pressed (except shift), don't process as regular input
    if key_event.modifiers != KeyModifiers::NONE && key_event.modifiers != KeyModifiers::SHIFT {
        return Ok(false);
    }

    // Handle keys based on current mode
    match app_state.mode {
        AppMode::Chat => handle_chat_mode_key(key_event, app_state, prompt_tx).await,
        AppMode::Settings => handle_settings_mode_key(key_event, app_state),
        AppMode::EditSetting => handle_edit_setting_mode_key(key_event, app_state),
        AppMode::Help => handle_help_mode_key(key_event, app_state),
    }
}

/// Handles key events in chat mode
async fn handle_chat_mode_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut AppState,
    prompt_tx: &Sender<String>,
) -> Result<bool> {
    match key_event.code {
        KeyCode::Enter => {
            let prompt_text = app_state.input.drain(..).collect::<String>();
            if !prompt_text.is_empty() {
                // Format user input and push as plain string
                let formatted_prompt = format!("> {}", prompt_text);
                app_state.messages.push(formatted_prompt); // Push String directly
                app_state.status = "Processing...".to_string();
                app_state.is_auto_scrolling = true; // Re-enable auto-scroll on new prompt
                app_state.current_response.clear(); // Clear accumulator for new response
                // Send the raw prompt text to the engine task
                if prompt_tx.send(prompt_text).await.is_err() {
                    error!("Failed to send prompt to engine task: channel closed.");
                    // Update status instead of breaking
                    app_state.status =
                        "Error: Could not send prompt to engine. Please restart.".to_string();
                }
            }
        }
        KeyCode::Char(c) => {
            app_state.input.push(c);
        }
        KeyCode::Backspace => {
            app_state.input.pop();
        }
        KeyCode::Up => {
            // Scroll up (decrease offset), but not below 0
            app_state.scroll_offset = app_state.scroll_offset.saturating_sub(1);
            app_state.is_auto_scrolling = false; // Disable auto-scroll on manual scroll
        }
        KeyCode::Down => {
            // Simply increment offset; let ratatui handle clamping during render
            app_state.scroll_offset = app_state.scroll_offset.saturating_add(1);
            app_state.is_auto_scrolling = false; // Disable auto-scroll on manual scroll
        }
        // TODO: Handle PageUp/PageDown, Home/End for faster scrolling
        _ => {}
    }

    Ok(false) // Continue loop
}

/// Handles key events in settings mode
fn handle_settings_mode_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut AppState,
) -> Result<bool> {
    match key_event.code {
        KeyCode::Esc => {
            app_state.toggle_settings();
        }
        KeyCode::Enter => {
            // If we're on the provider dropdown and it's expanded, select the option
            if let Some(setting) = app_state.settings.selected_setting() {
                if setting.key == "provider" {
                    if let crate::tui::settings::SettingValue::Dropdown { expanded, .. } =
                        &setting.value
                    {
                        if *expanded {
                            // Handle dropdown selection
                            app_state.settings.toggle_dropdown().ok();
                            return Ok(false);
                        }
                    }
                }
            }

            // Otherwise, start editing the setting
            app_state.start_edit_setting();
        }
        KeyCode::Tab => {
            // Toggle between general settings and provider-specific settings
            app_state.settings.toggle_provider_selection_mode();
        }
        KeyCode::Up => {
            app_state.settings.select_prev();
        }
        KeyCode::Down => {
            app_state.settings.select_next();
        }
        KeyCode::Char(' ') => {
            // Toggle dropdown expansion
            if let Some(setting) = app_state.settings.selected_setting() {
                if let crate::tui::settings::SettingValue::Dropdown { .. } = &setting.value {
                    app_state.settings.toggle_dropdown().ok();
                }
            }
        }
        _ => {}
    }

    Ok(false) // Continue loop
}

/// Handles key events in edit setting mode
fn handle_edit_setting_mode_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut AppState,
) -> Result<bool> {
    match key_event.code {
        KeyCode::Esc => {
            app_state.cancel_edit_setting();
        }
        KeyCode::Enter => {
            app_state.save_setting();
            app_state.settings_modified = true;
        }
        KeyCode::Char(c) => {
            app_state.settings_input.push(c);
        }
        KeyCode::Backspace => {
            app_state.settings_input.pop();
        }
        _ => {}
    }

    Ok(false) // Continue loop
}

fn handle_help_mode_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut AppState,
) -> Result<bool> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('?') => {
            app_state.toggle_help();
        }
        _ => {}
    }
    Ok(false)
}

/// Handles mouse events
fn handle_mouse_event(
    mouse_event: MouseEvent,
    app_state: &mut AppState,
) {
    // Only process mouse clicks
    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
        let x = mouse_event.column;
        let y = mouse_event.row;

        // Check if we're in settings mode
        if app_state.mode == AppMode::Settings {
            // Check if click is on the settings expand/collapse button
            // The position depends on whether settings are collapsed or expanded
            let estimated_width = 120; // Reasonable estimate for terminal width

            let (button_x, button_y, button_width, button_height) = if app_state.settings.collapsed {
                // When collapsed, the button is on the right edge of the screen
                (
                    estimated_width - 4, // Position near the right edge
                    2,                   // Position at the top with a small offset
                    3,                   // Width of button
                    3,                   // Height of button
                )
            } else {
                // When expanded, the button is in the top-right of the settings panel
                let settings_panel_width = estimated_width * 30 / 100; // 30% of screen width
                let settings_panel_x = estimated_width - settings_panel_width;

                (
                    settings_panel_x + settings_panel_width - 5, // Position near the right edge of settings panel
                    1,                                          // Top of settings panel + 1
                    3,                                          // Width of button
                    3,                                          // Height of button
                )
            };

            debug!("Click at ({}, {}), button area: ({}, {}) to ({}, {})",
                   x, y, button_x, button_y, button_x + button_width, button_y + button_height);

            // Check if the click is within the button area
            // Add a small margin to make it easier to click
            let margin = 1;
            if x >= button_x - margin && x < button_x + button_width + margin &&
               y >= button_y - margin && y < button_y + button_height + margin {
                // Toggle settings expansion
                app_state.toggle_settings_expansion();
            }
        } else {
            // Check if click is on the settings gear button
            if app_state.is_click_on_settings_button(x, y) {
                app_state.toggle_settings();
            }
        }
    }
}

/// Handles stream events from the engine
pub fn handle_stream_event(stream_event: StreamEvent, app_state: &mut AppState) {
    match stream_event {
        StreamEvent::Chunk(chunk) => {
            // Append chunk to accumulator
            app_state.current_response.push_str(&chunk);
            // If it's the first chunk, add a new message entry
            if app_state
                .messages
                .last()
                .is_none_or(|m| m.starts_with("> "))
            {
                app_state
                    .messages
                    .push(format!("Assistant: {}", app_state.current_response));
            } else if let Some(last_message) = app_state.messages.last_mut() {
                // Otherwise, update the last message (which should be the assistant's)
                *last_message = format!("Assistant: {}", app_state.current_response);
            }
            app_state.status = "Streaming...".to_string();
            // Auto-scroll logic: Use u16::MAX to scroll to bottom
            if app_state.is_auto_scrolling {
                app_state.scroll_offset = u16::MAX;
            }
        }
        StreamEvent::End => {
            app_state.current_response.clear(); // Clear accumulator
            app_state.status = "Ready. Type your prompt and press Enter.".to_string();
            // Auto-scroll logic (repeat as after chunk): Use u16::MAX
            if app_state.is_auto_scrolling {
                app_state.scroll_offset = u16::MAX;
            }
        }
        StreamEvent::Error(e) => {
            // Format error message and push as plain string
            app_state.messages.push(format!("Error: {}", e)); // Push String directly
            app_state.current_response.clear(); // Clear accumulator on error too
            app_state.status = "Error occurred. Ready.".to_string();
            // Auto-scroll logic for error message
            if app_state.is_auto_scrolling {
                app_state.scroll_offset = u16::MAX;
            }
        }
    }
}

/// Handles the main event loop for the TUI
pub async fn handle_events<B: Backend>(
    terminal: &mut Terminal<B>,
    app_state: &mut AppState,
    prompt_tx: Sender<String>,
    event_rx: Receiver<StreamEvent>,
) -> Result<()> {
    loop {
        // Draw the UI
        terminal
            .draw(|f| draw_ui(f, app_state))
            .map_err(|e| Error::Io(e))?;

        // Poll for events with a timeout
        if event::poll(Duration::from_millis(100)).map_err(|e| Error::Io(e))? {
            match event::read().map_err(|e| Error::Io(e))? {
                Event::Key(key) => {
                    // Debug log to help diagnose modifier key issues - now goes to stderr
                    debug!(
                        "Key Event - code: {:?}, modifiers: {:?}, kind: {:?}, state: {:?}",
                        key.code, key.modifiers, key.kind, key.state
                    );

                    // Handle global key combinations first
                    let is_control = key.modifiers.contains(KeyModifiers::CONTROL);
                    let is_super = key.modifiers.contains(KeyModifiers::SUPER);

                    if is_control || is_super {
                        match key.code {
                            KeyCode::Char('d') | KeyCode::Char('D') => {
                                break; // Exit the application
                            }
                            KeyCode::Char('.') => {
                                app_state.toggle_settings();
                                continue;
                            }
                            _ => {}
                        }
                    }

                    if handle_key_event(key, app_state, &prompt_tx).await? {
                        break; // Exit loop if handler returns true
                    }
                }
                Event::Mouse(mouse_event) => {
                    // Handle mouse events
                    handle_mouse_event(mouse_event, app_state);
                }
                Event::Resize(_, _) => {} // Ratatui handles resize automatically
                _ => {}                   // Ignore other event types
            }
        }

        // Check for events from the engine task (non-blocking)
        match event_rx.try_recv() {
            Ok(stream_event) => {
                handle_stream_event(stream_event, app_state);
            }
            Err(async_channel::TryRecvError::Empty) => {
                // No event from engine, continue loop
            }
            Err(async_channel::TryRecvError::Closed) => {
                error!("Engine task event channel closed unexpectedly.");
                break; // Exit loop if channel is closed
            }
        }
    }

    Ok(())
}
