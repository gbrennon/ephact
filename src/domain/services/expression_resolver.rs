use crate::domain::{
    errors::EvalError,
    services::{ExpressionEvaluator, expression_parser::ExpressionParser},
    value_objects::{ContextValue, EvaluationContext},
};

/// Marker that opens an interpolated expression inside a template string.
const EXPRESSION_OPEN: &str = "${{";

/// Marker that closes an interpolated expression inside a template string.
const EXPRESSION_CLOSE: &str = "}}";

/// Substitutes every `${{ ... }}` expression in a template string with its
/// evaluated value.
///
/// Values render the way the GitHub Actions runner renders them: strings
/// verbatim, booleans and numbers via their literal form, `null` and unknown
/// context entries as the empty string, and objects/arrays as compact JSON.
pub struct ExpressionResolver;

impl ExpressionResolver {
    /// Resolves all expressions in `template` against `context`.
    ///
    /// Text outside `${{ ... }}` is preserved byte for byte, and an unclosed
    /// expression is left untouched so shell syntax is never mangled.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::TypeError`] when an expression cannot be parsed.
    pub fn resolve_text(template: &str, context: &EvaluationContext) -> Result<String, EvalError> {
        let mut resolved = String::with_capacity(template.len());
        let mut remainder = template;

        while let Some(open) = remainder.find(EXPRESSION_OPEN) {
            let (literal, expression_start) = remainder.split_at(open);
            resolved.push_str(literal);

            let body_start = &expression_start[EXPRESSION_OPEN.len()..];
            let Some(close) = body_start.find(EXPRESSION_CLOSE) else {
                resolved.push_str(expression_start);
                return Ok(resolved);
            };

            resolved.push_str(&Self::evaluate(body_start[..close].trim(), context)?);
            remainder = &body_start[close + EXPRESSION_CLOSE.len()..];
        }

        resolved.push_str(remainder);
        Ok(resolved)
    }

    fn evaluate(body: &str, context: &EvaluationContext) -> Result<String, EvalError> {
        let expression = ExpressionParser::parse_text(body).map_err(|error| {
            EvalError::TypeError(format!(
                "invalid expression '{body}': {} at position {}",
                error.message(),
                error.position()
            ))
        })?;

        Ok(ExpressionEvaluator::new(context)
            .evaluate(&expression)
            .as_ref()
            .map_or_else(|_| String::new(), Self::stringify))
    }

    fn stringify(value: &ContextValue) -> String {
        match value {
            ContextValue::Null => String::new(),
            ContextValue::Text(text) => text.clone(),
            other => other.to_json_text(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context_with_secret(name: &str, value: &str) -> EvaluationContext {
        let secrets = ContextValue::mapping([(name.to_owned(), ContextValue::text(value))]);
        EvaluationContext::new().with_secrets(secrets)
    }

    #[test]
    fn resolve_text_substitutes_secret_value() {
        let context = context_with_secret("TOKEN", "staging-token");

        let resolved =
            ExpressionResolver::resolve_text("publish --token ${{ secrets.TOKEN }}", &context)
                .unwrap();

        assert_eq!(resolved, "publish --token staging-token");
    }

    #[test]
    fn resolve_text_substitutes_multiple_expressions() {
        let inputs = ContextValue::mapping([("mode".to_owned(), ContextValue::text("staging"))]);
        let context = context_with_secret("TOKEN", "abc").with_inputs(inputs);

        let resolved = ExpressionResolver::resolve_text(
            "${{ inputs.mode }}:${{ secrets.TOKEN }}:${{ inputs.mode }}",
            &context,
        )
        .unwrap();

        assert_eq!(resolved, "staging:abc:staging");
    }

    #[test]
    fn resolve_text_keeps_literal_text_untouched() {
        let resolved = ExpressionResolver::resolve_text(
            "echo \"no expressions here\"",
            &EvaluationContext::new(),
        )
        .unwrap();

        assert_eq!(resolved, "echo \"no expressions here\"");
    }

    #[test]
    fn resolve_text_renders_unknown_secret_as_empty_string() {
        let resolved = ExpressionResolver::resolve_text(
            "token=${{ secrets.MISSING }}",
            &EvaluationContext::new(),
        )
        .unwrap();

        assert_eq!(resolved, "token=");
    }

    #[test]
    fn resolve_text_preserves_shell_parameter_expansion() {
        let resolved =
            ExpressionResolver::resolve_text("echo ${HOME} ${#args[@]}", &EvaluationContext::new())
                .unwrap();

        assert_eq!(resolved, "echo ${HOME} ${#args[@]}");
    }

    #[test]
    fn resolve_text_leaves_unclosed_expression_untouched() {
        let resolved =
            ExpressionResolver::resolve_text("echo ${{ secrets.TOKEN", &EvaluationContext::new())
                .unwrap();

        assert_eq!(resolved, "echo ${{ secrets.TOKEN");
    }

    #[test]
    fn resolve_text_renders_boolean_literals() {
        let resolved =
            ExpressionResolver::resolve_text("flag=${{ true }}", &EvaluationContext::new())
                .unwrap();

        assert_eq!(resolved, "flag=true");
    }

    #[test]
    fn resolve_text_renders_function_results() {
        let resolved = ExpressionResolver::resolve_text(
            "${{ format('{0}-{1}', 'a', 'b') }}",
            &EvaluationContext::new(),
        )
        .unwrap();

        assert_eq!(resolved, "a-b");
    }

    #[test]
    fn resolve_text_errors_on_unparsable_expression() {
        let error = ExpressionResolver::resolve_text("${{ secrets. }}", &EvaluationContext::new())
            .unwrap_err();

        assert!(
            matches!(error, EvalError::TypeError(message) if message.contains("invalid expression"))
        );
    }
}
