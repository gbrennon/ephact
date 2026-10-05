use std::sync::Arc;

use ephact::{
    application::{dtos::requests::PullJobImageRequest, ports::outbound::PullJobImagePort},
    infrastructure::containers::PullJobImageService,
};

use crate::common::fakes::{
    fake_runtime::FakeRuntime, stub_failing_container_runtime::StubFailingContainerRuntime,
};

#[test]
fn pulls_the_requested_container_image() {
    let runtime = Arc::new(FakeRuntime::new());
    let service = PullJobImageService::new(runtime.clone());

    let image = service
        .pull(PullJobImageRequest::new("python:3.12-slim".to_string()))
        .unwrap();

    assert_eq!(image, "python:3.12-slim");
    assert_eq!(
        runtime.pulled_images.lock().clone(),
        vec!["python:3.12-slim"]
    );
}

#[test]
fn does_not_translate_or_fallback_from_a_runner_label() {
    let runtime = Arc::new(FakeRuntime::new());
    let service = PullJobImageService::new(runtime.clone());

    let image = service
        .pull(PullJobImageRequest::new("codeberg-medium".to_string()))
        .unwrap();

    assert_eq!(image, "codeberg-medium");
    assert_eq!(
        runtime.pulled_images.lock().clone(),
        vec!["codeberg-medium"]
    );
}

#[test]
fn propagates_a_container_image_pull_failure() {
    let service = PullJobImageService::new(Arc::new(StubFailingContainerRuntime));

    let result = service.pull(PullJobImageRequest::new("ubuntu:24.04".to_string()));

    assert!(result.is_err());
}
