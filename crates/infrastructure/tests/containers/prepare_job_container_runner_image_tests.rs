#[cfg(test)]
mod tests {
    use std::path::Path;

    use ephact::{
        application::{
            dtos::requests::PrepareJobContainerRequest,
            ports::outbound::job_container_preparer_port::JobContainerPreparerPort,
        },
        infrastructure::containers::{GITHUB_HOSTED_RUNNER_IMAGE, PrepareJobContainerService},
    };

    use crate::common::fakes::{
        fake_copy_repository_to_container_port::FakeCopyRepositoryToContainerPort,
        fake_create_job_container_port::FakeCreateJobContainerPort,
        fake_pull_job_image_port::FakePullJobImagePort,
    };

    struct HostedRunnerImageTest;

    impl HostedRunnerImageTest {
        fn request() -> PrepareJobContainerRequest {
            PrepareJobContainerRequest::new(
                "build".to_string(),
                None,
                Path::new("/repo").to_path_buf(),
                false,
            )
        }

        fn service(puller: FakePullJobImagePort) -> PrepareJobContainerService {
            PrepareJobContainerService::with_default_image(
                GITHUB_HOSTED_RUNNER_IMAGE,
                Box::new(puller),
                Box::new(FakeCreateJobContainerPort::new()),
                Box::new(FakeCopyRepositoryToContainerPort::new()),
            )
        }
    }

    #[test]
    fn jobs_without_an_explicit_image_use_the_hosted_runner_image() {
        let puller = FakePullJobImagePort::returning("pulled:image");
        let service = HostedRunnerImageTest::service(puller.clone());

        service.prepare(HostedRunnerImageTest::request()).unwrap();

        assert_eq!(
            puller.requested_images(),
            vec![GITHUB_HOSTED_RUNNER_IMAGE.to_string()]
        );
    }
}
