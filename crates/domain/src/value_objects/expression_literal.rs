use std::fmt;

/// Expression literal values used by the workflow expression language.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionLiteral {
    Boolean(bool),
    Null,
    Integer(i64),
    Float(f64),
    String(String),
}

impl fmt::Display for ExpressionLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionLiteral::Boolean(b) => write!(f, "{}", b),
            ExpressionLiteral::Null => write!(f, "null"),
            ExpressionLiteral::Integer(n) => write!(f, "{}", n),
            ExpressionLiteral::Float(n) => write!(f, "{}", n),
            ExpressionLiteral::String(s) => write!(f, "'{}'", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_bool_true() {
        assert_eq!(ExpressionLiteral::Boolean(true).to_string(), "true");
    }

    #[test]
    fn display_bool_false() {
        assert_eq!(ExpressionLiteral::Boolean(false).to_string(), "false");
    }

    #[test]
    fn display_null() {
        assert_eq!(ExpressionLiteral::Null.to_string(), "null");
    }

    #[test]
    fn display_int() {
        assert_eq!(ExpressionLiteral::Integer(42).to_string(), "42");
    }

    #[test]
    fn display_float() {
        assert_eq!(ExpressionLiteral::Float(2.71).to_string(), "2.71");
    }

    #[test]
    fn display_string() {
        assert_eq!(
            ExpressionLiteral::String("hello".into()).to_string(),
            "'hello'"
        );
    }
}
