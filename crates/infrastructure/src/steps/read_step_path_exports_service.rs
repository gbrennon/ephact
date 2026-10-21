use std::collections::HashMap;

use super::super::containers::workspace::RUNNER_PATH_FILE;
use crate::application::{
    dtos::requests::ReadStepPathExportsRequest, ports::outbound::ReadStepPathExportsPort,
};

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
    fn read(
        &self,
        _request: ReadStepPathExportsRequest,
        container: &dyn crate::application::ports::outbound::container_port::ContainerPort,
    ) -> Vec<String> {
        container
            .exec(
                &["cat".into(), RUNNER_PATH_FILE.into()],
                None,
                &HashMap::new(),
            )
            .map(|output| {
                output
                    .stdout()
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    }
}
