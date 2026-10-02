use std::collections::HashMap;

use serde::{Deserialize, Deserializer, de};

/// Deserializes a YAML mapping of scalar values into a string map.
///
/// Workflow and action `env`/`with` values are strings at runtime, but authors
/// routinely write unquoted scalars such as `RUSTUP_PERMIT_COPY_RENAME: 1` or
/// `fetch-depth: 0`. GitHub and Woodpecker coerce these to strings; this mirrors
/// that by accepting string, boolean, number, and null scalars while rejecting
/// nested sequences or mappings, which are not valid scalar values.
pub fn deserialize_scalar_string_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = HashMap::<String, serde_yaml::Value>::deserialize(deserializer)?;
    raw.into_iter()
        .map(|(key, value)| Ok((key, scalar_to_string(&value).map_err(de::Error::custom)?)))
        .collect()
}

fn scalar_to_string(value: &serde_yaml::Value) -> Result<String, String> {
    match value {
        serde_yaml::Value::String(text) => Ok(text.clone()),
        serde_yaml::Value::Bool(flag) => Ok(flag.to_string()),
        serde_yaml::Value::Number(number) => Ok(number.to_string()),
        serde_yaml::Value::Null => Ok(String::new()),
        serde_yaml::Value::Sequence(_)
        | serde_yaml::Value::Mapping(_)
        | serde_yaml::Value::Tagged(_) => {
            Err("expected a scalar value (string, number, or boolean)".to_owned())
        }
    }
}

/// Deserializes an optional scalar YAML value into an optional string.
///
/// Fields such as `continue-on-error` are strings at runtime but are commonly
/// authored as unquoted booleans (`continue-on-error: true`). This accepts any
/// scalar and normalizes it to its string form, leaving absent values as `None`.
pub fn deserialize_optional_scalar_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_yaml::Value>::deserialize(deserializer)?;
    match value {
        None | Some(serde_yaml::Value::Null) => Ok(None),
        Some(scalar) => scalar_to_string(&scalar)
            .map(Some)
            .map_err(de::Error::custom),
    }
}
