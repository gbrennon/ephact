use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn parse_eq() {
        let expr = ExpressionParser::parse_text("a == b").unwrap();

        assert_eq!(
            expr,
            Expression::Comparison(
                ComparisonOperator::Equal,
                Box::new(Expression::Variable("a".into())),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_neq() {
        let expr = ExpressionParser::parse_text("a != b").unwrap();

        assert_eq!(
            expr,
            Expression::Comparison(
                ComparisonOperator::NotEqual,
                Box::new(Expression::Variable("a".into())),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_lt() {
        let expr = ExpressionParser::parse_text("a < b").unwrap();

        assert_eq!(
            expr,
            Expression::Comparison(
                ComparisonOperator::LessThan,
                Box::new(Expression::Variable("a".into())),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_gt() {
        let expr = ExpressionParser::parse_text("a > b").unwrap();

        assert_eq!(
            expr,
            Expression::Comparison(
                ComparisonOperator::GreaterThan,
                Box::new(Expression::Variable("a".into())),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_lte() {
        let expr = ExpressionParser::parse_text("a <= b").unwrap();

        assert_eq!(
            expr,
            Expression::Comparison(
                ComparisonOperator::LessThanOrEqual,
                Box::new(Expression::Variable("a".into())),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_gte() {
        let expr = ExpressionParser::parse_text("a >= b").unwrap();

        assert_eq!(
            expr,
            Expression::Comparison(
                ComparisonOperator::GreaterThanOrEqual,
                Box::new(Expression::Variable("a".into())),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_and() {
        let expr = ExpressionParser::parse_text("a && b").unwrap();

        assert_eq!(
            expr,
            Expression::Logical(
                LogicalOperator::And,
                Box::new(Expression::Variable("a".into())),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_or() {
        let expr = ExpressionParser::parse_text("a || b").unwrap();

        assert_eq!(
            expr,
            Expression::Logical(
                LogicalOperator::Or,
                Box::new(Expression::Variable("a".into())),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_and_or_left_assoc() {
        let expr = ExpressionParser::parse_text("a && b || c").unwrap();

        assert_eq!(
            expr,
            Expression::Logical(
                LogicalOperator::Or,
                Box::new(Expression::Logical(
                    LogicalOperator::And,
                    Box::new(Expression::Variable("a".into())),
                    Box::new(Expression::Variable("b".into()))
                )),
                Box::new(Expression::Variable("c".into()))
            )
        );
    }

    #[test]

    fn parse_or_and_left_assoc() {
        let expr = ExpressionParser::parse_text("a || b && c").unwrap();

        assert_eq!(
            expr,
            Expression::Logical(
                LogicalOperator::And,
                Box::new(Expression::Logical(
                    LogicalOperator::Or,
                    Box::new(Expression::Variable("a".into())),
                    Box::new(Expression::Variable("b".into()))
                )),
                Box::new(Expression::Variable("c".into()))
            )
        );
    }

    #[test]

    fn parse_not() {
        let expr = ExpressionParser::parse_text("!a").unwrap();

        assert_eq!(
            expr,
            Expression::Not(Box::new(Expression::Variable("a".into())))
        );
    }

    #[test]

    fn parse_double_not() {
        let expr = ExpressionParser::parse_text("!!a").unwrap();

        assert_eq!(
            expr,
            Expression::Not(Box::new(Expression::Not(Box::new(Expression::Variable(
                "a".into()
            )))))
        );
    }

    #[test]

    fn parse_not_compare() {
        let expr = ExpressionParser::parse_text("!a == b").unwrap();

        assert_eq!(
            expr,
            Expression::Comparison(
                ComparisonOperator::Equal,
                Box::new(Expression::Not(Box::new(Expression::Variable("a".into())))),
                Box::new(Expression::Variable("b".into()))
            )
        );
    }

    #[test]

    fn parse_parens() {
        let expr = ExpressionParser::parse_text("(a)").unwrap();

        assert_eq!(expr, Expression::Variable("a".into()));
    }

    #[test]

    fn parse_parens_override_precedence() {
        let expr = ExpressionParser::parse_text("(a || b) && c").unwrap();

        assert_eq!(
            expr,
            Expression::Logical(
                LogicalOperator::And,
                Box::new(Expression::Logical(
                    LogicalOperator::Or,
                    Box::new(Expression::Variable("a".into())),
                    Box::new(Expression::Variable("b".into()))
                )),
                Box::new(Expression::Variable("c".into()))
            )
        );
    }

    #[test]

    fn parse_complex_expression() {
        let expr =
            ExpressionParser::parse_text("github.ref == 'refs/heads/main' && success()").unwrap();

        assert_eq!(
            expr,
            Expression::Logical(
                LogicalOperator::And,
                Box::new(Expression::Comparison(
                    ComparisonOperator::Equal,
                    Box::new(Expression::PropertyAccess(
                        Box::new(Expression::Variable("github".into())),
                        "ref".into()
                    )),
                    Box::new(Expression::Literal(ExpressionLiteral::String(
                        "refs/heads/main".into()
                    )))
                )),
                Box::new(Expression::FunctionCall("success".into(), vec![]))
            )
        );
    }
}
