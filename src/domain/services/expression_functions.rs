use crate::domain::{errors::EvalError, value_objects::ContextValue};

/// Built-in function dispatcher for GitHub Actions expressions.
///
/// Exposes each built-in as a method, plus a generic
/// [`call`](Self::call) dispatcher that routes by function name
/// (case-insensitive).
#[derive(Default)]
pub struct ExpressionFunctions;

impl ExpressionFunctions {
    /// Creates a new `ExpressionFunctions` dispatcher.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Returns `true` if `search` contains `item`.
    ///
    /// - **String search**: case-insensitive substring match.
    /// - **Array search**: exact element match.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::TypeError`] if `search` is neither a string
    /// nor an array.
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

    /// Returns `true` if the string `search` starts with `prefix`
    /// (case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::TypeError`] if either argument is not a string.
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

    /// Returns `true` if the string `search` ends with `suffix`
    /// (case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::TypeError`] if either argument is not a string.
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

    /// Formats a template string by replacing `{0}`, `{1}`, ... with the
    /// string representations of the positional arguments.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::TypeError`] if `template` is not a string.
    /// Returns [`EvalError::FormatError`] if a placeholder index is
    /// out of range or malformed.
    pub fn format(
        &self,
        template: &ContextValue,
        args: &[ContextValue],
    ) -> Result<ContextValue, EvalError> {
        let tmpl = Self::expect_string(template, "format", "template")?;
        let mut result = String::with_capacity(tmpl.len());
        let mut rest = tmpl;
        let mut chars = rest.char_indices();

        while let Some((i, ch)) = chars.next() {
            Self::process_format_char(ch, i, &mut chars, &mut rest, &mut result, args)?;
        }
        Ok(ContextValue::Text(result))
    }

    fn process_format_char<'a>(
        ch: char,
        i: usize,
        chars: &mut std::str::CharIndices<'a>,
        rest: &mut &'a str,
        result: &mut String,
        args: &[ContextValue],
    ) -> Result<(), EvalError> {
        match ch {
            '{' => {
                let (end, idx) = Self::parse_placeholder(chars, rest, i + 1)?;
                Self::append_formatted_arg(result, args, idx)?;
                *rest = &rest[end + 1..];
                *chars = rest.char_indices();
                Ok(())
            }
            '}' => Err(EvalError::FormatError(
                "format: single '}' encountered in template".into(),
            )),
            other => {
                result.push(other);
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

    fn parse_placeholder(
        chars: &mut std::str::CharIndices<'_>,
        rest: &str,
        start: usize,
    ) -> Result<(usize, usize), EvalError> {
        let end = Self::find_placeholder_end(chars, start)?;
        let idx_str = &rest[start..end];
        let idx: usize = idx_str.parse().map_err(|_| {
            EvalError::FormatError(format!("format: invalid placeholder index '{idx_str}'"))
        })?;
        Ok((end, idx))
    }

    fn find_placeholder_end(
        chars: &mut std::str::CharIndices<'_>,
        start: usize,
    ) -> Result<usize, EvalError> {
        let mut end = start;
        let mut found_close = false;
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

    /// Joins the elements of `array` into a single string, separated by
    /// `sep`.
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::TypeError`] if `array` is not an array or
    /// `sep` is not a string.
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

    /// Renders `value` as JSON text.
    ///
    /// # Errors
    ///
    /// Never fails; the result type mirrors the other built-ins so the
    /// dispatcher can treat every function alike.
    pub fn to_json(&self, value: &ContextValue) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Text(value.to_json_text()))
    }

    /// Reads JSON text into a [`ContextValue`].
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::TypeError`] if `value` is not a string.
    /// Returns [`EvalError::JsonError`] if the text is not valid JSON.
    pub fn from_json(&self, value: &ContextValue) -> Result<ContextValue, EvalError> {
        let s = Self::expect_string(value, "fromJson", "value")?;
        ContextValue::from_json_text(s)
            .map_err(|error| EvalError::JsonError(format!("fromJson: {error}")))
    }

    /// Always returns `true` - stub for job status check.
    ///
    /// In a full implementation this would consult the workflow context
    /// to determine whether all previous steps succeeded.
    pub fn success(&self) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Boolean(true))
    }

    /// Always returns `true` - stub for unconditional execution check.
    pub fn always(&self) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Boolean(true))
    }

    /// Always returns `false` - stub for cancellation check.
    pub fn cancelled(&self) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Boolean(false))
    }

    /// Always returns `false` - stub for failure check.
    pub fn failure(&self) -> Result<ContextValue, EvalError> {
        Ok(ContextValue::Boolean(false))
    }

    /// Dispatches a function call by name (case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns [`EvalError::ArgCount`] if the wrong number of arguments
    /// is supplied. Returns [`EvalError::TypeError`] or other variants
    /// as propagated from the individual function implementations.
    pub fn call(&self, name: &str, args: &[ContextValue]) -> Result<ContextValue, EvalError> {
        let lower = name.to_lowercase();
        if let Some(res) = self.call_status_fn(&lower, name, args)? {
            return Ok(res);
        }
        if let Some(res) = self.call_json_fn(&lower, name, args)? {
            return Ok(res);
        }
        self.call_string_fn(&lower, name, args)
    }

    fn call_status_fn(
        &self,
        lower: &str,
        name: &str,
        args: &[ContextValue],
    ) -> Result<Option<ContextValue>, EvalError> {
        let status = match lower {
            "success" => self.success(),
            "always" => self.always(),
            "cancelled" => self.cancelled(),
            "failure" => self.failure(),
            _ => return Ok(None),
        };
        Self::expect_arg_count(name, args.len(), 0, 0)?;
        status.map(Some)
    }

    fn call_json_fn(
        &self,
        lower: &str,
        name: &str,
        args: &[ContextValue],
    ) -> Result<Option<ContextValue>, EvalError> {
        match lower {
            "tojson" => {
                Self::expect_arg_count(name, args.len(), 1, 1)?;
                self.to_json(&args[0]).map(Some)
            }
            "fromjson" => {
                Self::expect_arg_count(name, args.len(), 1, 1)?;
                self.from_json(&args[0]).map(Some)
            }
            _ => Ok(None),
        }
    }

    fn call_string_fn(
        &self,
        lower: &str,
        name: &str,
        args: &[ContextValue],
    ) -> Result<ContextValue, EvalError> {
        if let Some(res) = self.call_predicate_fn(lower, name, args)? {
            return Ok(res);
        }
        self.call_transform_fn(lower, name, args)
    }

    fn call_predicate_fn(
        &self,
        lower: &str,
        name: &str,
        args: &[ContextValue],
    ) -> Result<Option<ContextValue>, EvalError> {
        match lower {
            "contains" => {
                Self::expect_arg_count(name, args.len(), 2, 2)?;
                self.contains(&args[0], &args[1]).map(Some)
            }
            "startswith" => {
                Self::expect_arg_count(name, args.len(), 2, 2)?;
                self.starts_with(&args[0], &args[1]).map(Some)
            }
            "endswith" => {
                Self::expect_arg_count(name, args.len(), 2, 2)?;
                self.ends_with(&args[0], &args[1]).map(Some)
            }
            _ => Ok(None),
        }
    }

    fn call_transform_fn(
        &self,
        lower: &str,
        name: &str,
        args: &[ContextValue],
    ) -> Result<ContextValue, EvalError> {
        match lower {
            "format" => {
                if args.is_empty() {
                    return Err(EvalError::ArgCount(format!(
                        "{name}: expected at least 1 argument, got 0"
                    )));
                }
                self.format(&args[0], &args[1..])
            }
            "join" => {
                Self::expect_arg_count(name, args.len(), 2, 2)?;
                self.join(&args[0], &args[1])
            }
            other => Err(EvalError::TypeError(format!("unknown function: {other}"))),
        }
    }
    /// Extracts a string reference from a value, or returns a type error.
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

    /// Validates that `actual` arg count falls within `[min, max]`.
    fn expect_arg_count(
        func: &str,
        actual: usize,
        min: usize,
        max: usize,
    ) -> Result<(), EvalError> {
        if actual < min || actual > max {
            let expected = if min == max {
                min.to_string()
            } else {
                format!("{min}..{max}")
            };
            return Err(EvalError::ArgCount(format!(
                "{func}: expected {expected} argument(s), got {actual}"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fns() -> ExpressionFunctions {
        ExpressionFunctions::new()
    }

    #[test]
    fn contains_string_match_case_insensitive() {
        let f = fns();
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
        let f = fns();
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
        let f = fns();
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
        let f = fns();
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
        let f = fns();
        let err = f
            .contains(&ContextValue::Integer(42), &ContextValue::text("x"))
            .unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn starts_with_match() {
        let f = fns();
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
        let f = fns();
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
        let f = fns();
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
        let f = fns();
        let result = f
            .ends_with(
                &ContextValue::text("Hello World"),
                &ContextValue::text("Hello"),
            )
            .unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn format_basic() {
        let f = fns();
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
        let f = fns();
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
        let f = fns();
        let result = f
            .format(&ContextValue::text("no placeholders"), &[])
            .unwrap();
        assert_eq!(result, ContextValue::text("no placeholders"));
    }

    #[test]
    fn format_index_out_of_range() {
        let f = fns();
        let err = f
            .format(
                &ContextValue::text("Hello {5}"),
                &[ContextValue::text("world")],
            )
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn join_basic() {
        let f = fns();
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
        let f = fns();
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
        let f = fns();
        let result = f
            .join(&ContextValue::list([]), &ContextValue::text(", "))
            .unwrap();
        assert_eq!(result, ContextValue::text(""));
    }

    #[test]
    fn to_json_roundtrip() {
        let f = fns();
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
        let f = fns();
        let err = f.from_json(&ContextValue::text("not json")).unwrap_err();
        assert!(matches!(err, EvalError::JsonError(_)));
    }

    #[test]
    fn success_returns_true() {
        let f = fns();
        assert_eq!(f.success().unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn always_returns_true() {
        let f = fns();
        assert_eq!(f.always().unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn cancelled_returns_false() {
        let f = fns();
        assert_eq!(f.cancelled().unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn failure_returns_false() {
        let f = fns();
        assert_eq!(f.failure().unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn call_unknown_function_error() {
        let f = fns();
        let err = f.call("nonexistent", &[]).unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn call_wrong_arg_count() {
        let f = fns();
        let err = f
            .call("contains", &[ContextValue::text("only one")])
            .unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn call_case_insensitive() {
        let f = fns();
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
        let f = fns();
        let result = f.call("success", &[]).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn call_success_with_args_is_error() {
        let f = fns();
        let err = f
            .call("success", &[ContextValue::text("extra")])
            .unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn contains_errors_when_search_string_and_item_not_string() {
        let f = fns();
        let err = f
            .contains(&ContextValue::text("hello"), &ContextValue::Integer(42))
            .unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }

    #[test]
    fn format_invalid_placeholder_character() {
        let f = fns();
        let err = f
            .format(&ContextValue::text("{a}"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_unclosed_placeholder() {
        let f = fns();
        let err = f.format(&ContextValue::text("{0"), &[]).unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_invalid_placeholder_index() {
        let f = fns();
        let err = f
            .format(&ContextValue::text("{}"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn format_unexpected_closing_brace() {
        let f = fns();
        let err = f
            .format(&ContextValue::text("a}b"), &[ContextValue::text("x")])
            .unwrap_err();
        assert!(matches!(err, EvalError::FormatError(_)));
    }

    #[test]
    fn call_dispatches_starts_with() {
        let f = fns();
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
        let f = fns();
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
        let f = fns();
        let err = f.call("format", &[]).unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }

    #[test]
    fn call_dispatches_format() {
        let f = fns();
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
        let f = fns();
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
        let f = fns();
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
        let f = fns();
        assert_eq!(
            f.call("fromjson", &[ContextValue::text(r#"{"a":1}"#)])
                .unwrap(),
            ContextValue::mapping([("a".to_owned(), ContextValue::Integer(1))])
        );
    }

    #[test]
    fn call_dispatches_always_cancelled_failure() {
        let f = fns();
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
    fn expect_string_error_names_value_type() {
        let f = fns();
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
    fn format_null_and_bool_and_array_replacement() {
        let f = fns();
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
    fn expect_arg_count_reports_range() {
        let err = ExpressionFunctions::expect_arg_count("x", 4, 1, 3).unwrap_err();
        assert!(matches!(err, EvalError::ArgCount(_)));
    }
}
