#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn to_json_roundtrip() {
        let f = ExpressionFunctions::new();
        let original = ContextValue::mapping([
            ("key".to_owned(), ContextValue::text("value")),
            ("num".to_owned(), ContextValue::Integer(42)),
        ]);
        let json_str = f.to_json(&original).unwrap();
        let parsed = f.from_json(&json_str).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn from_json_invalid() {
        let f = ExpressionFunctions::new();
        let err = f.from_json(&ContextValue::text("not json")).unwrap_err();
        assert!(matches!(err, EvalError::JsonError(_)));
    }
}
