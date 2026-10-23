use super::{EXPRESSION_CLOSE, EXPRESSION_OPEN, ExpressionResolver};
use crate::{application::errors::EvalError, domain::value_objects::EvaluationContext};

pub(crate) struct ExpressionParts<'a> {
    remainder: &'a str,
    context: &'a EvaluationContext,
    finished: bool,
}

impl<'a> ExpressionParts<'a> {
    pub(crate) fn new(remainder: &'a str, context: &'a EvaluationContext) -> Self {
        Self {
            remainder,
            context,
            finished: false,
        }
    }
}

impl Iterator for ExpressionParts<'_> {
    type Item = Result<String, EvalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let Some(open) = self.remainder.find(EXPRESSION_OPEN) else {
            self.finished = true;
            return Some(Ok(self.remainder.to_owned()));
        };
        let (literal, expression_start) = self.remainder.split_at(open);
        let body_start = &expression_start[EXPRESSION_OPEN.len()..];
        let Some(close) = body_start.find(EXPRESSION_CLOSE) else {
            self.finished = true;
            return Some(Ok([literal, expression_start].concat()));
        };

        let value = match ExpressionResolver::evaluate(body_start[..close].trim(), self.context) {
            Ok(value) => value,
            Err(error) => return Some(Err(error)),
        };
        self.remainder = &body_start[close + EXPRESSION_CLOSE.len()..];
        Some(Ok([literal, value.as_str()].concat()))
    }
}
