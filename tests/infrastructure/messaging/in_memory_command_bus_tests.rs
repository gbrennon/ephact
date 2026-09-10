#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf, sync::Arc};

    use ephact::application::commands::{
        ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
    };
    use ephact::application::dtos::requests::ExecuteActionRequest;
    use ephact::application::dtos::responses::ExecuteActionResponse;
    use ephact::application::dtos::responses::ExecutedStepResponse;
    use ephact::application::dtos::responses::JobExecutionResponse;
    use ephact::application::dtos::responses::JobSummaryResponse;
    use ephact::application::dtos::responses::WorkflowExecutionResponse;
    use ephact::application::ports::inbound::execute_action_port::ExecuteActionPort;
    use ephact::application::ports::inbound::execute_job_port::ExecuteJobPort;
    use ephact::application::ports::inbound::execute_step_port::ExecuteStepPort;
    use ephact::application::ports::inbound::execute_workflow_port::ExecuteWorkflowPort;
    use ephact::application::ports::outbound::CommandBusPort;
    use ephact::domain::ActRunConfig;
    use ephact::domain::RepoPath;
    use ephact::domain::Repository;
    use ephact::domain::RepositoryName;
    use ephact::domain::aggregates::Workflow;
    use ephact::domain::entities::Job;
    use ephact::domain::errors::StepError;
    use ephact::domain::value_objects::EvaluationContext;
    use ephact::domain::value_objects::WorkflowTrigger;
    use ephact::infrastructure::actions::ActionCommandHandler;
    use ephact::infrastructure::jobs::JobCommandHandler;
    use ephact::infrastructure::messaging::InMemoryCommandBus;
    use ephact::infrastructure::steps::StepCommandHandler;
    use ephact::infrastructure::workflows::WorkflowCommandHandler;

    use crate::common::fakes::stub_container::StubContainer;
    use ephact::infrastructure::workflows::yaml::StepYaml;

    struct StubWorkflowPort;
    impl ExecuteWorkflowPort for StubWorkflowPort {
        fn execute(
            &self,
            _request: ephact::application::dtos::requests::ExecuteWorkflowRequest<'_>,
        ) -> Result<WorkflowExecutionResponse, Box<dyn std::error::Error>> {
            Ok(WorkflowExecutionResponse::new(
                "dispatched-wf".to_string(),
                Vec::new(),
                vec!["c1".to_string()],
                true,
            ))
        }
    }

    struct StubJobPort;
    impl ExecuteJobPort for StubJobPort {
        fn execute(
            &self,
            _request: ephact::application::dtos::requests::ExecuteJobRequest<'_>,
        ) -> Result<JobExecutionResponse, Box<dyn std::error::Error>> {
            Ok(JobExecutionResponse::new(
                JobSummaryResponse::new(
                    "j1".to_string(),
                    Some("job 1".to_string()),
                    Vec::new(),
                    true,
                ),
                "c1".to_string(),
            ))
        }
    }

    struct StubStepPort;
    impl ExecuteStepPort for StubStepPort {
        fn execute(
            &self,
            request: ephact::application::dtos::requests::ExecuteStepRequest<'_>,
        ) -> Result<ExecutedStepResponse, StepError> {
            Ok(ExecutedStepResponse::new(
                request.step().clone(),
                ExecuteActionResponse::new(0, "step out".to_string(), String::new()),
            ))
        }
    }

    struct StubActionPort;
    impl ExecuteActionPort for StubActionPort {
        fn execute(
            &self,
            _request: ExecuteActionRequest,
        ) -> Result<ExecuteActionResponse, StepError> {
            Ok(ExecuteActionResponse::new(
                0,
                "action out".to_string(),
                String::new(),
            ))
        }
    }

    #[test]
    fn command_bus_dispatches_workflow_to_workflow_handler() {
        let bus = InMemoryCommandBus::new(
            Box::new(WorkflowCommandHandler::new(Box::new(StubWorkflowPort))),
            Box::new(JobCommandHandler::new(Box::new(StubJobPort))),
            Box::new(StepCommandHandler::new(Box::new(StubStepPort))),
            Box::new(ActionCommandHandler::new(Box::new(StubActionPort))),
        );

        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".git")).unwrap();
        let repository = Repository::new(
            RepoPath::new(tmp.path().to_path_buf()).unwrap(),
            RepositoryName::new("test-repo".into()).unwrap(),
        );

        let cmd = ExecuteWorkflowCommand::new(
            "name: CI\non: [push]\n".to_string(),
            ActRunConfig::new(),
            repository,
        );

        let result = bus.dispatch_workflow(cmd).unwrap();
        assert_eq!(result.workflow_name(), "dispatched-wf");
    }

    #[test]
    fn command_bus_dispatches_action_to_action_handler() {
        let bus = InMemoryCommandBus::new(
            Box::new(WorkflowCommandHandler::new(Box::new(StubWorkflowPort))),
            Box::new(JobCommandHandler::new(Box::new(StubJobPort))),
            Box::new(StepCommandHandler::new(Box::new(StubStepPort))),
            Box::new(ActionCommandHandler::new(Box::new(StubActionPort))),
        );

        let step = serde_yaml::from_str::<StepYaml>("uses: actions/checkout@v4")
            .unwrap()
            .into_domain();
        let cmd = ExecuteActionCommand::new(
            "actions/checkout@v4".into(),
            step,
            PathBuf::from("/repo"),
            HashMap::new(),
            EvaluationContext::new(),
            Arc::new(StubContainer),
        );

        let result = bus.dispatch_action(cmd).unwrap();
        assert_eq!(result.stdout(), "action out");
        assert_eq!(result.exit_code(), 0);
    }
}
