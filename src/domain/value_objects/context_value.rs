use crate::domain::value_objects::json_text_reader::JsonTextReader;
use std::collections::BTreeMap;

use crate::domain::errors::JsonTextError;

/// Value produced and consumed by expression evaluation.
///
/// Mirrors the JSON data model that the workflow expression context exposes
/// to `${{ }}` expressions, without depending on a serialization library. Mappings keep
/// their keys sorted so JSON rendering is deterministic.
///
/// # Example
///
/// ```rust
/// use ephact::domain::value_objects::ContextValue;
///
/// let value = ContextValue::from_json_text(r#"{"b":2,"a":"x"}"#).unwrap();
/// assert_eq!(value.to_json_text(), r#"{"a":"x","b":2}"#);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ContextValue {
    /// Absence of a value.
    Null,
    /// Boolean value.
    Boolean(bool),
    /// Integral number.
    Integer(i64),
    /// Fractional number.
    Decimal(f64),
    /// Textual value.
    Text(String),
    /// Ordered sequence of values.
    List(Vec<ContextValue>),
    /// Key-sorted association of names to values.
    Mapping(BTreeMap<String, ContextValue>),
}

impl ContextValue {
    /// Creates a textual value.
    #[must_use]
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    /// Creates a mapping without entries.
    #[must_use]
    pub fn empty_mapping() -> Self {
        Self::Mapping(BTreeMap::new())
    }

    /// Creates a mapping from name and value pairs.
    #[must_use]
    pub fn mapping(entries: impl IntoIterator<Item = (String, ContextValue)>) -> Self {
        Self::Mapping(entries.into_iter().collect())
    }

    /// Creates a list from the given items.
    #[must_use]
    pub fn list(items: impl IntoIterator<Item = ContextValue>) -> Self {
        Self::List(items.into_iter().collect())
    }

    /// Returns the value stored under `name`, or `None` when this value is not
    /// a mapping or has no such entry.
    #[must_use]
    pub fn property(&self, name: &str) -> Option<&ContextValue> {
        match self {
            Self::Mapping(entries) => entries.get(name),
            _ => None,
        }
    }

    /// Returns the item at `index`, or `None` when this value is not a list or
    /// the index is out of bounds.
    #[must_use]
    pub fn element(&self, index: usize) -> Option<&ContextValue> {
        match self {
            Self::List(items) => items.get(index),
            _ => None,
        }
    }

    /// Returns the text of a [`ContextValue::Text`], `None` otherwise.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the items of a [`ContextValue::List`], `None` otherwise.
    #[must_use]
    pub fn as_list(&self) -> Option<&[ContextValue]> {
        match self {
            Self::List(items) => Some(items),
            _ => None,
        }
    }

    /// Returns the numeric value of an integer or decimal, `None` otherwise.
    #[must_use]
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Integer(value) => Some(*value as f64),
            Self::Decimal(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns `true` when this value is a mapping.
    #[must_use]
    pub fn is_mapping(&self) -> bool {
        matches!(self, Self::Mapping(_))
    }

    /// Returns `true` when workflow expression semantics treat this
    /// value as truthy.
    ///
    /// Falsy values are `null`, `false`, zero numbers and the empty text.
    /// Lists and mappings are always truthy.
    #[must_use]
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Boolean(value) => *value,
            Self::Integer(value) => *value != 0,
            Self::Decimal(value) => *value != 0.0,
            Self::Text(value) => !value.is_empty(),
            Self::List(_) | Self::Mapping(_) => true,
        }
    }

    /// Returns the type name used in expression error messages.
    #[must_use]
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Boolean(_) => "bool",
            Self::Integer(_) | Self::Decimal(_) => "number",
            Self::Text(_) => "string",
            Self::List(_) => "array",
            Self::Mapping(_) => "object",
        }
    }

    /// Renders this value the way `format` and `join` render their arguments.
    ///
    /// Scalars render as their literal form and composites as compact JSON.
    #[must_use]
    pub fn to_display_text(&self) -> String {
        match self {
            Self::Null => "null".to_owned(),
            Self::Boolean(value) => value.to_string(),
            Self::Integer(value) => value.to_string(),
            Self::Decimal(value) => Self::decimal_text(*value),
            Self::Text(value) => value.clone(),
            Self::List(_) | Self::Mapping(_) => self.to_json_text(),
        }
    }

    /// Renders this value as compact JSON text with sorted mapping keys.
    #[must_use]
    pub fn to_json_text(&self) -> String {
        let mut text = String::new();
        self.write_json_text(&mut text);
        text
    }

    /// Reads JSON text into a value.
    ///
    /// Numbers without a fraction or exponent become [`ContextValue::Integer`]
    /// and all other numbers become [`ContextValue::Decimal`]. Repeated
    /// mapping keys keep the last occurrence.
    ///
    /// # Errors
    ///
    /// Returns [`JsonTextError`] when the text is not a single well-formed
    /// JSON document.
    pub fn from_json_text(text: &str) -> Result<Self, JsonTextError> {
        JsonTextReader::new(text).read_document()
    }

    fn write_json_text(&self, out: &mut String) {
        match self {
            Self::Null => out.push_str("null"),
            Self::Boolean(value) => out.push_str(Self::bool_text(*value)),
            Self::Integer(value) => out.push_str(&value.to_string()),
            Self::Decimal(value) => out.push_str(&Self::decimal_text(*value)),
            Self::Text(value) => Self::write_json_text_literal(value, out),
            Self::List(items) => Self::write_json_list(items, out),
            Self::Mapping(entries) => Self::write_json_mapping(entries, out),
        }
    }

    fn bool_text(value: bool) -> &'static str {
        match value {
            true => "true",
            false => "false",
        }
    }

    fn decimal_text(value: f64) -> String {
        match value.is_finite() && value.fract() == 0.0 {
            true => format!("{value:.1}"),
            false => format!("{value}"),
        }
    }

    fn write_json_list(items: &[ContextValue], out: &mut String) {
        out.push('[');
        for (index, item) in items.iter().enumerate() {
            Self::push_item_separator(index, out);
            item.write_json_text(out);
        }
        out.push(']');
    }

    fn write_json_mapping(entries: &BTreeMap<String, ContextValue>, out: &mut String) {
        out.push('{');
        for (index, (name, value)) in entries.iter().enumerate() {
            Self::push_item_separator(index, out);
            Self::write_json_text_literal(name, out);
            out.push(':');
            value.write_json_text(out);
        }
        out.push('}');
    }

    fn push_item_separator(index: usize, out: &mut String) {
        match index {
            0 => (),
            _ => out.push(','),
        }
    }

    fn write_json_text_literal(value: &str, out: &mut String) {
        out.push('"');
        for character in value.chars() {
            Self::push_escaped_character(character, out);
        }
        out.push('"');
    }

    fn push_escaped_character(character: char, out: &mut String) {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            control if control < ' ' => out.push_str(&format!("\\u{:04x}", control as u32)),
            other => out.push(other),
        }
    }
    const NUMBER_CHARACTERS: [char; 5] = ['-', '+', '.', 'e', 'E'];
    pub(crate) fn is_number_character(character: char) -> bool {
        character.is_ascii_digit() || Self::NUMBER_CHARACTERS.contains(&character)
    }

    pub(crate) fn parse_number_literal(literal: &str) -> Result<ContextValue, JsonTextError> {
        match literal.contains(['.', 'e', 'E']) {
            true => literal
                .parse::<f64>()
                .map(ContextValue::Decimal)
                .map_err(|_| Self::invalid_number_error(literal)),
            false => literal
                .parse::<i64>()
                .map(ContextValue::Integer)
                .map_err(|_| Self::invalid_number_error(literal)),
        }
    }

    fn invalid_number_error(literal: &str) -> JsonTextError {
        JsonTextError::new(format!("invalid number '{literal}'"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_keys_are_rendered_in_sorted_order() {
        let value = ContextValue::mapping([
            ("b".to_owned(), ContextValue::Integer(2)),
            ("a".to_owned(), ContextValue::text("x")),
        ]);
        assert_eq!(value.to_json_text(), r#"{"a":"x","b":2}"#);
    }

    #[test]
    fn json_text_round_trips_nested_values() {
        let text = r#"{"a":"x","b":[1,2.5,true,null]}"#;
        let value = ContextValue::from_json_text(text).unwrap();
        assert_eq!(value.to_json_text(), text);
    }

    #[test]
    fn reading_json_text_sorts_mapping_keys() {
        let value = ContextValue::from_json_text(r#"{"b":1,"a":2}"#).unwrap();
        assert_eq!(value.to_json_text(), r#"{"a":2,"b":1}"#);
    }

    #[test]
    fn whole_decimals_keep_a_fractional_part() {
        assert_eq!(ContextValue::Decimal(1.0).to_json_text(), "1.0");
        assert_eq!(ContextValue::Decimal(2.5).to_json_text(), "2.5");
        assert_eq!(ContextValue::Integer(3).to_json_text(), "3");
    }

    #[test]
    fn text_escapes_match_json_encoding() {
        let value = ContextValue::text("q\"b\\s\nt\tu\u{1}");
        assert_eq!(value.to_json_text(), r#""q\"b\\s\nt\tu\u0001""#);
    }

    #[test]
    fn escaped_text_is_decoded() {
        let value = ContextValue::from_json_text(r#""a\"b\\c\nd\u0041\ud83d\ude00""#).unwrap();
        assert_eq!(value.as_text(), Some("a\"b\\c\nd\u{41}\u{1f600}"));
    }

    #[test]
    fn repeated_mapping_keys_keep_the_last_value() {
        let value = ContextValue::from_json_text(r#"{"a":1,"a":2}"#).unwrap();
        assert_eq!(value.property("a"), Some(&ContextValue::Integer(2)));
    }

    #[test]
    fn trailing_input_is_rejected() {
        assert!(ContextValue::from_json_text("{} extra").is_err());
    }

    #[test]
    fn unterminated_text_is_rejected() {
        assert!(ContextValue::from_json_text("\"abc").is_err());
    }

    #[test]
    fn invalid_literal_is_rejected() {
        assert!(ContextValue::from_json_text("tru").is_err());
    }

    #[test]
    fn empty_composites_round_trip() {
        assert_eq!(
            ContextValue::from_json_text("[]").unwrap(),
            ContextValue::List(Vec::new())
        );
        assert_eq!(
            ContextValue::from_json_text("{ }").unwrap(),
            ContextValue::empty_mapping()
        );
    }

    #[test]
    fn whitespace_between_tokens_is_ignored() {
        let value = ContextValue::from_json_text(" { \"a\" : [ 1 , 2 ] } ").unwrap();
        assert_eq!(value.to_json_text(), r#"{"a":[1,2]}"#);
    }

    #[test]
    fn truthiness_follows_expression_semantics() {
        assert!(!ContextValue::Null.is_truthy());
        assert!(!ContextValue::Boolean(false).is_truthy());
        assert!(!ContextValue::Integer(0).is_truthy());
        assert!(!ContextValue::Decimal(0.0).is_truthy());
        assert!(!ContextValue::text("").is_truthy());
        assert!(ContextValue::Integer(42).is_truthy());
        assert!(ContextValue::text("hello").is_truthy());
        assert!(ContextValue::List(Vec::new()).is_truthy());
        assert!(ContextValue::empty_mapping().is_truthy());
    }

    #[test]
    fn type_names_describe_each_variant() {
        assert_eq!(ContextValue::Null.type_name(), "null");
        assert_eq!(ContextValue::Boolean(true).type_name(), "bool");
        assert_eq!(ContextValue::Integer(1).type_name(), "number");
        assert_eq!(ContextValue::Decimal(1.5).type_name(), "number");
        assert_eq!(ContextValue::text("a").type_name(), "string");
        assert_eq!(ContextValue::List(Vec::new()).type_name(), "array");
        assert_eq!(ContextValue::empty_mapping().type_name(), "object");
    }

    #[test]
    fn display_text_renders_scalars_and_composites() {
        assert_eq!(ContextValue::Null.to_display_text(), "null");
        assert_eq!(ContextValue::Boolean(true).to_display_text(), "true");
        assert_eq!(ContextValue::Integer(7).to_display_text(), "7");
        assert_eq!(ContextValue::Decimal(2.5).to_display_text(), "2.5");
        assert_eq!(ContextValue::text("hi").to_display_text(), "hi");
        assert_eq!(
            ContextValue::list([ContextValue::Integer(1), ContextValue::Integer(2)])
                .to_display_text(),
            "[1,2]"
        );
    }

    #[test]
    fn accessors_reach_into_composites() {
        let value = ContextValue::mapping([(
            "items".to_owned(),
            ContextValue::list([ContextValue::text("first")]),
        )]);
        assert_eq!(
            value.property("items").and_then(|items| items.element(0)),
            Some(&ContextValue::text("first"))
        );
        assert_eq!(value.property("missing"), None);
        assert_eq!(value.element(0), None);
        assert!(value.is_mapping());
        assert_eq!(ContextValue::Integer(4).as_number(), Some(4.0));
        assert_eq!(ContextValue::Decimal(1.5).as_number(), Some(1.5));
        assert_eq!(ContextValue::text("x").as_number(), None);
        assert_eq!(
            ContextValue::list([ContextValue::Null]).as_list(),
            Some([ContextValue::Null].as_slice())
        );
    }
}
