use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),
}

impl<T: std::fmt::Debug> From<nom::Err<T>> for CoreError {
    fn from(value: nom::Err<T>) -> Self {
        Self::SyntaxError(format!("{}", value))
    }
}
