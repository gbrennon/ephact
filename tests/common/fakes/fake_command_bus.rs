#![allow(dead_code)]
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
    sync::Arc,
};

use ephact::{
    application::{
        dtos::responses::{
            ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse, JobSummaryResponse,
            WorkflowExecutionResponse,
        },
        ports::outbound::{command_bus_port::CommandBusPort, container_port::ContainerPort},
    },
    domain::{
        entities::Step,
        errors::StepError,
        messages::commands::{
            ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
        },
        value_objects::EvaluationContext,
    },
};

#[derive(Clone, Debug)]
pub struct DispatchedStepSnapshot {
    step: Step,
    env: HashMap<String, String>,
    context: EvaluationContext,
    repo_path: PathBuf,
}

impl DispatchedStepSnapshot {
    pub fn new(
        step: Step,
        env: HashMap<String, String>,
        context: EvaluationContext,
        repo_path: PathBuf,
    ) -> Self {
        Self {
            step,
            env,
            context,
            repo_path,
        }
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}

#[derive(Clone, Debug)]
pub struct DispatchedActionSnapshot {
    action_ref: String,
    step: Step,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
}

impl DispatchedActionSnapshot {
    pub fn new(
        action_ref: String,
        step: Step,
        repo_path: PathBuf,
        env: HashMap<String, String>,
        context: EvaluationContext,
    ) -> Self {
        Self {
            action_ref,
            step,
            repo_path,
            env,
            context,
        }
    }

    pub fn action_ref(&self) -> &str {
        &self.action_ref
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }
}

/// Records every dispatched command and answers it with a prepared outcome, so
/// a coordination service can be tested on what it publishes instead of on
/// what the next service does. Shares its recordings across clones.
#[derive(Clone, Default)]
pub struct FakeCommandBus {
    pub dispatched_workflows: Arc<Mutex<Vec<ExecuteWorkflowCommand>>>,
    pub dispatched_jobs: Arc<Mutex<Vec<ExecuteJobCommand>>>,
    pub dispatched_steps: Arc<Mutex<Vec<DispatchedStepSnapshot>>>,
    pub dispatched_actions: Arc<Mutex<Vec<DispatchedActionSnapshot>>>,
    workflow_result: Option<WorkflowExecutionResponse>,
    action_result: Option<ExecuteActionResponse>,
    failing_jobs: Vec<String>,
    step_exit_codes: Arc<Mutex<Vec<i64>>>,
    step_error: Option<String>,
    job_error: Option<String>,
}

impl FakeCommandBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_workflow_result(mut self, result: WorkflowExecutionResponse) -> Self {
        self.workflow_result = Some(result);
        self
    }

    pub fn with_action_result(mut self, result: ExecuteActionResponse) -> Self {
        self.action_result = Some(result);
        self
    }

    /// Answers the named jobs with a failed summary.
    pub fn failing_jobs(mut self, job_ids: Vec<String>) -> Self {
        self.failing_jobs = job_ids;
        self
    }

    /// Answers every job dispatch with an error.
    pub fn failing_job_dispatch(mut self, message: &str) -> Self {
        self.job_error = Some(message.to_string());
        self
    }

    /// Answers every step dispatch with an error.
    pub fn failing_step_dispatch(mut self, message: &str) -> Self {
        self.step_error = Some(message.to_string());
        self
    }

    /// Answers step dispatches with the queued exit codes, in order.
    pub fn queueing_step_exit_codes(self, exit_codes: Vec<i64>) -> Self {
        *self.step_exit_codes.lock() = exit_codes.into_iter().rev().collect();
        self
    }

    /// Environments carried by the dispatched step commands, in order.
    pub fn dispatched_step_environments(&self) -> Vec<HashMap<String, String>> {
        self.dispatched_steps
            .lock()
            .iter()
            .map(|cmd| cmd.env().clone())
            .collect()
    }

    pub fn dispatched_job_ids(&self) -> Vec<String> {
        self.dispatched_jobs
            .lock()
            .iter()
            .map(|cmd| cmd.job_id().to_owned())
            .collect()
    }

    pub fn dispatched_action_refs(&self) -> Vec<String> {
        self.dispatched_actions
            .lock()
            .iter()
            .map(|cmd| cmd.action_ref().to_owned())
            .collect()
    }
}

impl CommandBusPort<ExecuteWorkflowCommand> for FakeCommandBus {
    type Response = WorkflowExecutionResponse;
    type Error = Box<dyn Error>;

    fn dispatch(&self, cmd: ExecuteWorkflowCommand) -> Result<Self::Response, Self::Error> {
        self.dispatched_workflows.lock().push(cmd);
        Ok(self
            .workflow_result
            .clone()
            .unwrap_or(WorkflowExecutionResponse::new(
                "fake-workflow".to_string(),
                Vec::new(),
                vec!["c1".to_string()],
                true,
            )))
    }
}

impl CommandBusPort<ExecuteJobCommand> for FakeCommandBus {
    type Response = JobExecutionResponse;
    type Error = Box<dyn Error>;

    fn dispatch(&self, cmd: ExecuteJobCommand) -> Result<Self::Response, Self::Error> {
        let job_id = cmd.job_id().to_owned();
        let name = cmd.job().name().map(|s| s.to_owned());
        self.dispatched_jobs.lock().push(cmd);
        if let Some(message) = &self.job_error {
            return Err(message.clone().into());
        }
        Ok(JobExecutionResponse::new(
            JobSummaryResponse::new(
                job_id.clone(),
                name,
                Vec::new(),
                !self.failing_jobs.contains(&job_id),
            ),
            format!("container-{job_id}"),
        ))
    }
}

impl<'a> CommandBusPort<ExecuteStepCommand<'a, dyn ContainerPort>> for FakeCommandBus {
    type Response = ExecutedStepResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        cmd: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        let (step, env, context, _container, repo_path) = cmd.into_parts();
        self.dispatched_steps
            .lock()
            .push(DispatchedStepSnapshot::new(
                step.clone(),
                env,
                context,
                repo_path,
            ));
        if let Some(message) = &self.step_error {
            return Err(StepError::new(message.clone()));
        }
        Ok(ExecutedStepResponse::new(
            step,
            ExecuteActionResponse::new(
                self.step_exit_codes.lock().pop().unwrap_or(0),
                String::new(),
                String::new(),
            ),
        ))
    }
}

impl<'a> CommandBusPort<ExecuteActionCommand<'a, dyn ContainerPort>> for FakeCommandBus {
    type Response = ExecuteActionResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        cmd: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        let (action_ref, step, repo_path, env, context, _container) = cmd.into_parts();
        self.dispatched_actions
            .lock()
            .push(DispatchedActionSnapshot::new(
                action_ref, step, repo_path, env, context,
            ));
        Ok(self
            .action_result
            .clone()
            .unwrap_or(ExecuteActionResponse::new(0, String::new(), String::new())))
    }
}
