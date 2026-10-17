use crate::application::dtos::responses::WorkflowInputSourceResponse;

/// Parses CLI input and secret specifications.
pub struct InputParser {}

impl InputParser {
    /// Splits a `KEY=VALUE` argument at its first equals sign.
    pub fn parse_key_value(value: &str) -> Result<(String, String), String> {
        value
            .split_once('=')
            .map(|(key, parsed_value)| (key.to_owned(), parsed_value.to_owned()))
            .ok_or_else(|| format!("expected KEY=VALUE, got '{value}'"))
    }

    /// Parses a workflow input and identifies environment-backed values.
    pub fn parse_input_source(
        value: &str,
    ) -> Result<(String, WorkflowInputSourceResponse), String> {
        let (key, parsed_value) = Self::parse_key_value(value)?;
        let source = parsed_value
            .strip_prefix("env:")
            .map(|name| WorkflowInputSourceResponse::environment_variable(name.to_owned()))
            .unwrap_or_else(|| WorkflowInputSourceResponse::literal(parsed_value.to_owned()));
        Ok((key, source))
    }

    /// Resolves an inline secret value or reads it from the matching environment variable.
    pub fn parse_secret(value: &str) -> Result<(String, String), String> {
        match value.split_once('=') {
            Some((name, secret)) => Ok((name.to_owned(), secret.to_owned())),
            None => std::env::var(value)
                .map(|secret| (value.to_owned(), secret))
                .map_err(|_| {
                    format!("secret '{value}' has no value and no environment variable is set")
                }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InputParser;

    #[test]
    fn parse_key_value_splits_at_first_equals() {
        let parsed = InputParser::parse_key_value("KEY=value=tail").unwrap();

        assert_eq!(parsed, ("KEY".to_owned(), "value=tail".to_owned()));
    }

    #[test]
    fn parse_secret_reads_an_environment_value_when_no_inline_value_exists() {
        let parsed = InputParser::parse_secret("CARGO_MANIFEST_DIR").unwrap();

        assert_eq!(parsed.0, "CARGO_MANIFEST_DIR");
        assert_eq!(parsed.1, std::env::var("CARGO_MANIFEST_DIR").unwrap());
    }
}
