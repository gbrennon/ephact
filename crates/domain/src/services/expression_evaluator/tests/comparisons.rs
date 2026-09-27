use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_compare_eq_numbers_true() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_eq_numbers_false() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(3))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn eval_compare_neq_numbers() {
        let expr = Expression::Comparison(
            ComparisonOperator::NotEqual,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(3))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_lt_numbers_true() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(2))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(10))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_lt_numbers_false() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(10))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(2))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn eval_compare_gt_numbers() {
        let expr = Expression::Comparison(
            ComparisonOperator::GreaterThan,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(10))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(2))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_lte_numbers_equal() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThanOrEqual,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_lte_numbers_less() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThanOrEqual,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(3))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_gte_numbers() {
        let expr = Expression::Comparison(
            ComparisonOperator::GreaterThanOrEqual,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
            Box::new(Expression::Literal(ExpressionLiteral::Integer(5))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_strings_eq() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(ExpressionLiteral::String("abc".into()))),
            Box::new(Expression::Literal(ExpressionLiteral::String("abc".into()))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_strings_lt() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(ExpressionLiteral::String("abc".into()))),
            Box::new(Expression::Literal(ExpressionLiteral::String("xyz".into()))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_bools() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(ExpressionLiteral::Boolean(false))),
            Box::new(Expression::Literal(ExpressionLiteral::Boolean(true))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_type_mismatch() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(ExpressionLiteral::Integer(1))),
            Box::new(Expression::Literal(ExpressionLiteral::String("1".into()))),
        );
        let result = eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }
}
