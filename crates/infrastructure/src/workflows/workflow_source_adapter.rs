use std::{collections::BTreeSet, fs};

use super::{source_name::resolve_source_name, workflow_directories::WORKFLOW_DIRECTORIES};
use crate::{
    application::{
        dtos::responses::WorkflowSourceFileResponse, errors::WorkflowSourceError,
        ports::outbound::WorkflowSourcePort,
    },
    domain::entities::repository::Repository,
    workflows::workflow_document::WorkflowDocument,
};

/// Infrastructure adapter that reads workflow definitions from the filesystem.
///
/// This adapter is the concrete implementation of `WorkflowSourcePort` and lives
/// entirely in the infrastructure layer. It knows nothing about application
/// services or ports - it simply reads workflow files and returns their contents.
pub struct FilesystemWorkflowSource {
    workflow_dirs: Vec<String>,
}

impl FilesystemWorkflowSource {
    pub fn new(workflow_dirs: &[&str]) -> Self {
        Self {
            workflow_dirs: workflow_dirs.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn is_yaml_workflow(path: &std::path::Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "yml" || ext == "yaml")
            .unwrap_or(false)
    }

    fn collect_dir_workflows(
        workflows_dir: &std::path::Path,
        workflows: &mut Vec<std::path::PathBuf>,
    ) -> Result<(), WorkflowSourceError> {
        if !workflows_dir.exists() {
            return Ok(());
        }
        let entries = fs::read_dir(workflows_dir).map_err(WorkflowSourceError::Io)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if Self::is_yaml_workflow(&path) {
                workflows.push(path);
            }
        }
        Ok(())
    }

    fn find_workflow_files(
        &self,
        repo: &Repository,
    ) -> Result<Vec<std::path::PathBuf>, WorkflowSourceError> {
        let repo_path = repo.path().as_path();
        let mut workflows = Vec::new();

        for dir in &self.workflow_dirs {
            Self::collect_dir_workflows(&repo_path.join(dir), &mut workflows)?;
        }

        workflows.sort();
        Ok(workflows)
    }

    fn read_file_content(path: &std::path::Path) -> Result<String, WorkflowSourceError> {
        fs::read_to_string(path).map_err(WorkflowSourceError::Io)
    }

    fn extract_name(content: &str) -> Option<String> {
        WorkflowDocument::parse(content).ok()?.name()
    }

    fn extrworkflow_events(content: &str) -> Vec<String> {
        WorkflowDocument::parse(content)
            .map(|doc| doc.trigger_names())
            .unwrap_or_default()
    }

    fn workflow_response(
        file: &std::path::Path,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        let content = Self::read_file_content(file)?;
        let file_name = file
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_owned();
        Ok(WorkflowSourceFileResponse::new(content, file_name))
    }

    fn find_matching_workflow(
        files: &[std::path::PathBuf],
        name: &str,
    ) -> Result<Option<WorkflowSourceFileResponse>, WorkflowSourceError> {
        for file in files {
            let content = Self::read_file_content(file)?;
            if resolve_source_name(Self::extract_name(&content).as_deref(), file).as_deref()
                == Some(name)
            {
                let file_name = file
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .to_owned();
                return Ok(Some(WorkflowSourceFileResponse::new(content, file_name)));
            }
        }
        Ok(None)
    }

    fn extract_uses_action(line: &str) -> Option<String> {
        let trimmed = line.trim();
        let rest = trimmed
            .strip_prefix("- uses:")
            .or_else(|| trimmed.strip_prefix("uses:"))?
            .trim();
        if !rest.is_empty() && !rest.starts_with('#') {
            Some(rest.to_string())
        } else {
            None
        }
    }

    fn collect_actions_from_content(content: &str, actions: &mut BTreeSet<String>) {
        for line in content.lines() {
            if let Some(action) = Self::extract_uses_action(line) {
                actions.insert(action);
            }
        }
    }

    fn read_first_workflow(
        files: &[std::path::PathBuf],
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        match files.first() {
            Some(file) => Self::workflow_response(file),
            None => Err(WorkflowSourceError::Empty),
        }
    }

    fn read_named_workflow(
        files: &[std::path::PathBuf],
        name: &str,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        match Self::find_matching_workflow(files, name)? {
            Some(content) => Ok(content),
            None => Err(WorkflowSourceError::NotFound(format!(
                "workflow {:?} not found",
                name
            ))),
        }
    }
}

impl WorkflowSourcePort for FilesystemWorkflowSource {
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        let files = self.find_workflow_files(repository)?;
        match workflow_name {
            Some(name) => Self::read_named_workflow(&files, name),
            None => Self::read_first_workflow(&files),
        }
    }

    fn read_all_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError> {
        let files = self.find_workflow_files(repository)?;
        let mut contents = Vec::new();

        for file in files {
            contents.push(Self::workflow_response(&file)?);
        }

        Ok(contents)
    }

    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, WorkflowSourceError> {
        let files = self.find_workflow_files(repository)?;
        let mut actions = BTreeSet::new();

        for file in files {
            let content = Self::read_file_content(&file)?;
            Self::collect_actions_from_content(&content, &mut actions);
        }

        Ok(actions.into_iter().collect())
    }

    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<
        Vec<crate::application::dtos::responses::WorkflowListItemResponse>,
        WorkflowSourceError,
    > {
        let files = self.find_workflow_files(repository)?;
        let mut items = Vec::new();

        for file in files {
            let content = Self::read_file_content(&file)?;
            let name = resolve_source_name(Self::extract_name(&content).as_deref(), &file);
            items.push(
                crate::application::dtos::responses::WorkflowListItemResponse::new(
                    name,
                    Some(file.to_string_lossy().to_string()),
                    Self::extrworkflow_events(&content),
                ),
            );
        }

        Ok(items)
    }
}

/// Convenience constructor matching the old FilesystemWorkflowFileParser pattern.
impl Default for FilesystemWorkflowSource {
    fn default() -> Self {
        Self::new(&WORKFLOW_DIRECTORIES)
    }
}

unsafe impl Send for FilesystemWorkflowSource {}
unsafe impl Sync for FilesystemWorkflowSource {}
