#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use ephact::{
        application::{
            dtos::{
                requests::{BuildJobEnvironmentRequest, PrepareJobContainerRequest},
                responses::{BuildJobEnvironmentResponse, ExecResultResponse},
            },
            ports::outbound::{
                ContainerPort, JobContainerPreparerPort,
                job_environment_builder_port::JobEnvironmentBuilderPort,
            },
        },
        infrastructure::{
            containers::{
                ContainerRuntimeAdapter, CreateJobContainerService, GITHUB_HOSTED_RUNNER_IMAGE,
                PrepareJobContainerService, PullJobImageService, RepositoryContainerCopyAdapter,
            },
            workflows::workflow_document::WorkflowDocument,
        },
    };

    const COMCHAN_TYPOS_WORKFLOW: &str = r#"
name: Check for typos
on:
  push:
    branches: ["main"]
  pull_request:
    branches: ["main"]
jobs:
  typos-check:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Repository
        uses: actions/checkout@v6.0.2
      - name: Run typos checker
        uses: crate-ci/typos@master
"#;

    struct ContainerCleanup(Arc<dyn ContainerPort>);

    impl Drop for ContainerCleanup {
        fn drop(&mut self) {
            let _ = self.0.remove();
        }
    }

    struct HostedRunnerImageE2eTests;

    impl HostedRunnerImageE2eTests {
        fn run_baseline_check() -> Option<ExecResultResponse> {
            let workflow = WorkflowDocument::parse(COMCHAN_TYPOS_WORKFLOW)
                .unwrap()
                .into_domain();
            let job = workflow.job_named("typos-check").unwrap();

            assert_eq!(job.runs_on(), Some("ubuntu-latest"));
            assert!(job.container().is_none());

            let job_environment =
                ephact::infrastructure::jobs::RunnerEnvironmentAdapter::new().build(
                    BuildJobEnvironmentRequest::new(workflow.clone(), job.env().clone()),
                );
            let runtime = match ContainerRuntimeAdapter::detect() {
                Ok(runtime) => Arc::new(runtime),
                Err(error) => {
                    eprintln!("SKIP: container runtime unavailable: {error:?}");
                    return None;
                }
            };

            Some(Self::execute_baseline_check(runtime, &job_environment))
        }

        fn execute_baseline_check(
            runtime: Arc<ContainerRuntimeAdapter>,
            job_environment: &BuildJobEnvironmentResponse,
        ) -> ExecResultResponse {
            let repository = tempfile::tempdir().unwrap();
            let service = PrepareJobContainerService::with_default_image(
                GITHUB_HOSTED_RUNNER_IMAGE,
                Box::new(PullJobImageService::new(runtime.clone())),
                Box::new(CreateJobContainerService::new(runtime)),
                Box::new(RepositoryContainerCopyAdapter::new()),
            );
            let request = PrepareJobContainerRequest::new(
                "comchan-typos-check".to_string(),
                None,
                Path::new(repository.path()).to_path_buf(),
                false,
            );

            let prepared = service.prepare(request).unwrap();
            let container = ContainerCleanup(prepared.into_container());
            container
                .0
                .exec(
                    &[
                        "bash".to_string(),
                        "-lc".to_string(),
                        "test \"$RUNNER_TOOL_CACHE\" = /opt/hostedtoolcache && \
                         test \"$RUNNER_TEMP\" = /tmp && \
                         command -v wget && command -v jq && command -v node"
                            .to_string(),
                    ],
                    None,
                    job_environment.env(),
                )
                .unwrap()
        }
    }

    #[test]
    fn comchan_implicit_runner_job_has_the_required_tool_baseline() {
        let Some(result) = HostedRunnerImageE2eTests::run_baseline_check() else {
            return;
        };

        assert_eq!(
            result.exit_code(),
            0,
            "stdout: {}; stderr: {}",
            result.stdout(),
            result.stderr(),
        );
        assert!(result.stdout().contains("wget"));
        assert!(result.stdout().contains("jq"));
        assert!(result.stdout().contains("node"));
    }
}
