#[cfg(test)]
mod tests {
    use ephact::infrastructure::containers::{
        create_job_container_port::CreateJobContainerPort,
        create_job_container_service::CreateJobContainerService,
    };
    use std::{path::Path, sync::Arc};

    use ephact::application::dtos::requests::CreateJobContainerRequest;

    use crate::common::fakes::{
        fake_runtime::FakeRuntime, stub_failing_container_runtime::StubFailingContainerRuntime,
    };

    fn request<'a>(repo_path: &'a Path, allow_repo_writes: bool) -> CreateJobContainerRequest<'a> {
        CreateJobContainerRequest::new(
            "ubuntu:latest",
            "ephemeral-act-build-42",
            "ephemeral-act-build",
            repo_path,
            allow_repo_writes,
        )
    }

    #[test]
    fn execute_removes_the_legacy_name_then_the_current_one_before_creating() {
        let runtime = Arc::new(FakeRuntime::new());
        let service = CreateJobContainerService::new(runtime.clone());

        service.execute(request(Path::new("/repo"), false)).unwrap();

        assert_eq!(
            runtime.removed_containers.lock().clone(),
            vec!["ephemeral-act-build", "ephemeral-act-build-42"]
        );
        assert_eq!(runtime.created_containers.lock().len(), 1);
    }

    #[test]
    fn execute_mounts_the_repository_read_only_by_default() {
        let runtime = Arc::new(FakeRuntime::new());
        let service = CreateJobContainerService::new(runtime.clone());

        service.execute(request(Path::new("/repo"), false)).unwrap();

        let created = runtime.created_containers.lock();
        let config = created.first().unwrap();
        assert_eq!(config.binds(), vec!["/repo:/workspace:ro,Z".to_string()]);
    }

    #[test]
    fn execute_allows_repository_writes_when_opted_in() {
        let runtime = Arc::new(FakeRuntime::new());
        let service = CreateJobContainerService::new(runtime.clone());

        service.execute(request(Path::new("/repo"), true)).unwrap();

        let created = runtime.created_containers.lock();
        let config = created.first().unwrap();
        assert_eq!(config.binds(), vec!["/repo:/workspace:Z".to_string()]);
    }

    #[test]
    fn execute_preserves_container_workspace_configuration() {
        let runtime = Arc::new(FakeRuntime::new());
        let service = CreateJobContainerService::new(runtime.clone());

        service.execute(request(Path::new("/repo"), false)).unwrap();

        let created = runtime.created_containers.lock();
        let config = created.first().unwrap();
        assert_eq!(config.image(), "ubuntu:latest");
        assert_eq!(config.workdir(), Some("/workspace"));
        assert_eq!(
            config.cmd().unwrap(),
            vec!["sleep".to_string(), "infinity".to_string()]
        );
        assert_eq!(config.name(), Some("ephemeral-act-build-42"));
    }

    #[test]
    fn execute_errors_when_the_runtime_cannot_create_the_container() {
        let service = CreateJobContainerService::new(Arc::new(StubFailingContainerRuntime));

        let result = service.execute(request(Path::new("/repo"), false));

        assert!(result.is_err());
    }
}
