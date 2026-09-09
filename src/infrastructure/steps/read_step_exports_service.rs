use crate::application::dtos::{
    ReadStepEnvExportsRequest, ReadStepExportsRequest, ReadStepPathExportsRequest, StepExports,
};
use crate::{
    application::ports::outbound::read_step_exports_port::ReadStepExportsPort,
    infrastructure::steps::{
        read_step_env_exports_port::ReadStepEnvExportsPort,
        read_step_path_exports_port::ReadStepPathExportsPort,
    },
};

/// Service that reads everything a step exported to the steps that follow it.
pub struct ReadStepExportsService {
    path_reader: Box<dyn ReadStepPathExportsPort>,
    env_reader: Box<dyn ReadStepEnvExportsPort>,
}

impl ReadStepExportsService {
    pub fn new(
        path_reader: Box<dyn ReadStepPathExportsPort>,
        env_reader: Box<dyn ReadStepEnvExportsPort>,
    ) -> Self {
        Self {
            path_reader,
            env_reader,
        }
    }
}

impl ReadStepExportsPort for ReadStepExportsService {
    fn execute(&self, request: ReadStepExportsRequest<'_>) -> StepExports {
        StepExports::new(
            self.path_reader.execute(ReadStepPathExportsRequest::new(request.container())),
            self.env_reader.execute(ReadStepEnvExportsRequest::new(request.container())),
        )
    }
}
