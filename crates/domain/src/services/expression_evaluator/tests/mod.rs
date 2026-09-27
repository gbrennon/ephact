use super::*;

fn ctx() -> EvaluationContext {
    EvaluationContext::new()
}

fn eval(expr: &Expression) -> Result<ContextValue, EvalError> {
    let context = ctx();
    ExpressionEvaluator::new(&context).evaluate(expr)
}

mod access;
mod comparisons;
mod functions;
mod literals_variables;
mod truth_and_logic;
