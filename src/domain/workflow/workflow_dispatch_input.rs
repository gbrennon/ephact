use serde::Deserialize;

/// An input parameter for `workflow_dispatch` events.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WorkflowDispatchInput {
    description: Option<String>,

    #[serde(default)]
    required: bool,

    #[serde(default)]
    default: Option<String>,

    #[serde(rename = "type")]
    #[serde(default)]
    input_type: Option<String>,

    #[serde(default)]
    options: Vec<String>,
}

impl WorkflowDispatchInput {
    pub fn new(
        description: Option<String>,
        required: bool,
        default: Option<String>,
        input_type: Option<String>,
        options: Vec<String>,
    ) -> Self {
        Self {
            description,
            required,
            default,
            input_type,
            options,
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

    pub fn input_type(&self) -> Option<&str> {
        self.input_type.as_deref()
    }

    pub fn options(&self) -> &[String] {
        &self.options
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_fields() {
        let input = WorkflowDispatchInput::new(
            Some("description".into()),
            true,
            Some("default".into()),
            Some("choice".into()),
            vec!["one".into()],
        );

        assert_eq!(input.description(), Some("description"));
        assert!(input.required());
        assert_eq!(input.default_value(), Some("default"));
        assert_eq!(input.default(), Some("default"));
        assert_eq!(input.input_type(), Some("choice"));
        assert_eq!(input.options(), &["one".to_string()]);
    }
}
