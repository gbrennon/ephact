/// Response containing project branding metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowProjectBrandingInfoResponse {
    name: String,
    description: String,
    version: String,
    emblem: String,
}

impl ShowProjectBrandingInfoResponse {
    /// Creates a response from the project name, description, version, and emblem.
    pub fn new(name: String, description: String, version: String, emblem: String) -> Self {
        Self {
            name,
            description,
            version,
            emblem,
        }
    }

    /// Returns the project name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Consumes the response and returns the project name.
    pub fn into_name(self) -> String {
        self.name
    }

    /// Returns the project description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Consumes the response and returns the project description.
    pub fn into_description(self) -> String {
        self.description
    }

    /// Returns the project version.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Consumes the response and returns the project version.
    pub fn into_version(self) -> String {
        self.version
    }

    /// Returns the project emblem.
    pub fn emblem(&self) -> &str {
        &self.emblem
    }

    /// Consumes the response and returns the project emblem.
    pub fn into_emblem(self) -> String {
        self.emblem
    }
}
