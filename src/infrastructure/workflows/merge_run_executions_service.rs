use super::merge_run_executions_port::MergeRunExecutionsPort;
use std::error::Error;

use crate::application::dtos::{JobSummary, MergeRunExecutionsRequest, WorkflowExecution};

/// Summary name used when every workflow in the repository is executed.
pub const ALL_WORKFLOWS_SUMMARY_NAME: &str = "all-workflows";

/// Service that reduces a run's workflow executions to the single execution the
/// run reports.
///
/// A run of one workflow reports that workflow unchanged; a run of every
/// workflow is reported as one execution whose job names name the workflow they
/// came from.
pub struct MergeRunExecutionsService;

impl MergeRunExecutionsService {
    pub fn new() -> Self {
        Self
    }

    fn prefix_job(wf_name: &str, job: &JobSummary) -> JobSummary {
        let prefixed_name = job.name().map(|name| format!("{wf_name} / {name}"));
        JobSummary::new(
            job.job_id().to_string(),
            prefixed_name,
            job.steps().to_vec(),
            job.success(),
        )
    }
}

impl Default for MergeRunExecutionsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MergeRunExecutionsPort for MergeRunExecutionsService {
    fn execute(
        &self,
        request: MergeRunExecutionsRequest,
    ) -> Result<WorkflowExecution, Box<dyn Error>> {
        if !request.all_workflows() {
            return request
                .executions()
                .iter()
                .next()
                .cloned()
                .ok_or_else(|| "no workflow file resolved".into());
        }

        let mut merged_jobs: Vec<JobSummary> = Vec::new();
        let mut merged_containers: Vec<String> = Vec::new();
        let mut merged_success = true;

        for execution in request.executions() {
            let wf_name = execution.workflow_name();
            for job in execution.job_summaries() {
                merged_jobs.push(Self::prefix_job(wf_name, job));
            }
            merged_containers.extend(execution.container_names().iter().cloned());
            merged_success &= execution.success();
        }

        Ok(WorkflowExecution::new(
            ALL_WORKFLOWS_SUMMARY_NAME,
            merged_jobs,
            merged_containers,
            merged_success,
        ))
    }
}
