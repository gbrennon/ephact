use std::path::PathBuf;

use crate::{application::dtos::requests::FetchRemoteActionRequest, domain::errors::ActionError};

/// Inbound port for retrieving an action published on a forge.
pub trait FetchRemoteActionPort: Send + Sync {
    fn execute(&self, request: FetchRemoteActionRequest) -> Result<PathBuf, ActionError>;
}
