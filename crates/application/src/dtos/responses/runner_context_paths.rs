/// Filesystem paths exposed to a runner context.
#[derive(Debug, Clone)]
pub struct RunnerContextPaths {
    workspace: String,
    home: String,
    action_path: String,
    temp: String,
    tool_cache: String,
}

impl RunnerContextPaths {
    /// Creates runner context paths from their configured locations.
    pub fn new(
        workspace: String,
        home: String,
        action_path: String,
        temp: String,
        tool_cache: String,
    ) -> Self {
        Self {
            workspace,
            home,
            action_path,
            temp,
            tool_cache,
        }
    }

    /// Consumes the paths and returns their owned values.
    pub fn into_parts(self) -> (String, String, String, String, String) {
        (
            self.workspace,
            self.home,
            self.action_path,
            self.temp,
            self.tool_cache,
        )
    }
}
