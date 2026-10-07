#[cfg(test)]
mod tests {
    use crate::scenarios::{
        forgejo_actions_workflow_run::ForgejoActionsWorkflowRun,
        github_actions_workflow_run::GithubActionsWorkflowRun,
        woodpecker_workflow_run::WoodpeckerWorkflowRun,
    };

    struct SupportedWorkflowFormatsTests;
    const DEFAULT_CONTAINER_IMAGE: &str = "ubuntu:24.04";
    const WOODPECKER_CONTAINER_IMAGE: &str = "debian:bookworm-slim";

    impl SupportedWorkflowFormatsTests {
        fn github_actions_execute_through_the_application() {
            let run = GithubActionsWorkflowRun::execute();

            assert_eq!(run.outcome(), &Ok(()));
            assert!(run.activity().ran_script(GithubActionsWorkflowRun::SCRIPT));
            assert_eq!(
                run.activity().pulled_images(),
                vec![DEFAULT_CONTAINER_IMAGE]
            );
            assert_eq!(run.activity().stopped_containers().len(), 1);
            assert_eq!(run.activity().killed_containers().len(), 1);
        }

        fn forgejo_actions_execute_through_the_application() {
            let run = ForgejoActionsWorkflowRun::execute();

            assert_eq!(run.outcome(), &Ok(()));
            assert!(run.activity().ran_script(ForgejoActionsWorkflowRun::SCRIPT));
            assert_eq!(
                run.activity().pulled_images(),
                vec![DEFAULT_CONTAINER_IMAGE]
            );
            assert_eq!(run.activity().stopped_containers().len(), 1);
            assert_eq!(run.activity().killed_containers().len(), 1);
        }

        fn woodpecker_miru_typos_execute_through_the_application() {
            let run = WoodpeckerWorkflowRun::execute();

            assert_eq!(run.outcome(), &Ok(()));
            assert!(
                run.activity().ran_command_containing(
                    WoodpeckerWorkflowRun::NORMALIZED_DOWNLOAD_URL_FRAGMENT
                )
            );
            assert!(!run.activity().ran_command_containing("$$"));
            assert_eq!(
                run.activity().pulled_images(),
                vec![WOODPECKER_CONTAINER_IMAGE]
            );
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
    fn woodpecker_miru_typos_execute_through_the_application() {
        SupportedWorkflowFormatsTests::woodpecker_miru_typos_execute_through_the_application();
    }
}
