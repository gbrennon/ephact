use std::fmt;

use crate::errors::core_error::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryName(String);

impl RepositoryName {
    /// Creates a repository name from a string.
    pub fn new(name: String) -> Result<Self, CoreError> {
        if name.is_empty() {
            Err(CoreError::EmptyRepositoryName)
        } else {
            Ok(Self(name))
        }
    }

    /// Derives a name from the final component of a repository location.
    pub fn from_repo_path(repo_path: &super::repo_path::RepoPath) -> Result<Self, CoreError> {
        let name = repo_path
            .as_path()
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("")
            .to_owned();
        Self::new(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepositoryName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::repo_path::RepoPath, *};

    #[test]
    fn new_with_valid_name_succeeds() {
        let name = RepositoryName::new("my-repo".into()).unwrap();
        assert_eq!(name.as_str(), "my-repo");
    }

    #[test]
    fn new_with_empty_name_returns_empty_repository_name() {
        let result = RepositoryName::new("".into());
        assert_eq!(result, Err(CoreError::EmptyRepositoryName));
    }

    #[test]
    fn from_repo_path_derives_name_from_path() {
        let repo_path = RepoPath::new("workspace").unwrap();
        let name = RepositoryName::from_repo_path(&repo_path).unwrap();
        assert_eq!(name.as_str(), "workspace");
    }

    #[test]
    fn display_formats_inner_string() {
        let name = RepositoryName::new("my-repo".into()).unwrap();
        assert_eq!(format!("{name}"), "my-repo");
    }

    #[test]
    fn as_str_returns_inner() {
        let name = RepositoryName::new("my-repo".into()).unwrap();
        assert_eq!(name.as_str(), "my-repo");
    }
}
