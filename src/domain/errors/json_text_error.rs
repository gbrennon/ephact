/// Failure raised while reading JSON text into a [`ContextValue`].
///
/// [`ContextValue`]: crate::domain::value_objects::ContextValue
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct JsonTextError(String);

impl JsonTextError {
    /// Creates an error describing why the JSON text could not be read.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}
