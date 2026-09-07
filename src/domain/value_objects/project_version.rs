use std::fmt;

use crate::domain::errors::project_branding_error::ProjectBrandingError;

/// Value object representing a validated non-empty project version.
///
/// Encapsulates the project version string while maintaining the invariant that
/// the contained version cannot be empty.
///
/// # Examples
///
/// ```
/// # use ephact::domain::value_objects::ProjectVersion;
/// let version: ProjectVersion = ProjectVersion::new("0.1.0".to_string()).unwrap();
/// assert_eq!(version.as_str(), "0.1.0");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectVersion {
    value: String,
}

impl ProjectVersion {
    /// Creates a validated project version from a string.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectBrandingError::EmptyVersion`] if the provided string is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ephact::domain::value_objects::ProjectVersion;
    /// # use ephact::domain::errors::ProjectBrandingError;
    /// let valid: Result<ProjectVersion, ProjectBrandingError> =
    ///     ProjectVersion::new("0.1.0".to_string());
    /// assert!(valid.is_ok());
    ///
    /// let invalid: Result<ProjectVersion, ProjectBrandingError> =
    ///     ProjectVersion::new("".to_string());
    /// assert_eq!(invalid, Err(ProjectBrandingError::EmptyVersion));
    /// ```
    pub fn new(value: String) -> Result<Self, ProjectBrandingError> {
        match value.is_empty() {
            true => Err(ProjectBrandingError::EmptyVersion),
            false => Ok(Self { value }),
        }
    }

    /// Returns the project version as an immutable string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for ProjectVersion {
    /// Formats the project version using its inner string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
#[test]
fn new_with_valid_version_succeeds() {
    let version = ProjectVersion::new("0.1.0".to_string()).unwrap();
    assert_eq!(version.as_str(), "0.1.0");
}

#[cfg(test)]
#[test]
fn new_with_empty_version_returns_empty_version_error() {
    let result = ProjectVersion::new("".to_string());
    assert_eq!(result, Err(ProjectBrandingError::EmptyVersion));
}

#[cfg(test)]
#[test]
fn display_formats_inner_string() {
    let version = ProjectVersion::new("0.1.0".to_string()).unwrap();
    assert_eq!(format!("{version}"), "0.1.0");
}
