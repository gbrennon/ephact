use crate::domain::value_objects::ContextValue;

/// Maps a YAML value onto the expression context value it represents.
///
/// Mapping keys that are not strings are rendered with their YAML scalar text
/// so every entry keeps a name.
#[must_use]
pub fn context_value_from_yaml(value: serde_yaml::Value) -> ContextValue {
    match value {
        serde_yaml::Value::Null => ContextValue::Null,
        serde_yaml::Value::Bool(flag) => ContextValue::Boolean(flag),
        serde_yaml::Value::Number(number) => number_from_yaml(&number),
        serde_yaml::Value::String(text) => ContextValue::Text(text),
        serde_yaml::Value::Sequence(items) => {
            ContextValue::list(items.into_iter().map(context_value_from_yaml))
        }
        serde_yaml::Value::Mapping(entries) => mapping_from_yaml(entries),
        serde_yaml::Value::Tagged(tagged) => context_value_from_yaml(tagged.value),
    }
}

fn number_from_yaml(number: &serde_yaml::Number) -> ContextValue {
    match number.as_i64() {
        Some(integer) => ContextValue::Integer(integer),
        None => ContextValue::Decimal(number.as_f64().unwrap_or_default()),
    }
}

fn mapping_from_yaml(entries: serde_yaml::Mapping) -> ContextValue {
    ContextValue::mapping(
        entries
            .into_iter()
            .map(|(key, value)| (key_text(key), context_value_from_yaml(value))),
    )
}

fn key_text(key: serde_yaml::Value) -> String {
    match key {
        serde_yaml::Value::String(text) => text,
        other => serde_yaml::to_string(&other)
            .unwrap_or_default()
            .trim_end()
            .to_owned(),
    }
}
