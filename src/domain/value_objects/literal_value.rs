use std::fmt;

/// LiteralValue value types in workflow expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Boolean(bool),
    Null,
    Integer(i64),
    Float(f64),
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
            LiteralValue::Integer(n) => write!(f, "{}", n),
            LiteralValue::Float(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "'{}'", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_bool_true() {
        assert_eq!(LiteralValue::Boolean(true).to_string(), "true");
    }

    #[test]
    fn display_bool_false() {
        assert_eq!(LiteralValue::Boolean(false).to_string(), "false");
    }

    #[test]
    fn display_null() {
        assert_eq!(LiteralValue::Null.to_string(), "null");
    }

    #[test]
    fn display_int() {
        assert_eq!(LiteralValue::Integer(42).to_string(), "42");
    }

    #[test]
    fn display_float() {
        assert_eq!(LiteralValue::Float(2.71).to_string(), "2.71");
    }

    #[test]
    fn display_string() {
        assert_eq!(LiteralValue::String("hello".into()).to_string(), "'hello'");
    }
}
