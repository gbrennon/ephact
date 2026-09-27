use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_literal_bool_true() {
        let result = eval(&Expression::Literal(ExpressionLiteral::Boolean(true))).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_literal_bool_false() {
        let result = eval(&Expression::Literal(ExpressionLiteral::Boolean(false))).unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn eval_literal_int() {
        let result = eval(&Expression::Literal(ExpressionLiteral::Integer(42))).unwrap();
        assert_eq!(result, ContextValue::Integer(42));
    }

    #[test]
    fn eval_literal_int_negative() {
        let result = eval(&Expression::Literal(ExpressionLiteral::Integer(-7))).unwrap();
        assert_eq!(result, ContextValue::Integer(-7));
    }

    #[test]
    fn eval_literal_float() {
        let result = eval(&Expression::Literal(ExpressionLiteral::Float(
            std::f64::consts::PI,
        )))
        .unwrap();
        assert_eq!(result, ContextValue::Decimal(std::f64::consts::PI));
    }

    #[test]
    fn eval_literal_string() {
        let result = eval(&Expression::Literal(ExpressionLiteral::String(
            "hello".into(),
        )))
        .unwrap();
        assert_eq!(result, ContextValue::text("hello"));
    }

    #[test]
    fn eval_literal_null() {
        let result = eval(&Expression::Literal(ExpressionLiteral::Null)).unwrap();
        assert_eq!(result, ContextValue::Null);
    }

    #[test]
    fn eval_variable_stub() {
        let result = eval(&Expression::Variable("foo".into())).unwrap();
        assert_eq!(result, ContextValue::text("${{ foo }}"));
    }

    #[test]
    fn eval_variable_stub_env() {
        let result = eval(&Expression::Variable("bar".into())).unwrap();
        assert_eq!(result, ContextValue::text("${{ bar }}"));
    }

    #[test]
    fn eval_variable_from_context() {
        let expr = Expression::Variable("github".into());
        let c = ctx();
        let result = ExpressionEvaluator::new(&c).evaluate(&expr).unwrap();
        assert!(result.is_mapping());
    }
}
