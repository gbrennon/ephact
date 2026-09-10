use std::collections::BTreeMap;

use super::ContextValue;
use crate::domain::errors::JsonTextError;

/// Recursive-descent reader turning JSON text into a [`ContextValue`].
pub(crate) struct JsonTextReader<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> JsonTextReader<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        Self { text, position: 0 }
    }

    pub(crate) fn read_document(mut self) -> Result<ContextValue, JsonTextError> {
        let value = self.read_value()?;
        self.skip_whitespace();
        match self.peek() {
            None => Ok(value),
            Some(character) => Err(JsonTextError::new(format!(
                "unexpected trailing character '{character}'"
            ))),
        }
    }

    fn read_value(&mut self) -> Result<ContextValue, JsonTextError> {
        self.skip_whitespace();
        let character = self
            .peek()
            .ok_or_else(|| JsonTextError::new("unexpected end of JSON text"))?;
        match character {
            '{' => self.read_mapping(),
            '[' => self.read_list(),
            '"' => self.read_text_literal().map(ContextValue::Text),
            't' | 'f' | 'n' => self.read_keyword(),
            _ => self.read_number(),
        }
    }

    fn read_mapping(&mut self) -> Result<ContextValue, JsonTextError> {
        self.expect('{')?;
        let mut entries = BTreeMap::new();
        self.skip_whitespace();
        if self.consume('}') {
            return Ok(ContextValue::Mapping(entries));
        }
        loop {
            let (name, value) = self.read_mapping_entry()?;
            entries.insert(name, value);
            self.skip_whitespace();
            if !self.consume(',') {
                break;
            }
        }
        self.expect('}')?;
        Ok(ContextValue::Mapping(entries))
    }

    fn read_mapping_entry(&mut self) -> Result<(String, ContextValue), JsonTextError> {
        self.skip_whitespace();
        let name = self.read_text_literal()?;
        self.skip_whitespace();
        self.expect(':')?;
        let value = self.read_value()?;
        Ok((name, value))
    }

    fn read_list(&mut self) -> Result<ContextValue, JsonTextError> {
        self.expect('[')?;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.consume(']') {
            return Ok(ContextValue::List(items));
        }
        loop {
            items.push(self.read_value()?);
            self.skip_whitespace();
            if !self.consume(',') {
                break;
            }
        }
        self.expect(']')?;
        Ok(ContextValue::List(items))
    }

    fn read_keyword(&mut self) -> Result<ContextValue, JsonTextError> {
        let keywords = [
            ("true", ContextValue::Boolean(true)),
            ("false", ContextValue::Boolean(false)),
            ("null", ContextValue::Null),
        ];
        for (keyword, value) in keywords {
            if self.consume_keyword(keyword) {
                return Ok(value);
            }
        }
        Err(JsonTextError::new("invalid JSON literal"))
    }

    fn read_number(&mut self) -> Result<ContextValue, JsonTextError> {
        let start = self.position;
        let end = self.text[start..]
            .find(|character: char| !ContextValue::is_number_character(character))
            .map_or(self.text.len(), |offset| start + offset);
        let literal = &self.text[start..end];
        self.position = end;
        match literal.is_empty() {
            true => Err(JsonTextError::new("unexpected character in JSON text")),
            false => ContextValue::parse_number_literal(literal),
        }
    }

    fn read_text_literal(&mut self) -> Result<String, JsonTextError> {
        self.expect('"')?;
        let mut value = String::new();
        while let Some(character) = self.next_character() {
            match character {
                '"' => return Ok(value),
                '\\' => value.push(self.read_escape()?),
                other => value.push(other),
            }
        }
        Err(JsonTextError::new("unterminated JSON string"))
    }

    fn read_escape(&mut self) -> Result<char, JsonTextError> {
        let marker = self
            .next_character()
            .ok_or_else(|| JsonTextError::new("unterminated escape sequence"))?;
        match marker {
            '"' => Ok('"'),
            '\\' => Ok('\\'),
            '/' => Ok('/'),
            'b' => Ok('\u{08}'),
            'f' => Ok('\u{0c}'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            'u' => self.read_unicode_escape(),
            other => Err(JsonTextError::new(format!(
                "invalid escape sequence '\\{other}'"
            ))),
        }
    }

    fn read_unicode_escape(&mut self) -> Result<char, JsonTextError> {
        let code = self.read_hexadecimal_quad()?;
        match code {
            0xD800..=0xDBFF => self.read_surrogate_pair(code),
            _ => char::from_u32(code)
                .ok_or_else(|| JsonTextError::new("invalid unicode escape sequence")),
        }
    }

    fn read_surrogate_pair(&mut self, high: u32) -> Result<char, JsonTextError> {
        self.expect('\\')?;
        self.expect('u')?;
        let low = match self.read_hexadecimal_quad()? {
            code @ 0xDC00..=0xDFFF => code,
            _ => return Err(JsonTextError::new("invalid low surrogate escape sequence")),
        };
        let code = 0x1_0000 + ((high - 0xD800) << 10) + (low - 0xDC00);
        char::from_u32(code).ok_or_else(|| JsonTextError::new("invalid surrogate pair"))
    }

    fn read_hexadecimal_quad(&mut self) -> Result<u32, JsonTextError> {
        let digits: String = (0..4).filter_map(|_| self.next_character()).collect();
        u32::from_str_radix(&digits, 16)
            .map_err(|_| JsonTextError::new(format!("invalid unicode escape '{digits}'")))
    }

    fn skip_whitespace(&mut self) {
        let remainder = &self.text[self.position..];
        let offset = remainder
            .find(|character: char| !character.is_whitespace())
            .unwrap_or(remainder.len());
        self.position += offset;
    }

    fn peek(&self) -> Option<char> {
        self.text[self.position..].chars().next()
    }

    fn next_character(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.position += character.len_utf8();
        Some(character)
    }

    fn consume(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(character) if character == expected => {
                self.position += expected.len_utf8();
                true
            }
            _ => false,
        }
    }

    fn consume_keyword(&mut self, keyword: &str) -> bool {
        match self.text[self.position..].starts_with(keyword) {
            true => {
                self.position += keyword.len();
                true
            }
            false => false,
        }
    }

    fn expect(&mut self, expected: char) -> Result<(), JsonTextError> {
        match self.consume(expected) {
            true => Ok(()),
            false => Err(JsonTextError::new(format!(
                "expected '{expected}' in JSON text"
            ))),
        }
    }
}
