use crate::domain::{
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
