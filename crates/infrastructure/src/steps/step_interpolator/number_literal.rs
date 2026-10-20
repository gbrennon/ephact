use super::{ExpressionCursor, ExpressionToken};

pub struct NumberLiteral;

impl NumberLiteral {
    pub fn starts(cursor: &ExpressionCursor<'_>) -> bool {
        cursor.current().is_some_and(|ch| ch.is_ascii_digit())
            || (cursor.current() == Some('-')
                && cursor.peek_next().is_some_and(|ch| ch.is_ascii_digit()))
    }

    pub fn recognize(cursor: ExpressionCursor<'_>) -> (ExpressionToken, ExpressionCursor<'_>) {
        let (sign, cursor) = match cursor.current() {
            Some('-') => ("-", cursor.advance()),
            _ => ("", cursor),
        };
        let (digits, cursor) = cursor.take_while(|ch| ch.is_ascii_digit());
        let literal = format!("{sign}{digits}");
        if cursor.current() == Some('.') && cursor.peek_next().is_some_and(|ch| ch.is_ascii_digit())
        {
            let cursor = cursor.advance();
            let (fraction, cursor) = cursor.take_while(|ch| ch.is_ascii_digit());
            let literal = format!("{literal}.{fraction}");
            (
                ExpressionToken::Float(
                    literal
                        .parse()
                        .expect("lex_number produced an unparseable float"),
                ),
                cursor,
            )
        } else {
            (
                ExpressionToken::Integer(
                    literal
                        .parse()
                        .expect("lex_number produced an unparseable int"),
                ),
                cursor,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{ExpressionCursor, ExpressionToken},
        NumberLiteral,
    };

    #[test]
    fn recognizes_integers_negative_numbers_and_fractional_numbers() {
        for (source, expected) in [
            ("42", ExpressionToken::Integer(42)),
            ("-7", ExpressionToken::Integer(-7)),
            ("2.5", ExpressionToken::Float(2.5)),
        ] {
            let cursor = ExpressionCursor::new(source);
            assert!(NumberLiteral::starts(&cursor));
            let (token, cursor) = NumberLiteral::recognize(cursor);
            assert_eq!(token, expected);
            assert_eq!(cursor.current(), None);
        }
        assert!(!NumberLiteral::starts(&ExpressionCursor::new("-x")));
    }
}
