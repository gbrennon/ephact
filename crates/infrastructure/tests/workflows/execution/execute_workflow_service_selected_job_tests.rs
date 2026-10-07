#[cfg(test)]
mod tests {
    use std::path::Path;

    use ephact::{
        application::{
            dtos::{requests::ExecuteWorkflowRequest, responses::WorkflowExecutionResponse},
            ports::inbound::execute_workflow_port::ExecuteWorkflowPort,
        },
        domain::value_objects::EvaluationContext,
        infrastructure::workflows::execution::execute_workflow_service::ExecuteWorkflowService,
    };

    use crate::common::fakes::{
        fake_command_bus::FakeCommandBus, fake_event_bus::FakeEventBus,
        fake_workflow_loader_port::FakeWorkflowLoaderPort,
    };

    const REQUESTED_CONTENT: &str = "name: Ci\non: push\njobs: {}\n";
    const TWO_JOBS: &str = "name: Ci\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: build\n  publish:\n    needs: build\n    runs-on: ubuntu-latest\n    steps:\n      - run: publish\n";

    #[test]
    fn execute_selected_job_includes_only_its_dependency_closure() {
        let workflow = "name: Ci\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: build\n  test:\n    needs: build\n    runs-on: ubuntu-latest\n    steps:\n      - run: test\n  publish:\n    needs: test\n    runs-on: ubuntu-latest\n    steps:\n      - run: publish\n  lint:\n    runs-on: ubuntu-latest\n    steps:\n      - run: lint\n";
        let command_bus = FakeCommandBus::new();

        let execution = execute_with_selected_job(
            FakeWorkflowLoaderPort::holding(workflow),
            command_bus.clone(),
            "publish",
        );

        let execution = execution.unwrap();
        assert_eq!(
            command_bus.dispatched_job_ids(),
            vec![
                "build".to_string(),
                "test".to_string(),
                "publish".to_string()
            ]
        );
        assert_eq!(execution.job_summaries().len(), 3);
    }

    #[test]
    fn execute_rejects_an_unknown_selected_job() {
        let command_bus = FakeCommandBus::new();

        let result = execute_with_selected_job(
            FakeWorkflowLoaderPort::holding(TWO_JOBS),
            command_bus.clone(),
            "missing",
        );

        let error = result.expect_err("unknown selected job should fail");
        assert!(error.to_string().contains("missing"));
        assert!(command_bus.dispatched_job_ids().is_empty());
    }

    fn execute_with_selected_job(
        loader: FakeWorkflowLoaderPort,
        command_bus: FakeCommandBus,
        selected_job: &str,
    ) -> Result<WorkflowExecutionResponse, ephact::application::errors::ExecuteWorkflowError> {
        ExecuteWorkflowService::new(
            Box::new(loader),
            Box::new(command_bus),
            Box::new(FakeEventBus::new()),
        )
        .execute(
            ExecuteWorkflowRequest::new(
                REQUESTED_CONTENT.to_string(),
                Path::new("/repo").to_path_buf(),
                EvaluationContext::new(),
                "test-run".to_string(),
                false,
            )
            .with_selected_job(selected_job),
        )
    }
}
