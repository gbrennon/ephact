/// Error returned when parsing fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Human-readable description of what went wrong.
    message: String,
    /// Approximate position in the input where the error occurred.
    position: usize,
}

impl ParseError {
    #[must_use]
    pub fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn position(&self) -> usize {
        self.position
    }
}
