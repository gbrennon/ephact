use std::collections::HashMap;

use super::{EvalContext, EvalError, ExpressionResolver};
use crate::domain::workflow::Step;

/// Produces a copy of a step with every `${{ }}` expression in its
/// user-supplied fields replaced by its evaluated value.
///
/// Interpolation covers the fields a step hands to the runner: `name`, `run`,
/// `uses`, `working-directory`, and the `with:`/`env:` maps. Control fields
/// (`if`, `continue-on-error`, `timeout-minutes`) are left as authored because
/// they are evaluated as expressions in their own right.
pub struct StepInterpolator;
type InterpolatedCoreFields = (Option<String>, Option<String>, Option<String>);
type InterpolatedActionFields = (
    Option<String>,
    HashMap<String, String>,
    HashMap<String, String>,
);

impl StepInterpolator {
    /// Interpolates `step` against `context`.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError`] when one of the step's expressions cannot be
    /// parsed.
    pub fn interpolate(step: &Step, context: &EvalContext) -> Result<Step, EvalError> {
        let (name, run, working_directory) = Self::interpolate_core_fields(step, context)?;
        let (uses, with, env) = Self::interpolate_action_fields(step, context)?;
        Ok(Step::new(
            step.id().map(str::to_string),
            name,
            step.r#if().map(str::to_string),
            run,
            step.shell().map(str::to_string),
            working_directory,
            uses,
            with,
            env,
            step.continue_on_error().map(str::to_string),
            step.timeout_minutes(),
        ))
    }

    fn interpolate_core_fields(
        step: &Step,
        context: &EvalContext,
    ) -> Result<InterpolatedCoreFields, EvalError> {
        let name = Self::interpolate_field(step.name(), context)?;
        let run = Self::interpolate_field(step.run(), context)?;
        let working_directory = Self::interpolate_field(step.working_directory(), context)?;
        Ok((name, run, working_directory))
    }

    fn interpolate_action_fields(
        step: &Step,
        context: &EvalContext,
    ) -> Result<InterpolatedActionFields, EvalError> {
        let uses = Self::interpolate_field(step.uses(), context)?;
        let with = Self::interpolate_map(step.with(), context)?;
        let env = Self::interpolate_map(step.env(), context)?;
        Ok((uses, with, env))
    }

    fn interpolate_field(
        field: Option<&str>,
        context: &EvalContext,
    ) -> Result<Option<String>, EvalError> {
        field
            .map(|value| ExpressionResolver::resolve_text(value, context))
            .transpose()
    }

    fn interpolate_map(
        map: &HashMap<String, String>,
        context: &EvalContext,
    ) -> Result<HashMap<String, String>, EvalError> {
        map.iter()
            .map(|(key, value)| {
                ExpressionResolver::resolve_text(value, context)
                    .map(|resolved| (key.clone(), resolved))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    fn context_with_secret(name: &str, value: &str) -> EvalContext {
        let mut secrets = serde_json::Map::new();
        secrets.insert(name.into(), Value::String(value.into()));
        EvalContext::new().with_secrets(Value::Object(secrets))
    }

    fn step_from(yaml: &str) -> Step {
        serde_yaml::from_str(yaml).unwrap()
    }

    #[test]
    fn interpolate_resolves_secrets_in_run_script() {
        let step = step_from("run: cargo publish --token ${{ secrets.TOKEN }}\n");

        let interpolated =
            StepInterpolator::interpolate(&step, &context_with_secret("TOKEN", "abc123")).unwrap();
        assert!(interpolated.run().unwrap().contains("abc123"));
    }

    #[test]
    fn interpolate_resolves_env_values() {
        let step = step_from("run: publish\nenv:\n  TOKEN: ${{ secrets.TOKEN }}\n");

        let interpolated =
            StepInterpolator::interpolate(&step, &context_with_secret("TOKEN", "abc123")).unwrap();

        assert_eq!(
            interpolated.env().get("TOKEN").map(String::as_str),
            Some("abc123")
        );
    }

    #[test]
    fn interpolate_resolves_with_values() {
        let mut inputs = serde_json::Map::new();
        inputs.insert("mode".into(), Value::String("staging".into()));
        let context = EvalContext::new().with_inputs(Value::Object(inputs));
        let step = step_from("uses: ./action\nwith:\n  mode: ${{ inputs.mode }}\n");

        let interpolated = StepInterpolator::interpolate(&step, &context).unwrap();

        assert_eq!(
            interpolated.with().get("mode").map(String::as_str),
            Some("staging")
        );
    }

    #[test]
    fn interpolate_resolves_action_reference() {
        let mut inputs = serde_json::Map::new();
        inputs.insert("version".into(), Value::String("v4".into()));
        let context = EvalContext::new().with_inputs(Value::Object(inputs));
        let step = step_from("uses: actions/cache@${{ inputs.version }}\n");

        let interpolated = StepInterpolator::interpolate(&step, &context).unwrap();

        assert_eq!(interpolated.uses(), Some("actions/cache@v4"));
    }

    #[test]
    fn interpolate_keeps_condition_as_authored() {
        let step = step_from("run: echo hi\nif: ${{ success() }}\n");

        let interpolated = StepInterpolator::interpolate(&step, &EvalContext::new()).unwrap();

        assert_eq!(interpolated.r#if().as_deref(), Some("${{ success() }}"));
    }

    #[test]
    fn interpolate_errors_on_unparsable_expression() {
        let step = step_from("run: echo ${{ secrets. }}\n");

        assert!(StepInterpolator::interpolate(&step, &EvalContext::new()).is_err());
    }
}
