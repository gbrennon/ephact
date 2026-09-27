use super::ExpressionToken;
use crate::{errors::LexerError, value_objects::expression_cursor::ExpressionCursor};

pub struct StringLiteral;

impl StringLiteral {
    pub fn recognize(cursor: &mut ExpressionCursor<'_>) -> Result<ExpressionToken, LexerError> {
        let start = cursor.position();
        cursor.advance();
        let mut value = String::new();
        while let Some(ch) = cursor.current() {
            cursor.advance();
            if ch != '\'' {
                value.push(ch);
                continue;
            }
            if cursor.current() == Some('\'') {
                cursor.advance();
                value.push('\'');
                continue;
            }
            return Ok(ExpressionToken::String(value));
        }
        Err(LexerError::UnterminatedString(start))
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
