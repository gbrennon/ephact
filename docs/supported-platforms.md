# Supported Platforms and Runtimes

## Container Runtimes

`ephact` requires a reachable Docker or Podman runtime and automatically detects one at startup.

The detection order is:

1. **Docker** — tried first
2. **Podman** — fallback if Docker is not reachable

Execution fails with an error when neither runtime is reachable.

See [`docs/usage.md#runtime-and-safety`](docs/usage.md#runtime-and-safety) for details on the read-only `/workspace` mount, `--allow-repo-writes`, networking, cleanup, and secret-handling policies.

## Git Hosting Platforms

The currently supported Git hosting platforms and their workflow discovery directories are:

| Platform | Workflow Directory |
|----------|-------------------|
| Forgejo  | `.forgejo/workflows` |
| GitHub   | `.github/workflows` |

Workflows are discovered automatically from these directories.

## Workflow Scope

The current execution path handles workflows declaring the `pull_request` event, matching the existing usage documentation. This does not imply complete support for every GitHub Actions or Forgejo Actions feature.

## Repository Safety

Runtime and safety details — read-only `/workspace` mount, `--allow-repo-writes`, networking, cleanup, and secret handling — are documented in [`docs/usage.md`](docs/usage.md).