use super::{
    ComparisonOperator, Expression, ExpressionLiteral, ExpressionToken, LogicalOperator,
    expression_lexer::ExpressionLexer,
};
use crate::application::errors::ParseError;

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
    pub fn parse(self) -> Result<Expression, ParseError> {
        let (parser, expr) = self.parse_expression()?;
        if parser.pos < parser.tokens.len() {
            return Err(parser.error("unexpected tokens after expression"));
        }
        Ok(expr)
    }

    fn parse_expression(self) -> Result<(Self, Expression), ParseError> {
        self.parse_logical()
    }

    fn parse_logical(self) -> Result<(Self, Expression), ParseError> {
        let (parser, left) = self.parse_compare()?;
        parser.parse_logical_tail(left)
    }

    fn parse_logical_tail(self, left: Expression) -> Result<(Self, Expression), ParseError> {
        let op = match self.peek().cloned() {
            Some(ExpressionToken::And) => LogicalOperator::And,
            Some(ExpressionToken::Or) => LogicalOperator::Or,
            _ => return Ok((self, left)),
        };
        let parser = self.advance();
        let (parser, right) = parser.parse_compare()?;
        let left = Expression::Logical(op, Box::new(left), Box::new(right));
        parser.parse_logical_tail(left)
    }

    fn parse_compare(self) -> Result<(Self, Expression), ParseError> {
        let (parser, left) = self.parse_unary()?;
        let op = match parser.peek().cloned() {
            Some(ExpressionToken::Equal) => ComparisonOperator::Equal,
            Some(ExpressionToken::NotEqual) => ComparisonOperator::NotEqual,
            Some(ExpressionToken::LessThan) => ComparisonOperator::LessThan,
            Some(ExpressionToken::LessThanOrEqual) => ComparisonOperator::LessThanOrEqual,
            Some(ExpressionToken::GreaterThan) => ComparisonOperator::GreaterThan,
            Some(ExpressionToken::GreaterThanOrEqual) => ComparisonOperator::GreaterThanOrEqual,
            _ => return Ok((parser, left)),
        };
        let parser = parser.advance();
        let (parser, right) = parser.parse_unary()?;
        Ok((
            parser,
            Expression::Comparison(op, Box::new(left), Box::new(right)),
        ))
    }

    fn parse_unary(self) -> Result<(Self, Expression), ParseError> {
        match self.peek().cloned() {
            Some(ExpressionToken::Not) => {
                let parser = self.advance();
                let (parser, inner) = parser.parse_unary()?;
                Ok((parser, Expression::Not(Box::new(inner))))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(self) -> Result<(Self, Expression), ParseError> {
        let (parser, expr) = self.parse_primary()?;
        parser.parse_postfix_tail(expr)
    }

    fn parse_postfix_tail(self, expr: Expression) -> Result<(Self, Expression), ParseError> {
        let before = self.pos;
        let (parser, expr) = self.parse_next_postfix(expr)?;
        if parser.pos == before {
            Ok((parser, expr))
        } else {
            parser.parse_postfix_tail(expr)
        }
    }

    fn parse_next_postfix(self, expr: Expression) -> Result<(Self, Expression), ParseError> {
        match self.peek().cloned() {
            Some(ExpressionToken::Dot) => self.parse_dot_postfix(expr),
            Some(ExpressionToken::LeftBracket) => self.parse_index_postfix(expr),
            Some(ExpressionToken::LeftParenthesis) => self.parse_call_postfix(expr),
            _ => Ok((self, expr)),
        }
    }

    fn parse_dot_postfix(self, expr: Expression) -> Result<(Self, Expression), ParseError> {
        let parser = self.advance();
        if parser.peek() == Some(&ExpressionToken::Star) {
            let parser = parser.advance();
            Ok((parser, Expression::ArrayDereference(Box::new(expr))))
        } else {
            let (parser, ident) = parser.expect_ident("property name after '.'")?;
            Ok((parser, Expression::PropertyAccess(Box::new(expr), ident)))
        }
    }

    fn parse_index_postfix(self, expr: Expression) -> Result<(Self, Expression), ParseError> {
        let parser = self.advance();
        let (parser, idx) = parser.parse_expression()?;
        let parser = parser.expect(ExpressionToken::RightBracket, "expected ']'")?;
        Ok((
            parser,
            Expression::IndexAccess(Box::new(expr), Box::new(idx)),
        ))
    }

    fn parse_call_postfix(self, expr: Expression) -> Result<(Self, Expression), ParseError> {
        let parser = self.advance();
        let (parser, args) = parser.parse_args()?;
        let parser = parser.expect(ExpressionToken::RightParenthesis, "expected ')'")?;
        let name = match expr {
            Expression::Variable(name) => name,
            _ => return Err(parser.error("function call requires a function name before '('")),
        };
        Ok((parser, Expression::FunctionCall(name, args)))
    }

    fn parse_primary(self) -> Result<(Self, Expression), ParseError> {
        match self.peek().cloned() {
            Some(ExpressionToken::Identifier(name)) => {
                Ok((self.advance(), Expression::Variable(name)))
            }
            Some(ExpressionToken::String(s)) => Ok((
                self.advance(),
                Expression::Literal(ExpressionLiteral::String(s)),
            )),
            Some(ExpressionToken::Integer(n)) => Ok((
                self.advance(),
                Expression::Literal(ExpressionLiteral::Integer(n)),
            )),
            Some(ExpressionToken::Float(f)) => Ok((
                self.advance(),
                Expression::Literal(ExpressionLiteral::Float(f)),
            )),
            Some(ExpressionToken::Boolean(b)) => Ok((
                self.advance(),
                Expression::Literal(ExpressionLiteral::Boolean(b)),
            )),
            Some(ExpressionToken::Null) => {
                Ok((self.advance(), Expression::Literal(ExpressionLiteral::Null)))
            }
            Some(ExpressionToken::LeftParenthesis) => {
                let parser = self.advance();
                let (parser, expr) = parser.parse_expression()?;
                let parser = parser.expect(ExpressionToken::RightParenthesis, "expected ')'")?;
                Ok((parser, expr))
            }
            Some(other) => Err(self.error(&format!("unexpected token: {other:?}"))),
            None => Err(self.error("unexpected end of expression")),
        }
    }

    fn parse_args(self) -> Result<(Self, Vec<Expression>), ParseError> {
        if self.peek() == Some(&ExpressionToken::RightParenthesis) {
            Ok((self, Vec::new()))
        } else {
            self.parse_more_args(Vec::new())
        }
    }

    fn parse_more_args(self, args: Vec<Expression>) -> Result<(Self, Vec<Expression>), ParseError> {
        let (parser, arg) = self.parse_expression()?;
        let args = args
            .into_iter()
            .chain(std::iter::once(arg))
            .collect::<Vec<_>>();
        if parser.peek() == Some(&ExpressionToken::Comma) {
            parser.advance().parse_more_args(args)
        } else {
            Ok((parser, args))
        }
    }

    fn peek(&self) -> Option<&ExpressionToken> {
        self.tokens.get(self.pos)
    }

    fn advance(self) -> Self {
        Self {
            tokens: self.tokens,
            pos: self.pos + 1,
        }
    }

    fn expect(self, expected: ExpressionToken, msg: &str) -> Result<Self, ParseError> {
        match self.peek().cloned() {
            Some(token) if token == expected => Ok(self.advance()),
            Some(token) => Err(self.error(&format!("{msg}, found {token:?}"))),
            None => Err(self.error(&format!("{msg}, found end of input"))),
        }
    }

    fn expect_ident(self, msg: &str) -> Result<(Self, String), ParseError> {
        match self.peek().cloned() {
            Some(ExpressionToken::Identifier(name)) => Ok((self.advance(), name)),
            Some(token) => Err(self.error(&format!("{msg}, found {token:?}"))),
            None => Err(self.error(&format!("{msg}, found end of input"))),
        }
    }

    fn error(&self, msg: &str) -> ParseError {
        ParseError::new(msg.to_string(), self.pos)
    }
}

impl ExpressionParser {
    fn collect_tokens(
        lexer: ExpressionLexer<'_>,
        tokens: Vec<ExpressionToken>,
    ) -> Result<Vec<ExpressionToken>, ParseError> {
        let (lexer, token) = match lexer.next_token() {
            Ok(result) => result,
            Err(error) => {
                return Err(ParseError::new(format!("lexer error: {error:?}"), 0));
            }
        };
        if matches!(&token, ExpressionToken::EndOfInput) {
            Ok(tokens)
        } else {
            let tokens = tokens.into_iter().chain(std::iter::once(token)).collect();
            Self::collect_tokens(lexer, tokens)
        }
    }

    /// Lexes and parses an expression string.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if lexing or parsing fails.
    pub fn parse_text(input: &str) -> Result<Expression, ParseError> {
        let tokens = Self::collect_tokens(ExpressionLexer::new(input), Vec::new())?;
        Self::new(&tokens).parse()
    }
}
#[cfg(test)]
mod tests {
    use super::{
        super::{ComparisonOperator, Expression, ExpressionLiteral, LogicalOperator},
        *,
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
