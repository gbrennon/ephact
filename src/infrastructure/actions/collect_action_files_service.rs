use super::collect_action_files_port::CollectActionFilesPort;
use std::{
    fs::{read, read_dir},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use crate::application::dtos::CollectActionFilesRequest;
use crate::application::dtos::CollectActionFilesResponse;
use crate::application::dtos::FileEntry;
use crate::domain::errors::StepError;

/// Directory never copied into the container along with an action.
const GIT_DIRECTORY: &str = ".git";

/// Service that reads every file making up an action, so it can be copied into
/// the container that runs it.
pub struct CollectActionFilesService;

impl CollectActionFilesService {
    pub fn new() -> Self {
        Self
    }

    /// Walks `directory`, reading each file it holds into `files`.
    fn file_mode(path: &Path) -> u32 {
        path.metadata()
            .map(|metadata| metadata.permissions().mode() & 0o7777)
            .unwrap_or(0o644)
    }

    fn read_file_entry(root: &Path, path: &Path) -> Result<FileEntry, StepError> {
        let relative = path
            .strip_prefix(root)
            .map_err(|error| StepError::new(format!("action file outside action: {error}")))?;
        let content = read(path).map_err(|error| {
            StepError::new(format!("failed to read {}: {error}", path.display()))
        })?;
        let mode = Self::file_mode(path);

        Ok(FileEntry::new(relative.display().to_string(), content, mode))
    }

    fn process_entry(
        root: &Path,
        path: PathBuf,
        files: &mut Vec<FileEntry>,
    ) -> Result<(), StepError> {
        if path.file_name().is_some_and(|name| name == GIT_DIRECTORY) {
            return Ok(());
        }
        if path.is_dir() {
            return Self::collect_files_into(root, &path, files);
        }
        files.push(Self::read_file_entry(root, &path)?);
        Ok(())
    }

    /// Walks `directory`, reading each file it holds into `files`.
    fn collect_files_into(
        root: &Path,
        directory: &Path,
        files: &mut Vec<FileEntry>,
    ) -> Result<(), StepError> {
        let listing = read_dir(directory).map_err(|error| {
            StepError::new(format!(
                "failed to read action directory {}: {error}",
                directory.display()
            ))
        })?;

        for entry in listing {
            let path: PathBuf = entry
                .map_err(|error| StepError::new(format!("failed to read action entry: {error}")))?
                .path();
            Self::process_entry(root, path, files)?;
        }

        Ok(())
    }
}

impl Default for CollectActionFilesService {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectActionFilesPort for CollectActionFilesService {
    fn execute(
        &self,
        request: CollectActionFilesRequest<'_>,
    ) -> Result<CollectActionFilesResponse, StepError> {
        let mut files = Vec::new();
        Self::collect_files_into(request.action_dir(), request.action_dir(), &mut files)?;
        Ok(CollectActionFilesResponse::new(files))
    }
}
