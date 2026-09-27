#[cfg(test)]
mod tests {
    use crate::{services::expression_lexer::ExpressionLexer, value_objects::ExpressionToken};

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
}
