#[cfg(test)]
mod tests {
    use crate::scenarios::{
        forgejo_actions_workflow_run::ForgejoActionsWorkflowRun,
        github_actions_workflow_run::GithubActionsWorkflowRun,
        woodpecker_workflow_run::WoodpeckerWorkflowRun,
    };

    struct SupportedWorkflowFormatsTests;

    impl SupportedWorkflowFormatsTests {
        fn github_actions_execute_through_the_application() {
            let run = GithubActionsWorkflowRun::execute();

            assert_eq!(run.outcome(), &Ok(()));
            assert!(run.activity().ran_script(GithubActionsWorkflowRun::SCRIPT));
            assert_eq!(run.activity().pulled_images(), vec!["e2e-runner:latest"]);
            assert_eq!(run.activity().stopped_containers().len(), 1);
            assert_eq!(run.activity().killed_containers().len(), 1);
        }

        fn forgejo_actions_execute_through_the_application() {
            let run = ForgejoActionsWorkflowRun::execute();

            assert_eq!(run.outcome(), &Ok(()));
            assert!(run.activity().ran_script(ForgejoActionsWorkflowRun::SCRIPT));
            assert_eq!(run.activity().pulled_images(), vec!["e2e-runner:latest"]);
            assert_eq!(run.activity().stopped_containers().len(), 1);
            assert_eq!(run.activity().killed_containers().len(), 1);
        }

        fn woodpecker_execute_through_the_application() {
            let run = WoodpeckerWorkflowRun::execute();

            assert_eq!(run.outcome(), &Ok(()));
            assert!(run.activity().ran_script(WoodpeckerWorkflowRun::SCRIPT));
            assert_eq!(run.activity().pulled_images(), vec!["e2e-runner:latest"]);
            assert_eq!(run.activity().stopped_containers().len(), 1);
            assert_eq!(run.activity().killed_containers().len(), 1);
        }
    }

    #[test]
    fn github_actions_execute_through_the_application() {
        SupportedWorkflowFormatsTests::github_actions_execute_through_the_application();
    }

    #[test]
    fn forgejo_actions_execute_through_the_application() {
        SupportedWorkflowFormatsTests::forgejo_actions_execute_through_the_application();
    }

    #[test]
    fn woodpecker_execute_through_the_application() {
        SupportedWorkflowFormatsTests::woodpecker_execute_through_the_application();
    }
}
