// crates/tauri/src/main.rs
mod app;
mod commands;
mod error;
mod prelude;

use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .init();

    info!("Starting Lyn Desktop...");

    // Run the app
    app::run_app().await?;

    info!("Lyn Desktop exited.");
    Ok(())
}
