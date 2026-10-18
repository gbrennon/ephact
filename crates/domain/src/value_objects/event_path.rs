use std::path::{Path, PathBuf};

use crate::errors::CoreError;

/// A user-supplied path containing a workflow event payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventPath {
    path: PathBuf,
}

impl EventPath {
    /// Creates an event payload path from a non-empty path value.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, CoreError> {
        let path = path.into();
        if path.as_os_str().is_empty() {
            return Err(CoreError::InvalidEventPath(
                "event path cannot be empty".to_owned(),
            ));
        }
        Ok(Self { path })
    }

    /// Returns the path supplied at the boundary.
    pub fn as_path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_path() {
        let path = PathBuf::from("fixtures/event.json");

        assert_eq!(EventPath::new(path.clone()).unwrap().as_path(), path);
    }

    #[test]
    fn new_rejects_empty_path() {
        assert_eq!(
            EventPath::new(PathBuf::new()),
            Err(CoreError::InvalidEventPath(
                "event path cannot be empty".to_owned(),
            ))
        );
    }
}
