#[cfg(test)]
mod tests {
    use crate::common::fakes::{
        stub_run_all_workflows_port::StubRunAllWorkflowsPort,
        stub_run_workflow_port::StubRunWorkflowPort,
    };
    use ephact::{
        application::dtos::{JobSummary, RunSummary},
        presentation::{
            cli::{parse_run_test_args, run_handler::RunHandler},
            components::terminal::SystemTerminal,
        },
    };

    fn summary(success: bool) -> RunSummary {
        RunSummary {
            name: "test".into(),
            job_summaries: vec![JobSummary {
                job_id: "job".into(),
                name: None,
                steps: vec![],
                success,
            }],
            success,
            duration: std::time::Duration::ZERO,
        }
    }

    #[test]
    fn handle_success() {
        let args = parse_run_test_args(&[]);
        let wf_port = StubRunWorkflowPort {
            result: Ok(summary(true)),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(summary(true)),
        };
        let terminal = SystemTerminal;
        assert!(RunHandler::handle(args, &wf_port, &all_wf_port, &terminal).is_ok());
    }

    #[test]
    fn handle_propagates_workflow_failure() {
        let args = parse_run_test_args(&[]);
        let wf_port = StubRunWorkflowPort {
            result: Ok(summary(false)),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(summary(false)),
        };
        let terminal = SystemTerminal;
        let err = RunHandler::handle(args, &wf_port, &all_wf_port, &terminal).unwrap_err();
        assert!(err.to_string().contains("workflow failed"));
    }

    #[test]
    fn handle_propagates_port_error() {
        let args = parse_run_test_args(&[]);
        let wf_port = StubRunWorkflowPort {
            result: Err("port failure".into()),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Err("port failure".into()),
        };
        let terminal = SystemTerminal;
        let err = RunHandler::handle(args, &wf_port, &all_wf_port, &terminal).unwrap_err();
        assert!(err.to_string().contains("port failure"));
    }
}
