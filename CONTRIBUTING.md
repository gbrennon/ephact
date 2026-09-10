# Contributing to ephact

Thank you for contributing to `ephact`. This document describes the development
environment, tooling, available `just` commands, testing strategies, and project
conventions.

## Prerequisites

- **Rust**: The crate uses edition 2024, as declared in
  [`Cargo.toml`](Cargo.toml). The repository selects the nightly toolchain with
  the `rustfmt` and `clippy` components in
  [`rust-toolchain.toml`](rust-toolchain.toml).
- **just**: Install the development task runner separately
  ([https://github.com/casey/just](https://github.com/casey/just)).
- **actionlint**: Install the workflow-file checker separately.
- **lefthook**: Install the Git hook manager separately.
- **semgrep**: Install it separately if you intend to run `just semgrep`.

After installing those prerequisites, install the Rust development components,
coverage tool, and Git hooks:

```sh
just tools
just install-hooks
```

`just tools` installs `rustfmt`, `clippy`, and `cargo-llvm-cov`.
`just install-hooks` invokes the separately installed `lefthook` executable.

## Just Commands Reference

All common contributor tasks are automated via `just`. Run `just` or
`just --list` to display all available recipes.

| Recipe                         | Description                                                                                                                        | Underlying command                                                                     |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `just`                         | List the available recipes                                                                                                         | `just --list`                                                                          |
| `just build`                   | Build the project in debug mode                                                                                                    | `cargo build`                                                                          |
| `just run *args`               | Run the application with passthrough arguments                                                                                     | `cargo run -- run {{args}}`                                                            |
| `just run-all-workflows *args` | Run all workflows in the repository                                                                                                | `cargo run -- run --all-workflows {{args}}`                                            |
| `just list-workflows`          | List workflows discovered in the repository                                                                                        | `cargo run -- list-workflows`                                                          |
| `just list-actions`            | List actions referenced across workflows                                                                                           | `cargo run -- list-actions`                                                            |
| `just test`                    | Run all default-feature test targets with aggregate line-coverage enforcement at 80%; excludes feature-gated container integration | `COVERAGE_THRESHOLD=80 ./scripts/check_coverage.sh`                                    |
| `just test-local`              | Run all default-feature test targets without coverage; excludes feature-gated container integration                                | `cargo test`                                                                           |
| `just lint`                    | Run Clippy and deny warnings                                                                                                       | `cargo clippy -- -D warnings`                                                          |
| `just lint-fix *args`          | Apply Clippy fixes; optional values are cargo-clippy arguments, not source-file filters                                            | `cargo clippy --fix --allow-dirty --allow-staged {{files}}`                            |
| `just fmt *files`              | Run `cargo fmt`; optional values are forwarded to cargo-fmt                                                                        | `cargo fmt {{files}}`                                                                  |
| `just fmt-check`               | Check formatting without modifying files                                                                                           | `cargo fmt --check`                                                                    |
| `just tools`                   | Install `rustfmt`, `clippy`, and `cargo-llvm-cov`                                                                                  | `rustup component add rustfmt clippy && cargo install cargo-llvm-cov --locked --force` |
| `just clean`                   | Remove Cargo build artifacts                                                                                                       | `cargo clean`                                                                          |
| `just lint-workflows`          | Lint Forgejo Actions workflows                                                                                                     | `actionlint -config-file .actionlint.yaml .forgejo/workflows/*.yml`                    |
| `just semgrep`                 | Run the configured Semgrep rules and report findings as errors                                                                     | `semgrep scan --config .semgrep --error .`                                             |
| `just install-hooks`           | Install the configured `lefthook` Git hooks                                                                                        | `lefthook install`                                                                     |
| `just install-dev`             | Install a debug binary for local iteration                                                                                         | `cargo install --path . --debug`                                                       |
| `just install`                 | Install a release binary to `~/.cargo/bin`                                                                                         | `cargo install --path .`                                                               |

## Testing

### Default-feature Tests

Run all default-feature test targets with aggregate line coverage enforced at
80%:

```sh
just test
```

Run the same default-feature scope without calculating coverage:

```sh
just test-local
```

Neither command enables the feature-gated container integration target.

### Container Integration Tests

The real container integration suite requires at least one available Docker or
Podman daemon and is gated behind the `container-integration` feature:

```sh
cargo test --features container-integration
```

## Continuous Integration Pipeline

The CI workflow in `.forgejo/workflows/ci.yml` runs on pushes to `main`, pull
requests targeting `main`, and manual dispatch:

1. **Validate** runs `cargo fmt --all -- --check`.
1. **Test** runs `just test`, covering default-feature test targets with the 80%
   aggregate line threshold while excluding feature-gated container integration.
1. **Lint** runs `cargo clippy --all-targets --locked -- -D warnings`.

Workflow linting is available locally through `just lint-workflows`, but it is
not currently a CI step.

Before opening a pull request, run the CI commands locally:

```sh
cargo fmt --all -- --check
just test
cargo clippy --all-targets --locked -- -D warnings
```

## Development Standards

- **Manual prose review**: Use American English and avoid emoji in
  documentation, commit messages, and pull requests. These conventions are not
  checked automatically.
- **Comments**: Prefer domain-revealing names over explanatory code comments.
  Use comments when needed for tool directives or non-obvious constraints. This
  is a manual review convention.
- **Architecture**: Adhere to hexagonal architecture layer boundaries (`domain`,
  `application`, `infrastructure`, and `presentation`).
- **Complexity**: Target a maximum control-flow nesting depth of 2.
  `just semgrep` checks the configured nesting rule, but neither CI nor
  `lefthook` runs it. No cyclomatic-complexity threshold is currently automated.
- **Git hooks**: After `lefthook install`, the pre-commit hook formats staged
  Rust paths and runs Clippy auto-fix. It reports when fixes leave staged paths
  modified in the worktree, but that report does not fail the hook or restage
  files. Do not bypass installed hooks. Tests, Semgrep, and workflow lint are
  not part of the current pre-commit configuration.
