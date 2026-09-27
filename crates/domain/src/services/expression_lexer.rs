use crate::{
    errors::LexerError,
    value_objects::{
        ExpressionToken, expression_cursor::ExpressionCursor, identifier_token::IdentifierToken,
        number_literal::NumberLiteral, operator_token::OperatorToken,
        string_literal::StringLiteral,
    },
};

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
    pub fn peek_token(&mut self) -> Result<ExpressionToken, LexerError> {
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
mod tests;
