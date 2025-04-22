//! TUI Application State

use common::config::AppConfig;
use common::prelude::*;
use tui_framework_experiment::button::Button;

use crate::tui::settings::{SettingValue, Settings};

/// Represents the different modes of the application
#[derive(Debug, PartialEq)]
pub enum AppMode {
    /// Normal chat mode
    Chat,
    /// Settings dialog mode
    Settings,
    /// Editing a specific setting
    EditSetting,
    /// Help screen mode
    Help,
}

/// Represents the state of the TUI application.
#[derive(Debug)]
pub struct AppState {
    /// Current value of the input field.
    pub input: String,
    /// History of messages (user prompts and assistant responses) as plain strings.
    pub messages: Vec<String>, // Store plain Strings
    /// Accumulates the current streaming response before parsing.
    pub current_response: String,
    /// Current status message or indicator.
    pub status: String,
    /// Vertical scroll offset for the messages area.
    pub scroll_offset: u16,
    /// Whether the message view should automatically scroll to the bottom.
    pub is_auto_scrolling: bool,
    /// Current application mode
    pub mode: AppMode,
    /// Settings for the application
    pub settings: Settings,
    /// Input field for editing settings
    pub settings_input: String,
    /// Whether settings have been modified and need to be saved
    pub settings_modified: bool,
    /// Settings button widget
    pub settings_button: Button<'static>,
    /// Settings expand/collapse button widget
    pub settings_expand_button: Button<'static>,
}

impl AppState {
    /// Creates a new instance of AppState.
    pub fn new() -> Self {
        // Create a default app config for initial settings
        let config = AppConfig::default();

        // Create the settings button
        let settings_button = Button::new("⚙️");

        // Create the settings expand/collapse button
        let settings_expand_button = Button::new("◀");

        Self {
            input: String::new(),
            messages: Vec::new(),
            current_response: String::new(), // Initialize empty
            status: "Ready. Type your prompt and press Enter.".to_string(),
            scroll_offset: 0,
            is_auto_scrolling: true, // Default to auto-scrolling
            mode: AppMode::Chat,
            settings: Settings::from_app_config(&config),
            settings_input: String::new(),
            settings_modified: false,
            settings_button,
            settings_expand_button,
        }
    }

    /// Creates a new instance of AppState with the given config.
    pub fn with_config(config: &AppConfig) -> Self {
        let mut state = Self::new();
        state.settings = Settings::from_app_config(config);
        state
    }

    /// Toggles the settings dialog
    pub fn toggle_settings(&mut self) {
        match self.mode {
            AppMode::Chat => {
                self.mode = AppMode::Settings;
                self.status = "Settings: Use arrow keys to navigate, Enter to edit, Tab to switch sections, Esc to exit".to_string();
                // Update button state to pressed
                self.settings_button.press();
                // Set expand button to normal state (expanded by default)
                self.settings_expand_button = Button::new("◀");
            }
            AppMode::Settings | AppMode::EditSetting => {
                self.mode = AppMode::Chat;
                self.status = "Ready. Type your prompt and press Enter.".to_string();
                // Update button state to normal
                self.settings_button.normal();
            }
            AppMode::Help => {
                self.mode = AppMode::Chat;
                self.status = "Ready. Type your prompt and press Enter.".to_string();
                // Update button state to normal
                self.settings_button.normal();
            }
        }
    }

    /// Toggles the expansion state of the settings panel
    pub fn toggle_settings_expansion(&mut self) {
        if self.mode == AppMode::Settings {
            // Toggle the collapsed state
            self.settings.collapsed = !self.settings.collapsed;

            // Update the button icon based on the new state
            if self.settings.collapsed {
                // Change to collapsed state (right arrow)
                self.settings_expand_button = Button::new("▶");
            } else {
                // Change to expanded state (left arrow)
                self.settings_expand_button = Button::new("◀");
            }
        }
    }

    /// Starts editing the currently selected setting
    pub fn start_edit_setting(&mut self) {
        if self.mode == AppMode::Settings {
            if let Some(setting) = self.settings.selected_setting() {
                match &setting.value {
                    SettingValue::String(value) => {
                        self.settings_input = value.clone();
                        self.mode = AppMode::EditSetting;
                        self.status = format!(
                            "Editing {}: Press Enter to save, Esc to cancel",
                            setting.name
                        );
                    }
                    SettingValue::Bool(_) => {
                        // Toggle boolean value directly
                        self.settings.toggle_bool_value().ok();
                        self.settings_modified = true;
                    }
                    SettingValue::Enum { .. } => {
                        // Cycle through enum values
                        self.settings.cycle_enum_value().ok();
                        self.settings_modified = true;
                    }
                    SettingValue::Dropdown { .. } => {
                        // Toggle dropdown expansion
                        self.settings.toggle_dropdown().ok();
                    }
                }
            }
        }
    }

    /// Saves the current setting being edited
    pub fn save_setting(&mut self) {
        if self.mode == AppMode::EditSetting {
            if let Some(setting) = self.settings.selected_setting() {
                match &setting.value {
                    SettingValue::String(_) => {
                        self.settings
                            .update_string_value(self.settings_input.clone())
                            .ok();
                        self.settings_input.clear();
                        self.mode = AppMode::Settings;
                        self.settings_modified = true;
                        self.status = "Setting updated. Use arrow keys to navigate, Enter to edit, Esc to exit".to_string();
                    }
                    _ => {} // Other types are handled directly
                }
            }
        }
    }

    /// Cancels editing the current setting
    pub fn cancel_edit_setting(&mut self) {
        if self.mode == AppMode::EditSetting {
            self.settings_input.clear();
            self.mode = AppMode::Settings;
            self.status =
                "Edit canceled. Use arrow keys to navigate, Enter to edit, Esc to exit".to_string();
        }
    }

    pub fn toggle_help(&mut self) {
        match self.mode {
            AppMode::Help => {
                self.mode = AppMode::Chat;
                self.status = "Ready. Type your prompt and press Enter.".to_string();
            }
            _ => {
                self.mode = AppMode::Help;
                self.status = "Help: Press ? or Esc to exit".to_string();
            }
        }
    }

    /// Checks if a click is on the settings gear button
    pub fn is_click_on_settings_button(&self, x: u16, y: u16) -> bool {
        // This should match the position in draw_settings_button
        // The button position depends on whether settings are open and expanded

        // Since we don't have direct access to the terminal size here,
        // we'll use a reasonable estimate for the right edge
        let estimated_width = 120; // Reasonable estimate for terminal width

        // Calculate button boundaries based on the mode and settings state
        let (button_x, button_y, button_width, button_height) = if self.mode == AppMode::Settings && !self.settings.collapsed {
            // When settings are open and expanded, the button is to the left of the settings panel
            let settings_panel_width = estimated_width * 30 / 100; // 30% of screen width
            let settings_panel_x = estimated_width - settings_panel_width;

            (
                settings_panel_x - 6, // Position to the left of the settings panel
                2,                    // Position a bit down from the top
                5,                    // Width of button
                3,                    // Height of button
            )
        } else {
            // In normal mode or when settings are collapsed, position at the right of the chat area
            (
                estimated_width - 7, // Position a bit more to the left
                2,                   // Position a bit down from the top
                5,                   // Width of button
                3,                   // Height of button
            )
        };

        // Add a small margin around the button to make it easier to click
        let margin = 1;

        // Check if the click is within the button area (with margin)
        let result = x >= button_x - margin
            && x < button_x + button_width + margin
            && y >= button_y - margin
            && y < button_y + button_height + margin;

        debug!(
            "Checking if click at ({}, {}) is on settings button: {}",
            x, y, result
        );
        result
    }
}
