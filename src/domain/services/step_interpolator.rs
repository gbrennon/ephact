use std::collections::HashMap;

use crate::domain::{
    entities::Step, errors::EvalError, services::ExpressionResolver,
    value_objects::EvaluationContext,
};

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
    pub fn interpolate(step: &Step, context: &EvaluationContext) -> Result<Step, EvalError> {
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
        context: &EvaluationContext,
    ) -> Result<InterpolatedCoreFields, EvalError> {
        let name = Self::interpolate_field(step.name(), context)?;
        let run = Self::interpolate_field(step.run(), context)?;
        let working_directory = Self::interpolate_field(step.working_directory(), context)?;
        Ok((name, run, working_directory))
    }

    fn interpolate_action_fields(
        step: &Step,
        context: &EvaluationContext,
    ) -> Result<InterpolatedActionFields, EvalError> {
        let uses = Self::interpolate_field(step.uses(), context)?;
        let with = Self::interpolate_map(step.with(), context)?;
        let env = Self::interpolate_map(step.env(), context)?;
        Ok((uses, with, env))
    }

    fn interpolate_field(
        field: Option<&str>,
        context: &EvaluationContext,
    ) -> Result<Option<String>, EvalError> {
        field
            .map(|value| ExpressionResolver::resolve_text(value, context))
            .transpose()
    }

    fn interpolate_map(
        map: &HashMap<String, String>,
        context: &EvaluationContext,
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
    use super::*;

    fn context_with_secret(name: &str, value: &str) -> EvaluationContext {
        let secrets = crate::domain::value_objects::ContextValue::mapping([(
            name.to_owned(),
            crate::domain::value_objects::ContextValue::text(value),
        )]);
        EvaluationContext::new().with_secrets(secrets)
    }

    fn context_with_input(name: &str, value: &str) -> EvaluationContext {
        let inputs = crate::domain::value_objects::ContextValue::mapping([(
            name.to_owned(),
            crate::domain::value_objects::ContextValue::text(value),
        )]);
        EvaluationContext::new().with_inputs(inputs)
    }

    fn run_step(script: &str) -> Step {
        Step::new(
            None,
            None,
            None,
            Some(script.to_owned()),
            None,
            None,
            None,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        )
    }

    fn action_step(uses: &str, with: HashMap<String, String>) -> Step {
        Step::new(
            None,
            None,
            None,
            None,
            None,
            None,
            Some(uses.to_owned()),
            with,
            HashMap::new(),
            None,
            None,
        )
    }

    #[test]
    fn interpolate_resolves_secrets_in_run_script() {
        let step = run_step("cargo publish --token ${{ secrets.TOKEN }}");

        let interpolated =
            StepInterpolator::interpolate(&step, &context_with_secret("TOKEN", "abc123")).unwrap();
        assert!(interpolated.run().unwrap().contains("abc123"));
    }

    #[test]
    fn interpolate_resolves_env_values() {
        let step = Step::new(
            None,
            None,
            None,
            Some("publish".to_owned()),
            None,
            None,
            None,
            HashMap::new(),
            HashMap::from([("TOKEN".to_owned(), "${{ secrets.TOKEN }}".to_owned())]),
            None,
            None,
        );

        let interpolated =
            StepInterpolator::interpolate(&step, &context_with_secret("TOKEN", "abc123")).unwrap();

        assert_eq!(
            interpolated.env().get("TOKEN").map(String::as_str),
            Some("abc123")
        );
    }

    #[test]
    fn interpolate_resolves_with_values() {
        let step = action_step(
            "./action",
            HashMap::from([("mode".to_owned(), "${{ inputs.mode }}".to_owned())]),
        );

        let interpolated =
            StepInterpolator::interpolate(&step, &context_with_input("mode", "staging")).unwrap();

        assert_eq!(
            interpolated.with().get("mode").map(String::as_str),
            Some("staging")
        );
    }

    #[test]
    fn interpolate_resolves_action_reference() {
        let step = action_step("actions/cache@${{ inputs.version }}", HashMap::new());

        let interpolated =
            StepInterpolator::interpolate(&step, &context_with_input("version", "v4")).unwrap();

        assert_eq!(interpolated.uses(), Some("actions/cache@v4"));
    }

    #[test]
    fn interpolate_keeps_condition_as_authored() {
        let step = Step::new(
            None,
            None,
            Some("${{ success() }}".to_owned()),
            Some("echo hi".to_owned()),
            None,
            None,
            None,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        let interpolated = StepInterpolator::interpolate(&step, &EvaluationContext::new()).unwrap();

        assert_eq!(interpolated.r#if(), Some("${{ success() }}"));
    }

    #[test]
    fn interpolate_errors_on_unparsable_expression() {
        let step = run_step("echo ${{ secrets. }}");

        assert!(StepInterpolator::interpolate(&step, &EvaluationContext::new()).is_err());
    }
}
