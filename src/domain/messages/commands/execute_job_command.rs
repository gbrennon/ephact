use std::path::PathBuf;

use crate::domain::{
    aggregates::Workflow, entities::Job, messages::commands::command::Command,
    value_objects::EvaluationContext,
};

/// Command representing the intention to execute one job of a workflow.
///
/// Published by the workflow coordination service once the execution plan is
/// known, and handled by the job command handler.
#[derive(Debug, Clone)]
pub struct ExecuteJobCommand {
    job: Job,
    job_id: String,
    workflow: Workflow,
    repo_path: PathBuf,
    context: EvaluationContext,
    run_id: String,
    allow_repo_writes: bool,
    allow_network: bool,
}

impl ExecuteJobCommand {
    pub fn new(
        job: Job,
        job_id: String,
        workflow: Workflow,
        repo_path: PathBuf,
        context: EvaluationContext,
    ) -> Self {
        Self {
            job,
            job_id,
            workflow,
            repo_path,
            context,
            run_id: String::new(),
            allow_repo_writes: false,
            allow_network: false,
        }
    }

    pub fn with_run_id(mut self, run_id: String) -> Self {
        self.run_id = run_id;
        self
    }

    pub fn with_allow_repo_writes(mut self, allow_repo_writes: bool) -> Self {
        self.allow_repo_writes = allow_repo_writes;
        self
    }

    pub fn with_allow_network(mut self, allow_network: bool) -> Self {
        self.allow_network = allow_network;
        self
    }

    pub fn job(&self) -> &Job {
        &self.job
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn workflow(&self) -> &Workflow {
        &self.workflow
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }
    pub fn allow_network(&self) -> bool {
        self.allow_network
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }

    pub fn into_parts(
        self,
    ) -> (
        Job,
        String,
        Workflow,
        PathBuf,
        EvaluationContext,
        String,
        bool,
    ) {
        (
            self.job,
            self.job_id,
            self.workflow,
            self.repo_path,
            self.context,
            self.run_id,
            self.allow_repo_writes,
        )
    }
}

impl Command for ExecuteJobCommand {}
