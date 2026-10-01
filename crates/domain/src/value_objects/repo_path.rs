use std::path::{Path, PathBuf};

use crate::errors::core_error::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoPath {
    path: PathBuf,
}

impl RepoPath {
    /// Creates a repository location from a non-empty path value.
    ///
    /// Filesystem resolution and repository-format validation belong to the
    /// infrastructure adapter that obtains this value, not to the domain.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, CoreError> {
        let path = path.into();
        if path.as_os_str().is_empty() {
            return Err(CoreError::InvalidRepositoryPath(
                "repository path cannot be empty".to_owned(),
            ));
        }
        Ok(Self { path })
    }

    /// Returns the path supplied by the boundary.
    pub fn as_path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_path() {
        let path = PathBuf::from("workspace");
        let repo_path = RepoPath::new(&path).unwrap();

        assert_eq!(repo_path.as_path(), path);
    }

    #[test]
    fn new_rejects_empty_path() {
        let result = RepoPath::new(PathBuf::new());

        assert_eq!(
            result,
            Err(CoreError::InvalidRepositoryPath(
                "repository path cannot be empty".to_owned(),
            ))
        );
    }
}
