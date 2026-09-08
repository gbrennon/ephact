# Development

`ephact` requires Rust edition 2024 (see [`rust-toolchain.toml`](../rust-toolchain.toml)), [`just`](https://github.com/casey/just), and `cargo-llvm-cov` for coverage.

For complete contribution guidelines, environment setup, and contributor expectations, see [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Setup

Install the required development components and git hooks:

```sh
just tools
just install-hooks
```

## Common Tasks

The project uses `just` for task automation:

| Recipe | Description | Underlying Command |
|---|---|---|
| `just build` | Build the project in debug mode | `cargo build` |
| `just run *args` | Run the application with passthrough arguments | `cargo run -- run {{args}}` |
| `just run-all-workflows *args` | Run all workflows in the repository | `cargo run -- run --all-workflows {{args}}` |
| `just list-workflows` | List workflows discovered in the repository | `cargo run -- list-workflows` |
| `just list-actions` | List actions referenced across workflows | `cargo run -- list-actions` |
| `just test` | Run all tests with coverage enforcement (threshold: 80%) | `COVERAGE_THRESHOLD=80 ./scripts/check_coverage.sh` |
| `just test-local` | Run tests without coverage (faster local iteration) | `cargo test` |
| `just lint` | Lint with Clippy (zero warnings enforced) | `cargo clippy -- -D warnings` |
| `just lint-fix +files` | Automatically apply Clippy suggestions | `cargo clippy --fix --allow-dirty --allow-staged` |
| `just fmt *files` | Format source files with `cargo fmt` | `cargo fmt` |
| `just fmt-check` | Check formatting without modifying files | `cargo fmt --check` |
| `just tools` | Install required dev tools (`rustfmt`, `clippy`, `cargo-llvm-cov`) | `rustup component add rustfmt clippy && cargo install cargo-llvm-cov` |
| `just clean` | Remove build artifacts and temporary files | `cargo clean` |
| `just lint-workflows` | Lint Forgejo Actions workflows with `actionlint` | `actionlint -config-file .actionlint.yaml .forgejo/workflows/*.yml` |
| `just install-hooks` | Install `lefthook` pre-commit hooks | `lefthook install` |
| `just install-dev` | Install debug binary for local iteration | `cargo install --path . --debug` |
| `just install` | Install release binary to `~/.cargo/bin` | `cargo install --path .` |

## Testing

### Unit and Architecture Tests

Run tests with test coverage enforcement:

```sh
just test
```

Run tests directly without calculating code coverage:

```sh
just test-local
```

### Container Integration Tests

Real Docker and Podman integration tests require a running container daemon and are feature-gated:

```sh
cargo test --features container-integration
```

## Continuous Integration Pipeline

The CI workflow in `.forgejo/workflows/ci.yml` runs three required jobs on pull requests and pushes to `main`:

1. **Validate**: Verifies code formatting (`cargo fmt --all -- --check`).
2. **Test**: Runs the full test suite with code coverage assertions (`.forgejo/actions/run-tests`).
3. **Lint**: Enforces zero Clippy warnings (`cargo clippy --all-targets --locked -- -D warnings`).
