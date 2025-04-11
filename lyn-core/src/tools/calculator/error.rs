#[derive(thiserror::Error, Debug)]
pub enum CalculatorError {
    #[error("Calculation failed: {0}")]
    Calculation(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error(transparent)]
    Meval(#[from] meval::Error),
}
