/// ExpressionToken types produced by the expression lexer.
///
/// Represents all terminal symbols in the workflow `${{ }}` expression
/// language: literals, identifiers, operators, punctuation, and end-of-file.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionToken {
    /// An identifier: `[a-zA-Z_][a-zA-Z0-9_-]*` (excluding keywords).
    Identifier(String),
    /// A single-quoted string literal with `''` escape for literal `'`.
    String(String),
    /// A signed 64-bit integer literal.
    Integer(i64),
    /// A 64-bit floating-point literal (must contain a decimal point).
    Float(f64),
    /// A boolean literal: `true` or `false`.
    Boolean(bool),
    /// The `null` literal.
    Null,
    /// `.` - property access operator.
    Dot,
    /// `[` - index/open bracket.
    LeftBracket,
    /// `]` - close bracket.
    RightBracket,
    /// `(` - open parenthesis.
    LeftParenthesis,
    /// `)` - close parenthesis.
    RightParenthesis,
    /// `!` - logical NOT.
    Not,
    /// `==` - equality comparison.
    Equal,
    /// `!=` - inequality comparison.
    NotEqual,
    /// `<` - less-than comparison.
    LessThan,
    /// `<=` - less-than-or-equal comparison.
    LessThanOrEqual,
    /// `>` - greater-than comparison.
    GreaterThan,
    /// `>=` - greater-than-or-equal comparison.
    GreaterThanOrEqual,
    /// `&&` - logical AND.
    And,
    /// `||` - logical OR.
    Or,
    /// `*` - array dereference / wildcard.
    Star,
    /// `,` - argument separator.
    Comma,
    /// End of input stream.
    EndOfInput,
}
