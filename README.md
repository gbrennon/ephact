# ephact

> *"Simply to survive by avoiding the weaknesses of an unchanging system."* —
> **The Puppet Master**, *Ghost in the Shell* (1995).
> [Why this inspires `ephact`](docs/inspiration.md).

`ephact` is a Rust crate that runs supported workflows locally in containers.
Currently it support GitHub and Forgejo workflows.

The selected repository is bind-mounted read-only at `/workspace` by default.
Pass `--allow-repo-writes` to permit workflow steps to modify the host working tree.

Runner-managed files are container-local, and failed runs write diagnostics under the system temporary directory.

## Quick start

Install `ephact` from crates.io:

```sh
cargo install ephact
```

Select a pull-request workflow interactively:

```sh
ephact run --interactive
```

Choose a workflow by number. `ephact` then walks the inputs discovered from the
workflow and local actions. For each value, enter a literal value or
`env:VARIABLE`; a blank keeps an existing or default value, but an unresolved
required input cannot be blank.

Run a specific pull-request workflow in the current repository:

```sh
ephact run --workflow CI
```

Run a specific pull-request workflow in another Git repository:

```sh
ephact run /path/to/repo --workflow CI
```

Run all discovered workflows that declare `pull_request`:

```sh
ephact run
```

Supported platforms are **Forgejo** and **GitHub**. Workflows are discovered
automatically from `.forgejo/workflows` and `.github/workflows`.

### Subcommands
- `run [PATH]`: Run supported pull-request workflows from a Git repository
  bind-mounted read-only into job containers by default. Use
  `--allow-repo-writes` to enable workflow writes.
- `list-actions [PATH]`: List actions referenced across workflows.

Built on Rust edition **2024**; the toolchain comes from
[`rust-toolchain.toml`](rust-toolchain.toml).


## 1. Using ephact

CLI subcommands, the currently supported pull-request event simulation, secrets
and inputs, and current runtime behavior are documented in
`docs/usage.md`.

## 2. Supported Platforms and Runtimes

Container runtimes (Docker, Podman), Git hosting platforms (GitHub, Forgejo),
workflow scope, and repository safety are documented in
`docs/supported-platforms.md`.

## 3. Architecture

The crate follows hexagonal architecture, with `domain`, `application`,
`infrastructure`, and `presentation` layers under `src/`. Ports, adapters,
command bus coordination, and test suites are documented in
`docs/architecture.md`.

## 4. Contributing and Development

Prerequisites, contribution guidelines, development workflows, and the `just`
command reference for contributors are documented in
`CONTRIBUTING.md` and `docs/development.md`.
