/// Error returned when parsing fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Human-readable description of what went wrong.
    message: String,
    /// Approximate position in the input where the error occurred.
    position: usize,
}

impl ParseError {
    pub fn new(message: String, position: usize) -> Self {
        Self { message, position }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn position(&self) -> usize {
        self.position
    }
}
