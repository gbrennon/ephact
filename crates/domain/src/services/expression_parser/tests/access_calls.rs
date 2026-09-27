use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn parse_property_access() {
        let expr = ExpressionParser::parse_text("github.ref").unwrap();

        assert_eq!(
            expr,
            Expression::PropertyAccess(
                Box::new(Expression::Variable("github".into())),
                "ref".into()
            )
        );
    }

    #[test]

    fn parse_nested_property_access() {
        let expr = ExpressionParser::parse_text("github.event_name").unwrap();

        assert_eq!(
            expr,
            Expression::PropertyAccess(
                Box::new(Expression::Variable("github".into())),
                "event_name".into()
            )
        );
    }

    #[test]

    fn parse_deep_property_access() {
        let expr = ExpressionParser::parse_text("a.b.c").unwrap();

        assert_eq!(
            expr,
            Expression::PropertyAccess(
                Box::new(Expression::PropertyAccess(
                    Box::new(Expression::Variable("a".into())),
                    "b".into()
                )),
                "c".into()
            )
        );
    }

    #[test]

    fn parse_index_access() {
        let expr = ExpressionParser::parse_text("arr[0]").unwrap();

        assert_eq!(
            expr,
            Expression::IndexAccess(
                Box::new(Expression::Variable("arr".into())),
                Box::new(Expression::Literal(ExpressionLiteral::Integer(0)))
            )
        );
    }

    #[test]

    fn parse_index_access_string_key() {
        let expr = ExpressionParser::parse_text("obj['key']").unwrap();

        assert_eq!(
            expr,
            Expression::IndexAccess(
                Box::new(Expression::Variable("obj".into())),
                Box::new(Expression::Literal(ExpressionLiteral::String("key".into())))
            )
        );
    }

    #[test]

    fn parse_array_deref() {
        let expr = ExpressionParser::parse_text("foo.*").unwrap();

        assert_eq!(
            expr,
            Expression::ArrayDereference(Box::new(Expression::Variable("foo".into())))
        );
    }

    #[test]

    fn parse_func_call_no_args() {
        let expr = ExpressionParser::parse_text("success()").unwrap();

        assert_eq!(expr, Expression::FunctionCall("success".into(), vec![]));
    }

    #[test]

    fn parse_func_call_one_arg() {
        let expr = ExpressionParser::parse_text("always()").unwrap();

        assert_eq!(expr, Expression::FunctionCall("always".into(), vec![]));
    }

    #[test]

    fn parse_func_call_two_args() {
        let expr = ExpressionParser::parse_text("contains('hello', 'll')").unwrap();

        assert_eq!(
            expr,
            Expression::FunctionCall(
                "contains".into(),
                vec![
                    Expression::Literal(ExpressionLiteral::String("hello".into())),
                    Expression::Literal(ExpressionLiteral::String("ll".into()))
                ]
            )
        );
    }

    #[test]

    fn parse_chained_postfix() {
        let expr = ExpressionParser::parse_text("foo.bar[0].baz").unwrap();

        assert_eq!(
            expr,
            Expression::PropertyAccess(
                Box::new(Expression::IndexAccess(
                    Box::new(Expression::PropertyAccess(
                        Box::new(Expression::Variable("foo".into())),
                        "bar".into()
                    )),
                    Box::new(Expression::Literal(ExpressionLiteral::Integer(0)))
                )),
                "baz".into()
            )
        );
    }
}
