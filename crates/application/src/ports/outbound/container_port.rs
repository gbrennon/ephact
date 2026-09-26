use std::collections::HashMap;

pub use crate::dtos::requests::ExecOptions;
use crate::{
    domain::{entities::FileEntry, errors::ContainerError, messages::events::OutputStream},
    dtos::responses::{ExecResultResponse, RunnerContextResponse},
};

pub trait ContainerPort: Send + Sync {
    fn exec(
        &self,
        cmd: &[String],
        workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError>;

    fn exec_streaming(
        &self,
        options: ExecOptions<'_>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError>;

    fn copy_to(&self, container_path: &str, entries: &[FileEntry]) -> Result<(), ContainerError>;

    fn copy_from(&self, container_path: &str) -> Result<Vec<FileEntry>, ContainerError>;

    fn remove(&self) -> Result<(), ContainerError>;

    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError>;
}
