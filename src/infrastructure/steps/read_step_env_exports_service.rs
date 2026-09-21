use std::collections::HashMap;

use super::{
    super::containers::workspace::RUNNER_ENV_FILE,
    read_step_env_exports_port::ReadStepEnvExportsPort,
};
use crate::application::dtos::requests::ReadStepEnvExportsRequest;

/// Service that reads the environment variables a step exported through
/// `GITHUB_ENV`.
///
/// Reading is best effort: a step that exported nothing never wrote the file,
/// so a failed read means no variables rather than a failure.
pub struct ReadStepEnvExportsService;

impl ReadStepEnvExportsService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadStepEnvExportsService {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadStepEnvExportsPort for ReadStepEnvExportsService {
    fn execute(
        &self,
        _request: ReadStepEnvExportsRequest,
        container: &dyn crate::application::ports::outbound::container_port::ContainerPort,
    ) -> HashMap<String, String> {
        let mut exported = HashMap::new();
        if let Ok(output) = container.exec(
            &["cat".into(), RUNNER_ENV_FILE.into()],
            None,
            &HashMap::new(),
        ) {
            for line in output.stdout().lines() {
                let trimmed = line.trim();
                if let Some((key, value)) = trimmed.split_once('=') {
                    exported.insert(key.to_string(), value.to_string());
                }
            }
        }
        exported
    }
}
