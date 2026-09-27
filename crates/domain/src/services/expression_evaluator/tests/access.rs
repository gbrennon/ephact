use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_property_access_on_non_object() {
        let result = eval(&Expression::PropertyAccess(
            Box::new(Expression::Literal(ExpressionLiteral::String(
                "hello".into(),
            ))),
            "length".into(),
        ));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_property_access_on_object() {
        let json_str = Expression::Literal(ExpressionLiteral::String(r#"{"x": 10}"#.into()));
        let from_json = Expression::FunctionCall("fromJson".into(), vec![json_str]);
        let prop_access = Expression::PropertyAccess(Box::new(from_json), "x".into());
        let result = eval(&prop_access).unwrap();
        assert_eq!(result, ContextValue::Integer(10));
    }

    #[test]
    fn eval_index_access_array() {
        let json_str = Expression::Literal(ExpressionLiteral::String(r#"["a", "b", "c"]"#.into()));
        let from_json = Expression::FunctionCall("fromJson".into(), vec![json_str]);
        let idx = Expression::IndexAccess(
            Box::new(from_json),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(1))),
        );
        let result = eval(&idx).unwrap();
        assert_eq!(result, ContextValue::text("b"));
    }

    #[test]
    fn eval_index_access_object() {
        let json_str = Expression::Literal(ExpressionLiteral::String(r#"{"key": "val"}"#.into()));
        let from_json = Expression::FunctionCall("fromJson".into(), vec![json_str]);
        let idx = Expression::IndexAccess(
            Box::new(from_json),
            Box::new(Expression::Literal(ExpressionLiteral::String("key".into()))),
        );
        let result = eval(&idx).unwrap();
        assert_eq!(result, ContextValue::text("val"));
    }

    #[test]
    fn eval_index_access_type_error() {
        let idx = Expression::IndexAccess(
            Box::new(Expression::Literal(ExpressionLiteral::String(
                "hello".into(),
            ))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(0))),
        );
        let result = eval(&idx);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_array_deref_stub() {
        let inner = Expression::Literal(ExpressionLiteral::Integer(99));
        let deref = Expression::ArrayDereference(Box::new(inner));
        let result = eval(&deref).unwrap();
        assert_eq!(result, ContextValue::Integer(99));
    }

    #[test]
    fn eval_property_access_missing_key_errors() {
        let json_str = Expression::Literal(ExpressionLiteral::String(r#"{"x": 10}"#.into()));
        let from_json = Expression::FunctionCall("fromJson".into(), vec![json_str]);
        let prop_access = Expression::PropertyAccess(Box::new(from_json), "missing".into());
        let result = eval(&prop_access);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_index_access_negative_index_errors() {
        let arr = Expression::FunctionCall(
            "fromJson".into(),
            vec![Expression::Literal(ExpressionLiteral::String(
                "[1,2,3]".into(),
            ))],
        );
        let idx = Expression::Literal(ExpressionLiteral::Integer(-1));
        let expr = Expression::IndexAccess(Box::new(arr), Box::new(idx));
        let result = eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_index_access_out_of_bounds_errors() {
        let arr = Expression::FunctionCall(
            "fromJson".into(),
            vec![Expression::Literal(ExpressionLiteral::String(
                "[1,2,3]".into(),
            ))],
        );
        let idx = Expression::Literal(ExpressionLiteral::Integer(10));
        let expr = Expression::IndexAccess(Box::new(arr), Box::new(idx));
        let result = eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }
}
