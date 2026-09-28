# Development

The crate uses Rust edition 2024, as declared in [`Cargo.toml`](../Cargo.toml).
The repository selects the nightly Rust toolchain with the `rustfmt` and
`clippy` components in [`rust-toolchain.toml`](../rust-toolchain.toml).

For complete contribution guidelines, environment setup, and contributor
expectations, see [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Setup

Install `rustup`, [`just`](https://github.com/casey/just), `lefthook`, and
`actionlint` separately. Install `semgrep` and `lizard` separately to run the
configured quality checks.

Then install the Rust development components, coverage tool, and Git hooks:

```sh
just tools
just install-hooks
```

`just tools` installs `rustfmt`, `clippy`, and `cargo-llvm-cov`.
`just install-hooks` invokes the separately installed `lefthook` executable.
The installed pre-push hook runs Semgrep, locked Clippy, and Lizard checks
sequentially, and blocks pushes when any check fails.

## Common Tasks

The project uses `just` for task automation:

| Recipe                         | Description                                                                                                                        | Underlying command                                                                     |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `just`                         | List the available recipes                                                                                                         | `just --list`                                                                          |
| `just build`                   | Build the project in debug mode                                                                                                    | `cargo build`                                                                          |
| `just run *args`               | Run the application with passthrough arguments                                                                                     | `cargo run -- run {{args}}`                                                            |
| `just run-all-workflows *args` | Run all workflows in the repository                                                                                                | `cargo run -- run --all-workflows {{args}}`                                            |
| `just list-workflows`          | List workflows discovered in the repository                                                                                        | `cargo run -- list-workflows`                                                          |
| `just list-actions`            | List actions referenced across workflows                                                                                           | `cargo run -- list-actions`                                                            |
| `just test`                    | Run all default-feature test targets with aggregate line-coverage enforcement at 80%; excludes feature-gated container integration | `COVERAGE_THRESHOLD=80 ./scripts/check_coverage.sh`                                    |
| `just test --crate domain`     | Run coverage for one crate; supported names are `root`, `domain`, `application`, `infrastructure`, and `presentation` | `COVERAGE_THRESHOLD=80 ./scripts/check_coverage.sh --crate domain` |
| `just test-local`              | Run all default-feature test targets without coverage; excludes feature-gated container integration                                | `cargo test`                                                                           |
| `just lint`                    | Run the same locked, all-target Clippy check used by CI                                                                             | `cargo clippy --all-targets --locked -- -D warnings`                                   |
| `just lint-fix *args`          | Apply Clippy fixes; optional values are cargo-clippy arguments, not source-file filters                                            | `cargo clippy --fix --allow-dirty --allow-staged {{files}}`                            |
| `just fmt`                    | Format the entire workspace                                                                                                          | `cargo fmt`                                                                            |
| `just fmt-check`               | Check formatting without modifying files                                                                                           | `cargo fmt --check`                                                                    |
| `just ci`                      | Run the local Validate, Test, and Lint checks in CI order                                                                          | `just fmt-check && just test && just lint`                                              |
| `just tools`                   | Install `rustfmt`, `clippy`, and `cargo-llvm-cov`                                                                                  | `rustup component add rustfmt clippy && cargo install cargo-llvm-cov --locked --force` |
| `just clean`                   | Remove Cargo build artifacts                                                                                                       | `cargo clean`                                                                          |
| `just lint-workflows`          | Lint Forgejo Actions workflows                                                                                                     | `actionlint -config-file .actionlint.yaml .forgejo/workflows/*.yml`                    |
| `just semgrep`                 | Run the configured Semgrep rules and report findings as errors                                                                     | `semgrep scan --config .semgrep --error .`                                             |
| `just lizard`                  | Run Lizard with the repository's complexity, length, argument-count, and ignore thresholds | `lizard -C 5 -L 50 -a 5 -i 0 .` |
| `just install-hooks`           | Install the configured `lefthook` Git hooks                                                                                        | `lefthook install`                                                                     |
| `just install-dev`             | Install a debug binary for local iteration                                                                                         | `cargo install --path . --debug`                                                       |
| `just install`                 | Install a release binary to `~/.cargo/bin`                                                                                         | `cargo install --path .`                                                               |

## Extending CLI Commands

CLI commands are defined with `clap` in `crates/presentation/src/cli`.
When adding a command, keep parsing and execution separate:

1. Add an argument type in its own `*_args.rs` module using `#[derive(clap::Args)]`.
2. Register the type as a variant in `cli/command.rs` with `#[derive(clap::Subcommand)]`.
3. Add the corresponding match arm in `Cli::execute_command` in `cli/cli_app.rs`.
4. Add parser and handler tests, then document the command in `README.md` and
   `docs/usage.md` when it is user-facing.

For example, a new `inspect` command would be registered alongside `run`:

```rust
#[derive(clap::Subcommand)]
enum Command {
    Run(Box<RunArgs>),
    Inspect(InspectArgs), // define InspectArgs in inspect_args.rs
}
```

The existing `run` command is invoked through Cargo's argument separator:

```sh
cargo run -- run --workflow CI
```

The first `--` passes arguments to `ephact`; `run` selects the application
subcommand and `--workflow CI` is parsed by `RunArgs`. Follow the same pattern
for a new command: register it in `Command`, add its arguments, and dispatch it
from `Cli`.

## Running Workflows Locally

Use the existing `just` recipe names to run the checks that correspond to the
Forgejo CI jobs:

| Local command    | CI job              | What it checks |
| ---------------- | ------------------- | -------------- |
| `just fmt-check` | Validate            | Confirms the workspace is formatted with Rustfmt. |
| `just test`      | Test                | Runs the default-feature test suite with the configured 80% coverage threshold. |
| `just lint`      | Lint                | Runs locked Clippy checks for all targets with warnings denied. |
| `just ci`        | Validate, Test, Lint | Runs the three local checks in the same order as CI. |

Run `just ci` before opening a pull request when you want the local equivalent
of the main CI checks. Install the tools from the [Setup](#setup) section
first. The `Verify secrets` CI job is intentionally not part of `just ci`:
it validates repository credentials and can only run reliably inside Forgejo.

## Testing

### Default-feature Tests

Run all default-feature test targets with aggregate line coverage enforced at
80%:

```sh
just test
```

Run coverage for one crate with the explicit `--crate` option:

```sh
just test --crate domain
```

Supported crate names are `root`, `domain`, `application`, `infrastructure`, and
`presentation`.

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
1. **Workflow lint** downloads the pinned actionlint release and validates every
   `.forgejo/workflows/*.yml` file with the repository configuration.
1. **Test** runs `just test`, covering default-feature test targets with the 80%
   aggregate line threshold while excluding feature-gated container integration.
1. **Lint** runs `cargo clippy --all-targets --locked -- -D warnings`.
1. **Verify secrets** verifies repository secrets for same-repository runs.

The Validate, Test, and Lint jobs share the
`.forgejo/actions/setup-rust-environment` composite action. It combines
`install-dependencies` with `cache-rust-deps` and accepts the same cache inputs:

```yaml
- uses: ./.forgejo/actions/setup-rust-environment
  with:
    cache-key-prefix: ephact
    rustc-version: stable
    include-sysroot: "false"
```

Workflow linting is available locally through `just lint-workflows` and runs in
CI as the **Workflow lint** job.

### Repository Secrets

Configure the following secrets in Codeberg repository settings under
**Settings > Actions > Secrets**:

| Secret | Purpose | Source / Value |
| ------ | ------- | -------------- |
| `CARGO_REGISTRY_TOKEN` | Crates.io production publishing | crates.io API token |
| `GH_RELEASE_TOKEN` | GitHub mirror release publishing | GitHub personal access token |

#### Built-in Tokens

`FORGEJO_TOKEN` is automatically created and injected by Forgejo Actions for each
workflow run. Never add `FORGEJO_TOKEN` to repository secrets; use the built-in
`forgejo.token` context instead.
