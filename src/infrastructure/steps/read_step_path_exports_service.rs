use super::read_step_path_exports_port::ReadStepPathExportsPort;
use std::collections::HashMap;

use super::super::containers::workspace::RUNNER_PATH_FILE;
use crate::application::dtos::requests::ReadStepPathExportsRequest;

/// Service that reads the directories a step exported through `GITHUB_PATH`.
///
/// Reading is best effort: a step that exported nothing never wrote the file,
/// so a failed read means no additions rather than a failure.
pub struct ReadStepPathExportsService;

impl ReadStepPathExportsService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadStepPathExportsService {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadStepPathExportsPort for ReadStepPathExportsService {
    fn execute(
        &self,
        _request: ReadStepPathExportsRequest,
        container: &dyn crate::application::ports::outbound::container_port::ContainerPort,
    ) -> Vec<String> {
        let mut additions = Vec::new();
        if let Ok(output) = container.exec(
            &["cat".into(), RUNNER_PATH_FILE.into()],
            None,
            &HashMap::new(),
        ) {
            for line in output.stdout().lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    additions.push(trimmed.to_string());
                }
            }
        }
        additions
    }
}
