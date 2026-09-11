#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowInputSourceResponse {
    Literal(String),
    EnvironmentVariable(String),
}

impl WorkflowInputSourceResponse {
    pub fn literal(value: impl Into<String>) -> Self {
        Self::Literal(value.into())
    }

    pub fn environment_variable(name: impl Into<String>) -> Self {
        Self::EnvironmentVariable(name.into())
    }

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
        let source = WorkflowInputSourceResponse::literal("staging");

        assert_eq!(source.resolve().unwrap(), "staging");
    }

    #[test]
    fn environment_variable_resolves_to_the_process_value() {
        unsafe {
            std::env::set_var("EPHACT_WORKFLOW_INPUT_SOURCE_TEST", "production");
        }
        let source =
            WorkflowInputSourceResponse::environment_variable("EPHACT_WORKFLOW_INPUT_SOURCE_TEST");

        assert_eq!(source.resolve().unwrap(), "production");
    }

    #[test]
    fn environment_variable_reports_missing_value() {
        unsafe {
            std::env::remove_var("EPHACT_WORKFLOW_INPUT_SOURCE_MISSING_TEST");
        }
        let source = WorkflowInputSourceResponse::environment_variable(
            "EPHACT_WORKFLOW_INPUT_SOURCE_MISSING_TEST",
        );

        let error = source.resolve().unwrap_err();

        assert!(error.contains("EPHACT_WORKFLOW_INPUT_SOURCE_MISSING_TEST"));
    }
}
