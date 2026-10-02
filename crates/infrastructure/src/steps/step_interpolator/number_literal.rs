use super::{ExpressionCursor, ExpressionToken};

pub struct NumberLiteral;

impl NumberLiteral {
    pub fn starts(cursor: &ExpressionCursor<'_>) -> bool {
        cursor.current().is_some_and(|ch| ch.is_ascii_digit())
            || (cursor.current() == Some('-')
                && cursor.peek_next().is_some_and(|ch| ch.is_ascii_digit()))
    }

    pub fn recognize(cursor: &mut ExpressionCursor<'_>) -> ExpressionToken {
        let mut literal = String::new();
        if cursor.current() == Some('-') {
            literal.push('-');
            cursor.advance();
        }
        Self::append_digits(cursor, &mut literal);
        let is_float = Self::append_fraction(cursor, &mut literal);
        if is_float {
            ExpressionToken::Float(
                literal
                    .parse()
                    .expect("lex_number produced an unparseable float"),
            )
        } else {
            ExpressionToken::Integer(
                literal
                    .parse()
                    .expect("lex_number produced an unparseable int"),
            )
        }
    }

    fn append_digits(cursor: &mut ExpressionCursor<'_>, literal: &mut String) {
        while cursor.current().is_some_and(|ch| ch.is_ascii_digit()) {
            literal.push(cursor.current().unwrap());
            cursor.advance();
        }
    }

    fn append_fraction(cursor: &mut ExpressionCursor<'_>, literal: &mut String) -> bool {
        if cursor.current() == Some('.') && cursor.peek_next().is_some_and(|ch| ch.is_ascii_digit())
        {
            literal.push('.');
            cursor.advance();
            Self::append_digits(cursor, literal);
            true
        } else {
            false
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
            let mut cursor = ExpressionCursor::new(source);
            assert!(NumberLiteral::starts(&cursor));
            assert_eq!(NumberLiteral::recognize(&mut cursor), expected);
            assert_eq!(cursor.current(), None);
        }
        assert!(!NumberLiteral::starts(&ExpressionCursor::new("-x")));
    }
}
