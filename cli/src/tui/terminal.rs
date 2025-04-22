//! Terminal setup and cleanup

use std::io;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    event::{DisableMouseCapture, EnableMouseCapture, PushKeyboardEnhancementFlags, KeyboardEnhancementFlags},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use common::prelude::*;

/// Sets up the terminal for the TUI
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    info!("Setting up terminal...");
    enable_raw_mode().map_err(|e| Error::Io(e))?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES |
            KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES |
            KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS |
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    ).map_err(|e| Error::Io(e))?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).map_err(|e| Error::Io(e))?;
    info!("Terminal setup complete.");
    Ok(terminal)
}

/// Restores the terminal to its original state
pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    info!("Restoring terminal...");
    disable_raw_mode().map_err(|e| Error::Io(e))?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).map_err(|e| Error::Io(e))?;
    terminal.show_cursor().map_err(|e| Error::Io(e))?;
    info!("Terminal restored.");
    Ok(())
}
