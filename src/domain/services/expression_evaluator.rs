/// AST evaluator for workflow `${{ }}` expressions.
///
/// Walks an [`Expression`] AST and produces a [`ContextValue`] result,
/// using the built-in [`ExpressionFunctions`] dispatcher for function calls.
use crate::domain::{
    errors::EvalError,
    services::ExpressionFunctions,
    value_objects::{
        ComparisonOperator, ContextValue, EvaluationContext, Expression, LiteralValue,
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

    fn eval_literal(&self, lit: &LiteralValue) -> Result<ContextValue, EvalError> {
        match lit {
            LiteralValue::Boolean(b) => Ok(ContextValue::Boolean(*b)),
            LiteralValue::Null => Ok(ContextValue::Null),
            LiteralValue::Integer(n) => Ok(ContextValue::Integer(*n)),
            LiteralValue::Float(f) => self.finite_decimal(*f),
            LiteralValue::String(s) => Ok(ContextValue::Text(s.clone())),
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
mod tests {
    use super::*;

    fn ctx() -> EvaluationContext {
        EvaluationContext::new()
    }

    fn eval(expr: &Expression) -> Result<ContextValue, EvalError> {
        let c = ctx();
        ExpressionEvaluator::new(&c).evaluate(expr)
    }

    #[test]
    fn eval_literal_bool_true() {
        let result = eval(&Expression::Literal(LiteralValue::Boolean(true))).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_literal_bool_false() {
        let result = eval(&Expression::Literal(LiteralValue::Boolean(false))).unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn eval_literal_int() {
        let result = eval(&Expression::Literal(LiteralValue::Integer(42))).unwrap();
        assert_eq!(result, ContextValue::Integer(42));
    }

    #[test]
    fn eval_literal_int_negative() {
        let result = eval(&Expression::Literal(LiteralValue::Integer(-7))).unwrap();
        assert_eq!(result, ContextValue::Integer(-7));
    }

    #[test]
    fn eval_literal_float() {
        let result = eval(&Expression::Literal(LiteralValue::Float(
            std::f64::consts::PI,
        )))
        .unwrap();
        assert_eq!(result, ContextValue::Decimal(std::f64::consts::PI));
    }

    #[test]
    fn eval_literal_string() {
        let result = eval(&Expression::Literal(LiteralValue::String("hello".into()))).unwrap();
        assert_eq!(result, ContextValue::text("hello"));
    }

    #[test]
    fn eval_literal_null() {
        let result = eval(&Expression::Literal(LiteralValue::Null)).unwrap();
        assert_eq!(result, ContextValue::Null);
    }

    #[test]
    fn eval_variable_stub() {
        let result = eval(&Expression::Variable("foo".into())).unwrap();
        assert_eq!(result, ContextValue::text("${{ foo }}"));
    }

    #[test]
    fn eval_variable_stub_env() {
        let result = eval(&Expression::Variable("bar".into())).unwrap();
        assert_eq!(result, ContextValue::text("${{ bar }}"));
    }

    #[test]
    fn eval_property_access_on_non_object() {
        let result = eval(&Expression::PropertyAccess(
            Box::new(Expression::Literal(LiteralValue::String("hello".into()))),
            "length".into(),
        ));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_property_access_on_object() {
        let json_str = Expression::Literal(LiteralValue::String(r#"{"x": 10}"#.into()));
        let from_json = Expression::FunctionCall("fromJson".into(), vec![json_str]);
        let prop_access = Expression::PropertyAccess(Box::new(from_json), "x".into());
        let result = eval(&prop_access).unwrap();
        assert_eq!(result, ContextValue::Integer(10));
    }

    #[test]
    fn eval_index_access_array() {
        let json_str = Expression::Literal(LiteralValue::String(r#"["a", "b", "c"]"#.into()));
        let from_json = Expression::FunctionCall("fromJson".into(), vec![json_str]);
        let idx = Expression::IndexAccess(
            Box::new(from_json),
            Box::new(Expression::Literal(LiteralValue::Integer(1))),
        );
        let result = eval(&idx).unwrap();
        assert_eq!(result, ContextValue::text("b"));
    }

    #[test]
    fn eval_index_access_object() {
        let json_str = Expression::Literal(LiteralValue::String(r#"{"key": "val"}"#.into()));
        let from_json = Expression::FunctionCall("fromJson".into(), vec![json_str]);
        let idx = Expression::IndexAccess(
            Box::new(from_json),
            Box::new(Expression::Literal(LiteralValue::String("key".into()))),
        );
        let result = eval(&idx).unwrap();
        assert_eq!(result, ContextValue::text("val"));
    }

    #[test]
    fn eval_index_access_type_error() {
        let idx = Expression::IndexAccess(
            Box::new(Expression::Literal(LiteralValue::String("hello".into()))),
            Box::new(Expression::Literal(LiteralValue::Integer(0))),
        );
        let result = eval(&idx);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_array_deref_stub() {
        let inner = Expression::Literal(LiteralValue::Integer(99));
        let deref = Expression::ArrayDereference(Box::new(inner));
        let result = eval(&deref).unwrap();
        assert_eq!(result, ContextValue::Integer(99));
    }

    #[test]
    fn eval_not_true() {
        let expr = Expression::Not(Box::new(Expression::Literal(LiteralValue::Boolean(true))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn eval_not_false() {
        let expr = Expression::Not(Box::new(Expression::Literal(LiteralValue::Boolean(false))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_not_empty_string() {
        let expr = Expression::Not(Box::new(Expression::Literal(LiteralValue::String(
            String::new(),
        ))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_not_non_empty_string() {
        let expr = Expression::Not(Box::new(Expression::Literal(LiteralValue::String(
            "hi".into(),
        ))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(false));
    }

    #[test]
    fn eval_not_null() {
        let expr = Expression::Not(Box::new(Expression::Literal(LiteralValue::Null)));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_not_zero() {
        let expr = Expression::Not(Box::new(Expression::Literal(LiteralValue::Integer(0))));
        let result = eval(&expr).unwrap();
        assert_eq!(result, ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_eq_numbers_true() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_eq_numbers_false() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
            Box::new(Expression::Literal(LiteralValue::Integer(3))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn eval_compare_neq_numbers() {
        let expr = Expression::Comparison(
            ComparisonOperator::NotEqual,
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
            Box::new(Expression::Literal(LiteralValue::Integer(3))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_lt_numbers_true() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(LiteralValue::Integer(2))),
            Box::new(Expression::Literal(LiteralValue::Integer(10))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_lt_numbers_false() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(LiteralValue::Integer(10))),
            Box::new(Expression::Literal(LiteralValue::Integer(2))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn eval_compare_gt_numbers() {
        let expr = Expression::Comparison(
            ComparisonOperator::GreaterThan,
            Box::new(Expression::Literal(LiteralValue::Integer(10))),
            Box::new(Expression::Literal(LiteralValue::Integer(2))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_lte_numbers_equal() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThanOrEqual,
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_lte_numbers_less() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThanOrEqual,
            Box::new(Expression::Literal(LiteralValue::Integer(3))),
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_gte_numbers() {
        let expr = Expression::Comparison(
            ComparisonOperator::GreaterThanOrEqual,
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_strings_eq() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(LiteralValue::String("abc".into()))),
            Box::new(Expression::Literal(LiteralValue::String("abc".into()))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_strings_lt() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(LiteralValue::String("abc".into()))),
            Box::new(Expression::Literal(LiteralValue::String("xyz".into()))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_bools() {
        let expr = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(LiteralValue::Boolean(false))),
            Box::new(Expression::Literal(LiteralValue::Boolean(true))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_compare_type_mismatch() {
        let expr = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(LiteralValue::Integer(1))),
            Box::new(Expression::Literal(LiteralValue::String("1".into()))),
        );
        let result = eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_logical_and_both_truthy() {
        let expr = Expression::Logical(
            LogicalOperator::And,
            Box::new(Expression::Literal(LiteralValue::Boolean(true))),
            Box::new(Expression::Literal(LiteralValue::Integer(42))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Integer(42));
    }

    #[test]
    fn eval_logical_and_short_circuit() {
        let expr = Expression::Logical(
            LogicalOperator::And,
            Box::new(Expression::Literal(LiteralValue::Boolean(false))),
            Box::new(Expression::Literal(LiteralValue::String("never".into()))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn eval_logical_or_both_falsy() {
        let expr = Expression::Logical(
            LogicalOperator::Or,
            Box::new(Expression::Literal(LiteralValue::Boolean(false))),
            Box::new(Expression::Literal(LiteralValue::Integer(0))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Integer(0));
    }

    #[test]
    fn eval_logical_or_short_circuit() {
        let expr = Expression::Logical(
            LogicalOperator::Or,
            Box::new(Expression::Literal(LiteralValue::String("first".into()))),
            Box::new(Expression::Literal(LiteralValue::String("never".into()))),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::text("first"));
    }

    #[test]
    fn eval_func_call_contains_true() {
        let expr = Expression::FunctionCall(
            "contains".into(),
            vec![
                Expression::Literal(LiteralValue::String("Hello World".into())),
                Expression::Literal(LiteralValue::String("world".into())),
            ],
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_func_call_contains_false() {
        let expr = Expression::FunctionCall(
            "contains".into(),
            vec![
                Expression::Literal(LiteralValue::String("Hello World".into())),
                Expression::Literal(LiteralValue::String("xyz".into())),
            ],
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(false));
    }

    #[test]
    fn eval_func_call_unknown() {
        let expr = Expression::FunctionCall("nonexistent".into(), vec![]);
        let result = eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_nested_logical_with_compare() {
        let left_cmp = Expression::Comparison(
            ComparisonOperator::GreaterThan,
            Box::new(Expression::Literal(LiteralValue::Integer(5))),
            Box::new(Expression::Literal(LiteralValue::Integer(3))),
        );
        let right_cmp = Expression::Comparison(
            ComparisonOperator::LessThan,
            Box::new(Expression::Literal(LiteralValue::Integer(10))),
            Box::new(Expression::Literal(LiteralValue::Integer(20))),
        );
        let expr = Expression::Logical(
            LogicalOperator::And,
            Box::new(left_cmp),
            Box::new(right_cmp),
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_nested_not_compare() {
        let cmp = Expression::Comparison(
            ComparisonOperator::Equal,
            Box::new(Expression::Literal(LiteralValue::Integer(1))),
            Box::new(Expression::Literal(LiteralValue::Integer(2))),
        );
        let expr = Expression::Not(Box::new(cmp));
        assert_eq!(eval(&expr).unwrap(), ContextValue::Boolean(true));
    }

    #[test]
    fn eval_variable_from_context() {
        let expr = Expression::Variable("github".into());
        let c = ctx();
        let result = ExpressionEvaluator::new(&c).evaluate(&expr).unwrap();
        assert!(result.is_mapping());
    }

    #[test]
    fn eval_property_access_missing_key_errors() {
        let json_str = Expression::Literal(LiteralValue::String(r#"{"x": 10}"#.into()));
        let from_json = Expression::FunctionCall("fromJson".into(), vec![json_str]);
        let prop_access = Expression::PropertyAccess(Box::new(from_json), "missing".into());
        let result = eval(&prop_access);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_index_access_negative_index_errors() {
        let arr = Expression::FunctionCall(
            "fromJson".into(),
            vec![Expression::Literal(LiteralValue::String("[1,2,3]".into()))],
        );
        let idx = Expression::Literal(LiteralValue::Integer(-1));
        let expr = Expression::IndexAccess(Box::new(arr), Box::new(idx));
        let result = eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn eval_index_access_out_of_bounds_errors() {
        let arr = Expression::FunctionCall(
            "fromJson".into(),
            vec![Expression::Literal(LiteralValue::String("[1,2,3]".into()))],
        );
        let idx = Expression::Literal(LiteralValue::Integer(10));
        let expr = Expression::IndexAccess(Box::new(arr), Box::new(idx));
        let result = eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::TypeError(_)));
    }

    #[test]
    fn from_json_round_trips_through_to_json_with_sorted_keys() {
        let source = Expression::Literal(LiteralValue::String(
            r#"{"b":[1,2.5,true,null],"a":"x"}"#.into(),
        ));
        let parsed = Expression::FunctionCall("fromJson".into(), vec![source]);
        let rendered = Expression::FunctionCall("toJson".into(), vec![parsed]);
        assert_eq!(
            eval(&rendered).unwrap(),
            ContextValue::text(r#"{"a":"x","b":[1,2.5,true,null]}"#)
        );
    }

    #[test]
    fn format_renders_a_parsed_list_as_compact_json() {
        let source = Expression::Literal(LiteralValue::String("[1,2]".into()));
        let parsed = Expression::FunctionCall("fromJson".into(), vec![source]);
        let expr = Expression::FunctionCall(
            "format".into(),
            vec![
                Expression::Literal(LiteralValue::String("{0}".into())),
                parsed,
            ],
        );
        assert_eq!(eval(&expr).unwrap(), ContextValue::text("[1,2]"));
    }
}
