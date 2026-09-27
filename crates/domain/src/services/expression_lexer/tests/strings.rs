#[cfg(test)]
mod tests {
    use crate::{
        errors::LexerError, services::expression_lexer::ExpressionLexer,
        value_objects::ExpressionToken,
    };

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
