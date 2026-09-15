use crate::application::dtos::responses::ExecResultResponse;
use crate::application::ports::outbound::container_port::{ContainerPort, ExecOptions};
use crate::domain::errors::ContainerError;
use crate::domain::messages::events::OutputStream;

pub trait ContainerStreaming {
    fn exec_streaming(
        &self,
        options: ExecOptions<'_>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError>;
}

impl<T: ContainerPort + ?Sized> ContainerStreaming for T {
    fn exec_streaming(
        &self,
        options: ExecOptions<'_>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError> {
        let result = self.exec(options.cmd(), options.workdir(), options.env())?;
        if !result.stdout().is_empty() {
            on_output(OutputStream::StandardOutput, result.stdout());
        }
        if !result.stderr().is_empty() {
            on_output(OutputStream::StandardError, result.stderr());
        }
        Ok(result)
    }
}
