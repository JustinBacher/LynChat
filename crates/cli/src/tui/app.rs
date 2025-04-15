//! TUI Application State


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
    // TODO: Add focus state, etc. as needed.
}

impl AppState {
    /// Creates a new instance of AppState.
    pub fn new() -> Self {
        Self {
            input: String::new(),
            messages: Vec::new(),
            current_response: String::new(), // Initialize empty
            status: "Ready. Type your prompt and press Enter.".to_string(),
            scroll_offset: 0,
            is_auto_scrolling: true, // Default to auto-scrolling
        }
    }

    // TODO: Add methods to manipulate state (e.g., add_message, set_status).
}
