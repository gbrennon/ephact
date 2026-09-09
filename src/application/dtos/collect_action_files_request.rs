use std::path::Path;

/// Request DTO for the
/// [`CollectActionFilesPort`](crate::application::ports::inbound::collect_action_files_port::CollectActionFilesPort)
/// inbound port.
pub struct CollectActionFilesRequest<'a> {
    /// Directory whose files are collected.
    pub action_dir: &'a Path,
}

impl<'a> CollectActionFilesRequest<'a> {
    /// Creates a new request.
    pub fn new(action_dir: &'a Path) -> Self {
        Self { action_dir }
    }

    /// Directory whose files are collected.
    pub fn action_dir(&self) -> &'a Path {
        self.action_dir
    }
}
