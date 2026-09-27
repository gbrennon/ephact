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
            BuiltinFunction::Contains | BuiltinFunction::StartsWith | BuiltinFunction::EndsWith => {
                self.predicate(function, args)
            }
            BuiltinFunction::Format | BuiltinFunction::Join => self.transform(function, args),
            BuiltinFunction::ToJson | BuiltinFunction::FromJson => self.json(function, args),
            _ => self.status(function),
        }
    }

    fn predicate(&self, function: BuiltinFunction, args: FunctionArgs<'_>) -> FunctionResult {
        match function {
            BuiltinFunction::Contains => self.contains(&args[0], &args[1]),
            BuiltinFunction::StartsWith => self.starts_with(&args[0], &args[1]),
            BuiltinFunction::EndsWith => self.ends_with(&args[0], &args[1]),
            _ => unreachable!(),
        }
    }

    fn transform(&self, function: BuiltinFunction, args: FunctionArgs<'_>) -> FunctionResult {
        match function {
            BuiltinFunction::Format => self.format(&args[0], &args[1..]),
            BuiltinFunction::Join => self.join(&args[0], &args[1]),
            _ => unreachable!(),
        }
    }

    fn json(&self, function: BuiltinFunction, args: FunctionArgs<'_>) -> FunctionResult {
        match function {
            BuiltinFunction::ToJson => self.to_json(&args[0]),
            BuiltinFunction::FromJson => self.from_json(&args[0]),
            _ => unreachable!(),
        }
    }

    fn status(&self, function: BuiltinFunction) -> FunctionResult {
        match function {
            BuiltinFunction::Success => self.success(),
            BuiltinFunction::Always => self.always(),
            BuiltinFunction::Cancelled => self.cancelled(),
            BuiltinFunction::Failure => self.failure(),
            _ => unreachable!(),
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
mod tests;
