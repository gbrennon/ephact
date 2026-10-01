use std::{
    fmt,
    path::{Path, PathBuf},
};

use crate::domain::{RepoPath, Repository, RepositoryName};

#[derive(Debug)]
pub enum RepositoryResolutionError {
    InvalidPath { path: PathBuf, reason: String },
    NotARepository(PathBuf),
    InvalidName(String),
}

impl fmt::Display for RepositoryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath { path, reason } => {
                write!(
                    formatter,
                    "cannot use repository path '{}': {reason}",
                    path.display()
                )
            }
            Self::NotARepository(path) => {
                write!(formatter, "'{}' is not a repository", path.display())
            }
            Self::InvalidName(error) => formatter.write_str(error),
        }
    }
}

impl std::error::Error for RepositoryResolutionError {}

pub struct RepositoryResolver;

impl RepositoryResolver {
    pub fn resolve(
        path: impl Into<PathBuf>,
        name: impl Into<String>,
    ) -> Result<Repository, RepositoryResolutionError> {
        let canonical = Self::canonical_repository_path(&path.into())?;
        Self::from_canonical_path(canonical, name)
    }

    pub fn resolve_from_path(
        path: impl Into<PathBuf>,
    ) -> Result<Repository, RepositoryResolutionError> {
        let canonical = Self::canonical_repository_path(&path.into())?;
        let name = canonical
            .file_name()
            .and_then(|value| value.to_str())
            .map(str::to_owned)
            .ok_or_else(|| {
                RepositoryResolutionError::InvalidName(canonical.display().to_string())
            })?;
        Self::from_canonical_path(canonical, name)
    }

    fn canonical_repository_path(path: &Path) -> Result<PathBuf, RepositoryResolutionError> {
        let canonical =
            path.canonicalize()
                .map_err(|error| RepositoryResolutionError::InvalidPath {
                    path: path.to_path_buf(),
                    reason: error.to_string(),
                })?;
        Self::validate_directory(&canonical)?;
        Self::validate_repository_marker(&canonical)?;
        Ok(canonical)
    }

    fn validate_directory(path: &Path) -> Result<(), RepositoryResolutionError> {
        if path.is_dir() {
            Ok(())
        } else {
            Err(RepositoryResolutionError::InvalidPath {
                path: path.to_path_buf(),
                reason: "path is not a directory".to_owned(),
            })
        }
    }

    fn validate_repository_marker(path: &Path) -> Result<(), RepositoryResolutionError> {
        if path.join(".git").is_dir() || path.join(".git").is_file() {
            Ok(())
        } else {
            Err(RepositoryResolutionError::NotARepository(
                path.to_path_buf(),
            ))
        }
    }

    fn from_canonical_path(
        path: PathBuf,
        name: impl Into<String>,
    ) -> Result<Repository, RepositoryResolutionError> {
        let repo_path =
            RepoPath::new(path).map_err(|error| RepositoryResolutionError::InvalidPath {
                path: PathBuf::new(),
                reason: error.to_string(),
            })?;
        let name = RepositoryName::new(name.into())
            .map_err(|error| RepositoryResolutionError::InvalidName(error.to_string()))?;
        Ok(Repository::new(repo_path, name))
    }
}
