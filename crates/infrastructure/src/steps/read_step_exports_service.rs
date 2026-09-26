use crate::{
    application::{
        dtos::{
            requests::{
                ReadStepEnvExportsRequest, ReadStepExportsRequest, ReadStepPathExportsRequest,
            },
            responses::StepExportsResponse,
        },
        ports::outbound::step_exports_reader_port::StepExportsReaderPort,
    },
    steps::{
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

impl StepExportsReaderPort for ReadStepExportsService {
    fn read(
        &self,
        _request: ReadStepExportsRequest,
        container: &dyn crate::application::ports::outbound::container_port::ContainerPort,
    ) -> StepExportsResponse {
        StepExportsResponse::new(
            self.path_reader
                .execute(ReadStepPathExportsRequest::new(), container),
            self.env_reader
                .execute(ReadStepEnvExportsRequest::new(), container),
        )
    }
}
