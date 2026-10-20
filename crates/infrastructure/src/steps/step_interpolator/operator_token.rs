use super::{ExpressionCursor, ExpressionToken};
use crate::application::errors::LexerError;

pub struct OperatorToken;

impl OperatorToken {
    pub fn recognize(
        cursor: ExpressionCursor<'_>,
    ) -> Result<(Option<ExpressionToken>, ExpressionCursor<'_>), LexerError> {
        let start = cursor.position();
        let (token, cursor) = match cursor.current() {
            Some(ch @ ('!' | '=')) => {
                let (token, cursor) = Self::not_or_equal(cursor, ch, start)?;
                (Some(token), cursor)
            }
            Some(ch @ ('<' | '>')) => {
                let (token, cursor) = Self::optional_operator(cursor, ch);
                (Some(token), cursor)
            }
            Some(ch @ ('&' | '|')) => {
                let (token, cursor) = Self::required_operator(cursor, ch, start)?;
                (Some(token), cursor)
            }
            Some(ch) => Self::single(cursor, ch),
            None => (None, cursor),
        };
        Ok((token, cursor))
    }

    fn not_or_equal(
        cursor: ExpressionCursor<'_>,
        operator: char,
        start: usize,
    ) -> Result<(ExpressionToken, ExpressionCursor<'_>), LexerError> {
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

    fn single(
        cursor: ExpressionCursor<'_>,
        ch: char,
    ) -> (Option<ExpressionToken>, ExpressionCursor<'_>) {
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
        match token {
            Some(token) => (Some(token), cursor.advance()),
            None => (None, cursor),
        }
    }

    fn optional_operator(
        cursor: ExpressionCursor<'_>,
        operator: char,
    ) -> (ExpressionToken, ExpressionCursor<'_>) {
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
        cursor: ExpressionCursor<'_>,
        operator: char,
        start: usize,
    ) -> Result<(ExpressionToken, ExpressionCursor<'_>), LexerError> {
        match operator {
            '&' => Self::required_compound(cursor, '&', '&', ExpressionToken::And, start),
            _ => Self::required_compound(cursor, '|', '|', ExpressionToken::Or, start),
        }
    }

    fn compound(
        cursor: ExpressionCursor<'_>,
        second: char,
        paired: ExpressionToken,
        single: ExpressionToken,
    ) -> (ExpressionToken, ExpressionCursor<'_>) {
        let cursor = cursor.advance();
        if cursor.current() == Some(second) {
            (paired, cursor.advance())
        } else {
            (single, cursor)
        }
    }

    fn required_compound(
        cursor: ExpressionCursor<'_>,
        second: char,
        unexpected: char,
        paired: ExpressionToken,
        start: usize,
    ) -> Result<(ExpressionToken, ExpressionCursor<'_>), LexerError> {
        let cursor = cursor.advance();
        if cursor.current() == Some(second) {
            Ok((paired, cursor.advance()))
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
            let (token, cursor) = OperatorToken::recognize(ExpressionCursor::new(input)).unwrap();
            assert_eq!(token, Some(expected));
            assert_eq!(cursor.current(), None);
        }
        assert!(matches!(
            OperatorToken::recognize(ExpressionCursor::new("&")),
            Err(LexerError::UnexpectedChar('&', 0))
        ));
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
            let (token, _) = OperatorToken::recognize(ExpressionCursor::new(input)).unwrap();
            assert_eq!(token, Some(expected));
        }
    }

    #[test]
    fn reports_incomplete_required_operators() {
        assert!(matches!(
            OperatorToken::recognize(ExpressionCursor::new("=")),
            Err(LexerError::UnexpectedChar('=', 0))
        ));
    }

    #[test]
    fn recognizes_no_token_for_an_empty_cursor() {
        let (token, cursor) = OperatorToken::recognize(ExpressionCursor::new("")).unwrap();
        assert_eq!(token, None);
        assert_eq!(cursor.current(), None);
    }
}
