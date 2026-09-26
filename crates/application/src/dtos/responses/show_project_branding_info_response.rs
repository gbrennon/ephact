#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowProjectBrandingInfoResponse {
    name: String,
    description: String,
    version: String,
    emblem: String,
}

impl ShowProjectBrandingInfoResponse {
    pub fn new(name: String, description: String, version: String, emblem: String) -> Self {
        Self {
            name,
            description,
            version,
            emblem,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_name(self) -> String {
        self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn into_description(self) -> String {
        self.description
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn into_version(self) -> String {
        self.version
    }

    pub fn emblem(&self) -> &str {
        &self.emblem
    }

    pub fn into_emblem(self) -> String {
        self.emblem
    }
}
