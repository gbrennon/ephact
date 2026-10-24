use super::ExpressionFunctions;
use crate::{application::errors::EvalError, domain::value_objects::ContextValue};

pub(crate) struct FormatParts<'a> {
    remainder: &'a str,
    args: &'a [ContextValue],
    finished: bool,
}

impl<'a> FormatParts<'a> {
    pub(crate) fn new(remainder: &'a str, args: &'a [ContextValue]) -> Self {
        Self {
            remainder,
            args,
            finished: false,
        }
    }
}

impl Iterator for FormatParts<'_> {
    type Item = Result<String, EvalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let Some(character) = self.remainder.chars().next() else {
            self.finished = true;
            return None;
        };

        match character {
            '{' => {
                let (end, index) = match ExpressionFunctions::parse_placeholder(self.remainder) {
                    Ok(value) => value,
                    Err(error) => {
                        self.finished = true;
                        return Some(Err(error));
                    }
                };
                let value = match ExpressionFunctions::formatted_argument(self.args, index) {
                    Ok(value) => value,
                    Err(error) => {
                        self.finished = true;
                        return Some(Err(error));
                    }
                };
                self.remainder = &self.remainder[end + 1..];
                Some(Ok(value))
            }
            '}' => {
                self.finished = true;
                Some(Err(EvalError::FormatError(
                    "format: single '}' encountered in template".into(),
                )))
            }
            other => {
                self.remainder = &self.remainder[other.len_utf8()..];
                Some(Ok(other.to_string()))
            }
        }
    }
}
