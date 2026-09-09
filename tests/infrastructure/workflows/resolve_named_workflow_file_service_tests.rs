use ephact::infrastructure::workflows::{
    resolve_named_workflow_file_port::ResolveNamedWorkflowFilePort,
    resolve_named_workflow_file_service::ResolveNamedWorkflowFileService,
};
use std::{fs, path::Path};

use ephact::application::dtos::ResolveNamedWorkflowFileRequest;

fn write_file(root: &Path, relative: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, "").unwrap();
}

#[test]
fn execute_resolves_a_path_relative_to_the_repository_root() {
    let tmp = tempfile::tempdir().unwrap();
    write_file(tmp.path(), ".forgejo/workflows/ci.yml");
    let service = ResolveNamedWorkflowFileService::new();

    let path = service
        .execute(ResolveNamedWorkflowFileRequest::new(".forgejo/workflows/ci.yml", tmp.path()))
        .unwrap();

    assert_eq!(path, tmp.path().join(".forgejo/workflows/ci.yml"));
}

#[test]
fn execute_resolves_a_bare_name_under_the_forgejo_directory() {
    let tmp = tempfile::tempdir().unwrap();
    write_file(tmp.path(), ".forgejo/workflows/ci.yml");
    let service = ResolveNamedWorkflowFileService::new();

    let path = service
        .execute(ResolveNamedWorkflowFileRequest::new("ci.yml", tmp.path()))
        .unwrap();

    assert_eq!(path, tmp.path().join(".forgejo/workflows/ci.yml"));
}

#[test]
fn execute_falls_back_to_the_github_directory() {
    let tmp = tempfile::tempdir().unwrap();
    write_file(tmp.path(), ".github/workflows/ci.yml");
    let service = ResolveNamedWorkflowFileService::new();

    let path = service
        .execute(ResolveNamedWorkflowFileRequest::new("ci.yml", tmp.path()))
        .unwrap();

    assert_eq!(path, tmp.path().join(".github/workflows/ci.yml"));
}

#[test]
fn execute_prefers_the_forgejo_directory_when_both_hold_the_name() {
    let tmp = tempfile::tempdir().unwrap();
    write_file(tmp.path(), ".forgejo/workflows/ci.yml");
    write_file(tmp.path(), ".github/workflows/ci.yml");
    let service = ResolveNamedWorkflowFileService::new();

    let path = service
        .execute(ResolveNamedWorkflowFileRequest::new("ci.yml", tmp.path()))
        .unwrap();

    assert_eq!(path, tmp.path().join(".forgejo/workflows/ci.yml"));
}

#[test]
fn execute_errors_when_the_named_workflow_is_absent() {
    let tmp = tempfile::tempdir().unwrap();
    let service = ResolveNamedWorkflowFileService::new();

    let error = service
        .execute(ResolveNamedWorkflowFileRequest::new("missing.yml", tmp.path()))
        .unwrap_err()
        .to_string();

    assert_eq!(error, "workflow file not found: missing.yml");
}
