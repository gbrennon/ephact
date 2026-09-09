use super::run::Run;

/// A stage is a group of runs that execute in parallel.
///
/// Stages are separated by dependency boundaries: all runs in a stage
/// must complete before the next stage begins.
#[derive(Debug, Clone, PartialEq)]
pub struct Stage {
    /// The runs in this stage (execute in parallel).
    runs: Vec<Run>,
}

impl Stage {
    #[must_use]
    pub fn new(runs: Vec<Run>) -> Self {
        Self { runs }
    }

    #[must_use]
    pub fn runs(&self) -> &[Run] {
        &self.runs
    }

    #[must_use]
    pub fn into_runs(self) -> Vec<Run> {
        self.runs
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_and_consumes_runs() {
        let stage = Stage::new(Vec::new());

        assert!(stage.runs().is_empty());
        assert!(stage.into_runs().is_empty());
    }
}
