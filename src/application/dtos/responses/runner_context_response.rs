use crate::application::dtos::responses::RunnerContextPaths;
use std::collections::HashMap;

/// Context describing the runner environment inside the container.
#[derive(Debug, Clone)]
pub struct RunnerContextResponse {
    workspace: String,
    home: String,
    action_path: String,
    temp: String,
    tool_cache: String,
    env: HashMap<String, String>,
}

impl RunnerContextResponse {
    pub fn new(paths: RunnerContextPaths, env: HashMap<String, String>) -> Self {
        let (workspace, home, action_path, temp, tool_cache) = paths.into_parts();
        Self {
            workspace,
            home,
            action_path,
            temp,
            tool_cache,
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

impl Default for RunnerContextResponse {
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
