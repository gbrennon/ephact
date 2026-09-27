use super::ExpressionToken;
use crate::{errors::LexerError, value_objects::expression_cursor::ExpressionCursor};

pub struct StringLiteral;

impl StringLiteral {
    pub fn recognize(cursor: &mut ExpressionCursor<'_>) -> Result<ExpressionToken, LexerError> {
        let start = cursor.position();
        cursor.advance();
        let mut value = String::new();
        loop {
            match cursor.current() {
                None => return Err(LexerError::UnterminatedString(start)),
                Some('\'') => {
                    cursor.advance();
                    if cursor.current() == Some('\'') {
                        cursor.advance();
                        value.push('\'');
                    } else {
                        return Ok(ExpressionToken::String(value));
                    }
                }
                Some(ch) => {
                    cursor.advance();
                    value.push(ch);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringLiteral;
    use crate::{
        errors::LexerError,
        value_objects::{ExpressionToken, expression_cursor::ExpressionCursor},
    };

    #[test]
    fn recognizes_quoted_and_escaped_strings_and_reports_unterminated() {
        for (source, value) in [("'hello'", "hello"), ("'it''s'", "it's"), ("''", "")] {
            let mut cursor = ExpressionCursor::new(source);
            assert_eq!(
                StringLiteral::recognize(&mut cursor).unwrap(),
                ExpressionToken::String(value.into())
            );
            assert_eq!(cursor.current(), None);
        }
        let mut cursor = ExpressionCursor::new("'bad");
        assert_eq!(
            StringLiteral::recognize(&mut cursor),
            Err(LexerError::UnterminatedString(0))
        );
    }
}
