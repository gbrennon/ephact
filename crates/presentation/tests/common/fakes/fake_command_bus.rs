use std::sync::Arc;

use ephact::{
    application::dtos::responses::{ExecuteActionResponse, WorkflowExecutionResponse},
    domain::messages::commands::{ExecuteJobPayload, ExecuteWorkflowPayload},
};
use parking_lot::Mutex;

use super::{
    dispatched_action_snapshot::DispatchedActionSnapshot,
    dispatched_step_snapshot::DispatchedStepSnapshot,
};

/// Records every published command and answers it with a prepared outcome, so
/// a coordination service can be tested on what it publishes instead of on
/// what the next service does. Shares its recordings across clones.
#[derive(Clone, Default)]
pub struct FakeCommandBus {
    pub dispatched_workflows: Arc<Mutex<Vec<ExecuteWorkflowPayload>>>,
    pub dispatched_jobs: Arc<Mutex<Vec<ExecuteJobPayload>>>,
    pub dispatched_steps: Arc<Mutex<Vec<DispatchedStepSnapshot>>>,
    pub dispatched_actions: Arc<Mutex<Vec<DispatchedActionSnapshot>>>,
    pub(crate) workflow_result: Option<WorkflowExecutionResponse>,
    pub(crate) action_result: Option<ExecuteActionResponse>,
    pub(crate) failing_jobs: Vec<String>,
    pub(crate) step_exit_codes: Arc<Mutex<Vec<i64>>>,
    pub(crate) step_error: Option<String>,
    pub(crate) job_error: Option<String>,
}
