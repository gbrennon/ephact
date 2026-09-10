use crate::domain::entities::JobRun;

/// A stage is a group of runs that execute in parallel.
///
/// Stages are separated by dependency boundaries: all runs in a stage
/// must complete before the next stage begins.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionStage {
    /// The runs in this stage (execute in parallel).
    runs: Vec<JobRun>,
}

impl ExecutionStage {
    #[must_use]
    pub fn new(runs: Vec<JobRun>) -> Self {
        Self { runs }
    }

    #[must_use]
    pub fn runs(&self) -> &[JobRun] {
        &self.runs
    }

    #[must_use]
    pub fn into_runs(self) -> Vec<JobRun> {
        self.runs
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_and_consumes_runs() {
        let stage = ExecutionStage::new(Vec::new());

        assert!(stage.runs().is_empty());
        assert!(stage.into_runs().is_empty());
    }
}
