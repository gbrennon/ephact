use crate::application::dtos::requests::SummarizeStepRequest;
use crate::application::dtos::responses::StepSummaryResponse;
use crate::application::dtos::responses::SummarizedStepResponse;
use crate::application::ports::outbound::summarize_step_port::SummarizeStepPort;

/// Service that turns a step's outcome into its run-summary entry, deciding
/// whether the outcome fails the job.
pub struct SummarizeStepService;

impl SummarizeStepService {
    pub fn new() -> Self {
        Self
    }
}
impl Default for SummarizeStepService {
    fn default() -> Self {
        Self::new()
    }
}

impl SummarizeStepPort for SummarizeStepService {
    fn execute(&self, request: SummarizeStepRequest<'_>) -> SummarizedStepResponse {
        let step_type = request.step().step_type();
        let continue_on_error = request.step().continues_on_error();

        let (exit_code, stdout, stderr, name, fails_job) = match request.outcome() {
            Ok(executed) => {
                let fails_job = executed.response().exit_code() != 0 && !continue_on_error;
                (
                    Some(executed.response().exit_code()),
                    executed.response().stdout().to_string(),
                    executed.response().stderr().to_string(),
                    executed.step().display_name().to_string(),
                    fails_job,
                )
            }
            Err(error) => (
                None,
                error.stdout().to_string(),
                format!("step error: {}\n{}", error.message(), error.stderr()),
                request.step().display_name().to_string(),
                !continue_on_error,
            ),
        };

        SummarizedStepResponse::new(
            StepSummaryResponse::new(
                name,
                step_type,
                exit_code,
                continue_on_error,
                request.duration(),
                stdout,
                stderr,
            ),
            fails_job,
        )
    }
}
