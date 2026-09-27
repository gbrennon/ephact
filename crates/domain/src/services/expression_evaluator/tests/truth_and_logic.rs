use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_not_true() {
        let expr = Expression::Not(Box::new(Expression::Literal(ExpressionLiteral::Boolean(
            true,
        ))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn eval_not_false() {
        let expr = Expression::Not(Box::new(Expression::Literal(ExpressionLiteral::Boolean(
            false,
        ))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_not_empty_string() {
        let expr = Expression::Not(Box::new(Expression::Literal(ExpressionLiteral::String(
            String::new(),
        ))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_not_non_empty_string() {
        let expr = Expression::Not(Box::new(Expression::Literal(ExpressionLiteral::String(
            "hi".into(),
        ))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn eval_not_null() {
        let expr = Expression::Not(Box::new(Expression::Literal(ExpressionLiteral::Null)));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_not_zero() {
        let expr = Expression::Not(Box::new(Expression::Literal(ExpressionLiteral::Integer(0))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_logical_and_both_truthy() {
        let expr = Expression::Logical(
            LogicalOperator::And,
            Box::new(Expression::Literal(ExpressionLiteral::Boolean(true))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(42))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Integer(42));
    }

    #[test]
    fn eval_logical_and_short_circuit() {
        let expr = Expression::Logical(
            LogicalOperator::And,
            Box::new(Expression::Literal(ExpressionLiteral::Boolean(false))),
            Box::new(Expression::Literal(ExpressionLiteral::String(
                "never".into(),
            ))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn eval_logical_or_both_falsy() {
        let expr = Expression::Logical(
            LogicalOperator::Or,
            Box::new(Expression::Literal(ExpressionLiteral::Boolean(false))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(0))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Integer(0));
    }

    #[test]
    fn eval_logical_or_short_circuit() {
        let expr = Expression::Logical(
            LogicalOperator::Or,
            Box::new(Expression::Literal(ExpressionLiteral::String(
                "first".into(),
            ))),
            Box::new(Expression::Literal(ExpressionLiteral::String(
                "never".into(),
            ))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::text("first"));
    }

    #[test]
    fn eval_nested_logical_with_compare() {
        let left_cmp = Expression::Comparison(
            ComparisonOperator::GreaterThan,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(3))),
        );
        let right_cmp = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(10))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(20))),
        );
        let expr = Expression::Logical(
            LogicalOperator::And,
            Box::new(left_cmp),
            Box::new(right_cmp),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_nested_not_compare() {
        let cmp = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(1))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(2))),
        );
        let expr = Expression::Not(Box::new(cmp));
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }
}
