use ephact::{
    application::dtos::requests::ListWorkflowsRequest, infrastructure::RepositoryResolver,
    presentation::cli::parse_list_workflows_test_args,
};

fn current_dir_repository() -> ephact::domain::Repository {
    RepositoryResolver::resolve_from_path(".").unwrap()
}

#[test]
fn to_domain_returns_ok() {
    let args = parse_list_workflows_test_args(&[]);

    let result = args.to_domain();

    let expected = ListWorkflowsRequest::new(
        current_dir_repository().path().as_path().to_path_buf(),
        current_dir_repository().name().as_str().to_string(),
    );
    assert_eq!(result.unwrap(), expected);
}
