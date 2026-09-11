use std::path::PathBuf;

use crate::application::dtos::responses::ExecuteActionResponse;

/// Where an action's files live, or why running it needs no work.
#[derive(Debug)]
pub enum ResolvedActionDirectoryResponse {
    /// Directory on the host holding the action to run.
    Directory(PathBuf),
    /// The action needs no work; this response is the step's result.
    Skipped(ExecuteActionResponse),
}
