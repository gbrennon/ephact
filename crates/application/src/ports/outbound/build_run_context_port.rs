use crate::dtos::{requests::BuildRunContextRequest, responses::BuildRunContextResponse};

/// Builds the evaluation context for a workflow run.
pub trait BuildRunContextPort: Send + Sync {
    /// Returns a context containing the configured secrets and inputs together
    /// with repository, event, workspace, and runner information.
    /// The `github` context includes the repository name, event name, event
    /// inputs, and `/workspace` as the workspace; runner facts identify Linux,
    /// X64, and `/tmp` as the temporary directory.
    /// The event name defaults to `workflow_dispatch`.
    fn build(&self, request: BuildRunContextRequest) -> BuildRunContextResponse;
}
