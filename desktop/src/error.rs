use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to initialize engine: {0}")]
    EngineInitFailed(String),

    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),

    #[error("Engine error: {0}")]
    EngineError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
