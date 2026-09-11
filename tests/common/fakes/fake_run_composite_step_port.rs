#![allow(dead_code)]
use parking_lot::Mutex;
use std::sync::Arc;

use ephact::application::dtos::requests::RunCompositeStepRequest;
use ephact::application::dtos::responses::ExecResultResponse;
use ephact::domain::entities::Step;
use ephact::domain::errors::StepError;
use ephact::infrastructure::steps::run_composite_step_port::RunCompositeStepPort;

/// Answers each composite step with the next queued result, recording the
/// steps it was asked to run.
#[derive(Clone, Default)]
pub struct FakeRunCompositeStepPort {
    results: Arc<Mutex<Vec<ExecResultResponse>>>,
    failure: Option<(String, String, String)>,
    steps: Arc<Mutex<Vec<Step>>>,
}

impl FakeRunCompositeStepPort {
    pub fn queueing(results: Vec<ExecResultResponse>) -> Self {
        Self {
            results: Arc::new(Mutex::new(results)),
            failure: None,
            steps: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn failing(error: StepError) -> Self {
        Self {
            results: Arc::new(Mutex::new(Vec::new())),
            failure: Some((
                error.message().to_owned().to_owned(),
                error.stdout().to_owned().to_owned(),
                error.stderr().to_owned().to_owned(),
            )),
            steps: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn steps(&self) -> Vec<Step> {
        self.steps.lock().clone()
    }
}

impl RunCompositeStepPort for FakeRunCompositeStepPort {
    fn execute(
        &self,
        request: RunCompositeStepRequest<'_>,
    ) -> Result<ExecResultResponse, StepError> {
        self.steps.lock().push(request.step().clone());

        if let Some((message, stdout, stderr)) = &self.failure {
            return Err(StepError::new(message.clone())
                .with_stdout(stdout.clone())
                .with_stderr(stderr.clone()));
        }

        let mut queued = self.results.lock();
        if queued.is_empty() {
            return Ok(ExecResultResponse::new(0, String::new(), String::new()));
        }
        Ok(queued.remove(0))
    }
}
