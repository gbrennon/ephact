use serde::Deserialize;

/// A declared input parameter for an action.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct ActionInput {
    #[serde(default)]
    description: Option<String>,

    #[serde(default)]
    required: bool,

    #[serde(default)]
    default: Option<String>,
}

impl ActionInput {
    pub fn new(description: Option<String>, required: bool, default: Option<String>) -> Self {
        Self {
            description,
            required,
            default,
        }
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn default_value(&self) -> Option<&str> {
        self.default.as_deref()
    }

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
