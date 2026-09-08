# ephact

> [!NOTE]
> If you are viewing this repository elsewhere, please be aware that it may be
> a read-only mirror. The original repository lives on Codeberg:
> [https://codeberg.org/gbrennon/ephact](https://codeberg.org/gbrennon/ephact).

`ephact` is a Rust crate that runs actions and workflows locally, in ephemeral
repositories, isolated from your operating system. Workflows execute inside
throwaway containers; by default nothing touches the host filesystem, and
network access is opt-in. Safe by default, real only when you ask for it.

## Quick start

Run a specific workflow in the current repository:

```sh
cargo run -- run --workflow CI --event push
```

Run a workflow in an explicit repository path:

```sh
cargo run -- run /path/to/repo --workflow CI --event push
```

Run all discovered workflows:

```sh
cargo run -- run
```

Supported platforms are **Forgejo** and **GitHub**. Workflows are discovered automatically from `.forgejo/workflows` and `.github/workflows`.

### Subcommands

- `run [PATH]`: Run workflows in an ephemeral copy of the repository (defaults to `.`).
- `list-workflows [PATH]`: List workflow names discovered across supported workflow directories.
- `list-actions [PATH]`: List actions referenced across workflows.

Built on Rust edition **2024**; the toolchain comes from
[`rust-toolchain.toml`](rust-toolchain.toml).
---

## 1. Using ephact

CLI subcommands, flags, event simulation, secrets and inputs, and the opt-in
flags that let runs touch real containers or the network are documented in
[`docs/usage.md`](docs/usage.md).

## 2. Architecture

The crate follows hexagonal architecture, with `domain`, `application`,
`infrastructure`, and `presentation` layers under `src/`. Ports, adapters,
command bus coordination, and test suites are documented in
[`docs/architecture.md`](docs/architecture.md).

---

## 3. Contributing and Development

Prerequisites, contribution guidelines, development workflows, and the `just`
command reference for contributors are documented in
[`CONTRIBUTING.md`](CONTRIBUTING.md) and [`docs/development.md`](docs/development.md).
