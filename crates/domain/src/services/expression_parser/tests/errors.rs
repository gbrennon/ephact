use super::*;

#[cfg(test)]
mod tests {
    use super::*;

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
}
