use std::collections::HashMap;

use crate::dtos::responses::RunnerContextPaths;

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
    pub fn with_env_extension(self, entries: impl IntoIterator<Item = (String, String)>) -> Self {
        Self {
            env: self.env.into_iter().chain(entries).collect(),
            ..self
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_extension_overwrites_existing_entries_and_preserves_others() {
        let context = RunnerContextResponse::default().with_env_extension([
            ("KEY".to_owned(), "base".to_owned()),
            ("KEEP".to_owned(), "value".to_owned()),
        ]);
        let extended = context.with_env_extension([
            ("KEY".to_owned(), "override".to_owned()),
            ("NEW".to_owned(), "new".to_owned()),
        ]);

        assert_eq!(
            extended.env().get("KEY").map(String::as_str),
            Some("override")
        );
        assert_eq!(
            extended.env().get("KEEP").map(String::as_str),
            Some("value")
        );
        assert_eq!(extended.env().get("NEW").map(String::as_str), Some("new"));
    }
}
