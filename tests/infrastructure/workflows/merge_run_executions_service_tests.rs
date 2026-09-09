use ephact::infrastructure::workflows::{
    merge_run_executions_port::MergeRunExecutionsPort,
    merge_run_executions_service::MergeRunExecutionsService,
};
use std::time::Duration;

use ephact::application::dtos::{
    JobSummary, MergeRunExecutionsRequest, StepSummary, WorkflowExecution,
};
use ephact::domain::workflow::StepType;

fn job(job_id: &str, name: Option<&str>, success: bool) -> JobSummary {
    JobSummary {
        job_id: job_id.to_string(),
        name: name.map(str::to_string),
        steps: vec![StepSummary::new(
            "step".to_string(),
            StepType::Run,
            Some(0),
            false,
            Duration::from_secs(0),
            String::new(),
            String::new(),
        )],
        success,
    }
}

fn execution(
    name: &str,
    jobs: Vec<JobSummary>,
    containers: &[&str],
    success: bool,
) -> WorkflowExecution {
    WorkflowExecution::new(
        name.to_string(),
        jobs,
        containers.iter().map(|c| c.to_string()).collect(),
        success,
    )
}

#[test]
fn execute_returns_a_single_run_unchanged() {
    let merged = MergeRunExecutionsService::new()
        .execute(MergeRunExecutionsRequest::new(
            vec![execution(
                "ci",
                vec![job("build", Some("Build"), true)],
                &["container-build"],
                true,
            )],
            false,
        ))
        .unwrap();

    assert_eq!(merged.workflow_name(), "ci");
    assert_eq!(merged.job_summaries()[0].name(), Some("Build"));
    assert_eq!(merged.container_names(), &["container-build".to_string()]);
    assert!(merged.success());
}

#[test]
fn execute_errors_for_a_single_run_with_no_execution() {
    let Err(error) =
        MergeRunExecutionsService::new().execute(MergeRunExecutionsRequest::new(Vec::new(), false))
    else {
        panic!("merging no executions should fail");
    };

    assert_eq!(error.to_string(), "no workflow file resolved");
}

#[test]
fn execute_names_an_all_workflows_run_and_prefixes_every_job_name() {
    let merged = MergeRunExecutionsService::new()
        .execute(MergeRunExecutionsRequest::new(
            vec![
                execution(
                    "ci",
                    vec![job("build", Some("build"), true)],
                    &["c-1"],
                    true,
                ),
                execution(
                    "release",
                    vec![job("publish", Some("publish"), true)],
                    &["c-2"],
                    true,
                ),
            ],
            true,
        ))
        .unwrap();

    assert_eq!(merged.workflow_name(), "all-workflows");
    assert_eq!(
        merged
            .job_summaries()
            .iter()
            .map(|job| job.name().map(str::to_string))
            .collect::<Vec<_>>(),
        vec![
            Some("ci / build".to_string()),
            Some("release / publish".to_string())
        ]
    );
    assert_eq!(
        merged.container_names(),
        &["c-1".to_string(), "c-2".to_string()]
    );
    assert!(merged.success());
}

#[test]
fn execute_fails_an_all_workflows_run_when_any_execution_failed() {
    let merged = MergeRunExecutionsService::new()
        .execute(MergeRunExecutionsRequest::new(
            vec![
                execution(
                    "ci",
                    vec![job("build", Some("build"), true)],
                    &["c-1"],
                    true,
                ),
                execution(
                    "release",
                    vec![job("publish", Some("publish"), false)],
                    &["c-2"],
                    false,
                ),
            ],
            true,
        ))
        .unwrap();

    assert!(!merged.success());
}

#[test]
fn execute_leaves_an_unnamed_job_unnamed() {
    let merged = MergeRunExecutionsService::new()
        .execute(MergeRunExecutionsRequest::new(
            vec![execution(
                "ci",
                vec![job("build", None, true)],
                &["c-1"],
                true,
            )],
            true,
        ))
        .unwrap();

    assert_eq!(merged.job_summaries()[0].name(), None);
}
