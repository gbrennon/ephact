use std::error::Error;

use crate::{
    application::dtos::responses::JobExecutionResponse,
    domain::messages::commands::ExecuteJobCommand,
};

/// Outbound port dispatching job execution commands.
///
/// Dispatches an [`ExecuteJobCommand`] to its command handler and returns the
/// handler's [`JobExecutionResponse`].
pub trait JobCommandBusPort: Send + Sync {
    /// Dispatches a job command and returns the handler's outcome.
    fn dispatch(&self, command: ExecuteJobCommand) -> Result<JobExecutionResponse, Box<dyn Error>>;
}
