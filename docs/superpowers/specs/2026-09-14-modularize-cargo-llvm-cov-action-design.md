# Modularize Cargo LLVM Coverage Installation

This design isolates `cargo-llvm-cov` installation to Forgejo Actions that run coverage tests.

## Architecture

Create `.forgejo/actions/install-cargo-llvm-cov/action.yml` as a composite action with one
responsibility: install the pinned `cargo-llvm-cov` release used by the project.

Move the existing version `v0.8.7` installation logic from
`.forgejo/actions/install-dependencies/action.yml` into the new action. Preserve the existing
architecture selection, download URL, installation path, and already-installed guard.

## Workflow usage

Keep general dependency installation limited to system packages, `just`, Rust, `rustfmt`, and
`clippy`.

Invoke the new action from `.forgejo/actions/run-tests/action.yml` immediately before `just test`.
The test action is used by the CI test job and the build workflow's test step, so both coverage
runs receive the required tool while validation and lint jobs do not install it.

No runner labels, test commands, coverage thresholds, or application code change.

## Validation

Run the Forgejo workflow linter and inspect the changed action references. Run the repository's
available formatting, test, lint, and structural quality checks when dependencies permit.

Success requires the new action to be referenced only by `run-tests`, the general dependency action
to contain no `cargo-llvm-cov` installation, and the existing pinned version to remain unchanged.
