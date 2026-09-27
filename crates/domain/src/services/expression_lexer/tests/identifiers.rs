#[cfg(test)]
mod tests {
    use crate::{services::expression_lexer::ExpressionLexer, value_objects::ExpressionToken};

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
        let tokens = ExpressionLexer::lex_all_for_test("github").unwrap();
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
}
