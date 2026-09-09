use super::stage::Stage;

/// A plan is the complete execution order for a workflow.
///
/// Stages execute sequentially; runs within a stage execute in parallel.
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    /// The ordered stages of execution.
    stages: Vec<Stage>,
}

impl Plan {
    #[must_use]
    pub fn new(stages: Vec<Stage>) -> Self {
        Self { stages }
    }

    #[must_use]
    pub fn stages(&self) -> &[Stage] {
        &self.stages
    }

    #[must_use]
    pub fn into_stages(self) -> Vec<Stage> {
        self.stages
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_and_consumes_stages() {
        let plan = Plan::new(Vec::new());

        assert!(plan.stages().is_empty());
        assert!(plan.into_stages().is_empty());
    }
}
