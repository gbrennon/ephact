use crate::{
    errors::ParseError,
    services::ExpressionLexer,
    value_objects::{
        ComparisonOperator, Expression, ExpressionLiteral, ExpressionToken, LogicalOperator,
    },
};

/// Recursive-descent parser for expression tokens.
pub struct ExpressionParser {
    tokens: Vec<ExpressionToken>,
    pos: usize,
}

impl ExpressionParser {
    pub fn new(tokens: &[ExpressionToken]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            pos: 0,
        }
    }

    /// Parses the full token stream into an [`Expression`] AST.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if the token stream is malformed.
    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_expression()?;
        if self.pos < self.tokens.len() {
            return Err(self.error("unexpected tokens after expression"));
        }
        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_logical()
    }

    fn parse_logical(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_compare()?;
        loop {
            let op = match self.peek() {
                Some(ExpressionToken::And) => LogicalOperator::And,
                Some(ExpressionToken::Or) => LogicalOperator::Or,
                _ => break,
            };
            self.advance();
            let right = self.parse_compare()?;
            left = Expression::Logical(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_compare(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_unary()?;
        let op = match self.peek() {
            Some(ExpressionToken::Equal) => ComparisonOperator::Equal,
            Some(ExpressionToken::NotEqual) => ComparisonOperator::NotEqual,
            Some(ExpressionToken::LessThan) => ComparisonOperator::LessThan,
            Some(ExpressionToken::LessThanOrEqual) => ComparisonOperator::LessThanOrEqual,
            Some(ExpressionToken::GreaterThan) => ComparisonOperator::GreaterThan,
            Some(ExpressionToken::GreaterThanOrEqual) => ComparisonOperator::GreaterThanOrEqual,
            _ => return Ok(left),
        };
        self.advance();
        let right = self.parse_unary()?;
        Ok(Expression::Comparison(op, Box::new(left), Box::new(right)))
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if self.peek() == Some(&ExpressionToken::Not) {
            self.advance();
            let inner = self.parse_unary()?;
            return Ok(Expression::Not(Box::new(inner)));
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            let before = self.pos;
            expr = self.parse_next_postfix(expr)?;
            if self.pos == before {
                return Ok(expr);
            }
        }
    }

    fn parse_next_postfix(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        match self.peek() {
            Some(ExpressionToken::Dot) => self.parse_dot_postfix(expr),
            Some(ExpressionToken::LeftBracket) => self.parse_index_postfix(expr),
            Some(ExpressionToken::LeftParenthesis) => self.parse_call_postfix(expr),
            _ => Ok(expr),
        }
    }

    fn parse_dot_postfix(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        self.advance();
        if self.peek() == Some(&ExpressionToken::Star) {
            self.advance();
            Ok(Expression::ArrayDereference(Box::new(expr)))
        } else {
            let ident = self.expect_ident("property name after '.'")?;
            Ok(Expression::PropertyAccess(Box::new(expr), ident))
        }
    }

    fn parse_index_postfix(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        self.advance();
        let idx = self.parse_expression()?;
        self.expect(ExpressionToken::RightBracket, "expected ']'")?;
        Ok(Expression::IndexAccess(Box::new(expr), Box::new(idx)))
    }

    fn parse_call_postfix(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        self.advance();
        let args = self.parse_args()?;
        self.expect(ExpressionToken::RightParenthesis, "expected ')'")?;
        let name = match &expr {
            Expression::Variable(n) => n.clone(),
            _ => return Err(self.error("function call requires a function name before '('")),
        };
        Ok(Expression::FunctionCall(name, args))
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek().cloned() {
            Some(ExpressionToken::Identifier(name)) => {
                self.advance();
                Ok(Expression::Variable(name))
            }
            Some(ExpressionToken::String(s)) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::String(s)))
            }
            Some(ExpressionToken::Integer(n)) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::Integer(n)))
            }
            Some(ExpressionToken::Float(f)) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::Float(f)))
            }
            Some(ExpressionToken::Boolean(b)) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::Boolean(b)))
            }
            Some(ExpressionToken::Null) => {
                self.advance();
                Ok(Expression::Literal(ExpressionLiteral::Null))
            }
            Some(ExpressionToken::LeftParenthesis) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(ExpressionToken::RightParenthesis, "expected ')'")?;
                Ok(expr)
            }
            Some(other) => Err(self.error(&format!("unexpected token: {other:?}"))),
            None => Err(self.error("unexpected end of expression")),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = Vec::new();
        if self.peek() == Some(&ExpressionToken::RightParenthesis) {
            return Ok(args);
        }
        loop {
            args.push(self.parse_expression()?);
            if self.peek() == Some(&ExpressionToken::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(args)
    }

    fn peek(&self) -> Option<&ExpressionToken> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: ExpressionToken, msg: &str) -> Result<(), ParseError> {
        match self.peek() {
            Some(t) if *t == expected => {
                self.advance();
                Ok(())
            }
            Some(t) => Err(self.error(&format!("{msg}, found {t:?}"))),
            None => Err(self.error(&format!("{msg}, found end of input"))),
        }
    }

    fn expect_ident(&mut self, msg: &str) -> Result<String, ParseError> {
        match self.peek() {
            Some(ExpressionToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            Some(t) => Err(self.error(&format!("{msg}, found {t:?}"))),
            None => Err(self.error(&format!("{msg}, found end of input"))),
        }
    }

    fn error(&self, msg: &str) -> ParseError {
        ParseError::new(msg.to_string(), self.pos)
    }
}

impl ExpressionParser {
    /// Lexes and parses an expression string.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if lexing or parsing fails.
    pub fn parse_text(input: &str) -> Result<Expression, ParseError> {
        let mut lexer = ExpressionLexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(ExpressionToken::EndOfInput) => break,
                Ok(tok) => tokens.push(tok),
                Err(error) => {
                    return Err(ParseError::new(format!("lexer error: {:?}", error), 0));
                }
            }
        }
        let mut parser = Self::new(&tokens);
        parser.parse()
    }
}
#[cfg(test)]
mod tests;
