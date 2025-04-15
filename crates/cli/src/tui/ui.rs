//! TUI User Interface Rendering

// Remove pulldown_cmark imports
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style}, // Remove unused Modifier
    widgets::{Block, Borders, Paragraph, Wrap}, // Import Wrap
    Frame,
};

use super::app::AppState; // Use super to access app module in the same directory

/// Renders the user interface widgets.
pub fn draw_ui(f: &mut Frame, app_state: &AppState) {
    // Define the main layout chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),      // Main content area (messages)
            Constraint::Length(3),   // Input area
            Constraint::Length(1),   // Status bar
        ].as_ref())
        .split(f.size());

    // --- Messages Area ---
    // Join the Vec<String> messages into a single String with newlines
    let messages_text = app_state.messages.join("\n");
    let messages_paragraph = Paragraph::new(messages_text) // Pass the single String
        .block(Block::default().borders(Borders::ALL).title("Conversation"))
        .wrap(Wrap { trim: true }); // Enable text wrapping, REMOVE .scroll() for testing
    f.render_widget(messages_paragraph, chunks[0]);

    // --- Input Area ---
    let input_paragraph = Paragraph::new(app_state.input.as_str())
        .style(Style::default().fg(Color::Yellow)) // Style for input text
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input_paragraph, chunks[1]);
    // TODO: Show cursor in the input box (requires more advanced state management)
    // f.set_cursor(
    //     chunks[1].x + app_state.input.len() as u16 + 1, // +1 for border
    //     chunks[1].y + 1, // +1 for border
    // );

    // --- Status Bar ---
    let status_paragraph = Paragraph::new(app_state.status.as_str())
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE)); // No border for status
    f.render_widget(status_paragraph, chunks[2]);
}

// Remove the markdown_to_text function entirely
