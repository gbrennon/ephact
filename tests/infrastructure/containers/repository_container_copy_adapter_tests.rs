#[cfg(test)]
mod tests {
    use std::fs::{create_dir_all, write};

    use ephact::{
        application::dtos::requests::CopyRepositoryToContainerRequest,
        infrastructure::containers::{
            CopyRepositoryToContainerPort, RepositoryContainerCopyAdapter,
        },
    };
    use tempfile::tempdir;

    use crate::common::fakes::stub_recording_container::StubRecordingContainer;

    #[test]
    fn execute_preserves_git_metadata_for_ci_commands() {
        let repository = tempdir().expect("repository directory");
        let git_config = repository.path().join(".git/config");
        create_dir_all(repository.path().join(".git")).expect("git directory");
        write(&git_config, "[core]\n").expect("git configuration");
        write(repository.path().join("workflow.yml"), "name: CI\n").expect("workflow file");
        let container = StubRecordingContainer::new();
        let request = CopyRepositoryToContainerRequest::new(
            repository.path().to_path_buf(),
            "/workspace".to_string(),
        );

        RepositoryContainerCopyAdapter::new()
            .execute(request, &container)
            .expect("repository copy");

        let mut copied_paths = container
            .copied_files()
            .first()
            .expect("copied file entries")
            .iter()
            .map(|file| file.path().to_string())
            .collect::<Vec<_>>();
        copied_paths.sort();

        assert_eq!(
            copied_paths,
            vec![".git/config".to_string(), "workflow.yml".to_string()]
        );
    }

    #[test]
    fn execute_excludes_nested_worktrees_from_repository_archive() {
        let repository = tempdir().expect("repository directory");
        let worktree_plan = repository
            .path()
            .join(".worktrees/chore-enforce-quality/docs/superpowers/plans");
        create_dir_all(&worktree_plan).expect("worktree plan directory");
        write(repository.path().join("workflow.yml"), "name: CI\n").expect("workflow file");
        write(
            worktree_plan.join("2026-09-14-modularize-cargo-llvm-cov-action.md"),
            "worktree plan",
        )
        .expect("worktree file");
        let container = StubRecordingContainer::new();
        let request = CopyRepositoryToContainerRequest::new(
            repository.path().to_path_buf(),
            "/workspace".to_string(),
        );

        RepositoryContainerCopyAdapter::new()
            .execute(request, &container)
            .expect("repository copy");

        let copied_files = container.copied_files();
        let copied_paths = copied_files
            .first()
            .expect("copied file entries")
            .iter()
            .map(|file| file.path().to_string())
            .collect::<Vec<_>>();
        assert_eq!(container.copied_paths(), vec!["/workspace".to_string()]);
        assert_eq!(copied_paths, vec!["workflow.yml".to_string()]);
    }
}
