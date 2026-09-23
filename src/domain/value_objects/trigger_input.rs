#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TriggerInput {
    description: Option<String>,
    required: bool,
    default_value: Option<String>,
    value_type: Option<String>,
    options: Vec<String>,
}

impl TriggerInput {
    pub fn new(
        description: Option<String>,
        required: bool,
        default_value: Option<String>,
        value_type: Option<String>,
        options: Vec<String>,
    ) -> Self {
        Self {
            description,
            required,
            default_value,
            value_type,
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
        self.default_value.as_deref()
    }

    pub fn value_type(&self) -> Option<&str> {
        self.value_type.as_deref()
    }

    pub fn options(&self) -> &[String] {
        &self.options
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_generic_parameter_definition() {
        let input = TriggerInput::new(
            Some("Environment name".into()),
            true,
            Some("test".into()),
            Some("string".into()),
            vec!["test".into(), "production".into()],
        );

        assert_eq!(input.description(), Some("Environment name"));
        assert!(input.required());
        assert_eq!(input.default_value(), Some("test"));
        assert_eq!(input.value_type(), Some("string"));
        assert_eq!(
            input.options(),
            &["test".to_string(), "production".to_string()]
        );
    }
}
