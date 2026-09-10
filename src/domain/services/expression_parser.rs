use crate::domain::{
    errors::ParseError,
    services::ExpressionLexer,
    value_objects::{
        ComparisonOperator, Expression, ExpressionToken, LiteralValue, LogicalOperator,
    },
};

/// Recursive-descent parser for expression tokens.
pub struct ExpressionParser {
    tokens: Vec<ExpressionToken>,
    pos: usize,
}

impl ExpressionParser {
    #[must_use]
    pub fn new(tokens: &[ExpressionToken]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            pos: 0,
        }
    }

    /// Parses the full token stream into an [`Expression`] AST.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if the token stream is malformed.
    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_expression()?;
        if self.pos < self.tokens.len() {
            return Err(self.error("unexpected tokens after expression"));
        }
        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_logical()
    }

    fn parse_logical(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_compare()?;
        loop {
            let op = match self.peek() {
                Some(ExpressionToken::And) => LogicalOperator::And,
                Some(ExpressionToken::Or) => LogicalOperator::Or,
                _ => break,
            };
            self.advance();
            let right = self.parse_compare()?;
            left = Expression::Logical(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_compare(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_unary()?;
        let op = match self.peek() {
            Some(ExpressionToken::Equal) => ComparisonOperator::Equal,
            Some(ExpressionToken::NotEqual) => ComparisonOperator::NotEqual,
            Some(ExpressionToken::LessThan) => ComparisonOperator::LessThan,
            Some(ExpressionToken::LessThanOrEqual) => ComparisonOperator::LessThanOrEqual,
            Some(ExpressionToken::GreaterThan) => ComparisonOperator::GreaterThan,
            Some(ExpressionToken::GreaterThanOrEqual) => ComparisonOperator::GreaterThanOrEqual,
            _ => return Ok(left),
        };
        self.advance();
        let right = self.parse_unary()?;
        Ok(Expression::Comparison(op, Box::new(left), Box::new(right)))
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if self.peek() == Some(&ExpressionToken::Not) {
            self.advance();
            let inner = self.parse_unary()?;
            return Ok(Expression::Not(Box::new(inner)));
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        while self.has_postfix_token() {
            expr = self.parse_next_postfix(expr)?;
        }
        Ok(expr)
    }

    fn has_postfix_token(&self) -> bool {
        matches!(
            self.peek(),
            Some(
                ExpressionToken::Dot
                    | ExpressionToken::LeftBracket
                    | ExpressionToken::LeftParenthesis
            )
        )
    }

    fn parse_next_postfix(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        match self.peek() {
            Some(ExpressionToken::Dot) => self.parse_dot_postfix(expr),
            Some(ExpressionToken::LeftBracket) => self.parse_index_postfix(expr),
            Some(ExpressionToken::LeftParenthesis) => self.parse_call_postfix(expr),
            _ => Ok(expr),
        }
    }

    fn parse_dot_postfix(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        self.advance();
        if self.peek() == Some(&ExpressionToken::Star) {
            self.advance();
            Ok(Expression::ArrayDereference(Box::new(expr)))
        } else {
            let ident = self.expect_ident("property name after '.'")?;
            Ok(Expression::PropertyAccess(Box::new(expr), ident))
        }
    }

    fn parse_index_postfix(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        self.advance();
        let idx = self.parse_expression()?;
        self.expect(ExpressionToken::RightBracket, "expected ']'")?;
        Ok(Expression::IndexAccess(Box::new(expr), Box::new(idx)))
    }

    fn parse_call_postfix(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        self.advance();
        let args = self.parse_args()?;
        self.expect(ExpressionToken::RightParenthesis, "expected ')'")?;
        let name = match &expr {
            Expression::Variable(n) => n.clone(),
            _ => return Err(self.error("function call requires a function name before '('")),
        };
        Ok(Expression::FunctionCall(name, args))
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek().cloned() {
            Some(ExpressionToken::Identifier(name)) => {
                self.advance();
                match name.as_str() {
                    "true" => return Ok(Expression::Literal(LiteralValue::Boolean(true))),
                    "false" => return Ok(Expression::Literal(LiteralValue::Boolean(false))),
                    "null" => return Ok(Expression::Literal(LiteralValue::Null)),
                    _ => {}
                }
                Ok(Expression::Variable(name))
            }
            Some(ExpressionToken::String(s)) => {
                self.advance();
                Ok(Expression::Literal(LiteralValue::String(s)))
            }
            Some(ExpressionToken::Integer(n)) => {
                self.advance();
                Ok(Expression::Literal(LiteralValue::Integer(n)))
            }
            Some(ExpressionToken::Float(f)) => {
                self.advance();
                Ok(Expression::Literal(LiteralValue::Float(f)))
            }
            Some(ExpressionToken::Boolean(b)) => {
                self.advance();
                Ok(Expression::Literal(LiteralValue::Boolean(b)))
            }
            Some(ExpressionToken::Null) => {
                self.advance();
                Ok(Expression::Literal(LiteralValue::Null))
            }
            Some(ExpressionToken::LeftParenthesis) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(ExpressionToken::RightParenthesis, "expected ')'")?;
                Ok(expr)
            }
            Some(other) => Err(self.error(&format!("unexpected token: {other:?}"))),
            None => Err(self.error("unexpected end of expression")),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = Vec::new();
        if self.peek() == Some(&ExpressionToken::RightParenthesis) {
            return Ok(args);
        }
        loop {
            args.push(self.parse_expression()?);
            if self.peek() == Some(&ExpressionToken::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(args)
    }

    fn peek(&self) -> Option<&ExpressionToken> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: ExpressionToken, msg: &str) -> Result<(), ParseError> {
        match self.peek() {
            Some(t) if *t == expected => {
                self.advance();
                Ok(())
            }
            Some(t) => Err(self.error(&format!("{msg}, found {t:?}"))),
            None => Err(self.error(&format!("{msg}, found end of input"))),
        }
    }

    fn expect_ident(&mut self, msg: &str) -> Result<String, ParseError> {
        match self.peek() {
            Some(ExpressionToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            Some(t) => Err(self.error(&format!("{msg}, found {t:?}"))),
            None => Err(self.error(&format!("{msg}, found end of input"))),
        }
    }

    fn error(&self, msg: &str) -> ParseError {
        ParseError::new(msg.to_string(), self.pos)
    }
}

impl ExpressionParser {
    /// Lexes and parses an expression string.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if lexing or parsing fails.
    pub fn parse_text(input: &str) -> Result<Expression, ParseError> {
        let mut lexer = ExpressionLexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(ExpressionToken::EndOfInput) => break,
                Ok(tok) => tokens.push(tok),
                Err(error) => {
                    return Err(ParseError::new(format!("lexer error: {:?}", error), 0));
                }
            }
        }
        let mut parser = Self::new(&tokens);
        parser.parse()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Result<Expression, ParseError> {
        ExpressionParser::parse_text(input)
    }

    #[test]
    fn parse_bool_true() {
        let expr = parse("true").unwrap();
        assert_eq!(expr, Expression::Literal(LiteralValue::Boolean(true)));
    }

    #[test]
    fn parse_bool_false() {
        let expr = parse("false").unwrap();
        assert_eq!(expr, Expression::Literal(LiteralValue::Boolean(false)));
    }

    #[test]
    fn parse_null() {
        let expr = parse("null").unwrap();
        assert_eq!(expr, Expression::Literal(LiteralValue::Null));
    }

    #[test]
    fn parse_int() {
        let expr = parse("42").unwrap();
        assert_eq!(expr, Expression::Literal(LiteralValue::Integer(42)));
    }

    #[test]
    fn parse_negative_int() {
        let expr = parse("7").unwrap();
        assert_eq!(expr, Expression::Literal(LiteralValue::Integer(7)));
    }

    #[test]
    fn parse_float() {
        let expr = parse("2.71").unwrap();
        assert_eq!(expr, Expression::Literal(LiteralValue::Float(2.71)));
    }

    #[test]
    fn parse_string() {
        let expr = parse("'hello'").unwrap();
        assert_eq!(
            expr,
            Expression::Literal(LiteralValue::String("hello".into()))
        );
    }

    #[test]
    fn parse_variable() {
        let expr = parse("github").unwrap();
        assert_eq!(expr, Expression::Variable("github".into()));
    }

    #[test]
    fn parse_variable_env() {
        let expr = parse("env").unwrap();
        assert_eq!(expr, Expression::Variable("env".into()));
    }

    #[test]
    fn parse_property_access() {
        let expr = parse("github.ref").unwrap();
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
        let expr = parse("github.event_name").unwrap();
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
        let expr = parse("a.b.c").unwrap();
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
        let expr = parse("arr[0]").unwrap();
        assert_eq!(
            expr,
            Expression::IndexAccess(
                Box::new(Expression::Variable("arr".into())),
                Box::new(Expression::Literal(LiteralValue::Integer(0)))
            )
        );
    }

    #[test]
    fn parse_index_access_string_key() {
        let expr = parse("obj['key']").unwrap();
        assert_eq!(
            expr,
            Expression::IndexAccess(
                Box::new(Expression::Variable("obj".into())),
                Box::new(Expression::Literal(LiteralValue::String("key".into())))
            )
        );
    }

    #[test]
    fn parse_array_deref() {
        let expr = parse("foo.*").unwrap();
        assert_eq!(
            expr,
            Expression::ArrayDereference(Box::new(Expression::Variable("foo".into())))
        );
    }

    #[test]
    fn parse_func_call_no_args() {
        let expr = parse("success()").unwrap();
        assert_eq!(expr, Expression::FunctionCall("success".into(), vec![]));
    }

    #[test]
    fn parse_func_call_one_arg() {
        let expr = parse("always()").unwrap();
        assert_eq!(expr, Expression::FunctionCall("always".into(), vec![]));
    }

    #[test]
    fn parse_func_call_two_args() {
        let expr = parse("contains('hello', 'll')").unwrap();
        assert_eq!(
            expr,
            Expression::FunctionCall(
                "contains".into(),
                vec![
                    Expression::Literal(LiteralValue::String("hello".into())),
                    Expression::Literal(LiteralValue::String("ll".into()))
                ]
            )
        );
    }

    #[test]
    fn parse_eq() {
        let expr = parse("a == b").unwrap();
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
        let expr = parse("a != b").unwrap();
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
        let expr = parse("a < b").unwrap();
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
        let expr = parse("a > b").unwrap();
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
        let expr = parse("a <= b").unwrap();
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
        let expr = parse("a >= b").unwrap();
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
        let expr = parse("a && b").unwrap();
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
        let expr = parse("a || b").unwrap();
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
        let expr = parse("a && b || c").unwrap();
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
        let expr = parse("a || b && c").unwrap();
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
        let expr = parse("!a").unwrap();
        assert_eq!(
            expr,
            Expression::Not(Box::new(Expression::Variable("a".into())))
        );
    }

    #[test]
    fn parse_double_not() {
        let expr = parse("!!a").unwrap();
        assert_eq!(
            expr,
            Expression::Not(Box::new(Expression::Not(Box::new(Expression::Variable(
                "a".into()
            )))))
        );
    }

    #[test]
    fn parse_not_compare() {
        let expr = parse("!a == b").unwrap();
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
        let expr = parse("(a)").unwrap();
        assert_eq!(expr, Expression::Variable("a".into()));
    }

    #[test]
    fn parse_parens_override_precedence() {
        let expr = parse("(a || b) && c").unwrap();
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
        let expr = parse("github.ref == 'refs/heads/main' && success()").unwrap();
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
                    Box::new(Expression::Literal(LiteralValue::String(
                        "refs/heads/main".into()
                    )))
                )),
                Box::new(Expression::FunctionCall("success".into(), vec![]))
            )
        );
    }

    #[test]
    fn parse_chained_postfix() {
        let expr = parse("foo.bar[0].baz").unwrap();
        assert_eq!(
            expr,
            Expression::PropertyAccess(
                Box::new(Expression::IndexAccess(
                    Box::new(Expression::PropertyAccess(
                        Box::new(Expression::Variable("foo".into())),
                        "bar".into()
                    )),
                    Box::new(Expression::Literal(LiteralValue::Integer(0)))
                )),
                "baz".into()
            )
        );
    }

    #[test]
    fn parse_error_empty() {
        let result = parse("");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_trailing_tokens() {
        let result = parse("a b");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_unclosed_paren() {
        let result = parse("(a");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_unclosed_bracket() {
        let result = parse("a[0");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_func_call_on_non_variable() {
        let result = parse("(true)(x)");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_unexpected_token_in_primary() {
        let result = parse("&&");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_mismatched_delimiter() {
        let result = parse("a[1 2]");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_expect_ident_mismatch() {
        let result = parse("a.1");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_expect_ident_end() {
        let result = parse("a.");
        assert!(result.is_err());
    }

    #[test]
    fn parse_expression_reports_lexer_error() {
        let result = ExpressionParser::parse_text("'");
        assert!(result.is_err());
    }
}
