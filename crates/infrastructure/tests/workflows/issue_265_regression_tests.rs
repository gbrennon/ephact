//! Integration regression tests for issue #265.
//!
//! Reproduces the Woodpecker failure reported against the `miru` repository by
//! building a temporary repository on disk and driving it through the real
//! filesystem workflow source, workflow loader, and trigger detector. Before the
//! fix, a mapping-form `steps` block failed to parse (`steps: invalid type: map`)
//! and a `tag` event produced no supported triggers.

use std::{fs, path::Path};

use ephact::{
    application::{
        dtos::requests::LoadWorkflowRequest,
        ports::outbound::{DetectWorkflowTriggerPort, WorkflowLoaderPort, WorkflowSourcePort},
    },
    domain::{
        entities::repository::Repository,
        value_objects::{RefPattern, RepoPath, RepositoryName, TriggerKind, WorkflowTrigger},
    },
    infrastructure::workflows::{
        DetectWorkflowTriggerService, FilesystemWorkflowSource,
        load_workflow_service::LoadWorkflowService,
    },
};
use tempfile::TempDir;

/// Mirrors the shape of `miru`'s `.woodpecker/release.yml`: a tag-triggered
/// pipeline whose `steps` are authored as a name-keyed mapping, with an extra
/// top-level `clone` key and per-step `settings`/`when` that the parser ignores.
const MIRU_RELEASE: &str = r#"
clone:
  git:
    image: woodpeckerci/plugin-git
    settings:
      tags: true

when:
  - event: tag
    ref: refs/tags/v*

steps:
  build:
    image: ubuntu:24.04
    commands:
      - cmake --build build
  package:
    image: ubuntu:24.04
    commands:
      - tar czf pkg.tar.gz dist
  publish-release:
    image: woodpeckerci/plugin-release
    when:
      - event: tag
    settings:
      files:
        - "pkg.tar.gz"
"#;

fn git_repository_dir() -> TempDir {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join(".git")).unwrap();
    tmp
}

fn repository(root: &Path) -> Repository {
    Repository::new(
        RepoPath::new(root.to_path_buf()).unwrap(),
        RepositoryName::new("miru".to_string()).unwrap(),
    )
}

fn write_woodpecker(root: &Path, file: &str, body: &str) {
    let dir = root.join(".woodpecker");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(file), body).unwrap();
}

fn load_release_workflow(root: &Path) -> ephact::domain::aggregates::Workflow {
    let contents = FilesystemWorkflowSource::default()
        .read_all_workflows(&repository(root))
        .unwrap();
    let release = contents
        .iter()
        .find(|item| item.file_name() == "release.yml")
        .expect("release workflow is discovered");
    LoadWorkflowService::new()
        .load(LoadWorkflowRequest::new(
            release.content().to_string(),
            release.file_name().to_string(),
        ))
        .expect("release workflow parses")
}

#[test]
fn the_woodpecker_release_pipeline_is_discovered_and_parses() {
    let tmp = git_repository_dir();
    write_woodpecker(tmp.path(), "release.yml", MIRU_RELEASE);

    let workflow = load_release_workflow(tmp.path());

    assert_eq!(workflow.jobs().len(), 3);
}

#[test]
fn the_tag_pipeline_exposes_a_supported_tag_trigger() {
    let tmp = git_repository_dir();
    write_woodpecker(tmp.path(), "release.yml", MIRU_RELEASE);

    let workflow = load_release_workflow(tmp.path());

    assert!(workflow.triggers_on(TriggerKind::Tag));
}

#[test]
fn the_tag_trigger_preserves_the_declared_ref_pattern() {
    let tmp = git_repository_dir();
    write_woodpecker(tmp.path(), "release.yml", MIRU_RELEASE);

    let workflow = load_release_workflow(tmp.path());
    let tag = workflow
        .trigger()
        .iter()
        .find(|trigger| matches!(trigger, WorkflowTrigger::Tag(_)))
        .expect("a tag trigger is present");

    let filter = tag
        .filter()
        .expect("the tag trigger carries its ref filter");
    assert_eq!(filter.included_refs(), [RefPattern::tag("refs/tags/v*")]);
}

#[test]
fn mapping_form_steps_keep_their_declaration_order() {
    let tmp = git_repository_dir();
    write_woodpecker(tmp.path(), "release.yml", MIRU_RELEASE);

    let workflow = load_release_workflow(tmp.path());

    assert_eq!(
        workflow.jobs().get("step-0").expect("first job").steps()[0].name(),
        Some("build")
    );
    assert_eq!(
        workflow.jobs().get("step-1").expect("second job").steps()[0].name(),
        Some("package")
    );
    assert_eq!(
        workflow.jobs().get("step-1").expect("second job").needs(),
        ["step-0"]
    );
}

#[test]
fn the_trigger_detector_recognizes_the_tag_event_from_repository_content() {
    let tmp = git_repository_dir();
    write_woodpecker(tmp.path(), "release.yml", MIRU_RELEASE);

    let contents = FilesystemWorkflowSource::default()
        .read_all_workflows(&repository(tmp.path()))
        .unwrap();
    let release = contents
        .iter()
        .find(|item| item.file_name() == "release.yml")
        .unwrap();

    assert!(DetectWorkflowTriggerService::new().triggers_on_event(release.content(), "tag"));
}
