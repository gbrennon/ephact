# Workflow and Action Filename Names Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make every supported workflow and local action use its parsed `name` when present and its source filename, including extension, otherwise.

**Architecture:** Keep filename discovery and fallback resolution in `ephact-infrastructure`. Extend the workflow source boundary with source-file metadata so application loading can pass the resolved name into existing domain objects without teaching the domain about files or vendors. Make action YAML names optional and resolve them in the infrastructure action loader.

**Tech Stack:** Rust 2024, Cargo workspace, serde/serde_yaml, existing hexagonal ports and filesystem adapters.

## Global Constraints

- Name resolution is vendor-agnostic and implemented in infrastructure.
- Parsed non-empty names take precedence over filenames.
- Filenames include their extension, such as `build.yml` and `action.yaml`.
- Empty or whitespace-only names use the filename fallback.
- Add tests before production changes and observe each new test fail.
- Use American English and do not add code comments.
- Run the full Cargo test suite and repository quality checks before completion.

---

## File Map

- Modify `crates/infrastructure/src/workflows/workflow_source_adapter.rs` to resolve workflow names from parsed content and source paths, and return source metadata.
- Modify `crates/application/src/ports/outbound/workflow_source_port.rs` to define the source-file metadata contract.
- Add the source-file response type beside the existing application DTOs.
- Modify workflow loading requests/services and workflow execution/discovery consumers to preserve source filenames.
- Modify `crates/infrastructure/src/workflows/yaml/workflow_yaml.rs` only if parsing needs to normalize blank names.
- Modify `crates/infrastructure/src/workflows/yaml/action_definition_yaml.rs` to accept an optional YAML name.
- Modify `crates/infrastructure/src/actions/load_action_definition_service.rs` to resolve the selected definition filename.
- Extend infrastructure workflow and action tests, plus affected application fakes and service tests.

## Task 1: Add the infrastructure name-resolution contract

**Files:**
- Create: `crates/infrastructure/src/workflows/source_name.rs`
- Modify: `crates/infrastructure/src/workflows/mod.rs`
- Test: `crates/infrastructure/tests/workflows/source_name_tests.rs`

**Interfaces:**
- Produces `pub(crate) fn resolve_source_name(parsed_name: Option<&str>, source_file: &Path) -> Option<String>`.
- Produces `pub(crate) fn source_file_name(source_file: &Path) -> Option<String>`.

- [ ] **Step 1: Write failing tests**

Cover a non-empty parsed name, a missing parsed name, whitespace-only input, and a path with an extension. Assert that the result is the parsed name or the basename including extension.

- [ ] **Step 2: Run the focused test and verify the expected failure**

Run `cargo test -p ephact-infrastructure source_name -- --nocapture`.
Expected: compilation failure because the helper module does not yet exist.

- [ ] **Step 3: Implement the smallest helper**

Trim the parsed value, return it when non-empty, otherwise use `source_file.file_name()` and convert it to an owned string. Return `None` only when neither source is representable as UTF-8.

- [ ] **Step 4: Run the focused test and verify it passes**

Run `cargo test -p ephact-infrastructure source_name -- --nocapture`.
Expected: all focused tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/infrastructure/src/workflows/source_name.rs crates/infrastructure/src/workflows/mod.rs crates/infrastructure/tests/workflows/source_name_tests.rs
git commit -m "feat(workflows): add shared source name resolution"
```

## Task 2: Carry workflow source filenames through the workflow port

**Files:**
- Create: `crates/application/src/dtos/responses/workflow_source_file_response.rs`
- Modify: `crates/application/src/dtos/responses/mod.rs`
- Modify: `crates/application/src/ports/outbound/workflow_source_port.rs`
- Modify: `crates/infrastructure/src/workflows/shared_workflow_source.rs`
- Modify: `crates/infrastructure/src/workflows/workflow_source_adapter.rs`
- Modify: application, infrastructure, presentation, and root workflow-source fakes implementing the port
- Test: affected workflow source and service tests

**Interfaces:**
- `WorkflowSourceFileResponse::new(content: String, file_name: String)`.
- Accessors `content(&self) -> &str` and `file_name(&self) -> &str`.
- `WorkflowSourcePort::read_workflow` returns `Result<WorkflowSourceFileResponse, WorkflowSourceError>`.
- `WorkflowSourcePort::read_all_workflows` returns `Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError>`.

- [ ] **Step 1: Add failing contract tests**

Update filesystem source tests to assert that named and unnamed files return their content plus `ci.yml` or the relevant basename. Add a test that selecting an unnamed workflow by `ci.yml` succeeds.

- [ ] **Step 2: Run affected tests and verify the expected failure**

Run `cargo test -p ephact-infrastructure filesystem_workflow_source -- --nocapture`.
Expected: compilation failures from the new response type and changed trait signatures.

- [ ] **Step 3: Implement the response and adapter changes**

Return source metadata from the filesystem adapter, use `resolve_source_name` for matching named workflows, and preserve the existing error behavior for empty directories and missing names. Update delegating adapters and fakes with the exact new response type.

- [ ] **Step 4: Run focused tests and verify they pass**

Run `cargo test -p ephact-infrastructure filesystem_workflow_source -- --nocapture` and `cargo test -p ephact-application --lib`.
Expected: focused infrastructure and application tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/application crates/infrastructure crates/presentation tests
git commit -m "refactor(workflows): preserve source filenames"
```

## Task 3: Resolve workflow names during loading and execution

**Files:**
- Modify: `crates/application/src/dtos/requests/load_workflow_request.rs`
- Modify: `crates/infrastructure/src/workflows/load_workflow_service.rs`
- Modify: `crates/application/src/services/run_workflow_service.rs`
- Modify: `crates/application/src/services/run_all_workflows_service.rs`
- Modify: `crates/infrastructure/src/workflows/discover_run_inputs_service.rs`
- Modify: all affected workflow loader fakes and tests

**Interfaces:**
- `LoadWorkflowRequest::new(content: String, file_name: String)`.
- The loader attaches the already-resolved source filename to the domain workflow through the existing file metadata API.

- [ ] **Step 1: Write failing behavior tests**

Add a workflow-loader test for a content-only workflow with `ci.yml`, and execution tests asserting that progress and summaries report `ci.yml`; retain a test proving `name: CI` remains `CI`.

- [ ] **Step 2: Run the focused tests and verify failure**

Run `cargo test -p ephact-application run_workflow -- --nocapture` and the infrastructure loader tests.
Expected: the new assertions fail because workflow loading currently has no source filename.

- [ ] **Step 3: Implement metadata propagation**

Pass `WorkflowSourceFileResponse` into `LoadWorkflowRequest`, attach its filename to the parsed workflow, and replace workflow display/report fallbacks that use `unwrap_or("unnamed")` with the resolved workflow name. Keep raw content consumers using `content()`.

- [ ] **Step 4: Run focused tests and verify they pass**

Run the application workflow service tests and infrastructure workflow tests.
Expected: named and filename-fallback behavior passes without changing existing named workflow output.

- [ ] **Step 5: Commit**

```bash
git add crates/application crates/infrastructure
git commit -m "feat(workflows): use source filename as fallback name"
```

## Task 4: Resolve action definition names from their selected filename

**Files:**
- Modify: `crates/infrastructure/src/workflows/yaml/action_definition_yaml.rs`
- Modify: `crates/infrastructure/src/actions/load_action_definition_service.rs`
- Modify: `crates/infrastructure/tests/actions/load_action_definition_service_tests.rs`
- Modify: action definition parser tests and affected fixtures

**Interfaces:**
- `ActionDefinitionYaml.name` becomes optional at the infrastructure parsing boundary.
- `LoadActionDefinitionService` constructs the domain definition with the parsed name or the selected `action.yml`/`action.yaml` filename.

- [ ] **Step 1: Write failing tests**

Add tests for missing and blank names in both `action.yml` and `action.yaml`, asserting `action.yml` and `action.yaml` respectively. Retain the existing named-action assertions.

- [ ] **Step 2: Run the focused tests and verify failure**

Run `cargo test -p ephact-infrastructure load_action_definition -- --nocapture`.
Expected: parsing fails or the returned name is not the filename because the YAML field is currently required.

- [ ] **Step 3: Implement the loader fallback**

Make the YAML name optional, select the definition file as today, resolve the name with the shared infrastructure helper, and construct the existing domain `ActionDefinition` with the resulting string. Preserve missing-file and malformed-YAML errors.

- [ ] **Step 4: Run focused tests and verify they pass**

Run the action loader and action parser tests.
Expected: named, missing-name, blank-name, and both extension cases pass.

- [ ] **Step 5: Commit**

```bash
git add crates/infrastructure/src/actions crates/infrastructure/src/workflows/yaml crates/infrastructure/tests
git commit -m "feat(actions): use definition filename as fallback name"
```

## Task 5: Verify the complete change

**Files:**
- Modify: documentation only if behavior descriptions require correction.

- [ ] **Step 1: Run formatting and static checks**

Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and the repository quality command from `justfile`.

- [ ] **Step 2: Run the complete test suite**

Run `cargo test --workspace --all-targets`.
Expected: all tests pass with zero failures.

- [ ] **Step 3: Review the diff and status**

Run `git diff --check`, `git status --short`, and `git diff HEAD~4 --stat`. Confirm only the filename naming change and its tests are present.

- [ ] **Step 4: Commit any narrowly scoped documentation correction**

Use a Conventional Commit such as `docs(workflows): document filename name fallback` only if the verification reveals a user-facing document that is now inaccurate.
