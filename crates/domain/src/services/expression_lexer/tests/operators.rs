#[cfg(test)]
mod tests {
    use crate::{services::expression_lexer::ExpressionLexer, value_objects::ExpressionToken};

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
}
