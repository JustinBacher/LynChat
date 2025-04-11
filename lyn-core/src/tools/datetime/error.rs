#[derive(thiserror::Error, Debug)]
pub enum DateTimeError {
    #[error("Failed to parse date/time string: {0}")]
    Parse(String),

    #[error("Failed to format date/time string: {0}")]
    Format(String),
}
