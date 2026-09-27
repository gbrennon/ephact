#[cfg(test)]
mod tests {
    use crate::{services::expression_lexer::ExpressionLexer, value_objects::ExpressionToken};

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
            ExpressionLexer::lex_all_for_test("github.event_name == 'push' && !cancelled()")
                .unwrap();
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
}
