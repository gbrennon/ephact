use crate::entities::JobRun;

/// A group of runs at the same dependency level in a workflow plan.
///
/// The execution service processes stages in dependency order; this value does
/// not promise concurrent execution of its runs.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionStage {
    /// The runs in this dependency level.
    runs: Vec<JobRun>,
}

impl ExecutionStage {
    pub fn new(runs: Vec<JobRun>) -> Self {
        Self { runs }
    }

    pub fn runs(&self) -> &[JobRun] {
        &self.runs
    }

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
