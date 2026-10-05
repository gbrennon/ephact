use crate::{
    domain::messages::commands::ExecuteJobPayload, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

/// Publishes job execution commands.
pub trait JobCommandPublisherPort: Send + Sync {
    /// Routes `command` to a job handler and returns its execution response.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteJobError`] when dispatch or job execution fails.
    fn publish(&self, command: ExecuteJobPayload) -> Result<JobExecutionResponse, ExecuteJobError>;
}
