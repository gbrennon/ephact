use crate::dtos::responses::{JobSummaryResponse, WorkflowExecutionResponse};

/// Collects job summaries from workflow executions and qualifies their names.
#[derive(Debug, Default)]
pub struct WorkflowExecutionAggregator {
    _private: (),
}

impl WorkflowExecutionAggregator {
    /// Creates an aggregator with no execution state.
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Flattens workflow job summaries while preserving execution order.
    pub fn aggregate(&self, executions: &[WorkflowExecutionResponse]) -> Vec<JobSummaryResponse> {
        executions
            .iter()
            .flat_map(|execution| {
                execution
                    .job_summaries()
                    .iter()
                    .map(move |job| Self::qualify_job_summary(execution, job))
            })
            .collect()
    }

    fn qualify_job_summary(
        execution: &WorkflowExecutionResponse,
        job: &JobSummaryResponse,
    ) -> JobSummaryResponse {
        let qualified = job
            .name()
            .map(|name| format!("{} / {}", execution.workflow_name(), name));
        job.clone().with_name(qualified)
    }
}
