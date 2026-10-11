/// A declared input parameter for an action.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ActionInput {
    description: Option<String>,
    required: bool,
    default: Option<String>,
}

impl ActionInput {
    /// Creates an action input declaration.
    pub fn new(description: Option<String>, required: bool, default: Option<String>) -> Self {
        Self {
            description,
            required,
            default,
        }
    }

    /// Returns the input description, if one was provided.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns whether the input is required.
    pub fn required(&self) -> bool {
        self.required
    }

    /// Returns the input's declared default value, if present.
    pub fn default_value(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Returns the input's default value, if present.
    pub fn default(&self) -> Option<&str> {
        self.default.as_deref()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_fields() {
        let input = ActionInput::new(Some("description".into()), true, Some("default".into()));

        assert_eq!(input.description(), Some("description"));
        assert!(input.required());
        assert_eq!(input.default_value(), Some("default"));
        assert_eq!(input.default(), Some("default"));
    }
}
