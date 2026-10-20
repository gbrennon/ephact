use std::collections::BTreeMap;

use super::ContextValue;
use crate::errors::JsonTextError;

/// Parses one JSON text document into a [`ContextValue`], rejecting malformed
/// input and trailing non-whitespace text with [`JsonTextError`].
#[derive(Clone, Copy)]
pub struct JsonTextReader<'a> {
    text: &'a str,
    position: usize,
}

enum TextLiteralState<'a> {
    Reading(JsonTextReader<'a>),
    Character(JsonTextReader<'a>, char),
    Finished(JsonTextReader<'a>),
    Failed(String),
}

impl<'a> JsonTextReader<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, position: 0 }
    }

    pub fn read_document(self) -> Result<ContextValue, JsonTextError> {
        let (reader, value) = self.read_value()?;
        let reader = reader.skip_whitespace();
        match reader.peek() {
            None => Ok(value),
            Some(character) => Err(JsonTextError::new(format!(
                "unexpected trailing character '{character}'"
            ))),
        }
    }

    fn read_value(self) -> Result<(Self, ContextValue), JsonTextError> {
        let reader = self.skip_whitespace();
        let character = reader
            .peek()
            .ok_or_else(|| JsonTextError::new("unexpected end of JSON text"))?;
        reader.read_value_starting_with(character)
    }

    fn read_value_starting_with(
        self,
        character: char,
    ) -> Result<(Self, ContextValue), JsonTextError> {
        if character == '{' {
            self.read_mapping()
        } else if character == '[' {
            self.read_list()
        } else if character == '"' {
            self.read_text_literal()
                .map(|(reader, value)| (reader, ContextValue::Text(value)))
        } else if matches!(character, 't' | 'f' | 'n') {
            self.read_keyword()
        } else {
            self.read_number()
        }
    }

    fn read_mapping(self) -> Result<(Self, ContextValue), JsonTextError> {
        self.expect('{')?.read_mapping_entries(BTreeMap::new())
    }

    fn read_mapping_entries(
        self,
        entries: BTreeMap<String, ContextValue>,
    ) -> Result<(Self, ContextValue), JsonTextError> {
        let reader = self.skip_whitespace();
        let (reader, closed) = reader.consume('}');
        if closed {
            return Ok((reader, ContextValue::Mapping(entries)));
        }
        let (reader, (name, value)) = reader.read_mapping_entry()?;
        let entries = entries
            .into_iter()
            .chain(std::iter::once((name, value)))
            .collect::<BTreeMap<_, _>>();
        let reader = reader.skip_whitespace();
        let (reader, comma) = reader.consume(',');
        match comma {
            true => reader.read_mapping_entries(entries),
            false => reader
                .expect('}')
                .map(|reader| (reader, ContextValue::Mapping(entries))),
        }
    }

    fn read_mapping_entry(self) -> Result<(Self, (String, ContextValue)), JsonTextError> {
        let reader = self.skip_whitespace();
        let (reader, name) = reader.read_text_literal()?;
        let reader = reader.skip_whitespace().expect(':')?;
        let (reader, value) = reader.read_value()?;
        Ok((reader, (name, value)))
    }

    fn read_list(self) -> Result<(Self, ContextValue), JsonTextError> {
        self.expect('[')?.read_list_items(Vec::new())
    }

    fn read_list_items(
        self,
        items: Vec<ContextValue>,
    ) -> Result<(Self, ContextValue), JsonTextError> {
        let reader = self.skip_whitespace();
        let (reader, closed) = reader.consume(']');
        if closed {
            return Ok((reader, ContextValue::List(items)));
        }
        let (reader, item) = reader.read_value()?;
        let items = items
            .into_iter()
            .chain(std::iter::once(item))
            .collect::<Vec<_>>();
        let reader = reader.skip_whitespace();
        let (reader, comma) = reader.consume(',');
        match comma {
            true => reader.read_list_items(items),
            false => reader
                .expect(']')
                .map(|reader| (reader, ContextValue::List(items))),
        }
    }

    fn read_keyword(self) -> Result<(Self, ContextValue), JsonTextError> {
        let keyword = ["true", "false", "null"]
            .into_iter()
            .find(|keyword| self.text[self.position..].starts_with(keyword));
        match keyword {
            Some(keyword) => {
                let (reader, _) = self.consume_keyword(keyword);
                let value = match keyword {
                    "true" => ContextValue::Boolean(true),
                    "false" => ContextValue::Boolean(false),
                    "null" => ContextValue::Null,
                    _ => unreachable!("keyword list is exhaustive"),
                };
                Ok((reader, value))
            }
            None => Err(JsonTextError::new("invalid JSON literal")),
        }
    }

    fn read_number(self) -> Result<(Self, ContextValue), JsonTextError> {
        let start = self.position;
        let end = self.text[start..]
            .find(|character: char| !ContextValue::is_number_character(character))
            .map_or(self.text.len(), |offset| start + offset);
        let literal = &self.text[start..end];
        match literal.is_empty() {
            true => Err(JsonTextError::new("unexpected character in JSON text")),
            false => ContextValue::parse_number_literal(literal).map(|value| {
                (
                    Self {
                        text: self.text,
                        position: end,
                    },
                    value,
                )
            }),
        }
    }

    fn read_text_literal(self) -> Result<(Self, String), JsonTextError> {
        let reader = self.expect('"')?;
        let terminal = std::iter::successors(
            Some(TextLiteralState::Reading(reader)),
            Self::next_text_literal_state,
        )
        .last()
        .ok_or_else(|| JsonTextError::new("unterminated JSON string"))?;
        match terminal {
            TextLiteralState::Finished(final_reader) => {
                let value: String = std::iter::successors(
                    Some(TextLiteralState::Reading(reader)),
                    Self::next_text_literal_state,
                )
                .filter_map(|state| match state {
                    TextLiteralState::Character(_, character) => Some(character),
                    TextLiteralState::Reading(_)
                    | TextLiteralState::Finished(_)
                    | TextLiteralState::Failed(_) => None,
                })
                .collect();
                Ok((final_reader, value))
            }
            TextLiteralState::Failed(message) => Err(JsonTextError::new(message)),
            TextLiteralState::Reading(_) | TextLiteralState::Character(_, _) => {
                Err(JsonTextError::new("unterminated JSON string"))
            }
        }
    }

    fn next_text_literal_state(state: &TextLiteralState<'a>) -> Option<TextLiteralState<'a>> {
        match state {
            TextLiteralState::Reading(reader) => {
                let (reader, character) = (*reader).next_character();
                match character {
                    Some('"') => Some(TextLiteralState::Finished(reader)),
                    Some('\\') => match reader.read_escape() {
                        Ok((reader, character)) => {
                            Some(TextLiteralState::Character(reader, character))
                        }
                        Err(error) => Some(TextLiteralState::Failed(error.to_string())),
                    },
                    Some(character) => Some(TextLiteralState::Character(reader, character)),
                    None => Some(TextLiteralState::Failed(
                        "unterminated JSON string".to_owned(),
                    )),
                }
            }
            TextLiteralState::Character(reader, _) => Some(TextLiteralState::Reading(*reader)),
            TextLiteralState::Finished(_) | TextLiteralState::Failed(_) => None,
        }
    }

    fn read_escape(self) -> Result<(Self, char), JsonTextError> {
        let (reader, marker) = self.next_character();
        let marker = marker.ok_or_else(|| JsonTextError::new("unterminated escape sequence"))?;
        match marker {
            '"' => Ok((reader, '"')),
            '\\' => Ok((reader, '\\')),
            '/' => Ok((reader, '/')),
            'b' => Ok((reader, '\u{08}')),
            'f' => Ok((reader, '\u{0c}')),
            'n' => Ok((reader, '\n')),
            'r' => Ok((reader, '\r')),
            't' => Ok((reader, '\t')),
            'u' => reader.read_unicode_escape(),
            other => Err(JsonTextError::new(format!(
                "invalid escape sequence '\\{other}'"
            ))),
        }
    }

    fn read_unicode_escape(self) -> Result<(Self, char), JsonTextError> {
        let (reader, code) = self.read_hexadecimal_quad()?;
        match code {
            0xD800..=0xDBFF => reader.read_surrogate_pair(code),
            _ => char::from_u32(code)
                .map(|character| (reader, character))
                .ok_or_else(|| JsonTextError::new("invalid unicode escape sequence")),
        }
    }

    fn read_surrogate_pair(self, high: u32) -> Result<(Self, char), JsonTextError> {
        let reader = self.expect('\\')?.expect('u')?;
        let (reader, low) = reader.read_hexadecimal_quad()?;
        let low = match low {
            code @ 0xDC00..=0xDFFF => code,
            _ => return Err(JsonTextError::new("invalid low surrogate escape sequence")),
        };
        let code = 0x1_0000 + ((high - 0xD800) << 10) + (low - 0xDC00);
        char::from_u32(code)
            .map(|character| (reader, character))
            .ok_or_else(|| JsonTextError::new("invalid surrogate pair"))
    }

    fn read_hexadecimal_quad(self) -> Result<(Self, u32), JsonTextError> {
        let (reader, digits) = (0..4).fold((self, String::new()), |(reader, digits), _| {
            let (reader, character) = reader.next_character();
            let digits = match character {
                Some(character) => digits + &character.to_string(),
                None => digits,
            };
            (reader, digits)
        });
        u32::from_str_radix(&digits, 16)
            .map(|code| (reader, code))
            .map_err(|_| JsonTextError::new(format!("invalid unicode escape '{digits}'")))
    }

    fn skip_whitespace(self) -> Self {
        let remainder = &self.text[self.position..];
        let offset = remainder
            .find(|character: char| !character.is_whitespace())
            .unwrap_or(remainder.len());
        Self {
            text: self.text,
            position: self.position + offset,
        }
    }

    fn peek(&self) -> Option<char> {
        self.text[self.position..].chars().next()
    }

    fn next_character(self) -> (Self, Option<char>) {
        match self.peek() {
            Some(character) => (
                Self {
                    text: self.text,
                    position: self.position + character.len_utf8(),
                },
                Some(character),
            ),
            None => (self, None),
        }
    }

    fn consume(self, expected: char) -> (Self, bool) {
        match self.peek() {
            Some(character) if character == expected => (
                Self {
                    text: self.text,
                    position: self.position + expected.len_utf8(),
                },
                true,
            ),
            _ => (self, false),
        }
    }

    fn consume_keyword(self, keyword: &str) -> (Self, bool) {
        match self.text[self.position..].starts_with(keyword) {
            true => (
                Self {
                    text: self.text,
                    position: self.position + keyword.len(),
                },
                true,
            ),
            false => (self, false),
        }
    }

    fn expect(self, expected: char) -> Result<Self, JsonTextError> {
        let (reader, consumed) = self.consume(expected);
        match consumed {
            true => Ok(reader),
            false => Err(JsonTextError::new(format!(
                "expected '{expected}' in JSON text"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(text: &str) -> Result<ContextValue, JsonTextError> {
        JsonTextReader::new(text).read_document()
    }

    #[test]
    fn empty_and_blank_input_is_rejected() {
        assert!(read("").is_err());
        assert!(read("   ").is_err());
    }

    #[test]
    fn non_value_leading_character_is_rejected() {
        assert!(read("@").is_err());
    }

    #[test]
    fn unterminated_string_is_rejected() {
        assert!(read("\"abc").is_err());
    }

    #[test]
    fn dangling_escape_is_rejected() {
        assert!(read("\"\\").is_err());
    }

    #[test]
    fn short_escapes_decode_to_control_characters() {
        let decoded = read(r#""\"\\\/\b\f\n\r\t""#).unwrap();

        assert_eq!(decoded, ContextValue::text("\"\\/\u{08}\u{0c}\n\r\t"));
    }

    #[test]
    fn invalid_escape_marker_is_rejected() {
        assert!(read(r#""\x""#).is_err());
    }

    #[test]
    fn basic_unicode_escape_decodes() {
        let decoded = read(r#""\u0041""#).unwrap();

        assert_eq!(decoded, ContextValue::text("A"));
    }

    #[test]
    fn lone_low_surrogate_is_rejected() {
        assert!(read(r#""\uDC00""#).is_err());
    }

    #[test]
    fn high_surrogate_without_low_surrogate_is_rejected() {
        assert!(read(r#""\uD800\u0041""#).is_err());
    }

    #[test]
    fn non_hexadecimal_unicode_escape_is_rejected() {
        assert!(read(r#""\uZZZZ""#).is_err());
    }

    #[test]
    fn missing_closing_brace_is_rejected() {
        assert!(read(r#"{"a":1"#).is_err());
    }

    #[test]
    fn missing_entry_colon_is_rejected() {
        assert!(read(r#"{"a" 1}"#).is_err());
    }
}
