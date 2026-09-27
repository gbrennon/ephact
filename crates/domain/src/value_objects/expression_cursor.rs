pub struct ExpressionCursor<'a> {
    remaining: &'a str,
    position: usize,
}

impl<'a> ExpressionCursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            remaining: input,
            position: 0,
        }
    }

    pub fn current(&self) -> Option<char> {
        self.remaining.chars().next()
    }

    pub fn peek_next(&self) -> Option<char> {
        let mut chars = self.remaining.chars();
        chars.next();
        chars.next()
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn advance(&mut self) {
        if let Some(ch) = self.current() {
            self.remaining = &self.remaining[ch.len_utf8()..];
            self.position += ch.len_utf8();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExpressionCursor;

    #[test]
    fn advances_position_by_utf8_byte_length() {
        let mut cursor = ExpressionCursor::new("éx");
        assert_eq!(cursor.current(), Some('é'));
        cursor.advance();
        assert_eq!(cursor.position(), 2);
        assert_eq!(cursor.current(), Some('x'));
        assert_eq!(cursor.peek_next(), None);
    }
}
