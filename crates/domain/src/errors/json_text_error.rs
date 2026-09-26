/// Failure raised while reading JSON text into a [`ContextValue`].
///
/// [`ContextValue`]: crate::value_objects::ContextValue
#[derive(Debug)]
pub struct JsonTextError(String);

impl std::fmt::Display for JsonTextError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for JsonTextError {}

impl JsonTextError {
    /// Creates an error describing why the JSON text could not be read.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::JsonTextError;

    #[test]
    fn exposes_message_as_standard_error() {
        let error = JsonTextError::new("invalid JSON");

        assert_eq!(error.to_string(), "invalid JSON");
        assert!(error.source().is_none());
    }
}
