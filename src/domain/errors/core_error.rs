use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// The provided repository path is invalid or does not exist.
    InvalidRepositoryPath(String),

    /// The provided path is not a git repository (no .git directory found).
    NotAGitRepository(String),

    /// A repository name was required but an empty string was provided.
    EmptyRepositoryName,

    /// An unknown container engine was specified (only "podman" or "docker" are supported).
    UnknownContainerEngine(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRepositoryPath(message) => formatter.write_str(message),
            Self::NotAGitRepository(path) => {
                write!(formatter, "'{path}' is not a Git repository")
            }
            Self::EmptyRepositoryName => formatter.write_str("repository name cannot be empty"),
            Self::UnknownContainerEngine(engine) => {
                write!(formatter, "unsupported container engine '{engine}'")
            }
        }
    }
}

impl std::error::Error for CoreError {}
