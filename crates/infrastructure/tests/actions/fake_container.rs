use std::collections::HashMap;

use ephact_infrastructure::{
    application::{
        dtos::responses::{ExecResultResponse, RunnerContextResponse},
        ports::outbound::ContainerPort,
    },
    domain::{entities::FileEntry, errors::ContainerError, messages::events::OutputStream},
};

pub struct FakeContainer;

impl ContainerPort for FakeContainer {
    fn exec(
        &self,
        _: &[String],
        _: Option<&str>,
        _: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        unimplemented!()
    }

    fn exec_streaming(
        &self,
        _: ephact_infrastructure::application::dtos::requests::ExecOptions<'_>,
        _: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError> {
        unimplemented!()
    }

    fn copy_to(&self, _: &str, _: &[FileEntry]) -> Result<(), ContainerError> {
        unimplemented!()
    }

    fn copy_from(&self, _: &str) -> Result<Vec<FileEntry>, ContainerError> {
        unimplemented!()
    }

    fn remove(&self) -> Result<(), ContainerError> {
        unimplemented!()
    }

    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError> {
        unimplemented!()
    }
}
