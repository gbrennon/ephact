#[cfg(test)]
mod tests {
    use crate::{errors::LexerError, services::expression_lexer::ExpressionLexer};

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
}
