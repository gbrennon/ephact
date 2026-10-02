use super::{ExpressionCursor, ExpressionToken};
use crate::application::errors::LexerError;

pub struct OperatorToken;

impl OperatorToken {
    pub fn recognize(
        cursor: &mut ExpressionCursor<'_>,
    ) -> Result<Option<ExpressionToken>, LexerError> {
        let start = cursor.position();
        let token = match cursor.current() {
            Some(ch @ ('!' | '=')) => Some(Self::not_or_equal(cursor, ch, start)?),
            Some(ch @ ('<' | '>')) => Some(Self::optional_operator(cursor, ch)),
            Some(ch @ ('&' | '|')) => Some(Self::required_operator(cursor, ch, start)?),
            Some(ch) => Self::single(cursor, ch),
            None => None,
        };
        Ok(token)
    }

    fn not_or_equal(
        cursor: &mut ExpressionCursor<'_>,
        operator: char,
        start: usize,
    ) -> Result<ExpressionToken, LexerError> {
        match operator {
            '!' => Ok(Self::compound(
                cursor,
                '=',
                ExpressionToken::NotEqual,
                ExpressionToken::Not,
            )),
            _ => Self::required_compound(cursor, '=', '=', ExpressionToken::Equal, start),
        }
    }

    fn single(cursor: &mut ExpressionCursor<'_>, ch: char) -> Option<ExpressionToken> {
        let token = match ch {
            '.' => Some(ExpressionToken::Dot),
            '[' => Some(ExpressionToken::LeftBracket),
            ']' => Some(ExpressionToken::RightBracket),
            '(' => Some(ExpressionToken::LeftParenthesis),
            ')' => Some(ExpressionToken::RightParenthesis),
            '*' => Some(ExpressionToken::Star),
            ',' => Some(ExpressionToken::Comma),
            _ => None,
        };
        if token.is_some() {
            cursor.advance();
        }
        token
    }

    fn optional_operator(cursor: &mut ExpressionCursor<'_>, operator: char) -> ExpressionToken {
        match operator {
            '<' => Self::compound(
                cursor,
                '=',
                ExpressionToken::LessThanOrEqual,
                ExpressionToken::LessThan,
            ),
            _ => Self::compound(
                cursor,
                '=',
                ExpressionToken::GreaterThanOrEqual,
                ExpressionToken::GreaterThan,
            ),
        }
    }

    fn required_operator(
        cursor: &mut ExpressionCursor<'_>,
        operator: char,
        start: usize,
    ) -> Result<ExpressionToken, LexerError> {
        match operator {
            '&' => Self::required_compound(cursor, '&', '&', ExpressionToken::And, start),
            _ => Self::required_compound(cursor, '|', '|', ExpressionToken::Or, start),
        }
    }

    fn compound(
        cursor: &mut ExpressionCursor<'_>,
        second: char,
        paired: ExpressionToken,
        single: ExpressionToken,
    ) -> ExpressionToken {
        cursor.advance();
        if cursor.current() == Some(second) {
            cursor.advance();
            paired
        } else {
            single
        }
    }

    fn required_compound(
        cursor: &mut ExpressionCursor<'_>,
        second: char,
        unexpected: char,
        paired: ExpressionToken,
        start: usize,
    ) -> Result<ExpressionToken, LexerError> {
        cursor.advance();
        if cursor.current() == Some(second) {
            cursor.advance();
            Ok(paired)
        } else {
            Err(LexerError::UnexpectedChar(unexpected, start))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{ExpressionCursor, ExpressionToken},
        OperatorToken,
    };
    use crate::application::errors::LexerError;

    #[test]
    fn recognizes_single_and_compound_operators() {
        for (input, expected) in [
            (".", ExpressionToken::Dot),
            ("!=", ExpressionToken::NotEqual),
            ("<=", ExpressionToken::LessThanOrEqual),
            ("&&", ExpressionToken::And),
        ] {
            let mut cursor = ExpressionCursor::new(input);
            assert_eq!(
                OperatorToken::recognize(&mut cursor).unwrap(),
                Some(expected)
            );
            assert_eq!(cursor.current(), None);
        }
        let mut cursor = ExpressionCursor::new("&");
        assert_eq!(
            OperatorToken::recognize(&mut cursor),
            Err(LexerError::UnexpectedChar('&', 0))
        );
    }

    #[test]
    fn recognizes_every_compound_and_single_variant() {
        for (input, expected) in [
            ("==", ExpressionToken::Equal),
            (">=", ExpressionToken::GreaterThanOrEqual),
            ("||", ExpressionToken::Or),
            (">", ExpressionToken::GreaterThan),
            ("<", ExpressionToken::LessThan),
            ("!", ExpressionToken::Not),
        ] {
            let mut cursor = ExpressionCursor::new(input);
            assert_eq!(
                OperatorToken::recognize(&mut cursor).unwrap(),
                Some(expected)
            );
        }
    }

    #[test]
    fn reports_incomplete_required_operators() {
        let mut cursor = ExpressionCursor::new("=");

        assert_eq!(
            OperatorToken::recognize(&mut cursor),
            Err(LexerError::UnexpectedChar('=', 0))
        );
    }

    #[test]
    fn recognizes_no_token_for_an_empty_cursor() {
        let mut cursor = ExpressionCursor::new("");

        assert_eq!(OperatorToken::recognize(&mut cursor).unwrap(), None);
    }
}
