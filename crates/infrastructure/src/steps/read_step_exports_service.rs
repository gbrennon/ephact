use crate::application::{
    dtos::{
        requests::{ReadStepEnvExportsRequest, ReadStepExportsRequest, ReadStepPathExportsRequest},
        responses::StepExportsResponse,
    },
    ports::outbound::{
        ReadStepEnvExportsPort, ReadStepPathExportsPort,
        step_exports_reader_port::StepExportsReaderPort,
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
                .read(ReadStepPathExportsRequest::new(), container),
            self.env_reader
                .read(ReadStepEnvExportsRequest::new(), container),
        )
    }
}
