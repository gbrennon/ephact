use std::collections::HashMap;

use crate::domain::value_objects::ContextValue;

/// A matrix defining variable combinations for job expansion.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct JobMatrix {
    variables: HashMap<String, Vec<ContextValue>>,
    include: Vec<HashMap<String, ContextValue>>,
    exclude: Vec<HashMap<String, ContextValue>>,
}

impl JobMatrix {
    pub fn new(
        variables: HashMap<String, Vec<ContextValue>>,
        include: Vec<HashMap<String, ContextValue>>,
        exclude: Vec<HashMap<String, ContextValue>>,
    ) -> Self {
        Self {
            variables,
            include,
            exclude,
        }
    }

    pub fn variables(&self) -> &HashMap<String, Vec<ContextValue>> {
        &self.variables
    }

    pub fn include(&self) -> &[HashMap<String, ContextValue>] {
        &self.include
    }

    pub fn exclude(&self) -> &[HashMap<String, ContextValue>] {
        &self.exclude
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_variables_and_exceptions() {
        let matrix = JobMatrix::new(
            HashMap::from([("os".into(), vec![ContextValue::text("linux")])]),
            vec![HashMap::new()],
            vec![HashMap::new()],
        );

        assert_eq!(matrix.variables()["os"].len(), 1);
        assert_eq!(matrix.include().len(), 1);
        assert_eq!(matrix.exclude().len(), 1);
    }
}
