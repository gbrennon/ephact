use std::fmt;

/// Comparison operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComparisonOperator::Equal => write!(f, "=="),
            ComparisonOperator::NotEqual => write!(f, "!="),
            ComparisonOperator::LessThan => write!(f, "<"),
            ComparisonOperator::LessThanOrEqual => write!(f, "<="),
            ComparisonOperator::GreaterThan => write!(f, ">"),
            ComparisonOperator::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_eq() {
        assert_eq!(ComparisonOperator::Equal.to_string(), "==");
    }

    #[test]
    fn test_display_neq() {
        assert_eq!(ComparisonOperator::NotEqual.to_string(), "!=");
    }

    #[test]
    fn test_display_lt() {
        assert_eq!(ComparisonOperator::LessThan.to_string(), "<");
    }

    #[test]
    fn test_display_lte() {
        assert_eq!(ComparisonOperator::LessThanOrEqual.to_string(), "<=");
    }

    #[test]
    fn test_display_gt() {
        assert_eq!(ComparisonOperator::GreaterThan.to_string(), ">");
    }

    #[test]
    fn test_display_gte() {
        assert_eq!(ComparisonOperator::GreaterThanOrEqual.to_string(), ">=");
    }
}
