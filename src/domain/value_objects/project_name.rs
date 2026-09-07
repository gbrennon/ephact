use std::fmt;

use crate::domain::errors::project_branding_error::ProjectBrandingError;

/// Value object representing a validated non-empty project name.
///
/// Encapsulates the project name string while maintaining the invariant that
/// the contained name cannot be empty.
///
/// # Examples
///
/// ```
/// # use ephact::domain::value_objects::ProjectName;
/// let name: ProjectName = ProjectName::new("ephact".to_string()).unwrap();
/// assert_eq!(name.as_str(), "ephact");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectName {
    value: String,
}

impl ProjectName {
    /// Creates a validated project name from a string.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectBrandingError::EmptyName`] if the provided string is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ephact::domain::value_objects::ProjectName;
    /// # use ephact::domain::errors::ProjectBrandingError;
    /// let valid: Result<ProjectName, ProjectBrandingError> =
    ///     ProjectName::new("ephact".to_string());
    /// assert!(valid.is_ok());
    ///
    /// let invalid: Result<ProjectName, ProjectBrandingError> =
    ///     ProjectName::new("".to_string());
    /// assert_eq!(invalid, Err(ProjectBrandingError::EmptyName));
    /// ```
    pub fn new(value: String) -> Result<Self, ProjectBrandingError> {
        match value.is_empty() {
            true => Err(ProjectBrandingError::EmptyName),
            false => Ok(Self { value }),
        }
    }

    /// Returns the project name as an immutable string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for ProjectName {
    /// Formats the project name using its inner string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
#[test]
fn new_with_valid_name_succeeds() {
    let name = ProjectName::new("my-project".to_string()).unwrap();
    assert_eq!(name.as_str(), "my-project");
}

#[cfg(test)]
#[test]
fn new_with_empty_name_returns_empty_name_error() {
    let result = ProjectName::new("".to_string());
    assert_eq!(result, Err(ProjectBrandingError::EmptyName));
}

#[cfg(test)]
#[test]
fn display_formats_inner_string() {
    let name = ProjectName::new("my-project".to_string()).unwrap();
    assert_eq!(format!("{name}"), "my-project");
}
