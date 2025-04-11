//! TUI Application Runner

mod app;
mod ui;

use std::{io, time::Duration};

use async_channel::{Receiver, Sender, unbounded}; // Import async_channel
use async_std::task;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers}, // Add KeyModifiers
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    // Remove Text import if no longer needed directly here
};

use app::AppState;
use lyn_core::{core::Engine, prelude::*};
// Remove markdown_to_text import
use ui::draw_ui;

#[derive(Debug)]
enum StreamEvent {
    Chunk(String), // A piece of the response stream
    End,           // Stream finished successfully
    Error(String), // An error occurred during streaming
}

pub async fn run_tui(engine: &Engine) -> Result<()> {
    info!("Initializing TUI...");

    // --- Terminal Setup ---
    enable_raw_mode()?; // Put terminal in raw mode
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?; // Enter alternate screen, enable mouse capture
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    debug!("Terminal setup complete.");

    // --- Application State ---
    let mut app_state = AppState::new();
    let engine_clone = engine.clone(); // Clone engine for async task

    // --- Main Event Loop ---
    let run_result = run_event_loop(&mut terminal, &mut app_state, engine_clone).await;

    // --- Terminal Cleanup ---
    debug!("Restoring terminal...");
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    info!("TUI finished.");

    run_result // Return the result from the event loop
}

/// Handles the main TUI event loop: drawing UI and processing input.
async fn run_event_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    app_state: &mut AppState,
    engine: Engine, // Take ownership for the engine task
) -> Result<()> {
    // Create channels for communication
    // TUI -> Engine Task (Send prompts)
    let (prompt_tx, prompt_rx): (Sender<String>, Receiver<String>) = unbounded();
    // Engine Task -> TUI (Send stream events)
    let (event_tx, event_rx): (Sender<StreamEvent>, Receiver<StreamEvent>) = unbounded();

    // Spawn the engine processing task
    let engine_task = task::spawn(async move {
        info!("Engine task started.");
        // Clone the event sender for the task
        let event_tx_clone = event_tx.clone();
        while let Ok(prompt) = prompt_rx.recv().await {
            debug!("Engine task received prompt: '{}'", prompt);
            match engine.process_prompt_stream(&prompt).await {
                Ok(mut stream) => {
                    // Bring StreamExt into scope specifically for this block
                    use futures_util::StreamExt;
                    while let Some(result) = stream.next().await {
                        let event = match result {
                            Ok(chunk) => StreamEvent::Chunk(chunk),
                            Err(e) => StreamEvent::Error(e.to_string()),
                        };
                        if event_tx_clone.send(event).await.is_err() {
                            warn!("Engine task failed to send stream event: TUI receiver dropped.");
                            // Break inner loop, outer loop will check recv again
                            break;
                        }
                    }
                    // Send End event after stream finishes (if receiver still exists)
                    if event_tx_clone.send(StreamEvent::End).await.is_err() {
                        warn!("Engine task failed to send End event: TUI receiver dropped.");
                    }
                }
                Err(e) => {
                    // Send Error event if stream creation failed
                    if event_tx_clone
                        .send(StreamEvent::Error(e.to_string()))
                        .await
                        .is_err()
                    {
                        warn!("Engine task failed to send Error event: TUI receiver dropped.");
                    }
                }
            }
        }
        info!("Engine task finished.");
    });

    loop {
        // Draw the UI
        terminal.draw(|f| draw_ui(f, app_state))?;

        // Poll for events with a timeout
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    // Check for Ctrl+D to quit
                    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('d') {
                        info!("Ctrl+D pressed, quitting TUI loop.");
                        break; // Exit loop
                    }

                    match key.code {
                        // Remove the 'q' case
                        // KeyCode::Char('q') => { ... }
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
                                    app_state.status = "Error: Could not send prompt to engine. Please restart.".to_string();
                                    // Keep the loop running
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
                }
                Event::Mouse(_) => {}     // Ignore mouse events for now
                Event::Resize(_, _) => {} // Ratatui handles resize automatically
                _ => {}                   // Ignore other event types
            }
        }

        // Check for events from the engine task (non-blocking)
        match event_rx.try_recv() {
            Ok(StreamEvent::Chunk(chunk)) => {
                // Append chunk to accumulator
                app_state.current_response.push_str(&chunk);
                // If it's the first chunk, add a new message entry
                if app_state.messages.last().map_or(true, |m| m.starts_with("> ")) {
                     app_state.messages.push(format!("Assistant: {}", app_state.current_response));
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
            Ok(StreamEvent::End) => {
                app_state.current_response.clear(); // Clear accumulator
                app_state.status = "Ready. Type your prompt and press Enter.".to_string();
                 // Auto-scroll logic (repeat as after chunk): Use u16::MAX
                if app_state.is_auto_scrolling {
                    app_state.scroll_offset = u16::MAX;
                }
            }
            Ok(StreamEvent::Error(e)) => {
                // Format error message and push as plain string
                app_state.messages.push(format!("Error: {}", e)); // Push String directly
                app_state.current_response.clear(); // Clear accumulator on error too
                app_state.status = "Error occurred. Ready.".to_string();
                 // Auto-scroll logic for error message
                 if app_state.is_auto_scrolling {
                    app_state.scroll_offset = u16::MAX;
                }
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

    engine_task.await;
    // Cleanup: Drop the sender to signal the engine task to exit
    drop(prompt_tx);
    // Optionally wait for the engine task to finish, though dropping tx should suffice
    // engine_task.await; // Uncomment if explicit waiting is needed

    Ok(())
}
