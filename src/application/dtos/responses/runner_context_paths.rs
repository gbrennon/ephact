#[derive(Debug, Clone)]
pub struct RunnerContextPaths {
    workspace: String,
    home: String,
    action_path: String,
    temp: String,
    tool_cache: String,
}

impl RunnerContextPaths {
    pub fn new(
        workspace: impl Into<String>,
        home: impl Into<String>,
        action_path: impl Into<String>,
        temp: impl Into<String>,
        tool_cache: impl Into<String>,
    ) -> Self {
        Self {
            workspace: workspace.into(),
            home: home.into(),
            action_path: action_path.into(),
            temp: temp.into(),
            tool_cache: tool_cache.into(),
        }
    }

    pub(crate) fn into_parts(self) -> (String, String, String, String, String) {
        (
            self.workspace,
            self.home,
            self.action_path,
            self.temp,
            self.tool_cache,
        )
    }
}
