use crate::domain::value_objects::JobMatrix;

/// A matrix strategy for generating multiple job runs.
///
/// Each combination of matrix variables produces a separate job run.
/// Supports `include`/`exclude` for fine-grained control.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use ephact::domain::value_objects::{ContextValue, JobMatrix, JobStrategy};
///
/// let matrix = JobMatrix::new(
///     HashMap::from([(
///         "os".to_owned(),
///         vec![ContextValue::text("ubuntu-latest"), ContextValue::text("macos-latest")],
///     )]),
///     Vec::new(),
///     Vec::new(),
/// );
/// let strategy = JobStrategy::new(Some(matrix), true, None);
///
/// assert_eq!(strategy.combination_count(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct JobStrategy {
    /// The matrix of variables to expand.
    matrix: Option<JobMatrix>,

    /// Whether to cancel all in-progress jobs if any matrix job fails.
    fail_fast: bool,

    /// Maximum number of jobs to run in parallel.
    max_parallel: Option<usize>,
}

impl JobStrategy {
    pub fn new(matrix: Option<JobMatrix>, fail_fast: bool, max_parallel: Option<usize>) -> Self {
        Self {
            matrix,
            fail_fast,
            max_parallel,
        }
    }

    pub fn matrix(&self) -> Option<&JobMatrix> {
        self.matrix.as_ref()
    }

    pub fn fail_fast(&self) -> bool {
        self.fail_fast
    }

    pub fn max_parallel(&self) -> Option<usize> {
        self.max_parallel
    }
}

impl JobStrategy {
    /// Returns `true` if this strategy has a matrix defined.
    pub fn has_matrix(&self) -> bool {
        self.matrix.is_some()
    }

    /// Returns the number of matrix combinations (before include/exclude).
    pub fn combination_count(&self) -> usize {
        self.matrix
            .as_ref()
            .map(|m| {
                if m.variables().is_empty() {
                    0
                } else {
                    m.variables().values().map(|v| v.len()).product()
                }
            })
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::ContextValue;

    fn matrix(variables: &[(&str, usize)]) -> JobMatrix {
        let variables = variables
            .iter()
            .map(|(name, count)| {
                let values = (0..*count)
                    .map(|index| ContextValue::text(format!("{name}-{index}")))
                    .collect();
                ((*name).to_owned(), values)
            })
            .collect();
        JobMatrix::new(variables, Vec::new(), Vec::new())
    }

    #[test]
    fn a_strategy_with_a_matrix_reports_its_combinations() {
        let strategy = JobStrategy::new(Some(matrix(&[("os", 2), ("rust", 2)])), false, Some(2));

        assert!(strategy.has_matrix());
        assert!(!strategy.fail_fast());
        assert_eq!(strategy.max_parallel(), Some(2));
        assert_eq!(strategy.combination_count(), 4);
    }

    #[test]
    fn an_empty_matrix_has_no_combinations() {
        let strategy = JobStrategy::new(Some(matrix(&[])), true, None);

        assert_eq!(strategy.combination_count(), 0);
    }

    #[test]
    fn a_strategy_without_a_matrix_is_a_single_run() {
        let strategy = JobStrategy::new(None, true, None);

        assert!(!strategy.has_matrix());
        assert_eq!(strategy.combination_count(), 1);
    }
}
