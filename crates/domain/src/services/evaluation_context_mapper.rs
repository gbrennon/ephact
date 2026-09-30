use crate::{
    errors::JsonTextError,
    value_objects::{ContextValue, EvaluationContext},
};

/// Maps evaluation contexts to and from primitive JSON-text root values.
pub struct EvaluationContextMapper;

impl From<EvaluationContext> for Vec<(String, String)> {
    fn from(context: EvaluationContext) -> Self {
        EvaluationContextMapper::to_parts(&context)
    }
}

impl EvaluationContextMapper {
    /// Flattens every evaluation-context root into a named JSON string.
    pub fn to_parts(context: &EvaluationContext) -> Vec<(String, String)> {
        [
            ("github", context.github()),
            ("env", context.env()),
            ("job", context.job()),
            ("steps", context.steps()),
            ("runner", context.runner()),
            ("secrets", context.secrets()),
            ("vars", context.vars()),
            ("strategy", context.strategy()),
            ("matrix", context.matrix()),
            ("needs", context.needs()),
            ("inputs", context.inputs()),
        ]
        .into_iter()
        .map(|(name, value)| (name.to_string(), value.to_json_text()))
        .collect()
    }

    /// Reconstructs an evaluation context from named JSON string roots.
    pub fn from_parts(parts: Vec<(String, String)>) -> Result<EvaluationContext, JsonTextError> {
        let mut context = EvaluationContext::new();
        for (name, text) in parts {
            let value = ContextValue::from_json_text(&text)?;
            context = match name.as_str() {
                "github" => context.with_github(value),
                "env" => context.with_env(value),
                "job" => context.with_job(value),
                "steps" => context.with_steps(value),
                "runner" => context.with_runner(value),
                "secrets" => context.with_secrets(value),
                "vars" => context.with_vars(value),
                "strategy" => context.with_strategy(value),
                "matrix" => context.with_matrix(value),
                "needs" => context.with_needs(value),
                "inputs" => context.with_inputs(value),
                _ => context,
            };
        }
        Ok(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every_root_name() -> [&'static str; 11] {
        [
            "github", "env", "job", "steps", "runner", "secrets", "vars", "strategy", "matrix",
            "needs", "inputs",
        ]
    }

    #[test]
    fn to_parts_flattens_every_root_into_named_json() {
        let context = EvaluationContext::new().with_env(ContextValue::text("value"));

        let parts = EvaluationContextMapper::to_parts(&context);

        let names: Vec<&str> = parts.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, every_root_name());
    }

    #[test]
    fn from_parts_rebuilds_every_known_root() {
        let parts = every_root_name()
            .into_iter()
            .map(|name| (name.to_string(), "\"value\"".to_string()))
            .collect();

        let context = EvaluationContextMapper::from_parts(parts).unwrap();

        assert_eq!(context.env().as_text(), Some("value"));
        assert_eq!(context.inputs().as_text(), Some("value"));
    }

    #[test]
    fn from_parts_ignores_unknown_roots() {
        let parts = vec![("unknown".to_string(), "\"value\"".to_string())];

        let context = EvaluationContextMapper::from_parts(parts).unwrap();

        assert_eq!(context.github().as_text(), None);
    }

    #[test]
    fn from_parts_reports_invalid_json_text() {
        let parts = vec![("github".to_string(), "\"unterminated".to_string())];

        let result = EvaluationContextMapper::from_parts(parts);

        assert!(result.is_err());
    }

    #[test]
    fn into_vec_uses_the_mapper() {
        let context = EvaluationContext::new();

        let parts: Vec<(String, String)> = context.into();

        assert_eq!(parts.len(), 11);
    }
}
