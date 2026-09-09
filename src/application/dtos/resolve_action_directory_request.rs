use std::path::Path;

/// Request DTO for the
/// [`ResolveActionDirectoryPort`](crate::application::ports::inbound::resolve_action_directory_port::ResolveActionDirectoryPort)
/// inbound port.
pub struct ResolveActionDirectoryRequest<'a> {
    /// The `uses:` value naming the action.
    action_ref: &'a str,
    /// Root of the repository under test.
    repo_path: &'a Path,
}

impl<'a> ResolveActionDirectoryRequest<'a> {
    /// Creates a new request.
    pub fn new(action_ref: &'a str, repo_path: &'a Path) -> Self {
        Self {
            action_ref,
            repo_path,
        }
    }

    /// The `uses:` value naming the action.
    pub fn action_ref(&self) -> &'a str {
        self.action_ref
    }

    /// Root of the repository under test.
    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }
}
