#[cfg(test)]
mod tests {
    use super::super::*;

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
}
