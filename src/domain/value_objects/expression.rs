/// AST node types for GitHub Actions `${{ }}` expressions.
///
/// Represents the full expression language: literals, context access,
/// property/index dereferencing, comparisons, logical operators,
/// function calls, and the ternary-like `a && b || c` pattern.
use std::fmt;

use crate::domain::value_objects::{ComparisonOperator, LiteralValue, LogicalOperator};

/// A complete expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A literal value: string, number, boolean, or null.
    Literal(LiteralValue),
    /// A top-level context variable: `github`, `env`, `job`, `steps`, etc.
    Variable(String),
    /// Property access: `foo.bar`
    PropertyAccess(Box<Expression>, String),
    /// Index access: `foo[bar]` - string key or numeric index.
    IndexAccess(Box<Expression>, Box<Expression>),
    /// Array dereference: `foo.*` - flattens array of objects.
    ArrayDereference(Box<Expression>),
    /// Logical NOT: `!expr`
    Not(Box<Expression>),
    /// Comparison: `a == b`, `a != b`, `a < b`, etc.
    Comparison(ComparisonOperator, Box<Expression>, Box<Expression>),
    /// Logical AND/OR: `a && b`, `a || b`
    Logical(LogicalOperator, Box<Expression>, Box<Expression>),
    /// Function call: `contains(search, item)`
    FunctionCall(String, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{}", lit),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::PropertyAccess(obj, prop) => write!(f, "{}.{}", obj, prop),
            Expression::IndexAccess(obj, idx) => write!(f, "{}[{}]", obj, idx),
            Expression::ArrayDereference(obj) => write!(f, "{}.*", obj),
            Expression::Not(expr) => write!(f, "!{}", expr),
            Expression::Comparison(op, lhs, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Expression::Logical(op, lhs, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Expression::FunctionCall(name, args) => Self::format_func_call(f, name, args),
        }
    }
}

impl Expression {
    fn format_func_call(
        f: &mut fmt::Formatter<'_>,
        name: &str,
        args: &[Expression],
    ) -> fmt::Result {
        let rendered_args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        write!(f, "{}({})", name, rendered_args.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_literal_bool() {
        assert_eq!(LiteralValue::Boolean(true).to_string(), "true");
        assert_eq!(LiteralValue::Boolean(false).to_string(), "false");
    }

    #[test]
    fn display_literal_null() {
        assert_eq!(LiteralValue::Null.to_string(), "null");
    }

    #[test]
    fn display_literal_int() {
        assert_eq!(LiteralValue::Integer(42).to_string(), "42");
    }

    #[test]
    fn display_literal_string() {
        assert_eq!(LiteralValue::String("hello".into()).to_string(), "'hello'");
    }

    #[test]
    fn display_variable() {
        assert_eq!(Expression::Variable("github".into()).to_string(), "github");
    }

    #[test]
    fn display_property_access() {
        let expr = Expression::PropertyAccess(
            Box::new(Expression::Variable("github".into())),
            "event_name".into(),
        );
        assert_eq!(expr.to_string(), "github.event_name");
    }

    #[test]
    fn display_func_call() {
        let expr = Expression::FunctionCall(
            "contains".into(),
            vec![
                Expression::Literal(LiteralValue::String("hello".into())),
                Expression::Literal(LiteralValue::String("ll".into())),
            ],
        );
        assert_eq!(expr.to_string(), "contains('hello', 'll')");
    }

    #[test]
    fn display_compare() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Variable("github".into())),
            Box::new(Expression::Literal(LiteralValue::String("push".into()))),
        );
        assert_eq!(expr.to_string(), "github == 'push'");
    }

    #[test]
    fn display_logical() {
        let expr = Expression::Logical(
            LogicalOperator::And,
            Box::new(Expression::Literal(LiteralValue::Boolean(true))),
            Box::new(Expression::Literal(LiteralValue::Boolean(false))),
        );
        assert_eq!(expr.to_string(), "true && false");
    }

    #[test]
    fn display_index_access() {
        let expr = Expression::IndexAccess(
            Box::new(Expression::Variable("arr".into())),
            Box::new(Expression::Literal(LiteralValue::Integer(0))),
        );
        assert_eq!(expr.to_string(), "arr[0]");
    }

    #[test]
    fn display_array_deref() {
        let expr = Expression::ArrayDereference(Box::new(Expression::Variable("arr".into())));
        assert_eq!(expr.to_string(), "arr.*");
    }

    #[test]
    fn display_not() {
        let expr = Expression::Not(Box::new(Expression::Variable("flag".into())));
        assert_eq!(expr.to_string(), "!flag");
    }
}
