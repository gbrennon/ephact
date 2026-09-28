use ephact_application::{
    dtos::responses::{JobSummaryResponse, WorkflowExecutionResponse},
    services::WorkflowExecutionAggregator,
};

#[test]
fn qualifies_job_names_with_their_workflow_name() {
    let executions = vec![WorkflowExecutionResponse::new(
        "build",
        vec![JobSummaryResponse::new(
            "job-id",
            Some("test".to_owned()),
            vec![],
            true,
        )],
        vec![],
        true,
    )];

    let summaries = WorkflowExecutionAggregator::new().aggregate(&executions);

    assert_eq!(summaries[0].name(), Some("build / test"));
}

#[test]
fn preserves_jobs_without_names() {
    let executions = vec![WorkflowExecutionResponse::new(
        "build",
        vec![JobSummaryResponse::new("job-id", None, vec![], true)],
        vec![],
        true,
    )];

    let summaries = WorkflowExecutionAggregator::new().aggregate(&executions);

    assert_eq!(summaries[0].name(), None);
}

#[test]
fn aggregates_jobs_from_all_workflows_in_order() {
    let executions = vec![
        WorkflowExecutionResponse::new(
            "build",
            vec![JobSummaryResponse::new(
                "build-job",
                Some("compile".to_owned()),
                vec![],
                true,
            )],
            vec![],
            true,
        ),
        WorkflowExecutionResponse::new(
            "test",
            vec![JobSummaryResponse::new(
                "test-job",
                Some("unit".to_owned()),
                vec![],
                true,
            )],
            vec![],
            true,
        ),
    ];

    let summaries = WorkflowExecutionAggregator::new().aggregate(&executions);

    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0].job_id(), "build-job");
    assert_eq!(summaries[1].job_id(), "test-job");
}
