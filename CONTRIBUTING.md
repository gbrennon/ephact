# Contributing to ephact

Thank you for contributing to `ephact`. This document describes the development environment, tooling, available `just` commands, testing strategies, and project conventions.

## Prerequisites

- **Rust**: Edition 2024. The required toolchain is configured in [`rust-toolchain.toml`](rust-toolchain.toml).
- **just**: Command runner for development tasks ([https://github.com/casey/just](https://github.com/casey/just)).
- **cargo-llvm-cov**: Source-based code coverage tool for Rust.
- **actionlint**: Static checker for GitHub and Forgejo Actions workflow files.
- **lefthook**: Git hook manager.

Install the necessary development tools by running:

```sh
just tools
just install-hooks
```

## Just Commands Reference

All common contributor tasks are automated via `just`. Run `just` or `just --list` to display all available recipes.

| Command | Description | Underlying Command |
|---|---|---|
| `just build` | Build the project in debug mode | `cargo build` |
| `just run *args` | Run the application with passthrough arguments | `cargo run -- run {{args}}` |
| `just run-all-workflows *args` | Run all workflows discovered in the repository | `cargo run -- run --all-workflows {{args}}` |
| `just list-workflows` | List workflows in the repository | `cargo run -- list-workflows` |
| `just list-actions` | List actions referenced across workflows | `cargo run -- list-actions` |
| `just test` | Run test suite with coverage enforcement (80% minimum) | `COVERAGE_THRESHOLD=80 ./scripts/check_coverage.sh` |
| `just test-local` | Run tests without coverage (faster for local iteration) | `cargo test` |
| `just lint` | Run Clippy linter with zero-warning enforcement | `cargo clippy -- -D warnings` |
| `just lint-fix +files` | Automatically apply Clippy suggestions to files | `cargo clippy --fix --allow-dirty --allow-staged` |
| `just fmt *files` | Format source code using `rustfmt` | `cargo fmt` |
| `just fmt-check` | Verify formatting without modifying files | `cargo fmt --check` |
| `just tools` | Install required developer components and tools | `rustup component add rustfmt clippy && cargo install cargo-llvm-cov` |
| `just clean` | Remove compiler output and build artifacts | `cargo clean` |
| `just lint-workflows` | Lint Forgejo Actions workflows with `actionlint` | `actionlint -config-file .actionlint.yaml .forgejo/workflows/*.yml` |
| `just install-hooks` | Configure git pre-commit hooks | `lefthook install` |
| `just install-dev` | Install debug binary for local iteration | `cargo install --path . --debug` |
| `just install` | Install release binary to `~/.cargo/bin` | `cargo install --path .` |

## Testing

### Unit and Architecture Tests

Run the full test suite with coverage:

```sh
just test
```

Run tests quickly without calculating coverage:

```sh
just test-local
```

### Container Integration Tests

Real Docker and Podman integration tests require a local container daemon and are feature-gated behind `container-integration`:

```sh
cargo test --features container-integration
```

## Continuous Integration Pipeline

The CI workflow defined in `.forgejo/workflows/ci.yml` runs three stages for every pull request:

1. **Validation**: Checks code formatting (`just fmt-check`) and workflow file syntax (`just lint-workflows`).
2. **Testing**: Runs unit and architecture tests while asserting minimum 80% code coverage (`just test`).
3. **Linting**: Enforces zero Clippy warnings (`just lint`).

Ensure all three checks pass locally before opening a pull request.

## Development Standards

- **Language**: American English (en-US) only.
- **No Emojis**: Never use emojis in code, commits, pull requests, or documentation.
- **No Comments**: Do not write comments in code. Express intent through precise, domain-revealing names.
- **Architecture**: Adhere to hexagonal architecture layer boundaries (`domain`, `application`, `infrastructure`, `presentation`).
- **Complexity**: Keep functions focused. Maximum cyclomatic complexity is 5, and maximum nesting depth is 2.
- **Git Hooks**: Pre-commit hooks are enforced by `lefthook`. Never skip hooks.
