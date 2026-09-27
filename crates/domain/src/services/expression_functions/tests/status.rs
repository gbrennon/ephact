#[cfg(test)]
mod tests {
    use super::super::*;

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
