use crate::domain::{errors::LexerError, value_objects::ExpressionToken};

/// A hand-written lexer for GitHub Actions `${{ }}` expression syntax.
///
/// Tokenizes the input stream one token at a time. Supports single-character
/// lookahead via [`peek_token`](ExpressionLexer::peek_token).
enum StringChunk {
    Char(char),
    Terminated,
}

pub struct ExpressionLexer<'a> {
    /// Remaining characters to tokenize.
    chars: &'a str,
    /// Current byte position in the original input.
    pos: usize,
    /// Buffered token from a previous peek, if any.
    peeked: Option<ExpressionToken>,
}

impl<'a> ExpressionLexer<'a> {
    /// Creates a new lexer for the given input string.
    ///
    /// The lexer borrows the input; no copying is performed.
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input,
            pos: 0,
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
    pub fn peek_token(&mut self) -> Result<ExpressionToken, LexerError> {
        if let Some(ref token) = self.peeked {
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
        if let Some(token) = self.peeked.take() {
            return Ok(token);
        }
        self.advance()
    }

    /// Core tokenization logic. Skips whitespace, then dispatches on the
    /// current character to produce the next token.
    fn advance(&mut self) -> Result<ExpressionToken, LexerError> {
        self.skip_whitespace();
        let ch = match self.current_char() {
            Some(ch) => ch,
            None => return Ok(ExpressionToken::EndOfInput),
        };
        self.dispatch_char(ch)
    }

    fn dispatch_char(&mut self, ch: char) -> Result<ExpressionToken, LexerError> {
        if ch == '\x27' {
            return self.lex_string();
        }
        if let Some(op) = self.lex_operator(ch) {
            return op;
        }
        if self.is_number_start(ch) {
            return self.lex_number();
        }
        if Self::is_ident_start(ch) {
            return self.lex_ident_or_keyword();
        }
        Err(LexerError::UnexpectedChar(ch, self.pos))
    }

    fn is_ident_start(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_number_start(&self, ch: char) -> bool {
        ch.is_ascii_digit() || (ch == '-' && self.peek_is_digit())
    }

    fn peek_is_digit(&self) -> bool {
        match self.peek_next_char() {
            Some(n) => n.is_ascii_digit(),
            None => false,
        }
    }

    fn lex_operator(&mut self, ch: char) -> Option<Result<ExpressionToken, LexerError>> {
        match ch {
            '.' => Some(self.bump_token(ExpressionToken::Dot)),
            '[' => Some(self.bump_token(ExpressionToken::LeftBracket)),
            ']' => Some(self.bump_token(ExpressionToken::RightBracket)),
            '(' => Some(self.bump_token(ExpressionToken::LeftParenthesis)),
            ')' => Some(self.bump_token(ExpressionToken::RightParenthesis)),
            '*' => Some(self.bump_token(ExpressionToken::Star)),
            ',' => Some(self.bump_token(ExpressionToken::Comma)),
            _ => self.lex_compound_operator(ch),
        }
    }

    fn bump_token(&mut self, tok: ExpressionToken) -> Result<ExpressionToken, LexerError> {
        self.bump();
        Ok(tok)
    }

    fn lex_compound_operator(&mut self, ch: char) -> Option<Result<ExpressionToken, LexerError>> {
        match ch {
            '!' => Some(self.lex_exclamation()),
            '=' => Some(self.lex_equals()),
            '<' => Some(self.lex_less_than()),
            '>' => Some(self.lex_greater_than()),
            '&' => Some(self.lex_ampersand()),
            '|' => Some(self.lex_pipe()),
            _ => None,
        }
    }

    fn lex_exclamation(&mut self) -> Result<ExpressionToken, LexerError> {
        self.bump();
        if self.current_char() == Some('=') {
            self.bump();
            Ok(ExpressionToken::NotEqual)
        } else {
            Ok(ExpressionToken::Not)
        }
    }

    fn lex_equals(&mut self) -> Result<ExpressionToken, LexerError> {
        self.bump();
        if self.current_char() == Some('=') {
            self.bump();
            Ok(ExpressionToken::Equal)
        } else {
            Err(LexerError::UnexpectedChar('=', self.pos - 1))
        }
    }

    fn lex_less_than(&mut self) -> Result<ExpressionToken, LexerError> {
        self.bump();
        if self.current_char() == Some('=') {
            self.bump();
            Ok(ExpressionToken::LessThanOrEqual)
        } else {
            Ok(ExpressionToken::LessThan)
        }
    }

    fn lex_greater_than(&mut self) -> Result<ExpressionToken, LexerError> {
        self.bump();
        if self.current_char() == Some('=') {
            self.bump();
            Ok(ExpressionToken::GreaterThanOrEqual)
        } else {
            Ok(ExpressionToken::GreaterThan)
        }
    }

    fn lex_ampersand(&mut self) -> Result<ExpressionToken, LexerError> {
        self.bump();
        if self.current_char() == Some('&') {
            self.bump();
            Ok(ExpressionToken::And)
        } else {
            Err(LexerError::UnexpectedChar('&', self.pos - 1))
        }
    }

    fn lex_pipe(&mut self) -> Result<ExpressionToken, LexerError> {
        self.bump();
        if self.current_char() == Some('|') {
            self.bump();
            Ok(ExpressionToken::Or)
        } else {
            Err(LexerError::UnexpectedChar('|', self.pos - 1))
        }
    }

    /// Lexes a single-quoted string literal.
    ///
    /// Supports `''` as an escape sequence for a literal single quote within
    /// the string.
    fn lex_string(&mut self) -> Result<ExpressionToken, LexerError> {
        let start = self.pos;
        self.bump();

        let mut value = String::new();

        loop {
            match self.next_string_char(start)? {
                StringChunk::Char(c) => value.push(c),
                StringChunk::Terminated => return Ok(ExpressionToken::String(value)),
            }
        }
    }

    fn next_string_char(&mut self, start: usize) -> Result<StringChunk, LexerError> {
        let ch = match self.current_char() {
            Some(ch) => ch,
            None => return Err(LexerError::UnterminatedString(start)),
        };

        if ch == '\x27' {
            self.bump();
            if self.current_char() == Some('\x27') {
                self.bump();
                Ok(StringChunk::Char('\x27'))
            } else {
                Ok(StringChunk::Terminated)
            }
        } else {
            self.bump();
            Ok(StringChunk::Char(ch))
        }
    }

    /// Lexes a numeric literal: integer or float.
    ///
    /// Handles an optional leading `-` for negative numbers. A decimal point
    /// followed by at least one digit produces a [`ExpressionToken::Float`]; otherwise
    /// the result is a [`ExpressionToken::Integer`].
    fn lex_number(&mut self) -> Result<ExpressionToken, LexerError> {
        let mut num_str = String::new();
        if self.current_char() == Some('-') {
            num_str.push('-');
            self.bump();
        }
        self.consume_digits(&mut num_str);
        let is_float = self.consume_fractional_part(&mut num_str);
        Self::parse_number_token(&num_str, is_float)
    }

    fn consume_digits(&mut self, num_str: &mut String) {
        while self.current_char().is_some_and(|c| c.is_ascii_digit()) {
            num_str.push(self.current_char().unwrap());
            self.bump();
        }
    }

    fn consume_fractional_part(&mut self, num_str: &mut String) -> bool {
        if self.current_char() == Some('.')
            && self.peek_next_char().is_some_and(|c| c.is_ascii_digit())
        {
            num_str.push('.');
            self.bump();
            self.consume_digits(num_str);
            true
        } else {
            false
        }
    }
    fn parse_number_token(num_str: &str, is_float: bool) -> Result<ExpressionToken, LexerError> {
        if is_float {
            let value: f64 = num_str
                .parse()
                .expect("lex_number produced an unparseable float");
            Ok(ExpressionToken::Float(value))
        } else {
            let value: i64 = num_str
                .parse()
                .expect("lex_number produced an unparseable int");
            Ok(ExpressionToken::Integer(value))
        }
    }

    /// Lexes an identifier or keyword (`true`, `false`, `null`).
    ///
    /// Identifiers match `[a-zA-Z_][a-zA-Z0-9_-]*`. If the lexeme matches a
    /// reserved keyword, the corresponding keyword token is returned.
    fn lex_ident_or_keyword(&mut self) -> Result<ExpressionToken, LexerError> {
        let mut ident = String::new();

        ident.push(self.current_char().unwrap());
        self.bump();

        while self
            .current_char()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            ident.push(self.current_char().unwrap());
            self.bump();
        }

        match ident.as_str() {
            "true" => Ok(ExpressionToken::Boolean(true)),
            "false" => Ok(ExpressionToken::Boolean(false)),
            "null" => Ok(ExpressionToken::Null),
            _ => Ok(ExpressionToken::Identifier(ident)),
        }
    }

    /// Advances past any whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.current_char().is_some_and(|c| c.is_ascii_whitespace()) {
            self.bump();
        }
    }

    /// Returns the current character without consuming it.
    fn current_char(&self) -> Option<char> {
        self.chars.chars().next()
    }

    /// Returns the character after the current one without consuming anything.
    fn peek_next_char(&self) -> Option<char> {
        let mut iter = self.chars.chars();
        iter.next();
        iter.next()
    }

    /// Consumes the current character and advances the position.
    fn bump(&mut self) {
        if let Some(ch) = self.chars.chars().next() {
            self.chars = &self.chars[ch.len_utf8()..];
            self.pos += ch.len_utf8();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: collect all tokens from input into a Vec.
    fn lex_all(input: &str) -> Result<Vec<ExpressionToken>, LexerError> {
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

    #[test]
    fn lex_dot() {
        let tokens = lex_all(".").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Dot, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_brackets() {
        let tokens = lex_all("[]").unwrap();
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
        let tokens = lex_all("()").unwrap();
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
        let tokens = lex_all("*").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Star, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_comma() {
        let tokens = lex_all(",").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Comma, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_not() {
        let tokens = lex_all("!").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Not, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_eq() {
        let tokens = lex_all("==").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Equal, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_neq() {
        let tokens = lex_all("!=").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::NotEqual, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_lt() {
        let tokens = lex_all("<").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::LessThan, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_lte() {
        let tokens = lex_all("<=").unwrap();
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
        let tokens = lex_all(">").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::GreaterThan, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_gte() {
        let tokens = lex_all(">=").unwrap();
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
        let tokens = lex_all("&&").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::And, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_or() {
        let tokens = lex_all("||").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Or, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_simple_string() {
        let tokens = lex_all("'hello'").unwrap();
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
        let tokens = lex_all("''").unwrap();
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
        let tokens = lex_all("'it''s'").unwrap();
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
        let tokens = lex_all("'a''b''c'").unwrap();
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
        let err = lex_all("'no end").unwrap_err();
        assert_eq!(err, LexerError::UnterminatedString(0));
    }

    #[test]
    fn lex_positive_int() {
        let tokens = lex_all("42").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Integer(42), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_negative_int() {
        let tokens = lex_all("-7").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Integer(-7), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_zero() {
        let tokens = lex_all("0").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Integer(0), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_float() {
        let tokens = lex_all("2.71").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Float(2.71), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_negative_float() {
        let tokens = lex_all("-0.5").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Float(-0.5), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_float_with_trailing_dot_is_int() {
        let tokens = lex_all("42.").unwrap();
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
    fn lex_true() {
        let tokens = lex_all("true").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Boolean(true), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_false() {
        let tokens = lex_all("false").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Boolean(false), ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_null() {
        let tokens = lex_all("null").unwrap();
        assert_eq!(
            tokens,
            vec![ExpressionToken::Null, ExpressionToken::EndOfInput]
        );
    }

    #[test]
    fn lex_simple_ident() {
        let tokens = lex_all("github").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("github".into()),
                ExpressionToken::EndOfInput
            ]
        );
    }

    #[test]
    fn lex_ident_with_underscore() {
        let tokens = lex_all("event_name").unwrap();
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
        let tokens = lex_all("my-job").unwrap();
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
        let tokens = lex_all("step1").unwrap();
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
        let tokens = lex_all("_private").unwrap();
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
        let tokens = lex_all("  \t\n\r  foo  ").unwrap();
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
        let tokens = lex_all("github.event_name == 'push' && !cancelled()").unwrap();
        assert_eq!(
            tokens,
            vec![
                ExpressionToken::Identifier("github".into()),
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
        let tokens = lex_all("contains('hello', 'll')").unwrap();
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
    fn lex_unexpected_char() {
        let err = lex_all("@").unwrap_err();
        assert_eq!(err, LexerError::UnexpectedChar('@', 0));
    }

    #[test]
    fn lex_lone_ampersand_error() {
        let err = lex_all("&").unwrap_err();
        assert_eq!(err, LexerError::UnexpectedChar('&', 0));
    }

    #[test]
    fn lex_lone_pipe_error() {
        let err = lex_all("|").unwrap_err();
        assert_eq!(err, LexerError::UnexpectedChar('|', 0));
    }

    #[test]
    fn lex_lone_equals_error() {
        let err = lex_all("=").unwrap_err();
        assert_eq!(err, LexerError::UnexpectedChar('=', 0));
    }
}
