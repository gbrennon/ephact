use std::sync::Arc;

use ephact::{
    application::{
        dtos::{requests::RunCompositeStepRequest, responses::ExecResultResponse},
        ports::outbound::ContainerPort,
    },
    domain::{entities::Step, errors::StepError},
    infrastructure::steps::run_composite_step_port::RunCompositeStepPort,
};
use parking_lot::Mutex;

type StepOutcome = Result<ExecResultResponse, StepError>;
type SharedStepOutcomes = Arc<Mutex<Vec<StepOutcome>>>;

/// Answers each composite step with the next queued result, recording the
/// steps it was asked to run.
#[derive(Clone, Default)]
pub struct FakeRunCompositeStepPort {
    results: Arc<Mutex<Vec<ExecResultResponse>>>,
    failure: Option<(String, String, String)>,
    outcomes: Option<SharedStepOutcomes>,
    steps: Arc<Mutex<Vec<Step>>>,
}

impl FakeRunCompositeStepPort {
    pub fn queueing(results: Vec<ExecResultResponse>) -> Self {
        Self {
            results: Arc::new(Mutex::new(results)),
            failure: None,
            outcomes: None,
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
            outcomes: None,
            steps: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn sequence(outcomes: Vec<StepOutcome>) -> Self {
        Self {
            results: Arc::new(Mutex::new(Vec::new())),
            failure: None,
            outcomes: Some(Arc::new(Mutex::new(outcomes))),
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
        _container: std::sync::Arc<dyn ContainerPort>,
    ) -> Result<ExecResultResponse, StepError> {
        self.steps.lock().push(request.step().clone());

        if let Some((message, stdout, stderr)) = &self.failure {
            return Err(StepError::new(message.clone())
                .with_stdout(stdout.clone())
                .with_stderr(stderr.clone()));
        }

        if let Some(outcomes) = &self.outcomes {
            let mut outcomes = outcomes.lock();
            return outcomes.remove(0);
        }

        let mut queued = self.results.lock();
        if queued.is_empty() {
            return Ok(ExecResultResponse::new(0, String::new(), String::new()));
        }
        Ok(queued.remove(0))
    }
}
