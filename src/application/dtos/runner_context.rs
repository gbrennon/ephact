use std::collections::HashMap;

/// Context describing the runner environment inside the container.
#[derive(Debug, Clone)]
pub struct RunnerContext {
    pub workspace: String,
    pub home: String,
    pub action_path: String,
    pub temp: String,
    pub tool_cache: String,
    pub env: HashMap<String, String>,
}

impl RunnerContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workspace: impl Into<String>,
        home: impl Into<String>,
        action_path: impl Into<String>,
        temp: impl Into<String>,
        tool_cache: impl Into<String>,
        env: HashMap<String, String>,
    ) -> Self {
        Self {
            workspace: workspace.into(),
            home: home.into(),
            action_path: action_path.into(),
            temp: temp.into(),
            tool_cache: tool_cache.into(),
            env,
        }
    }

    pub fn workspace(&self) -> &str {
        &self.workspace
    }

    pub fn home(&self) -> &str {
        &self.home
    }

    pub fn action_path(&self) -> &str {
        &self.action_path
    }

    pub fn temp(&self) -> &str {
        &self.temp
    }

    pub fn tool_cache(&self) -> &str {
        &self.tool_cache
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn env_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.env
    }

    pub fn with_env_extension(
        mut self,
        entries: impl IntoIterator<Item = (String, String)>,
    ) -> Self {
        self.env.extend(entries);
        self
    }
}

impl Default for RunnerContext {
    fn default() -> Self {
        Self {
            workspace: "/workspace".into(),
            home: "/home".into(),
            action_path: "/actions".into(),
            temp: "/tmp".into(),
            tool_cache: "/tool-cache".into(),
            env: HashMap::new(),
        }
    }
}
