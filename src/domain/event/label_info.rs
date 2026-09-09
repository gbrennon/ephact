use serde::Serialize;

/// Label information.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct LabelInfo {
    name: String,
    color: String,
}

impl LabelInfo {
    pub fn new(name: String, color: String) -> Self {
        Self { name, color }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> &str {
        &self.color
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let label = LabelInfo::new("bug".into(), "ff0000".into());

        assert_eq!(label.name(), "bug");
        assert_eq!(label.color(), "ff0000");
    }
}
