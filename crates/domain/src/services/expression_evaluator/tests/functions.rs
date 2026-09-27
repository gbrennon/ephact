use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_func_call_contains_true() {
        let expr = Expression::FunctionCall(
            "contains".into(),
            vec![
                Expression::Literal(ExpressionLiteral::String("Hello World".into())),
                Expression::Literal(ExpressionLiteral::String("world".into())),
            ],
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_func_call_contains_false() {
        let expr = Expression::FunctionCall(
            "contains".into(),
            vec![
                Expression::Literal(ExpressionLiteral::String("Hello World".into())),
                Expression::Literal(ExpressionLiteral::String("xyz".into())),
            ],
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn eval_func_call_unknown() {
        let expr = Expression::FunctionCall("nonexistent".into(), vec![]);
        let result = eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn from_json_round_trips_through_to_json_with_sorted_keys() {
        let source = Expression::Literal(ExpressionLiteral::String(
            r#"{"b":[1,2.5,true,null],"a":"x"}"#.into(),
        ));
        let parsed = Expression::FunctionCall("fromJson".into(), vec![source]);
        let rendered = Expression::FunctionCall("toJson".into(), vec![parsed]);
        assert_eq!(
            eval(&rendered).unwrap(),
            ContextValue::text(r#"{"a":"x","b":[1,2.5,true,null]}"#)
        );
    }

    #[test]
    fn format_renders_a_parsed_list_as_compact_json() {
        let source = Expression::Literal(ExpressionLiteral::String("[1,2]".into()));
        let parsed = Expression::FunctionCall("fromJson".into(), vec![source]);
        let expr = Expression::FunctionCall(
            "format".into(),
            vec![
                Expression::Literal(ExpressionLiteral::String("{0}".into())),
                parsed,
            ],
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::text("[1,2]"));
    }
}
