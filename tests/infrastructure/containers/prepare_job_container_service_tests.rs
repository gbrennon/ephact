#[cfg(test)]
mod tests {
    use ephact::{
        application::ports::outbound::prepare_job_container_port::PrepareJobContainerPort,
        infrastructure::containers::prepare_job_container_service::PrepareJobContainerService,
    };
    use std::path::Path;

    use ephact::application::dtos::requests::PrepareJobContainerRequest;

    use crate::common::fakes::{
        fake_copy_repository_to_container_port::FakeCopyRepositoryToContainerPort,
        fake_create_job_container_port::FakeCreateJobContainerPort,
        fake_pull_job_image_port::FakePullJobImagePort,
    };

    fn request<'a>(repo_path: &'a Path) -> PrepareJobContainerRequest<'a> {
        PrepareJobContainerRequest::new("build", Some("ubuntu-latest"), repo_path, false)
    }

    fn request_with_writes<'a>(repo_path: &'a Path) -> PrepareJobContainerRequest<'a> {
        PrepareJobContainerRequest::new("build", Some("ubuntu-latest"), repo_path, true)
    }

    #[test]
    fn execute_names_the_container_after_the_job_and_the_process() {
        let service = PrepareJobContainerService::new(
            Box::new(FakePullJobImagePort::returning("ubuntu:latest")),
            Box::new(FakeCreateJobContainerPort::new()),
            Box::new(FakeCopyRepositoryToContainerPort::new()),
        );

        let prepared = service.execute(request(Path::new("/repo"))).unwrap();

        let name = prepared.container_name();
        let pid = std::process::id();
        assert!(
            name.starts_with(&format!("ephemeral-act-build-{}", pid)),
            "Container name '{}' should start with 'ephemeral-act-build-{}'",
            name,
            pid
        );
    }

    #[test]
    fn execute_passes_the_legacy_container_name_to_the_creator() {
        let creator = FakeCreateJobContainerPort::new();
        let service = PrepareJobContainerService::new(
            Box::new(FakePullJobImagePort::returning("ubuntu:latest")),
            Box::new(creator.clone()),
            Box::new(FakeCopyRepositoryToContainerPort::new()),
        );

        service.execute(request(Path::new("/repo"))).unwrap();

        let legacy_names = creator.legacy_container_names();
        assert_eq!(legacy_names.len(), 1);
        assert_eq!(legacy_names[0], "ephemeral-act-build");

        let container_names = creator.container_names();
        assert_eq!(container_names.len(), 1);
        let name = &container_names[0];
        let pid = std::process::id();
        assert!(
            name.starts_with(&format!("ephemeral-act-build-{}", pid)),
            "Container name '{}' should start with 'ephemeral-act-build-{}'",
            name,
            pid
        );
    }

    #[test]
    fn execute_creates_the_container_from_the_pulled_image() {
        let creator = FakeCreateJobContainerPort::new();
        let service = PrepareJobContainerService::new(
            Box::new(FakePullJobImagePort::returning("mapped:image")),
            Box::new(creator.clone()),
            Box::new(FakeCopyRepositoryToContainerPort::new()),
        );

        service.execute(request(Path::new("/repo"))).unwrap();

        assert_eq!(creator.images(), vec!["mapped:image".to_string()]);
    }

    #[test]
    fn execute_propagates_a_pull_failure() {
        let creator = FakeCreateJobContainerPort::new();
        let service = PrepareJobContainerService::new(
            Box::new(FakePullJobImagePort::failing("no such image")),
            Box::new(creator.clone()),
            Box::new(FakeCopyRepositoryToContainerPort::new()),
        );

        let Err(error) = service.execute(request(Path::new("/repo"))) else {
            panic!("a failing image pull should fail the preparation");
        };
        let error = error.to_string();

        assert_eq!(error, "no such image");
        assert!(creator.images().is_empty());
    }

    #[test]
    fn execute_copies_repository_to_container_in_default_isolated_mode() {
        let copier = FakeCopyRepositoryToContainerPort::new();
        let service = PrepareJobContainerService::new(
            Box::new(FakePullJobImagePort::returning("ubuntu:latest")),
            Box::new(FakeCreateJobContainerPort::new()),
            Box::new(copier.clone()),
        );

        service.execute(request(Path::new("/repo"))).unwrap();

        let requests = copier.requests();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].contains("/repo"));
        assert!(requests[0].contains("/workspace"));
    }

    #[test]
    fn execute_skips_repository_copy_when_allowing_repo_writes() {
        let copier = FakeCopyRepositoryToContainerPort::new();
        let service = PrepareJobContainerService::new(
            Box::new(FakePullJobImagePort::returning("ubuntu:latest")),
            Box::new(FakeCreateJobContainerPort::new()),
            Box::new(copier.clone()),
        );

        service
            .execute(request_with_writes(Path::new("/repo")))
            .unwrap();

        assert_eq!(copier.requests().len(), 0);
    }

    #[test]
    fn execute_propagates_copy_failure() {
        let service = PrepareJobContainerService::new(
            Box::new(FakePullJobImagePort::returning("ubuntu:latest")),
            Box::new(FakeCreateJobContainerPort::new()),
            Box::new(FakeCopyRepositoryToContainerPort::failing("copy failed")),
        );

        let result = service.execute(request(Path::new("/repo")));

        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "copy failed");
        }
    }
}
