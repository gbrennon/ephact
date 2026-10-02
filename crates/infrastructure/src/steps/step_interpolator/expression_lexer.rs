use super::{
    ExpressionCursor, ExpressionToken, IdentifierToken, NumberLiteral, OperatorToken, StringLiteral,
};
use crate::application::errors::LexerError;

/// A hand-written lexer for workflow `${{ }}` expression syntax.
///
/// Tokenizes the input stream one token at a time. Supports single-character
/// lookahead via [`peek_token`](ExpressionLexer::peek_token).
pub struct ExpressionLexer<'a> {
    cursor: ExpressionCursor<'a>,
    peeked: Option<ExpressionToken>,
}

impl<'a> ExpressionLexer<'a> {
    /// Creates a new lexer for the given input string.
    ///
    /// The lexer borrows the input; no copying is performed.
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: ExpressionCursor::new(input),
            peeked: None,
        }
    }

    /// Returns the next token without consuming it.
    ///
    /// Subsequent calls to `peek_token` return the same token. The token is
    /// only consumed when [`next_token`](ExpressionLexer::next_token) is called.
    ///
    /// # Errors
    ///
    /// Returns [`LexerError`] if the next characters form an invalid token.
    #[cfg(test)]
    fn peek_token(&mut self) -> Result<ExpressionToken, LexerError> {
        if let Some(token) = &self.peeked {
            return Ok(token.clone());
        }
        let token = self.advance()?;
        self.peeked = Some(token.clone());
        Ok(token)
    }

    /// Consumes and returns the next token from the input stream.
    ///
    /// Returns [`ExpressionToken::EndOfInput`] once the entire input has been consumed.
    ///
    /// # Errors
    ///
    /// Returns [`LexerError`] if the next characters form an invalid token.
    pub fn next_token(&mut self) -> Result<ExpressionToken, LexerError> {
        match self.peeked.take() {
            Some(token) => Ok(token),
            None => self.advance(),
        }
    }

    fn advance(&mut self) -> Result<ExpressionToken, LexerError> {
        self.skip_whitespace();
        self.dispatch_char()
    }

    fn skip_whitespace(&mut self) {
        while self
            .cursor
            .current()
            .is_some_and(|ch| ch.is_ascii_whitespace())
        {
            self.cursor.advance();
        }
    }

    fn dispatch_char(&mut self) -> Result<ExpressionToken, LexerError> {
        match self.cursor.current() {
            Some('\'') => StringLiteral::recognize(&mut self.cursor),
            Some(ch) => self.dispatch_non_string_char(ch),
            None => Ok(ExpressionToken::EndOfInput),
        }
    }

    fn dispatch_non_string_char(&mut self, ch: char) -> Result<ExpressionToken, LexerError> {
        if let Some(token) = OperatorToken::recognize(&mut self.cursor)? {
            return Ok(token);
        }
        if NumberLiteral::starts(&self.cursor) {
            return Ok(NumberLiteral::recognize(&mut self.cursor));
        }
        if IdentifierToken::starts(ch) {
            return Ok(IdentifierToken::recognize(&mut self.cursor));
        }
        Err(LexerError::UnexpectedChar(ch, self.cursor.position()))
    }
}

#[cfg(test)]
mod tests {
    use super::{ExpressionToken, *};

    impl<'a> ExpressionLexer<'a> {
        fn lex_all_for_test(input: &'a str) -> Result<Vec<ExpressionToken>, LexerError> {
            let mut lexer = ExpressionLexer::new(input);
            let mut tokens = Vec::new();
            loop {
                let token = lexer.next_token()?;
                let done = matches!(token, ExpressionToken::EndOfInput);
                tokens.push(token);
                if done {
                    break;
                }
            }
            Ok(tokens)
        }
    }

    #[test]
    fn lex_unexpected_char() {
        let err = ExpressionLexer::lex_all_for_test("@").unwrap_err();
        assert_eq!(err, LexerError::UnexpectedChar('@', 0));
    }

    #[test]
    fn lex_lone_ampersand_error() {
        let err = ExpressionLexer::lex_all_for_test("&").unwrap_err();
        assert_eq!(err, LexerError::UnexpectedChar('&', 0));
    }

    #[test]
    fn lex_lone_pipe_error() {
        let err = ExpressionLexer::lex_all_for_test("|").unwrap_err();
        assert_eq!(err, LexerError::UnexpectedChar('|', 0));
    }

    #[test]
    fn lex_lone_equals_error() {
        let err = ExpressionLexer::lex_all_for_test("=").unwrap_err();
        assert_eq!(err, LexerError::UnexpectedChar('=', 0));
    }

    #[test]
    fn lex_true() {
        let tokens = ExpressionLexer::lex_all_for_test("true").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Boolean(true), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_false() {
        let tokens = ExpressionLexer::lex_all_for_test("false").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Boolean(false), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_null() {
        let tokens = ExpressionLexer::lex_all_for_test("null").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Null, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_simple_ident() {
        let tokens = ExpressionLexer::lex_all_for_test("source").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("source".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_ident_with_underscore() {
        let tokens = ExpressionLexer::lex_all_for_test("event_name").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("event_name".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_ident_with_hyphen() {
        let tokens = ExpressionLexer::lex_all_for_test("my-job").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("my-job".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_ident_with_digits() {
        let tokens = ExpressionLexer::lex_all_for_test("step1").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("step1".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_ident_starting_with_underscore() {
        let tokens = ExpressionLexer::lex_all_for_test("_private").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("_private".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn peek_does_not_consume() {
        let mut lexer = ExpressionLexer::new("foo");
        let first = lexer.peek_token().unwrap();
        let second = lexer.peek_token().unwrap();
        assert_eq!(first, ExpressionToken::Identifier("foo".into()));
        assert_eq!(second, ExpressionToken::Identifier("foo".into()));
    }

    #[test]
    fn peek_then_next_consumes() {
        let mut lexer = ExpressionLexer::new("foo bar");
        assert_eq!(
            lexer.peek_token().unwrap(),
            ExpressionToken::Identifier("foo".into())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            ExpressionToken::Identifier("foo".into())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            ExpressionToken::Identifier("bar".into())
        );
        assert_eq!(lexer.next_token().unwrap(), ExpressionToken::EndOfInput);
    }

    #[test]
    fn peek_after_next_returns_next_token() {
        let mut lexer = ExpressionLexer::new("a b");
        assert_eq!(
            lexer.next_token().unwrap(),
            ExpressionToken::Identifier("a".into())
        );
        assert_eq!(
            lexer.peek_token().unwrap(),
            ExpressionToken::Identifier("b".into())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            ExpressionToken::Identifier("b".into())
        );
    }

    #[test]
    fn lex_skips_whitespace() {
        let tokens = ExpressionLexer::lex_all_for_test("  \t\n\r  foo  ").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("foo".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_full_expression() {
        let tokens =
            ExpressionLexer::lex_all_for_test("source.event_name == 'push' && !cancelled()")
                .unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("source".into()),
                ExpressionToken::Dot,
                ExpressionToken::Identifier("event_name".into()),
                ExpressionToken::Equal,
                ExpressionToken::String("push".into()),
                ExpressionToken::And,
                ExpressionToken::Not,
                ExpressionToken::Identifier("cancelled".into()),
                ExpressionToken::LeftParenthesis,
                ExpressionToken::RightParenthesis,
                ExpressionToken::EndOfInput,
            ]
        );
    }

    #[test]
    fn lex_function_call_with_args() {
        let tokens = ExpressionLexer::lex_all_for_test("contains('hello', 'll')").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("contains".into()),
                ExpressionToken::LeftParenthesis,
                ExpressionToken::String("hello".into()),
                ExpressionToken::Comma,
                ExpressionToken::String("ll".into()),
                ExpressionToken::RightParenthesis,
                ExpressionToken::EndOfInput,
            ]
        );
    }

    #[test]
    fn lex_positive_int() {
        let tokens = ExpressionLexer::lex_all_for_test("42").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Integer(42), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_negative_int() {
        let tokens = ExpressionLexer::lex_all_for_test("-7").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Integer(-7), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_zero() {
        let tokens = ExpressionLexer::lex_all_for_test("0").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Integer(0), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_float() {
        let tokens = ExpressionLexer::lex_all_for_test("2.71").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Float(2.71), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_negative_float() {
        let tokens = ExpressionLexer::lex_all_for_test("-0.5").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Float(-0.5), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_float_with_trailing_dot_is_int() {
        let tokens = ExpressionLexer::lex_all_for_test("42.").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Integer(42),
                ExpressionToken::Dot,
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_dot() {
        let tokens = ExpressionLexer::lex_all_for_test(".").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Dot, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_brackets() {
        let tokens = ExpressionLexer::lex_all_for_test("[]").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::LeftBracket,
                ExpressionToken::RightBracket,
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_parens() {
        let tokens = ExpressionLexer::lex_all_for_test("()").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::LeftParenthesis,
                ExpressionToken::RightParenthesis,
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_star() {
        let tokens = ExpressionLexer::lex_all_for_test("*").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Star, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_comma() {
        let tokens = ExpressionLexer::lex_all_for_test(",").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Comma, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_not() {
        let tokens = ExpressionLexer::lex_all_for_test("!").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Not, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_eq() {
        let tokens = ExpressionLexer::lex_all_for_test("==").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Equal, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_neq() {
        let tokens = ExpressionLexer::lex_all_for_test("!=").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::NotEqual, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_lt() {
        let tokens = ExpressionLexer::lex_all_for_test("<").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::LessThan, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_lte() {
        let tokens = ExpressionLexer::lex_all_for_test("<=").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::LessThanOrEqual,
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_gt() {
        let tokens = ExpressionLexer::lex_all_for_test(">").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::GreaterThan, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_gte() {
        let tokens = ExpressionLexer::lex_all_for_test(">=").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::GreaterThanOrEqual,
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_and() {
        let tokens = ExpressionLexer::lex_all_for_test("&&").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::And, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_or() {
        let tokens = ExpressionLexer::lex_all_for_test("||").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Or, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_simple_string() {
        let tokens = ExpressionLexer::lex_all_for_test("'hello'").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::String("hello".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_empty_string() {
        let tokens = ExpressionLexer::lex_all_for_test("''").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::String(String::new()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_string_with_escaped_quote() {
        let tokens = ExpressionLexer::lex_all_for_test("'it''s'").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::String("it's".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_string_with_multiple_escapes() {
        let tokens = ExpressionLexer::lex_all_for_test("'a''b''c'").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::String("a'b'c".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_unterminated_string_error() {
        let err = ExpressionLexer::lex_all_for_test("'no end").unwrap_err();
        assert_eq!(err, LexerError::UnterminatedString(0));
    }
}
