use std::collections::HashMap;

use serde::Deserialize;

/// A matrix defining variable combinations for job expansion.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct Matrix {
    #[serde(flatten)]
    variables: HashMap<String, Vec<serde_yaml::Value>>,

    #[serde(default)]
    include: Vec<HashMap<String, serde_yaml::Value>>,

    #[serde(default)]
    exclude: Vec<HashMap<String, serde_yaml::Value>>,
}

impl Matrix {
    pub fn new(
        variables: HashMap<String, Vec<serde_yaml::Value>>,
        include: Vec<HashMap<String, serde_yaml::Value>>,
        exclude: Vec<HashMap<String, serde_yaml::Value>>,
    ) -> Self {
        Self {
            variables,
            include,
            exclude,
        }
    }

    pub fn variables(&self) -> &HashMap<String, Vec<serde_yaml::Value>> {
        &self.variables
    }

    pub fn include(&self) -> &[HashMap<String, serde_yaml::Value>] {
        &self.include
    }

    pub fn exclude(&self) -> &[HashMap<String, serde_yaml::Value>] {
        &self.exclude
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_variables_and_exceptions() {
        let matrix = Matrix::new(
            HashMap::from([("os".into(), vec![serde_yaml::Value::String("linux".into())])]),
            vec![HashMap::new()],
            vec![HashMap::new()],
        );

        assert_eq!(matrix.variables()["os"].len(), 1);
        assert_eq!(matrix.include().len(), 1);
        assert_eq!(matrix.exclude().len(), 1);
    }
}
