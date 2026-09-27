/// AST evaluator for workflow `${{ }}` expressions.
///
/// Walks an [`Expression`] AST and produces a [`ContextValue`] result,
/// using the built-in [`ExpressionFunctions`] dispatcher for function calls.
use crate::{
    errors::EvalError,
    services::ExpressionFunctions,
    value_objects::{
        ComparisonOperator, ContextValue, EvaluationContext, Expression, ExpressionLiteral,
        LogicalOperator,
    },
};

/// Walks an expression AST and evaluates it to a [`ContextValue`].
pub struct ExpressionEvaluator<'a> {
    context: &'a EvaluationContext,
    functions: ExpressionFunctions,
}

impl<'a> ExpressionEvaluator<'a> {
    /// Creates a new `ExpressionEvaluator` bound to the given evaluation context.
    #[must_use]
    pub fn new(context: &'a EvaluationContext) -> Self {
        Self {
            context,
            functions: ExpressionFunctions::new(),
        }
    }

    /// Evaluates an expression AST node, returning the resulting [`ContextValue`].
    ///
    /// # Errors
    ///
    /// Returns [`EvalError`] for type errors, unknown functions, or
    /// other evaluation failures.
    pub fn evaluate(&self, expr: &Expression) -> Result<ContextValue, EvalError> {
        match expr {
            Expression::Literal(lit) => self.eval_literal(lit),
            Expression::Variable(name) => self.eval_variable(name),
            Expression::PropertyAccess(obj, prop) => self.eval_property_access(obj, prop),
            Expression::IndexAccess(obj, idx) => self.eval_index_access(obj, idx),
            Expression::ArrayDereference(obj) => self.eval_array_deref(obj),
            Expression::Not(inner) => self.eval_not(inner),
            Expression::Comparison(op, left, right) => self.eval_compare(*op, left, right),
            Expression::Logical(op, left, right) => self.eval_logical(*op, left, right),
            Expression::FunctionCall(name, args) => self.eval_func_call(name, args),
        }
    }

    fn eval_literal(&self, lit: &ExpressionLiteral) -> Result<ContextValue, EvalError> {
        match lit {
            ExpressionLiteral::Boolean(b) => Ok(ContextValue::Boolean(*b)),
            ExpressionLiteral::Null => Ok(ContextValue::Null),
            ExpressionLiteral::Integer(n) => Ok(ContextValue::Integer(*n)),
            ExpressionLiteral::Float(f) => self.finite_decimal(*f),
            ExpressionLiteral::String(s) => Ok(ContextValue::Text(s.clone())),
        }
    }

    fn eval_variable(&self, name: &str) -> Result<ContextValue, EvalError> {
        if let Some(val) = self.context.get(name) {
            return Ok(val.clone());
        }
        Ok(ContextValue::text(format!("${{{{ {name} }}}}")))
    }

    fn eval_property_access(
        &self,
        obj: &Expression,
        prop: &str,
    ) -> Result<ContextValue, EvalError> {
        let val = self.evaluate(obj)?;
        match val.is_mapping() {
            true => val.property(prop).cloned().ok_or_else(|| {
                EvalError::TypeError(format!("property '{prop}' not found on object"))
            }),
            false => Err(EvalError::TypeError(format!(
                "cannot access property '{prop}' on non-object value"
            ))),
        }
    }

    fn eval_index_access(
        &self,
        obj: &Expression,
        idx: &Expression,
    ) -> Result<ContextValue, EvalError> {
        let obj_val = self.evaluate(obj)?;
        let idx_val = self.evaluate(idx)?;
        match (&obj_val, &idx_val) {
            (ContextValue::List(items), index) if index.as_number().is_some() => {
                self.eval_array_index(items, index)
            }
            (ContextValue::Mapping(_), ContextValue::Text(key)) => {
                self.eval_object_index(&obj_val, key)
            }
            _ => Err(EvalError::TypeError(
                "index access requires array+number or object+string".into(),
            )),
        }
    }

    fn eval_array_deref(&self, obj: &Expression) -> Result<ContextValue, EvalError> {
        self.evaluate(obj)
    }

    fn eval_not(&self, inner: &Expression) -> Result<ContextValue, EvalError> {
        let val = self.evaluate(inner)?;
        Ok(ContextValue::Boolean(!val.is_truthy()))
    }

    fn eval_compare(
        &self,
        op: ComparisonOperator,
        left: &Expression,
        right: &Expression,
    ) -> Result<ContextValue, EvalError> {
        let lhs = self.evaluate(left)?;
        let rhs = self.evaluate(right)?;
        let ordering = self.compare_values(&lhs, &rhs)?;
        let result = match op {
            ComparisonOperator::Equal => ordering == std::cmp::Ordering::Equal,
            ComparisonOperator::NotEqual => ordering != std::cmp::Ordering::Equal,
            ComparisonOperator::LessThan => ordering == std::cmp::Ordering::Less,
            ComparisonOperator::LessThanOrEqual => ordering != std::cmp::Ordering::Greater,
            ComparisonOperator::GreaterThan => ordering == std::cmp::Ordering::Greater,
            ComparisonOperator::GreaterThanOrEqual => ordering != std::cmp::Ordering::Less,
        };
        Ok(ContextValue::Boolean(result))
    }

    fn eval_logical(
        &self,
        op: LogicalOperator,
        left: &Expression,
        right: &Expression,
    ) -> Result<ContextValue, EvalError> {
        let lhs = self.evaluate(left)?;
        match op {
            LogicalOperator::And => {
                if !lhs.is_truthy() {
                    return Ok(lhs);
                }
            }
            LogicalOperator::Or => {
                if lhs.is_truthy() {
                    return Ok(lhs);
                }
            }
        }
        self.evaluate(right)
    }

    fn eval_func_call(&self, name: &str, args: &[Expression]) -> Result<ContextValue, EvalError> {
        let evaluated: Result<Vec<ContextValue>, EvalError> =
            args.iter().map(|arg| self.evaluate(arg)).collect();
        self.functions.call(name, &evaluated?)
    }
    /// Wraps a float literal as a decimal value, rejecting non-finite numbers.
    fn finite_decimal(&self, value: f64) -> Result<ContextValue, EvalError> {
        match value.is_finite() {
            true => Ok(ContextValue::Decimal(value)),
            false => Err(EvalError::TypeError(format!(
                "invalid float literal: {value}"
            ))),
        }
    }

    /// Compares two values using expression language coercion rules.
    fn compare_values(
        &self,
        left: &ContextValue,
        right: &ContextValue,
    ) -> Result<std::cmp::Ordering, EvalError> {
        match (left, right) {
            (ContextValue::Text(a), ContextValue::Text(b)) => Ok(a.cmp(b)),
            (ContextValue::Boolean(a), ContextValue::Boolean(b)) => Ok(a.cmp(b)),
            _ => self.compare_numbers(left, right),
        }
    }

    fn compare_numbers(
        &self,
        left: &ContextValue,
        right: &ContextValue,
    ) -> Result<std::cmp::Ordering, EvalError> {
        let mismatch = || EvalError::TypeError("cannot compare values of different types".into());
        let a = left.as_number().ok_or_else(mismatch)?;
        let b = right.as_number().ok_or_else(mismatch)?;
        Ok(a.total_cmp(&b))
    }

    fn eval_array_index(
        &self,
        items: &[ContextValue],
        index: &ContextValue,
    ) -> Result<ContextValue, EvalError> {
        let position = self.array_position(index)?;
        items.get(position).cloned().ok_or_else(|| {
            EvalError::TypeError(format!(
                "index {position} out of bounds for array of length {}",
                items.len()
            ))
        })
    }

    fn array_position(&self, index: &ContextValue) -> Result<usize, EvalError> {
        let invalid = || EvalError::TypeError("index must be a non-negative integer".into());
        match index {
            ContextValue::Integer(value) => usize::try_from(*value).map_err(|_| invalid()),
            _ => Err(invalid()),
        }
    }

    fn eval_object_index(
        &self,
        mapping: &ContextValue,
        key: &str,
    ) -> Result<ContextValue, EvalError> {
        mapping
            .property(key)
            .cloned()
            .ok_or_else(|| EvalError::TypeError(format!("key '{key}' not found on object")))
    }
}

#[cfg(test)]
mod tests;
