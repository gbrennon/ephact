#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Permissions {
    allow_repo_writes: bool,
    allow_real_container: bool,
    allow_real_fetcher: bool,
    allow_network: bool,
}

impl Permissions {
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
    pub fn allow_real_container(&self) -> bool {
        self.allow_real_container
    }
    pub fn allow_real_fetcher(&self) -> bool {
        self.allow_real_fetcher
    }
    pub fn allow_network(&self) -> bool {
        self.allow_network
    }
    pub fn with_allow_repo_writes(mut self, value: bool) -> Self {
        self.allow_repo_writes = value;
        self
    }
    pub fn with_allow_real_container(mut self, value: bool) -> Self {
        self.allow_real_container = value;
        self
    }
    pub fn with_allow_real_fetcher(mut self, value: bool) -> Self {
        self.allow_real_fetcher = value;
        self
    }
    pub fn with_allow_network(mut self, value: bool) -> Self {
        self.allow_network = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_disables_every_permission() {
        let permissions = Permissions::default();

        assert!(!permissions.allow_repo_writes());
        assert!(!permissions.allow_real_container());
        assert!(!permissions.allow_real_fetcher());
        assert!(!permissions.allow_network());
    }

    #[test]
    fn with_allow_repo_writes_enables_repo_writes() {
        let permissions = Permissions::default().with_allow_repo_writes(true);

        assert!(permissions.allow_repo_writes());
    }

    #[test]
    fn with_allow_real_container_enables_real_container() {
        let permissions = Permissions::default().with_allow_real_container(true);

        assert!(permissions.allow_real_container());
    }

    #[test]
    fn with_allow_real_fetcher_enables_real_fetcher() {
        let permissions = Permissions::default().with_allow_real_fetcher(true);

        assert!(permissions.allow_real_fetcher());
    }

    #[test]
    fn with_allow_network_enables_network() {
        let permissions = Permissions::default().with_allow_network(true);

        assert!(permissions.allow_network());
    }
}
