use super::{ExpressionCursor, ExpressionToken};

pub struct IdentifierToken;

impl IdentifierToken {
    pub fn starts(ch: char) -> bool {
        ch == '_' || ch.is_ascii_alphabetic()
    }

    pub fn recognize(cursor: &mut ExpressionCursor<'_>) -> ExpressionToken {
        let mut name = String::new();
        while cursor.current().is_some_and(Self::continues) {
            name.push(cursor.current().unwrap());
            cursor.advance();
        }
        match name.as_str() {
            "true" => ExpressionToken::Boolean(true),
            "false" => ExpressionToken::Boolean(false),
            "null" => ExpressionToken::Null,
            _ => ExpressionToken::Identifier(name),
        }
    }

    fn continues(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{ExpressionCursor, ExpressionToken},
        IdentifierToken,
    };

    #[test]
    fn recognizes_names_and_keywords() {
        for (source, expected) in [
            ("a_b-2", ExpressionToken::Identifier("a_b-2".into())),
            ("true", ExpressionToken::Boolean(true)),
            ("false", ExpressionToken::Boolean(false)),
            ("null", ExpressionToken::Null),
            ("_x", ExpressionToken::Identifier("_x".into())),
        ] {
            assert!(IdentifierToken::starts(source.chars().next().unwrap()));
            let mut cursor = ExpressionCursor::new(source);
            assert_eq!(IdentifierToken::recognize(&mut cursor), expected);
            assert_eq!(cursor.current(), None);
        }
    }
}
