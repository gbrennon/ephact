#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn contains_string_match_case_insensitive() {
        let f = ExpressionFunctions::new();
        let result = f
            .contains(
                &ContextValue::text("Hello World"),
                &ContextValue::text("world"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn contains_string_no_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .contains(
                &ContextValue::text("Hello World"),
                &ContextValue::text("xyz"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn contains_array_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .contains(
                &ContextValue::list([
                    ContextValue::text("a"),
                    ContextValue::text("b"),
                    ContextValue::text("c"),
                ]),
                &ContextValue::text("b"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn contains_array_no_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .contains(
                &ContextValue::list([ContextValue::text("a"), ContextValue::text("b")]),
                &ContextValue::text("c"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn contains_type_error_on_number() {
        let f = ExpressionFunctions::new();
        let err = f
            .contains(&ContextValue::Integer(42), &ContextValue::text("x"))
            .unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn starts_with_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .starts_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("hello"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn starts_with_no_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .starts_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("World"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn ends_with_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .ends_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("WORLD"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn ends_with_no_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .ends_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("Hello"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn contains_errors_when_search_string_and_item_not_string() {
        let f = ExpressionFunctions::new();
        let err = f
            .contains(&ContextValue::text("hello"), &ContextValue::Integer(42))
            .unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn expect_string_error_names_value_type() {
        let f = ExpressionFunctions::new();
        assert!(
            f.starts_with(&ContextValue::Null, &ContextValue::text("a"))
                .is_err()
        );
        assert!(
            f.starts_with(&ContextValue::Boolean(true), &ContextValue::text("a"))
                .is_err()
        );
        assert!(
            f.starts_with(&ContextValue::Integer(42), &ContextValue::text("a"))
                .is_err()
        );
        assert!(
            f.starts_with(
                &ContextValue::list([ContextValue::Integer(1), ContextValue::Integer(2)]),
                &ContextValue::text("a")
            )
            .is_err()
        );
        assert!(
            f.starts_with(
                &ContextValue::mapping([("k".to_owned(), ContextValue::Integer(1))]),
                &ContextValue::text("a")
            )
            .is_err()
        );
    }
}
