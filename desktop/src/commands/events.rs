pub mod chat {
    /// Emitted when a chat session starts.
    pub const START: &str = "chat:start";

    /// Emitted when a chunk of the response is available.
    pub const CHUNK: &str = "chat:chunk";

    /// Emitted when the response is complete.
    pub const COMPLETE: &str = "chat:complete";

    /// Emitted when an error occurs.
    pub const ERROR: &str = "chat:error";

    /// Emitted when PII is detected in a message.
    pub const PII_DETECTED: &str = "chat:pii_detected";
}

pub mod settings {
    /// Emitted when settings are updated.
    pub const UPDATED: &str = "settings:updated";

    /// Emitted when settings are reset.
    pub const RESET: &str = "settings:reset";
}
