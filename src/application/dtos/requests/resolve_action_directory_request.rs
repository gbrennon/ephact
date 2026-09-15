use std::path::{Path, PathBuf};

/// Request DTO for the
/// [`ResolveActionDirectoryPort`](crate::application::ports::inbound::resolve_action_directory_port::ResolveActionDirectoryPort)
/// inbound port.
pub struct ResolveActionDirectoryRequest {
    /// The `uses:` value naming the action.
    action_ref: String,
    /// Root of the repository under test.
    repo_path: PathBuf,
}

impl ResolveActionDirectoryRequest {
    /// Creates a new request.
    pub fn new(action_ref: String, repo_path: PathBuf) -> Self {
        Self {
            action_ref,
            repo_path,
        }
    }

    /// The `uses:` value naming the action.
    pub fn action_ref(&self) -> &str {
        &self.action_ref
    }

    /// Root of the repository under test.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}
