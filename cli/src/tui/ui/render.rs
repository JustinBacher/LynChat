//! UI rendering logic

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::tui::app::{AppMode, AppState};
use crate::tui::settings::SettingValue;

/// Renders the user interface widgets.
pub fn draw_ui(f: &mut Frame, app_state: &AppState) {
    let size = f.size();

    match app_state.mode {
        AppMode::Help => draw_help_screen(f, size),
        _ => {
            // Determine layout based on settings mode and collapsed state
            let chunks = if app_state.mode == AppMode::Settings {
                if app_state.settings.collapsed {
                    // When settings are collapsed, chat takes full width
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(100)])
                        .split(size)
                } else {
                    // When settings are expanded, chat takes 70% and settings 30%
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                        .split(size)
                }
            } else {
                // Normal chat mode - full width
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)])
                    .split(size)
            };

            // Draw chat area
            draw_chat_ui(f, app_state, chunks[0]);

            // Draw settings panel if open and not collapsed
            if app_state.mode == AppMode::Settings && !app_state.settings.collapsed {
                draw_settings_panel(f, app_state, chunks[1]);
            } else if app_state.mode == AppMode::Settings && app_state.settings.collapsed {
                // When collapsed, we still need to draw the collapsed settings panel
                // but it will be a minimal version with just the expand button
                draw_collapsed_settings_panel(f, app_state, size);
            }
        }
    }
}

/// Draws the main chat UI components
pub fn draw_chat_ui(f: &mut Frame, app_state: &AppState, area: Rect) {
    // Create vertical layout for chat area
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),    // Messages area
                Constraint::Length(3), // Input area
                Constraint::Length(1), // Status bar
            ]
            .as_ref(),
        )
        .split(area);

    // --- Messages Area ---
    // Join the Vec<String> messages into a single String with newlines
    let messages_text = app_state.messages.join("\n");
    let messages_paragraph = Paragraph::new(messages_text)
        .block(Block::default().borders(Borders::ALL).title("Conversation"))
        .wrap(Wrap { trim: true }); // Enable text wrapping
    f.render_widget(messages_paragraph, chunks[0]);

    // Draw settings gear button at top right
    draw_settings_button(f, app_state, chunks[0]);

    // --- Input Area ---
    let input_title = match app_state.mode {
        AppMode::Chat => "Input",
        AppMode::EditSetting => "Edit Setting",
        _ => "Input",
    };

    let input_text = match app_state.mode {
        AppMode::Chat => app_state.input.as_str(),
        AppMode::EditSetting => app_state.settings_input.as_str(),
        _ => app_state.input.as_str(),
    };

    let input_paragraph = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input_paragraph, chunks[1]);

    // --- Status Bar ---
    let status_paragraph = Paragraph::new(app_state.status.as_str())
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(status_paragraph, chunks[2]);
}

/// Draws the settings panel on the right side
pub fn draw_settings_panel(f: &mut Frame, app_state: &AppState, area: Rect) {
    // Render the settings panel block
    let settings_block = Block::default()
        .title("Settings")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));
    f.render_widget(settings_block, area);

    // Draw the expand/collapse button in the top-right corner of the settings panel
    let button_area = Rect::new(
        area.x + area.width - 5, // Position near the right edge
        area.y + 1,             // Position at the top with a small offset
        3,                      // Width of button
        3,                      // Height of button (increased for better visibility)
    );
    f.render_widget(&app_state.settings_expand_button, button_area);

    // If settings are collapsed, don't draw the content
    if app_state.settings.collapsed {
        return;
    }

    // Create inner area for content
    let inner_area = Rect::new(
        area.x + 2,
        area.y + 1,
        area.width.saturating_sub(4),
        area.height.saturating_sub(2),
    );

    // Split the inner area into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Provider selection
                Constraint::Min(1),    // Provider settings
                Constraint::Length(1), // Help text
            ]
            .as_ref(),
        )
        .split(inner_area);

    // Draw provider selection dropdown
    draw_provider_dropdown(f, app_state, chunks[0]);

    // Draw provider settings
    if app_state.settings.provider_selection_mode {
        draw_provider_settings(f, app_state, chunks[1]);
    } else {
        draw_general_settings(f, app_state, chunks[1]);
    }

    // Draw help text
    let help_text = match app_state.mode {
        AppMode::Settings => "↑/↓: Navigate | Enter: Edit | Tab: Switch | Esc: Close",
        AppMode::EditSetting => "Enter: Save | Esc: Cancel",
        _ => "",
    };

    let help_paragraph = Paragraph::new(help_text).style(Style::default().fg(Color::Gray));

    f.render_widget(help_paragraph, chunks[2]);
}

/// Draws the provider selection dropdown
fn draw_provider_dropdown(f: &mut Frame, app_state: &AppState, area: Rect) {
    // Get the provider setting
    if let Some(provider_setting) = app_state.settings.settings.first() {
        if let SettingValue::Dropdown {
            selected,
            options,
            expanded,
        } = &provider_setting.value
        {
            // Draw the dropdown header
            let dropdown_header = Paragraph::new(format!("Provider: {}", options[*selected]))
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Provider"));
            f.render_widget(dropdown_header, area);

            // Draw the dropdown options if expanded
            if *expanded {
                let dropdown_area =
                    Rect::new(area.x, area.y + 1, area.width, options.len() as u16 + 2);

                // Render a clear background
                f.render_widget(Clear, dropdown_area);

                // Create dropdown items
                let items: Vec<ListItem> = options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let style = if i == *selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        };

                        ListItem::new(Line::from(Span::styled(option, style)))
                    })
                    .collect();

                // Create and render the dropdown list
                let dropdown_list = List::new(items)
                    .block(Block::default().borders(Borders::ALL))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                f.render_widget(dropdown_list, dropdown_area);
            }
        }
    }
}

/// Draws the general settings
fn draw_general_settings(f: &mut Frame, app_state: &AppState, area: Rect) {
    // Create list items from settings
    let items: Vec<ListItem> = app_state
        .settings
        .settings
        .iter()
        .enumerate()
        .map(|(i, setting)| {
            let is_selected = i == app_state.settings.selected_index;
            let value_str = match &setting.value {
                SettingValue::String(s) => s.clone(),
                SettingValue::Bool(b) => {
                    if *b {
                        "Enabled".to_string()
                    } else {
                        "Disabled".to_string()
                    }
                }
                SettingValue::Enum { current, .. } => current.clone(),
                SettingValue::Dropdown {
                    selected, options, ..
                } => options[*selected].clone(),
            };

            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = Line::from(vec![
                Span::styled(format!("{}: ", setting.name), style),
                Span::styled(value_str, style),
            ]);

            ListItem::new(content)
        })
        .collect();

    // Create and render the list
    let settings_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("General Settings"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(settings_list, area);
}

/// Draws the provider-specific settings
fn draw_provider_settings(f: &mut Frame, app_state: &AppState, area: Rect) {
    // Get the selected provider
    if let Some(provider) = app_state.settings.selected_provider() {
        // Create list items from provider settings
        let items: Vec<ListItem> = provider
            .settings
            .iter()
            .enumerate()
            .map(|(i, setting)| {
                let is_selected = i == app_state.settings.selected_index;
                let value_str = match &setting.value {
                    SettingValue::String(s) => s.clone(),
                    SettingValue::Bool(b) => {
                        if *b {
                            "Enabled".to_string()
                        } else {
                            "Disabled".to_string()
                        }
                    }
                    SettingValue::Enum { current, .. } => current.clone(),
                    SettingValue::Dropdown {
                        selected, options, ..
                    } => options[*selected].clone(),
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let content = Line::from(vec![
                    Span::styled(format!("{}: ", setting.name), style),
                    Span::styled(value_str, style),
                ]);

                ListItem::new(content)
            })
            .collect();

        // Create and render the list
        let settings_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} Settings", provider.name)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(settings_list, area);
    }
}

fn draw_help_screen(f: &mut Frame, area: Rect) {
    let help_text = format!(
        "Keyboard Shortcuts
==================

General
-------
?          Show/hide this help screen
{}          Open/close settings
Ctrl + D    Quit application

Mouse Support
------------
⚙️          Click gear icon (top-right) to open/close settings
◀/▶        Click to expand/collapse settings panel

Chat Mode
---------
Enter      Send message
↑/↓        Scroll chat history

Settings Mode
-------------
↑/↓        Navigate settings
Enter      Edit selected setting
Tab        Switch sections
Space      Toggle dropdown
Esc        Exit settings",
        if cfg!(target_os = "macos") {
            "Cmd + ."
        } else {
            "Ctrl + ."
        }
    );

    let help_paragraph = Paragraph::new(help_text)
        .block(Block::default().title("Help").borders(Borders::ALL))
        .style(Style::default());

    f.render_widget(help_paragraph, area);
}

/// Draws a collapsed version of the settings panel with just the expand button
fn draw_collapsed_settings_panel(f: &mut Frame, app_state: &AppState, area: Rect) {
    // Calculate the position for the collapsed panel
    // We'll make it a narrow strip on the right side with just the expand button
    let collapsed_width = 5; // Just enough for the button

    let collapsed_area = Rect::new(
        area.width - collapsed_width,
        0,
        collapsed_width,
        area.height,
    );

    // Draw a minimal block for the collapsed panel
    let collapsed_block = Block::default()
        .borders(Borders::LEFT)
        .style(Style::default().bg(Color::Black));
    f.render_widget(collapsed_block, collapsed_area);

    // Draw the expand button in the collapsed panel
    let button_area = Rect::new(
        area.width - 4,        // Position near the right edge
        2,                     // Position at the top with a small offset
        3,                     // Width of button
        3,                     // Height of button
    );
    f.render_widget(&app_state.settings_expand_button, button_area);
}

/// Draws the settings gear button near the top right of the conversation area
fn draw_settings_button(f: &mut Frame, app_state: &AppState, area: Rect) {
    // Position the button differently based on whether settings are open or not
    let button_area = if app_state.mode == AppMode::Settings && !app_state.settings.collapsed {
        // When settings are open and expanded, position the button at the left edge of the settings panel
        // We need to estimate the position since we don't have direct access to the settings panel area
        let estimated_width = f.area().width;
        let settings_panel_width = estimated_width * 30 / 100; // 30% of screen width
        let settings_panel_x = estimated_width - settings_panel_width;

        Rect::new(
            settings_panel_x - 6, // Position to the left of the settings panel
            area.y + 2,          // Position a bit down from the top
            5,                   // Width of button
            3,                   // Height of button
        )
    } else {
        // In normal mode or when settings are collapsed, position at the right of the chat area
        Rect::new(
            area.x + area.width - 7, // Position a bit more to the left
            area.y + 2,             // Position a bit down from the top
            5,                      // Width of button
            3,                      // Height of button
        )
    };

    // Render the button using the one from app_state
    f.render_widget(&app_state.settings_button, button_area);
}
