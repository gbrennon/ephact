use std::collections::HashMap;

use super::{super::containers::workspace::RUNNER_OUTPUT_FILE, runner_export_parser};
use crate::application::{
    dtos::requests::ReadStepOutputExportsRequest,
    ports::outbound::{ContainerPort, ReadStepOutputExportsPort},
};

/// Reads the values a step exported for later steps.
///
/// The runner-specific file location stays in infrastructure; callers receive
/// only provider-neutral output values.
pub struct ReadStepOutputExportsService;

impl ReadStepOutputExportsService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadStepOutputExportsService {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadStepOutputExportsPort for ReadStepOutputExportsService {
    fn read(
        &self,
        _request: ReadStepOutputExportsRequest,
        container: &dyn ContainerPort,
    ) -> HashMap<String, String> {
        container
            .exec(
                &["cat".into(), RUNNER_OUTPUT_FILE.into()],
                None,
                &HashMap::new(),
            )
            .map(|result| runner_export_parser::parse(result.stdout()))
            .unwrap_or_default()
    }
}
