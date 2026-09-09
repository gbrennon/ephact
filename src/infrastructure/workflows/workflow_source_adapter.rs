use std::{collections::BTreeSet, error::Error, fs};

use crate::{
    application::ports::outbound::WorkflowSourcePort, domain::entities::repository::Repository,
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
    ) -> Result<(), Box<dyn Error>> {
        if !workflows_dir.exists() {
            return Ok(());
        }
        let entries = fs::read_dir(workflows_dir)?;
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
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
        let repo_path = repo.path().as_path();
        let mut workflows = Vec::new();

        for dir in &self.workflow_dirs {
            Self::collect_dir_workflows(&repo_path.join(dir), &mut workflows)?;
        }

        workflows.sort();
        Ok(workflows)
    }

    fn read_file_content(path: &std::path::Path) -> Result<String, Box<dyn Error>> {
        fs::read_to_string(path).map_err(|e| e.into())
    }

    fn extract_name(content: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("name:") {
                let name = rest.trim().trim_matches(|c| c == '"' || c == '\'');
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    fn matches_workflow_name(content: &str, target_name: &str) -> bool {
        Self::extract_name(content).as_deref() == Some(target_name)
    }

    fn find_matching_workflow(
        files: &[std::path::PathBuf],
        name: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
        for file in files {
            let content = Self::read_file_content(file)?;
            if Self::matches_workflow_name(&content, name) {
                return Ok(Some(content));
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

    fn read_first_workflow(files: &[std::path::PathBuf]) -> Result<String, Box<dyn Error>> {
        match files.first() {
            Some(file) => Self::read_file_content(file),
            None => Err("no workflow files found".into()),
        }
    }

    fn read_named_workflow(
        files: &[std::path::PathBuf],
        name: &str,
    ) -> Result<String, Box<dyn Error>> {
        match Self::find_matching_workflow(files, name)? {
            Some(content) => Ok(content),
            None => Err(format!("workflow {:?} not found", name).into()),
        }
    }
}

impl WorkflowSourcePort for FilesystemWorkflowSource {
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let files = self.find_workflow_files(repository)?;
        match workflow_name {
            Some(name) => Self::read_named_workflow(&files, name),
            None => Self::read_first_workflow(&files),
        }
    }

    fn read_all_workflows(&self, repository: &Repository) -> Result<Vec<String>, Box<dyn Error>> {
        let files = self.find_workflow_files(repository)?;
        let mut contents = Vec::new();

        for file in files {
            let content = Self::read_file_content(&file)?;
            contents.push(content);
        }

        Ok(contents)
    }

    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, Box<dyn Error>> {
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
    ) -> Result<Vec<crate::application::dtos::WorkflowListItem>, Box<dyn Error>> {
        let files = self.find_workflow_files(repository)?;
        let mut items = Vec::new();

        for file in files {
            let content = Self::read_file_content(&file)?;
            if let Some(name) = Self::extract_name(&content) {
                items.push(crate::application::dtos::WorkflowListItem::new(
                    Some(name),
                    Some(file.to_string_lossy().to_string()),
                ));
            }
        }

        Ok(items)
    }
}

/// Convenience constructor matching the old FilesystemWorkflowFileParser pattern.
impl Default for FilesystemWorkflowSource {
    fn default() -> Self {
        use super::workflow_directories::WORKFLOW_DIRECTORIES;
        Self::new(&WORKFLOW_DIRECTORIES)
    }
}

unsafe impl Send for FilesystemWorkflowSource {}
unsafe impl Sync for FilesystemWorkflowSource {}
