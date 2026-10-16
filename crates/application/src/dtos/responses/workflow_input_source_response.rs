/// Source used to resolve a workflow input value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowInputSourceResponse {
    /// A literal workflow input value.
    Literal(String),
    /// An environment variable containing a workflow input value.
    EnvironmentVariable(String),
}

impl WorkflowInputSourceResponse {
    /// Creates a literal input source.
    pub fn literal(value: String) -> Self {
        Self::Literal(value)
    }

    /// Creates an environment-variable input source.
    pub fn environment_variable(name: String) -> Self {
        Self::EnvironmentVariable(name)
    }

    /// Resolves the source into its input value.
    pub fn resolve(&self) -> Result<String, String> {
        match self {
            Self::Literal(value) => Ok(value.clone()),
            Self::EnvironmentVariable(name) => std::env::var(name).map_err(|_| {
                format!("input environment variable '{name}' is missing or unavailable")
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_resolves_to_the_supplied_value() {
        let source = WorkflowInputSourceResponse::literal("staging".to_owned());

        assert_eq!(source.resolve().unwrap(), "staging");
    }

    #[test]
    fn environment_variable_resolves_to_the_process_value() {
        let variable = "CARGO_MANIFEST_DIR";
        let source = WorkflowInputSourceResponse::environment_variable(variable.to_owned());

        assert_eq!(
            source.resolve().unwrap(),
            std::env::var(variable).expect("Cargo sets the manifest directory")
        );
    }

    #[test]
    fn environment_variable_reports_missing_value() {
        let variable = missing_environment_variable_name();
        let source = WorkflowInputSourceResponse::environment_variable(variable.clone());

        let error = source.resolve().unwrap_err();

        assert!(error.contains(&variable));
    }

    fn missing_environment_variable_name() -> String {
        (0..)
            .map(|index| format!("EPHACT_MISSING_INPUT_SOURCE_{index}"))
            .find(|name| std::env::var(name).is_err())
            .expect("a test environment variable name must be available")
    }
}
