use crate::domain::value_objects::{
    ProjectDescription, ProjectEmblem, ProjectName, ProjectVersion,
};

/// Domain entity representing project branding information.
///
/// Encapsulates the visual and semantic identity of a project, consisting of
/// a validated name, description, version, and emblem identifier.
///
/// # Examples
///
/// ```
/// # use ephact::domain::entities::ProjectBranding;
/// # use ephact::domain::value_objects::{
/// #     ProjectDescription, ProjectEmblem, ProjectName, ProjectVersion,
/// # };
/// let name: ProjectName = ProjectName::new("ephact".to_string()).unwrap();
/// let description: ProjectDescription =
///     ProjectDescription::new("Ephemeral actions runner".to_string()).unwrap();
/// let version: ProjectVersion = ProjectVersion::new("0.1.0".to_string()).unwrap();
/// let emblem: ProjectEmblem = ProjectEmblem::new("shield".to_string()).unwrap();
///
/// let branding: ProjectBranding = ProjectBranding::new(name, description, version, emblem);
/// assert_eq!(branding.name().as_str(), "ephact");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectBranding {
    name: ProjectName,
    description: ProjectDescription,
    version: ProjectVersion,
    emblem: ProjectEmblem,
}

impl ProjectBranding {
    /// Creates a new project branding entity from validated value objects.
    ///
    /// # Arguments
    ///
    /// * `name` - Validated non-empty project name.
    /// * `description` - Validated non-empty project description.
    /// * `version` - Validated non-empty project version.
    /// * `emblem` - Validated non-empty project emblem.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ephact::domain::entities::ProjectBranding;
    /// # use ephact::domain::value_objects::{
    /// #     ProjectDescription, ProjectEmblem, ProjectName, ProjectVersion,
    /// # };
    /// let branding: ProjectBranding = ProjectBranding::new(
    ///     ProjectName::new("ephact".to_string()).unwrap(),
    ///     ProjectDescription::new("Ephemeral actions runner".to_string()).unwrap(),
    ///     ProjectVersion::new("0.1.0".to_string()).unwrap(),
    ///     ProjectEmblem::new("shield".to_string()).unwrap(),
    /// );
    /// assert_eq!(branding.version().as_str(), "0.1.0");
    /// ```
    pub fn new(
        name: ProjectName,
        description: ProjectDescription,
        version: ProjectVersion,
        emblem: ProjectEmblem,
    ) -> Self {
        Self {
            name,
            description,
            version,
            emblem,
        }
    }

    /// Returns an immutable reference to the project name.
    pub fn name(&self) -> &ProjectName {
        &self.name
    }

    /// Returns an immutable reference to the project description.
    pub fn description(&self) -> &ProjectDescription {
        &self.description
    }

    /// Returns an immutable reference to the project version.
    pub fn version(&self) -> &ProjectVersion {
        &self.version
    }

    /// Returns an immutable reference to the project emblem.
    pub fn emblem(&self) -> &ProjectEmblem {
        &self.emblem
    }
}

#[cfg(test)]
#[test]
fn new_assigns_all_fields_and_accessors_return_references() {
    let name = ProjectName::new("ephact".to_string()).unwrap();
    let description = ProjectDescription::new("Ephemeral actions runner".to_string()).unwrap();
    let version = ProjectVersion::new("0.1.0".to_string()).unwrap();
    let emblem = ProjectEmblem::new("shield".to_string()).unwrap();

    let branding = ProjectBranding::new(
        name.clone(),
        description.clone(),
        version.clone(),
        emblem.clone(),
    );

    assert_eq!(branding.name(), &name);
    assert_eq!(branding.description(), &description);
    assert_eq!(branding.version(), &version);
    assert_eq!(branding.emblem(), &emblem);
}
