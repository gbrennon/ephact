#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn call_unknown_function_error() {
        let f = ExpressionFunctions::new();
        let err = f.call("nonexistent", &[]).unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn call_wrong_arg_count() {
        let f = ExpressionFunctions::new();
        let err = f
            .call("contains", &[ContextValue::text("only one")])
            .unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn call_case_insensitive() {
        let f = ExpressionFunctions::new();
        let result = f
            .call(
                "CoNtAiNs",
                &[
                    ContextValue::text("Hello World"),
                    ContextValue::text("world"),
                ],
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn call_success_zero_args() {
        let f = ExpressionFunctions::new();
        let result = f.call("success", &[]).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn call_success_with_args_is_error() {
        let f = ExpressionFunctions::new();
        let err = f
            .call("success", &[ContextValue::text("extra")])
            .unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn call_dispatches_starts_with() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "startswith",
                &[ContextValue::text("Hello"), ContextValue::text("he")]
            )
            .unwrap(),
            ContextValue::Boolean(true)
        );
    }

    #[test]
    fn call_dispatches_ends_with() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "endswith",
                &[ContextValue::text("Hello"), ContextValue::text("lo")]
            )
            .unwrap(),
            ContextValue::Boolean(true)
        );
    }

    #[test]
    fn call_format_with_no_args_errors() {
        let f = ExpressionFunctions::new();
        let err = f.call("format", &[]).unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn call_dispatches_format() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "format",
                &[ContextValue::text("Hi {0}"), ContextValue::text("bob")]
            )
            .unwrap(),
            ContextValue::text("Hi bob")
        );
    }

    #[test]
    fn call_dispatches_join() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "join",
                &[
                    ContextValue::list([ContextValue::text("a"), ContextValue::text("b")]),
                    ContextValue::text("-")
                ]
            )
            .unwrap(),
            ContextValue::text("a-b")
        );
    }

    #[test]
    fn call_dispatches_to_json() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "tojson",
                &[ContextValue::mapping([(
                    "a".to_owned(),
                    ContextValue::Integer(1)
                )])]
            )
            .unwrap(),
            ContextValue::text(r#"{"a":1}"#)
        );
    }

    #[test]
    fn call_dispatches_from_json() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call("fromjson", &[ContextValue::text(r#"{"a":1}"#)])
                .unwrap(),
            ContextValue::mapping([("a".to_owned(), ContextValue::Integer(1))])
        );
    }

    #[test]
    fn call_dispatches_always_cancelled_failure() {
        let f = ExpressionFunctions::new();
        assert_eq!(f.call("always", &[]).unwrap(), ContextValue::Boolean(true));
        assert_eq!(
            f.call("cancelled", &[]).unwrap(),
            ContextValue::Boolean(false)
        );
        assert_eq!(
            f.call("failure", &[]).unwrap(),
            ContextValue::Boolean(false)
        );
    }

    #[test]
    fn expect_arg_count_reports_range() {
        let err = ExpressionFunctions::expect_arg_count("x", 4, 1, 3).unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }
}
