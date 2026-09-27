use crate::value_objects::RepositoryName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TempDirTemplate(String);

impl TempDirTemplate {
    pub fn from_repo_name(name: &RepositoryName) -> Self {
        Self(format!("ephact-run-{}-XXXXXX", name.as_str()))
    }

    /// Returns the template string (e.g. `ephact-run-my-repo-XXXXXX`).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
