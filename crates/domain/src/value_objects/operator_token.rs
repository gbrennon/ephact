use super::ExpressionToken;
use crate::{errors::LexerError, value_objects::expression_cursor::ExpressionCursor};

pub struct OperatorToken;

impl OperatorToken {
    pub fn recognize(
        cursor: &mut ExpressionCursor<'_>,
    ) -> Result<Option<ExpressionToken>, LexerError> {
        let start = cursor.position();
        let token = match cursor.current() {
            Some('!' | '=') => Some(Self::not_or_equal(cursor, start)?),
            Some('<' | '>') => Some(Self::optional_operator(cursor)),
            Some('&' | '|') => Some(Self::required_operator(cursor, start)?),
            Some(ch) => Self::single(cursor, ch),
            None => None,
        };
        Ok(token)
    }

    fn not_or_equal(
        cursor: &mut ExpressionCursor<'_>,
        start: usize,
    ) -> Result<ExpressionToken, LexerError> {
        match cursor.current() {
            Some('!') => Ok(Self::compound(
                cursor,
                '=',
                ExpressionToken::NotEqual,
                ExpressionToken::Not,
            )),
            Some('=') => Self::required_compound(cursor, '=', '=', ExpressionToken::Equal, start),
            _ => unreachable!("not_or_equal called for another operator"),
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

    fn optional_operator(cursor: &mut ExpressionCursor<'_>) -> ExpressionToken {
        match cursor.current() {
            Some('<') => Self::compound(
                cursor,
                '=',
                ExpressionToken::LessThanOrEqual,
                ExpressionToken::LessThan,
            ),
            Some('>') => Self::compound(
                cursor,
                '=',
                ExpressionToken::GreaterThanOrEqual,
                ExpressionToken::GreaterThan,
            ),
            _ => unreachable!("optional_operator called for a non-optional operator"),
        }
    }

    fn required_operator(
        cursor: &mut ExpressionCursor<'_>,
        start: usize,
    ) -> Result<ExpressionToken, LexerError> {
        match cursor.current() {
            Some('&') => Self::required_compound(cursor, '&', '&', ExpressionToken::And, start),
            Some('|') => Self::required_compound(cursor, '|', '|', ExpressionToken::Or, start),
            _ => unreachable!("required_operator called for a non-required operator"),
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
    use super::OperatorToken;
    use crate::{
        errors::LexerError,
        value_objects::{ExpressionToken, expression_cursor::ExpressionCursor},
    };

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
}
