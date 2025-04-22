mod prelude;
mod tui;
mod logging;

use clap::{Parser, ValueEnum}; // Import ValueEnum
use tracing::{Level, debug, error, info}; // Import tracing macros and Level
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use common::{core::Engine, prelude::*};
use tui::run_tui; // Import the TUI runner function // Import core engine and prelude

// Define the client modes
#[derive(ValueEnum, Clone, Debug, Default)]
#[value(rename_all = "lowercase")]
enum ClientMode {
    #[default] // Default to TUI
    Tui,
    Gui,
    Web,
}

/// Lyn AI Assistant CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Initial prompt to send to the assistant
    #[arg(short, long)]
    prompt: Option<String>,

    /// Enable verbose logging (debug level)
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Client mode to run (tui, gui, web)
    #[arg(long, value_enum, default_value_t = ClientMode::Tui)]
    client: ClientMode,
}

#[async_std::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    logging::initialize_logging()?;

    info!("Starting Lyn CLI...");
    debug!("Parsed CLI arguments: {:?}", args);

    // Initialize the core engine
    // Handle potential errors during engine initialization
    let engine = match Engine::new().await {
        Ok(engine) => {
            info!("Core engine initialized successfully.");
            engine
        }
        Err(e) => {
            error!("Failed to initialize core engine: {}", e);
            // Consider specific error handling or mapping if needed
            return Err(e); // Propagate the error
        }
    };

    // --- Handle initial prompt if provided ---
    if let Some(initial_prompt) = args.prompt {
        info!("Processing initial prompt: '{}'", initial_prompt);
        match engine.process_prompt(&initial_prompt).await {
            Ok(response) => {
                println!("Assistant: {}", response); // Print response directly
            }
            Err(e) => {
                error!("Error processing initial prompt: {}", e);
                // Decide if we should exit or continue to interactive mode
                // For now, let's exit on initial prompt error
                return Err(e);
            }
        }
    } else {
        // --- Enter selected client mode ---
        match args.client {
            ClientMode::Tui => {
                info!("Entering TUI mode...");
                if let Err(e) = run_tui(&engine).await {
                    error!("TUI application error: {}", e);
                    // Optionally return the error to exit the CLI with an error code
                    // return Err(e);
                }
            }
            ClientMode::Gui => {
                info!("GUI mode selected.");
                println!("GUI client not yet implemented. Exiting.");
                // TODO: Launch GUI client process
            }
            ClientMode::Web => {
                info!("Web mode selected.");
                println!("Web client not yet implemented. Exiting.");
                // TODO: Launch Web client process/server
            }
        }
    }

    Ok(())
}

// TODO: Define run_tui function (LYN-17)
// async fn run_tui(engine: &Engine) -> Result<()> {
//     // Setup terminal, run TUI loop, restore terminal
//     Ok(())
// }
