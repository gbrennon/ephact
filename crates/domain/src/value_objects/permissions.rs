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
