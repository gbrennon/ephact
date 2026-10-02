mod builtin_function;
mod comparison_operator;
mod expression;
mod expression_cursor;
mod expression_evaluator;
mod expression_functions;
mod expression_lexer;
mod expression_literal;
mod expression_parser;
mod expression_resolver;
mod expression_token;
mod identifier_token;
mod logical_operator;
mod number_literal;
mod operator_token;
mod string_literal;

use std::collections::HashMap;

use self::expression_resolver::ExpressionResolver;
pub(super) use self::{
    builtin_function::BuiltinFunction, comparison_operator::ComparisonOperator,
    expression::Expression, expression_cursor::ExpressionCursor,
    expression_literal::ExpressionLiteral, expression_token::ExpressionToken,
    identifier_token::IdentifierToken, logical_operator::LogicalOperator,
    number_literal::NumberLiteral, operator_token::OperatorToken, string_literal::StringLiteral,
};
use crate::{
    application::{errors::EvalError, ports::outbound::StepInterpolatorPort},
    domain::{entities::Step, value_objects::EvaluationContext},
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
        Ok(Step::new(step.id().map(str::to_string), name, run, uses)
            .with_if_condition(step.r#if().map(str::to_string))
            .with_shell(step.shell().map(str::to_string))
            .with_working_directory(working_directory)
            .with_inputs(with)
            .with_env(env)
            .with_continue_on_error(step.continue_on_error().map(str::to_string))
            .with_timeout_minutes(step.timeout_minutes()))
    }
    /// Returns whether a step's `if` condition permits execution.
    pub fn should_run(step: &Step, context: &EvaluationContext) -> Result<bool, EvalError> {
        step.if_condition().map_or(Ok(true), |condition| {
            ExpressionResolver::evaluate_condition(condition, context)
        })
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

impl StepInterpolatorPort for StepInterpolator {
    fn interpolate(&self, step: &Step, context: &EvaluationContext) -> Result<Step, EvalError> {
        Self::interpolate(step, context)
    }

    fn should_run(&self, step: &Step, context: &EvaluationContext) -> Result<bool, EvalError> {
        Self::should_run(step, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl StepInterpolator {
        fn context_with_secret_for_test(name: &str, value: &str) -> EvaluationContext {
            let secrets = crate::domain::value_objects::ContextValue::mapping([(
                name.to_owned(),
                crate::domain::value_objects::ContextValue::text(value),
            )]);
            EvaluationContext::new().with_root("credentials", secrets)
        }

        fn context_with_input_for_test(name: &str, value: &str) -> EvaluationContext {
            let inputs = crate::domain::value_objects::ContextValue::mapping([(
                name.to_owned(),
                crate::domain::value_objects::ContextValue::text(value),
            )]);
            EvaluationContext::new().with_root("parameters", inputs)
        }

        fn run_step_for_test(script: &str) -> Step {
            Step::new(None, None, Some(script.to_owned()), None)
        }

        fn action_step_for_test(uses: &str, with: HashMap<String, String>) -> Step {
            Step::new(None, None, None, Some(uses.to_owned())).with_inputs(with)
        }
    }

    #[test]
    fn interpolate_resolves_secrets_in_run_script() {
        let step =
            StepInterpolator::run_step_for_test("cargo publish --token ${{ credentials.TOKEN }}");

        let interpolated = StepInterpolator::interpolate(
            &step,
            &StepInterpolator::context_with_secret_for_test("TOKEN", "abc123"),
        )
        .unwrap();
        assert!(interpolated.run().unwrap().contains("abc123"));
    }

    #[test]
    fn interpolate_resolves_env_values() {
        let step = Step::new(None, None, Some("publish".to_owned()), None).with_env(HashMap::from(
            [("TOKEN".to_owned(), "${{ credentials.TOKEN }}".to_owned())],
        ));

        let interpolated = StepInterpolator::interpolate(
            &step,
            &StepInterpolator::context_with_secret_for_test("TOKEN", "abc123"),
        )
        .unwrap();

        assert_eq!(
            interpolated.env().get("TOKEN").map(String::as_str),
            Some("abc123")
        );
    }

    #[test]
    fn interpolate_resolves_with_values() {
        let step = StepInterpolator::action_step_for_test(
            "./action",
            HashMap::from([("mode".to_owned(), "${{ parameters.mode }}".to_owned())]),
        );

        let interpolated = StepInterpolator::interpolate(
            &step,
            &StepInterpolator::context_with_input_for_test("mode", "staging"),
        )
        .unwrap();

        assert_eq!(
            interpolated.with().get("mode").map(String::as_str),
            Some("staging")
        );
    }

    #[test]
    fn interpolate_resolves_action_reference() {
        let step = StepInterpolator::action_step_for_test(
            "actions/cache@${{ parameters.version }}",
            HashMap::new(),
        );

        let interpolated = StepInterpolator::interpolate(
            &step,
            &StepInterpolator::context_with_input_for_test("version", "v4"),
        )
        .unwrap();

        assert_eq!(interpolated.uses(), Some("actions/cache@v4"));
    }

    #[test]
    fn interpolate_keeps_condition_as_authored() {
        let step = Step::new(None, None, Some("echo hi".to_owned()), None)
            .with_if_condition(Some("${{ success() }}".to_owned()));

        let interpolated = StepInterpolator::interpolate(&step, &EvaluationContext::new()).unwrap();

        assert_eq!(interpolated.r#if(), Some("${{ success() }}"));
    }

    #[test]
    fn interpolate_errors_on_unparsable_expression() {
        let step = StepInterpolator::run_step_for_test("echo ${{ credentials. }}");

        assert!(StepInterpolator::interpolate(&step, &EvaluationContext::new()).is_err());
    }
}
