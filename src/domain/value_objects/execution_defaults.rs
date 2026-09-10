use crate::domain::value_objects::RunStepDefaults;

/// Default settings for all jobs in a workflow.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExecutionDefaults {
    run: Option<RunStepDefaults>,
}

impl ExecutionDefaults {
    pub fn new(run: Option<RunStepDefaults>) -> Self {
        Self { run }
    }

    pub fn run(&self) -> Option<&RunStepDefaults> {
        self.run.as_ref()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_run_defaults() {
        let defaults =
            ExecutionDefaults::new(Some(RunStepDefaults::new(Some("bash".into()), None)));

        assert_eq!(
            defaults.run().and_then(RunStepDefaults::shell),
            Some("bash")
        );
    }
}
