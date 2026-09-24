#[cfg(test)]
mod tests {
    use std::fs;

    use ephact::{
        application::{
            dtos::{requests::DiscoverRunInputsRequest, responses::RunInputDeclarationResponse},
            ports::outbound::DiscoverRunInputsPort,
        },
        domain::{RepoPath, Repository, RepositoryName, value_objects::ActRunConfig},
        infrastructure::workflows::{FilesystemRunInputDiscoveryService, FilesystemWorkflowSource},
    };

    #[test]
    fn discovers_required_workflow_and_local_action_inputs() {
        let temporary = tempfile::tempdir().unwrap();
        fs::create_dir(temporary.path().join(".git")).unwrap();
        let workflow_dir = temporary.path().join(".forgejo/workflows");
        let action_dir = temporary.path().join(".forgejo/actions/check");
        fs::create_dir_all(&workflow_dir).unwrap();
        fs::create_dir_all(&action_dir).unwrap();
        fs::write(
            workflow_dir.join("ci.yml"),
            "name: CI\non:\n  workflow_dispatch:\n    inputs:\n      workflow_name:\n        required: true\n      optional:\n        required: false\n        default: false\n        type: boolean\njobs:\n  test:\n    steps:\n      - uses: ./.forgejo/actions/check\n",
        )
        .unwrap();
        fs::write(
            action_dir.join("action.yml"),
            "name: Check\ninputs:\n  action_name:\n    required: true\n    default: false\nruns:\n  using: composite\n  steps:\n    - run: echo check\n",
        )
        .unwrap();
        let repository = Repository::new(
            RepoPath::new(temporary.path().to_path_buf()).unwrap(),
            RepositoryName::new("test".to_string()).unwrap(),
        );
        let service =
            FilesystemRunInputDiscoveryService::new(Box::new(FilesystemWorkflowSource::new(&[
                ".forgejo/workflows",
            ])));

        let declarations = service
            .execute(DiscoverRunInputsRequest::new(
                ActRunConfig::new(),
                repository,
            ))
            .unwrap();
        let mut names = declarations
            .iter()
            .map(RunInputDeclarationResponse::name)
            .collect::<Vec<_>>();
        names.sort_unstable();

        assert_eq!(names, vec!["action_name", "optional", "workflow_name"]);
        let optional = declarations
            .iter()
            .find(|declaration| declaration.name() == "optional")
            .expect("optional input");
        assert_eq!(optional.input_type(), Some("boolean"));
        let action = declarations
            .iter()
            .find(|declaration| declaration.name() == "action_name")
            .expect("action input");
        assert_eq!(action.default(), Some("false"));
    }
}
