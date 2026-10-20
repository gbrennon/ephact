use super::{ExpressionCursor, ExpressionToken};
use crate::application::errors::LexerError;

pub struct StringLiteral;

impl StringLiteral {
    pub fn recognize(
        cursor: ExpressionCursor<'_>,
    ) -> Result<(ExpressionToken, ExpressionCursor<'_>), LexerError> {
        let start = cursor.position();
        let cursor = cursor.advance();
        let remaining = cursor.remaining();
        let (closing, pending) =
            remaining
                .char_indices()
                .fold((None, None), |(closing, pending), (index, ch)| {
                    if closing.is_some() {
                        (closing, pending)
                    } else if ch == '\'' {
                        match pending {
                            Some(_) => (None, None),
                            None => (None, Some(index)),
                        }
                    } else {
                        match pending {
                            Some(index) => (Some(index), None),
                            None => (None, None),
                        }
                    }
                });
        match closing.or(pending) {
            Some(index) => {
                let value = remaining[..index].replace("''", "'");
                let cursor = cursor.advance_bytes(index + 1);
                Ok((ExpressionToken::String(value), cursor))
            }
            None => Err(LexerError::UnterminatedString(start)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{ExpressionCursor, ExpressionToken},
        StringLiteral,
    };
    use crate::application::errors::LexerError;

    #[test]
    fn recognizes_quoted_and_escaped_strings_and_reports_unterminated() {
        for (source, value) in [("'hello'", "hello"), ("'it''s'", "it's"), ("''", "")] {
            let (token, cursor) = StringLiteral::recognize(ExpressionCursor::new(source)).unwrap();
            assert_eq!(token, ExpressionToken::String(value.into()));
            assert_eq!(cursor.current(), None);
        }
        let result = StringLiteral::recognize(ExpressionCursor::new("'bad"));
        assert!(matches!(result, Err(LexerError::UnterminatedString(0))));
    }
}
