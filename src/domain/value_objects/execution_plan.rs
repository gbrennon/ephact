use crate::domain::value_objects::ExecutionStage;

/// A plan is the complete execution order for a workflow.
///
/// Stages execute sequentially; runs within a stage execute in parallel.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionPlan {
    /// The ordered stages of execution.
    stages: Vec<ExecutionStage>,
}

impl ExecutionPlan {
    #[must_use]
    pub fn new(stages: Vec<ExecutionStage>) -> Self {
        Self { stages }
    }

    #[must_use]
    pub fn stages(&self) -> &[ExecutionStage] {
        &self.stages
    }

    #[must_use]
    pub fn into_stages(self) -> Vec<ExecutionStage> {
        self.stages
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_and_consumes_stages() {
        let plan = ExecutionPlan::new(Vec::new());

        assert!(plan.stages().is_empty());
        assert!(plan.into_stages().is_empty());
    }
}
