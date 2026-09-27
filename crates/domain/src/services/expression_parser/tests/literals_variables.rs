use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn parse_bool_true() {
        let expr = ExpressionParser::parse_text("true").unwrap();

        assert_eq!(expr, Expression::Literal(ExpressionLiteral::Boolean(true)));
    }

    #[test]

    fn parse_bool_false() {
        let expr = ExpressionParser::parse_text("false").unwrap();

        assert_eq!(expr, Expression::Literal(ExpressionLiteral::Boolean(false)));
    }

    #[test]

    fn parse_null() {
        let expr = ExpressionParser::parse_text("null").unwrap();

        assert_eq!(expr, Expression::Literal(ExpressionLiteral::Null));
    }

    #[test]

    fn parse_int() {
        let expr = ExpressionParser::parse_text("42").unwrap();

        assert_eq!(expr, Expression::Literal(ExpressionLiteral::Integer(42)));
    }

    #[test]

    fn parse_negative_int() {
        let expr = ExpressionParser::parse_text("7").unwrap();

        assert_eq!(expr, Expression::Literal(ExpressionLiteral::Integer(7)));
    }

    #[test]

    fn parse_float() {
        let expr = ExpressionParser::parse_text("2.71").unwrap();

        assert_eq!(expr, Expression::Literal(ExpressionLiteral::Float(2.71)));
    }

    #[test]

    fn parse_string() {
        let expr = ExpressionParser::parse_text("'hello'").unwrap();

        assert_eq!(
            expr,
            Expression::Literal(ExpressionLiteral::String("hello".into()))
        );
    }

    #[test]

    fn parse_variable() {
        let expr = ExpressionParser::parse_text("github").unwrap();

        assert_eq!(expr, Expression::Variable("github".into()));
    }

    #[test]

    fn parse_variable_env() {
        let expr = ExpressionParser::parse_text("env").unwrap();

        assert_eq!(expr, Expression::Variable("env".into()));
    }
}
