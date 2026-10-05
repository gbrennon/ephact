use crate::value_objects::ExecutionStage;

/// A dependency-ordered execution plan for a workflow.
///
/// Stages are processed in order; this value describes dependency levels and
/// does not promise concurrent execution within a stage.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionPlan {
    /// The ordered stages of execution.
    stages: Vec<ExecutionStage>,
}

impl ExecutionPlan {
    pub fn new(stages: Vec<ExecutionStage>) -> Self {
        Self { stages }
    }

    pub fn stages(&self) -> &[ExecutionStage] {
        &self.stages
    }

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
