use std::path::Path;

pub struct CopyRepositoryToContainerRequest<'a> {
    repo_path: &'a Path,
    container_path: &'a str,
}

impl<'a> CopyRepositoryToContainerRequest<'a> {
    pub fn new(repo_path: &'a Path, container_path: &'a str) -> Self {
        Self {
            repo_path,
            container_path,
        }
    }

    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }

    pub fn container_path(&self) -> &'a str {
        self.container_path
    }
}
