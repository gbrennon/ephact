use crate::{
    errors::EvalError,
    value_objects::{ContextValue, builtin_function::BuiltinFunction},
};

type FunctionArgs<'a> = &'a [ContextValue];
type FunctionResult = Result<ContextValue, EvalError>;

#[derive(Default)]
pub struct ExpressionFunctions;

impl ExpressionFunctions {
    pub fn new() -> Self {
        Self
    }

    pub fn contains(
        &self,
        search: &ContextValue,
        item: &ContextValue,
    ) -> Result<ContextValue, EvalError> {
        match search {
            ContextValue::Text(s) => {
                let item_str = item.as_text().ok_or_else(|| {
                    EvalError::TypeError(
                        "contains: item must be a string when search is a string".into(),
                    )
                })?;
                Ok(ContextValue::Boolean(
                    s.to_lowercase().contains(&item_str.to_lowercase()),
                ))
            }
            ContextValue::List(items) => Ok(ContextValue::Boolean(items.contains(item))),
            other => Err(EvalError::TypeError(format!(
                "contains: search must be a string or array, got {}",
                other.type_name()
            ))),
        }
    }

    pub fn starts_with(
        &self,
        search: &ContextValue,
        prefix: &ContextValue,
    ) -> Result<ContextValue, EvalError> {
        let s = Self::expect_string(search, "startsWith", "search")?;
        let p = Self::expect_string(prefix, "startsWith", "prefix")?;
        Ok(ContextValue::Boolean(
            s.to_lowercase().starts_with(&p.to_lowercase()),
        ))
    }

    pub fn ends_with(
        &self,
        search: &ContextValue,
        suffix: &ContextValue,
    ) -> Result<ContextValue, EvalError> {
        let s = Self::expect_string(search, "endsWith", "search")?;
        let sfx = Self::expect_string(suffix, "endsWith", "suffix")?;
        Ok(ContextValue::Boolean(
            s.to_lowercase().ends_with(&sfx.to_lowercase()),
        ))
    }

    pub fn format(
        &self,
        template: &ContextValue,
        args: &[ContextValue],
    ) -> Result<ContextValue, EvalError> {
        let tmpl = Self::expect_string(template, "format", "template")?;
        let mut result = String::with_capacity(tmpl.len());
        let mut rest = tmpl;

        while let Some(ch) = rest.chars().next() {
            Self::process_format_char(ch, &mut rest, &mut result, args)?;
        }
        Ok(ContextValue::Text(result))
    }

    fn process_format_char(
        ch: char,
        rest: &mut &str,
        result: &mut String,
        args: &[ContextValue],
    ) -> Result<(), EvalError> {
        match ch {
            '{' => {
                let (end, idx) = Self::parse_placeholder(rest)?;
                Self::append_formatted_arg(result, args, idx)?;
                *rest = &rest[end + 1..];
                Ok(())
            }
            '}' => Err(EvalError::FormatError(
                "format: single '}' encountered in template".into(),
            )),
            other => {
                result.push(other);
                *rest = &rest[other.len_utf8()..];
                Ok(())
            }
        }
    }

    fn append_formatted_arg(
        result: &mut String,
        args: &[ContextValue],
        idx: usize,
    ) -> Result<(), EvalError> {
        let replacement = args.get(idx).ok_or_else(|| {
            EvalError::FormatError(format!(
                "format: placeholder index {idx} out of range (have {} args)",
                args.len()
            ))
        })?;
        result.push_str(&replacement.to_display_text());
        Ok(())
    }

    fn parse_placeholder(rest: &str) -> Result<(usize, usize), EvalError> {
        let mut chars = rest.char_indices();
        chars.next();
        let end = Self::find_placeholder_end(&mut chars)?;
        let idx_str = &rest[1..end];
        let idx: usize = idx_str.parse().map_err(|_| {
            EvalError::FormatError(format!("format: invalid placeholder index '{idx_str}'"))
        })?;
        Ok((end, idx))
    }

    fn find_placeholder_end(chars: &mut std::str::CharIndices<'_>) -> Result<usize, EvalError> {
        let mut found_close = false;
        let mut end = 1;
        for (j, c) in chars.by_ref() {
            if c == '}' {
                found_close = true;
                end = j;
                break;
            }
            if !c.is_ascii_digit() {
                return Err(EvalError::FormatError(format!(
                    "format: invalid placeholder character '{c}' at position {j}"
                )));
            }
        }
        if !found_close {
            return Err(EvalError::FormatError(
                "format: unclosed placeholder".into(),
            ));
        }
        Ok(end)
    }

    pub fn join(
        &self,
        array: &ContextValue,
        sep: &ContextValue,
    ) -> Result<ContextValue, EvalError> {
        let items = array
            .as_list()
            .ok_or_else(|| EvalError::TypeError("join: first argument must be an array".into()))?;
        let separator = Self::expect_string(sep, "join", "separator")?;
        let parts: Vec<String> = items.iter().map(ContextValue::to_display_text).collect();
        Ok(ContextValue::Text(parts.join(separator)))
    }

    pub fn to_json(&self, value: &ContextValue) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Text(value.to_json_text()))
    }

    pub fn from_json(&self, value: &ContextValue) -> Result<ContextValue, EvalError> {
        let s = Self::expect_string(value, "fromJson", "value")?;
        ContextValue::from_json_text(s)
            .map_err(|error| EvalError::JsonError(format!("fromJson: {error}")))
    }

    pub fn success(&self) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Boolean(true))
    }

    pub fn always(&self) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Boolean(true))
    }

    pub fn cancelled(&self) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Boolean(false))
    }

    pub fn failure(&self) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Boolean(false))
    }

    /// Dispatches a function call by name, case-insensitively.
    pub fn call(&self, name: &str, args: &[ContextValue]) -> Result<ContextValue, EvalError> {
        let function = BuiltinFunction::from_name(name).ok_or_else(|| {
            EvalError::TypeError(format!("unknown function: {}", name.to_lowercase()))
        })?;
        Self::expect_builtin_arg_count(name, args.len(), &function)?;
        self.invoke(function, args)
    }

    fn invoke(&self, function: BuiltinFunction, args: FunctionArgs<'_>) -> FunctionResult {
        match function {
            BuiltinFunction::Contains => self.contains(&args[0], &args[1]),
            BuiltinFunction::StartsWith => self.starts_with(&args[0], &args[1]),
            BuiltinFunction::EndsWith => self.ends_with(&args[0], &args[1]),
            BuiltinFunction::Format => self.format(&args[0], &args[1..]),
            BuiltinFunction::Join => self.join(&args[0], &args[1]),
            BuiltinFunction::ToJson => self.to_json(&args[0]),
            BuiltinFunction::FromJson => self.from_json(&args[0]),
            BuiltinFunction::Success => self.success(),
            BuiltinFunction::Always => self.always(),
            BuiltinFunction::Cancelled => self.cancelled(),
            BuiltinFunction::Failure => self.failure(),
        }
    }

    fn expect_builtin_arg_count(
        name: &str,
        actual: usize,
        function: &BuiltinFunction,
    ) -> Result<(), EvalError> {
        if function.name() == "format" {
            return if actual == 0 {
                Err(EvalError::ArgCount(format!(
                    "{name}: expected at least 1 argument, got 0"
                )))
            } else {
                Ok(())
            };
        }
        let required = function.required_arguments();
        Self::expect_arg_count(name, actual, required, required)
    }

    fn expect_string<'v>(
        value: &'v ContextValue,
        func: &str,
        arg_name: &str,
    ) -> Result<&'v str, EvalError> {
        value.as_text().ok_or_else(|| {
            EvalError::TypeError(format!(
                "{func}: {arg_name} must be a string, got {}",
                value.type_name()
            ))
        })
    }

    fn expect_arg_count(
        func: &str,
        actual: usize,
        min: usize,
        max: usize,
    ) -> Result<(), EvalError> {
        if actual >= min && actual <= max {
            return Ok(());
        }
        let expected = if min == max {
            min.to_string()
        } else {
            format!("{min}..{max}")
        };
        Err(EvalError::ArgCount(format!(
            "{func}: expected {expected} argument(s), got {actual}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_unknown_function_error() {
        let f = ExpressionFunctions::new();
        let err = f.call("nonexistent", &[]).unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn call_wrong_arg_count() {
        let f = ExpressionFunctions::new();
        let err = f
            .call("contains", &[ContextValue::text("only one")])
            .unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn call_case_insensitive() {
        let f = ExpressionFunctions::new();
        let result = f
            .call(
                "CoNtAiNs",
                &[
                    ContextValue::text("Hello World"),
                    ContextValue::text("world"),
                ],
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn call_success_zero_args() {
        let f = ExpressionFunctions::new();
        let result = f.call("success", &[]).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn call_success_with_args_is_error() {
        let f = ExpressionFunctions::new();
        let err = f
            .call("success", &[ContextValue::text("extra")])
            .unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn call_dispatches_starts_with() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "startswith",
                &[ContextValue::text("Hello"), ContextValue::text("he")]
            )
            .unwrap(),
            ContextValue::Boolean(true)
        );
    }

    #[test]
    fn call_dispatches_ends_with() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "endswith",
                &[ContextValue::text("Hello"), ContextValue::text("lo")]
            )
            .unwrap(),
            ContextValue::Boolean(true)
        );
    }

    #[test]
    fn call_format_with_no_args_errors() {
        let f = ExpressionFunctions::new();
        let err = f.call("format", &[]).unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn call_dispatches_format() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "format",
                &[ContextValue::text("Hi {0}"), ContextValue::text("bob")]
            )
            .unwrap(),
            ContextValue::text("Hi bob")
        );
    }

    #[test]
    fn call_dispatches_join() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "join",
                &[
                    ContextValue::list([ContextValue::text("a"), ContextValue::text("b")]),
                    ContextValue::text("-")
                ]
            )
            .unwrap(),
            ContextValue::text("a-b")
        );
    }

    #[test]
    fn call_dispatches_to_json() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call(
                "tojson",
                &[ContextValue::mapping([(
                    "a".to_owned(),
                    ContextValue::Integer(1)
                )])]
            )
            .unwrap(),
            ContextValue::text(r#"{"a":1}"#)
        );
    }

    #[test]
    fn call_dispatches_from_json() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.call("fromjson", &[ContextValue::text(r#"{"a":1}"#)])
                .unwrap(),
            ContextValue::mapping([("a".to_owned(), ContextValue::Integer(1))])
        );
    }

    #[test]
    fn call_dispatches_always_cancelled_failure() {
        let f = ExpressionFunctions::new();
        assert_eq!(f.call("always", &[]).unwrap(), ContextValue::Boolean(true));
        assert_eq!(
            f.call("cancelled", &[]).unwrap(),
            ContextValue::Boolean(false)
        );
        assert_eq!(
            f.call("failure", &[]).unwrap(),
            ContextValue::Boolean(false)
        );
    }

    #[test]
    fn expect_arg_count_reports_range() {
        let err = ExpressionFunctions::expect_arg_count("x", 4, 1, 3).unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn format_basic() {
        let f = ExpressionFunctions::new();
        let result = f
            .format(
                &ContextValue::text("Hello {0}"),
                &[ContextValue::text("world")],
            )
            .unwrap();
        assert_eq!(result, ContextValue::text("Hello world"));
    }

    #[test]
    fn format_multiple_args() {
        let f = ExpressionFunctions::new();
        let result = f
            .format(
                &ContextValue::text("{0} + {1} = {2}"),
                &[
                    ContextValue::Integer(1),
                    ContextValue::Integer(2),
                    ContextValue::Integer(3),
                ],
            )
            .unwrap();
        assert_eq!(result, ContextValue::text("1 + 2 = 3"));
    }

    #[test]
    fn format_no_placeholders() {
        let f = ExpressionFunctions::new();
        let result = f
            .format(&ContextValue::text("no placeholders"), &[])
            .unwrap();
        assert_eq!(result, ContextValue::text("no placeholders"));
    }

    #[test]
    fn format_index_out_of_range() {
        let f = ExpressionFunctions::new();
        let err = f
            .format(
                &ContextValue::text("Hello {5}"),
                &[ContextValue::text("world")],
            )
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_invalid_placeholder_character() {
        let f = ExpressionFunctions::new();
        let err = f
            .format(&ContextValue::text("{a}"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_unclosed_placeholder() {
        let f = ExpressionFunctions::new();
        let err = f.format(&ContextValue::text("{0"), &[]).unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_invalid_placeholder_index() {
        let f = ExpressionFunctions::new();
        let err = f
            .format(&ContextValue::text("{}"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_unexpected_closing_brace() {
        let f = ExpressionFunctions::new();
        let err = f
            .format(&ContextValue::text("a}b"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_null_and_bool_and_array_replacement() {
        let f = ExpressionFunctions::new();
        assert_eq!(
            f.format(&ContextValue::text("{0}"), &[ContextValue::Null])
                .unwrap(),
            ContextValue::text("null")
        );
        assert_eq!(
            f.format(&ContextValue::text("{0}"), &[ContextValue::Boolean(true)])
                .unwrap(),
            ContextValue::text("true")
        );
        assert_eq!(
            f.format(
                &ContextValue::text("{0}"),
                &[ContextValue::list([
                    ContextValue::Integer(1),
                    ContextValue::Integer(2)
                ])]
            )
            .unwrap(),
            ContextValue::text("[1,2]")
        );
    }

    #[test]
    fn join_basic() {
        let f = ExpressionFunctions::new();
        let result = f
            .join(
                &ContextValue::list([
                    ContextValue::text("a"),
                    ContextValue::text("b"),
                    ContextValue::text("c"),
                ]),
                &ContextValue::text(", "),
            )
            .unwrap();
        assert_eq!(result, ContextValue::text("a, b, c"));
    }

    #[test]
    fn join_single_element() {
        let f = ExpressionFunctions::new();
        let result = f
            .join(
                &ContextValue::list([ContextValue::text("only")]),
                &ContextValue::text(", "),
            )
            .unwrap();
        assert_eq!(result, ContextValue::text("only"));
    }

    #[test]
    fn join_empty_array() {
        let f = ExpressionFunctions::new();
        let result = f
            .join(&ContextValue::list([]), &ContextValue::text(", "))
            .unwrap();
        assert_eq!(result, ContextValue::text(""));
    }

    #[test]
    fn join_first_argument_must_be_an_array() {
        let f = ExpressionFunctions::new();
        let result = f.join(&ContextValue::text("nope"), &ContextValue::text(", "));
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn to_json_roundtrip() {
        let f = ExpressionFunctions::new();
        let original = ContextValue::mapping([
            ("key".to_owned(), ContextValue::text("value")),
            ("num".to_owned(), ContextValue::Integer(42)),
        ]);
        let json_str = f.to_json(&original).unwrap();
        let parsed = f.from_json(&json_str).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn from_json_invalid() {
        let f = ExpressionFunctions::new();
        let err = f.from_json(&ContextValue::text("not json")).unwrap_err();
        assert!(matches!(err, EvalError::JsonError(_)));
    }

    #[test]
    fn contains_string_match_case_insensitive() {
        let f = ExpressionFunctions::new();
        let result = f
            .contains(
                &ContextValue::text("Hello World"),
                &ContextValue::text("world"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn contains_string_no_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .contains(
                &ContextValue::text("Hello World"),
                &ContextValue::text("xyz"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn contains_array_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .contains(
                &ContextValue::list([
                    ContextValue::text("a"),
                    ContextValue::text("b"),
                    ContextValue::text("c"),
                ]),
                &ContextValue::text("b"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn contains_array_no_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .contains(
                &ContextValue::list([ContextValue::text("a"), ContextValue::text("b")]),
                &ContextValue::text("c"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn contains_type_error_on_number() {
        let f = ExpressionFunctions::new();
        let err = f
            .contains(&ContextValue::Integer(42), &ContextValue::text("x"))
            .unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn starts_with_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .starts_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("hello"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn starts_with_no_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .starts_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("World"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn ends_with_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .ends_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("WORLD"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn ends_with_no_match() {
        let f = ExpressionFunctions::new();
        let result = f
            .ends_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("Hello"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn contains_errors_when_search_string_and_item_not_string() {
        let f = ExpressionFunctions::new();
        let err = f
            .contains(&ContextValue::text("hello"), &ContextValue::Integer(42))
            .unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn expect_string_error_names_value_type() {
        let f = ExpressionFunctions::new();
        assert!(
            f.starts_with(&ContextValue::Null, &ContextValue::text("a"))
                .is_err()
        );
        assert!(
            f.starts_with(&ContextValue::Boolean(true), &ContextValue::text("a"))
                .is_err()
        );
        assert!(
            f.starts_with(&ContextValue::Integer(42), &ContextValue::text("a"))
                .is_err()
        );
        assert!(
            f.starts_with(
                &ContextValue::list([ContextValue::Integer(1), ContextValue::Integer(2)]),
                &ContextValue::text("a")
            )
            .is_err()
        );
        assert!(
            f.starts_with(
                &ContextValue::mapping([("k".to_owned(), ContextValue::Integer(1))]),
                &ContextValue::text("a")
            )
            .is_err()
        );
    }

    #[test]
    fn success_returns_true() {
        let f = ExpressionFunctions::new();
        assert_eq!(f.success().unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn always_returns_true() {
        let f = ExpressionFunctions::new();
        assert_eq!(f.always().unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn cancelled_returns_false() {
        let f = ExpressionFunctions::new();
        assert_eq!(f.cancelled().unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn failure_returns_false() {
        let f = ExpressionFunctions::new();
        assert_eq!(f.failure().unwrap(), ContextValue::Boolean(false));
    }
}
