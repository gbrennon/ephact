use std::error::Error;

use crate::{
    application::dtos::responses::JobExecutionResponse,
    domain::messages::commands::ExecuteJobCommand,
};

/// Outbound port for dispatching a job execution command.
///
/// Implementations route an [`ExecuteJobCommand`] to whatever produces its
/// outcome and return the resulting [`JobExecutionResponse`].
pub trait JobCommandBusPort: Send + Sync {
    /// Dispatches a job command and returns its outcome.
    ///
    /// # Errors
    ///
    /// Returns a boxed [`Error`] when the command cannot be dispatched or its
    /// execution fails.
    fn dispatch(&self, command: ExecuteJobCommand) -> Result<JobExecutionResponse, Box<dyn Error>>;
}
