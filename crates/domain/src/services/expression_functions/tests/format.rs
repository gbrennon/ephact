#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn format_basic() {
        let f = ExpressionFunctions::new();
        let result = f
            .format(
                &ContextValue::text("Hello {0}"),
                &[ContextValue::text("world")],
            )
            .unwrap();
        assert_eq!(result, ContextValue::text("Hello world"));
    }

    #[test]
    fn format_multiple_args() {
        let f = ExpressionFunctions::new();
        let result = f
            .format(
                &ContextValue::text("{0} + {1} = {2}"),
                &[
                    ContextValue::Integer(1),
                    ContextValue::Integer(2),
                    ContextValue::Integer(3),
                ],
            )
            .unwrap();
        assert_eq!(result, ContextValue::text("1 + 2 = 3"));
    }

    #[test]
    fn format_no_placeholders() {
        let f = ExpressionFunctions::new();
        let result = f
            .format(&ContextValue::text("no placeholders"), &[])
            .unwrap();
        assert_eq!(result, ContextValue::text("no placeholders"));
    }

    #[test]
    fn format_index_out_of_range() {
        let f = ExpressionFunctions::new();
        let err = f
            .format(
                &ContextValue::text("Hello {5}"),
                &[ContextValue::text("world")],
            )
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_invalid_placeholder_character() {
        let f = ExpressionFunctions::new();
        let err = f
            .format(&ContextValue::text("{a}"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_unclosed_placeholder() {
        let f = ExpressionFunctions::new();
        let err = f.format(&ContextValue::text("{0"), &[]).unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_invalid_placeholder_index() {
        let f = ExpressionFunctions::new();
        let err = f
            .format(&ContextValue::text("{}"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_unexpected_closing_brace() {
        let f = ExpressionFunctions::new();
        let err = f
            .format(&ContextValue::text("a}b"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_null_and_bool_and_array_replacement() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.format(&ContextValue::text("{0}"), &[ContextValue::Null])
                .unwrap(),
            ContextValue::text("null")
        );
        assert_eq!(
            f.format(&ContextValue::text("{0}"), &[ContextValue::Boolean(true)])
                .unwrap(),
            ContextValue::text("true")
        );
        assert_eq!(
            f.format(
                &ContextValue::text("{0}"),
                &[ContextValue::list([
                    ContextValue::Integer(1),
                    ContextValue::Integer(2)
                ])]
            )
            .unwrap(),
            ContextValue::text("[1,2]")
        );
    }
}
