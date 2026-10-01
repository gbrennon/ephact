use super::expression_lexer::ExpressionLexer;
use crate::{
    errors::ParseError,
    value_objects::{
        ComparisonOperator, Expression, ExpressionLiteral, ExpressionToken, LogicalOperator,
    },
};

/// Recursive-descent parser for expression tokens.
pub struct ExpressionParser {
    tokens: Vec<ExpressionToken>,
    pos: usize,
}

impl ExpressionParser {
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
        loop {
            let before = self.pos;
            expr = self.parse_next_postfix(expr)?;
            if self.pos == before {
                return Ok(expr);
            }
        }
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
                Ok(Expression::Variable(name))
            }
            Some(ExpressionToken::String(s)) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::String(s)))
            }
            Some(ExpressionToken::Integer(n)) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::Integer(n)))
            }
            Some(ExpressionToken::Float(f)) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::Float(f)))
            }
            Some(ExpressionToken::Boolean(b)) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::Boolean(b)))
            }
            Some(ExpressionToken::Null) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::Null))
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
        self.pos += 1;
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
    use crate::value_objects::{
        ComparisonOperator, Expression, ExpressionLiteral, LogicalOperator,
    };

    #[test]
    fn parse_error_empty() {
        let result = ExpressionParser::parse_text("");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_trailing_tokens() {
        let result = ExpressionParser::parse_text("a b");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_unclosed_paren() {
        let result = ExpressionParser::parse_text("(a");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_unclosed_bracket() {
        let result = ExpressionParser::parse_text("a[0");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_func_call_on_non_variable() {
        let result = ExpressionParser::parse_text("(true)(x)");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_unexpected_token_in_primary() {
        let result = ExpressionParser::parse_text("&&");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_mismatched_delimiter() {
        let result = ExpressionParser::parse_text("a[1 2]");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_expect_ident_mismatch() {
        let result = ExpressionParser::parse_text("a.1");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_expect_ident_end() {
        let result = ExpressionParser::parse_text("a.");
        assert!(result.is_err());
    }

    #[test]
    fn parse_expression_reports_lexer_error() {
        let result = ExpressionParser::parse_text("'");
        assert!(result.is_err());
    }

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
        let expr = ExpressionParser::parse_text("source").unwrap();
        assert_eq!(expr, Expression::Variable("source".into()));
    }

    #[test]
    fn parse_variable_env() {
        let expr = ExpressionParser::parse_text("env").unwrap();
        assert_eq!(expr, Expression::Variable("env".into()));
    }

    #[test]
    fn parse_property_access() {
        let expr = ExpressionParser::parse_text("source.ref").unwrap();
        assert_eq!(
            expr,
            Expression::PropertyAccess(
                Box::new(Expression::Variable("source".into())),
                "ref".into()
            )
        );
    }

    #[test]
    fn parse_nested_property_access() {
        let expr = ExpressionParser::parse_text("source.event_name").unwrap();
        assert_eq!(
            expr,
            Expression::PropertyAccess(
                Box::new(Expression::Variable("source".into())),
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
            ExpressionParser::parse_text("source.ref == 'refs/heads/main' && success()").unwrap();
        assert_eq!(
            expr,
            Expression::Logical(
                LogicalOperator::And,
                Box::new(Expression::Comparison(
                    ComparisonOperator::Equal,
                    Box::new(Expression::PropertyAccess(
                        Box::new(Expression::Variable("source".into())),
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
