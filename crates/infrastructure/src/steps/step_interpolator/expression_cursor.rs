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
        self.remaining.chars().nth(1)
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn remaining(&self) -> &'a str {
        self.remaining
    }

    pub fn advance_bytes(self, bytes: usize) -> Self {
        let bytes = bytes.min(self.remaining.len());
        Self {
            remaining: &self.remaining[bytes..],
            position: self.position + bytes,
        }
    }

    pub fn take_while<F>(self, predicate: F) -> (&'a str, Self)
    where
        F: Fn(char) -> bool,
    {
        let bytes = self
            .remaining
            .char_indices()
            .find_map(|(index, ch)| (!predicate(ch)).then_some(index))
            .unwrap_or(self.remaining.len());
        let consumed = &self.remaining[..bytes];
        (
            consumed,
            Self {
                remaining: &self.remaining[bytes..],
                position: self.position + bytes,
            },
        )
    }

    pub fn advance(self) -> Self {
        let Some(ch) = self.current() else {
            return self;
        };
        self.advance_bytes(ch.len_utf8())
    }
}

#[cfg(test)]
mod tests {
    use super::ExpressionCursor;

    #[test]
    fn advances_position_by_utf8_byte_length() {
        let cursor = ExpressionCursor::new("éx");
        assert_eq!(cursor.current(), Some('é'));
        let cursor = cursor.advance();
        assert_eq!(cursor.position(), 2);
        assert_eq!(cursor.current(), Some('x'));
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn advancing_an_exhausted_cursor_is_a_no_op() {
        let cursor = ExpressionCursor::new("").advance();

        assert_eq!(cursor.position(), 0);
        assert_eq!(cursor.current(), None);
    }
}
