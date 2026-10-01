/// Coordinates an action definition in a remote source repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteActionReference {
    scheme: String,
    host: String,
    owner: String,
    repo: String,
    directory: Option<String>,
    revision: String,
}

impl RemoteActionReference {
    pub fn new(
        scheme: String,
        host: String,
        owner: String,
        repo: String,
        revision: String,
    ) -> Self {
        Self {
            scheme,
            host,
            owner,
            repo,
            directory: None,
            revision,
        }
    }

    pub fn with_directory(mut self, directory: Option<String>) -> Self {
        self.directory = directory;
        self
    }

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn repo(&self) -> &str {
        &self.repo
    }

    pub fn revision(&self) -> &str {
        &self.revision
    }

    pub fn directory(&self) -> Option<&str> {
        self.directory.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_for_test() -> RemoteActionReference {
        RemoteActionReference::new(
            "https".into(),
            "forge.example".into(),
            "actions".into(),
            "cache".into(),
            "v4".into(),
        )
    }

    #[test]
    fn accessors_expose_source_coordinates() {
        let reference = reference_for_test();

        assert_eq!(reference.scheme(), "https");
        assert_eq!(reference.host(), "forge.example");
        assert_eq!(reference.owner(), "actions");
        assert_eq!(reference.repo(), "cache");
        assert_eq!(reference.revision(), "v4");
    }

    #[test]
    fn directory_is_absent_for_root_actions() {
        assert_eq!(reference_for_test().directory(), None);
    }
}
