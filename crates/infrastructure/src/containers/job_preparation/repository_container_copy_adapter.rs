use std::{
    fs::{read, read_dir},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use crate::{
    application::{
        dtos::requests::CopyRepositoryToContainerRequest,
        errors::CopyRepositoryToContainerError,
        ports::outbound::{ContainerPort, CopyRepositoryToContainerPort},
    },
    domain::entities::FileEntry,
};

const EXCLUDED_DIRS: &[&str] = &[
    ".worktrees",
    "target",
    "node_modules",
    ".cargo",
    "dist",
    "build",
    ".idea",
    ".vscode",
    ".DS_Store",
];

pub struct RepositoryContainerCopyAdapter;

impl RepositoryContainerCopyAdapter {
    pub fn new() -> Self {
        Self
    }

    fn file_mode(path: &Path) -> u32 {
        path.metadata()
            .map(|metadata| metadata.permissions().mode() & 0o7777)
            .unwrap_or(0o644)
    }

    fn should_exclude(path: &Path, root: &Path) -> bool {
        path.strip_prefix(root)
            .ok()
            .and_then(|relative| relative.components().next())
            .and_then(|component| component.as_os_str().to_str())
            .is_some_and(|name| EXCLUDED_DIRS.contains(&name))
    }

    fn read_file_entry(
        root: &Path,
        path: &Path,
    ) -> Result<FileEntry, CopyRepositoryToContainerError> {
        let relative = path.strip_prefix(root).map_err(|error| {
            CopyRepositoryToContainerError::Filesystem(std::io::Error::other(error.to_string()))
        })?;
        let content = read(path).map_err(CopyRepositoryToContainerError::Filesystem)?;
        Ok(FileEntry::new(
            relative.display().to_string(),
            content,
            Self::file_mode(path),
        ))
    }

    fn process_entry(
        root: &Path,
        path: PathBuf,
    ) -> Result<Vec<FileEntry>, CopyRepositoryToContainerError> {
        if Self::should_exclude(&path, root) {
            return Ok(Vec::new());
        }
        if path.is_dir() {
            return Self::collect_files_into(root, &path);
        }
        Ok(vec![Self::read_file_entry(root, &path)?])
    }

    fn collect_files_into(
        root: &Path,
        directory: &Path,
    ) -> Result<Vec<FileEntry>, CopyRepositoryToContainerError> {
        read_dir(directory)
            .map_err(CopyRepositoryToContainerError::Filesystem)?
            .map(|entry| {
                entry
                    .map(|entry| entry.path())
                    .map_err(CopyRepositoryToContainerError::Filesystem)
            })
            .try_fold(Vec::new(), |files, path| {
                let path = path?;
                Self::process_entry(root, path)
                    .map(|entries| files.into_iter().chain(entries).collect())
            })
    }
}

impl Default for RepositoryContainerCopyAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl CopyRepositoryToContainerPort for RepositoryContainerCopyAdapter {
    fn copy(
        &self,
        request: CopyRepositoryToContainerRequest,
        container: &dyn ContainerPort,
    ) -> Result<(), CopyRepositoryToContainerError> {
        let files = Self::collect_files_into(request.repo_path(), request.repo_path())?;
        container
            .copy_to(request.container_path(), &files)
            .map_err(|error| CopyRepositoryToContainerError::Container(format!("{error:?}")))
    }
}
