use crate::{
    domain::messages::commands::ExecuteJobPayload, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

/// Handles job execution commands.
pub trait JobCommandHandlerPort: Send + Sync {
    /// Executes `command` and returns its job summary and container name.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteJobError`] when job execution fails.
    fn handle(&self, command: ExecuteJobPayload) -> Result<JobExecutionResponse, ExecuteJobError>;
}
