use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),
}
