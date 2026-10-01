/// Defaults used to complete an under-specified remote action coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReferenceDefaults {
    scheme: String,
    host: String,
    revision: String,
}

impl RemoteReferenceDefaults {
    pub fn new(scheme: String, host: String, revision: String) -> Self {
        Self {
            scheme,
            host,
            revision,
        }
    }

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn revision(&self) -> &str {
        &self.revision
    }
}
