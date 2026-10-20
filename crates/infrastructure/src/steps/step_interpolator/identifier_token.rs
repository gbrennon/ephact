use super::{ExpressionCursor, ExpressionToken};

pub struct IdentifierToken;

impl IdentifierToken {
    pub fn starts(ch: char) -> bool {
        ch == '_' || ch.is_ascii_alphabetic()
    }

    pub fn recognize(cursor: ExpressionCursor<'_>) -> (ExpressionToken, ExpressionCursor<'_>) {
        let (name, cursor) = cursor.take_while(Self::continues);
        let token = match name {
            "true" => ExpressionToken::Boolean(true),
            "false" => ExpressionToken::Boolean(false),
            "null" => ExpressionToken::Null,
            _ => ExpressionToken::Identifier(name.into()),
        };
        (token, cursor)
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
            let cursor = ExpressionCursor::new(source);
            let (token, cursor) = IdentifierToken::recognize(cursor);
            assert_eq!(token, expected);
            assert_eq!(cursor.current(), None);
        }
    }
}
