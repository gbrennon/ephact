# Using ephact

`ephact` provides three subcommands to inspect and run workflows locally in ephemeral repositories:

- `run`: Execute CI workflows in an ephemeral copy of a repository.
- `list-workflows`: Discover and list workflow names available in a repository.
- `list-actions`: Discover and list actions referenced across workflows.

## Running Workflows (`ephact run`)

### Syntax

```sh
ephact run [OPTIONS] [PATH]
```

`[PATH]` is an optional positional argument specifying the path to the target repository. It defaults to the current working directory (`.`).

### Options

| Flag | Argument | Description | Default |
|---|---|---|---|
| `[PATH]` | Path | Path to the repository to inspect and run | `.` |
| `--workflow` | `<NAME>` | Name or file of the workflow to run | Run all workflows |
| `--job` | `<JOB>` | Specific job name to run from the selected workflow | Run all jobs |
| `--event` | `<EVENT>` | Event to simulate (`push`, `pull_request`, `workflow_dispatch`, `release`) | `push` |
| `--input` | `<KEY=VALUE>` | Inject a workflow input as `${{ inputs.KEY }}` (repeatable) | None |
| `--secret` | `<KEY[=VALUE]>` | Inject a secret as `${{ secrets.KEY }}`. If `=VALUE` is omitted, reads the value from the host environment (repeatable) | None |
| `--all-workflows` | None | Force running every workflow discovered in the repository | Active if `--workflow` omitted |
| `--preserve` | None | Preserve the ephemeral repository directory after execution instead of removing it | Clean up on exit |
| `--verbose` | None | Show real-time step details (running steps and their output) in addition to the final status of each step | Terse summary |
| `--allow-real-container` | None | Use the real Docker or Podman adapter instead of the default simulated runtime | Disabled |
| `--allow-real-fetcher` | None | Fetch actions from remote forge instead of the local mirror | Disabled |
| `--allow-network` | None | Allow outbound network access inside containers (requires `--allow-real-container`) | Disabled |

### Examples

Run all workflows found in the current repository:

```sh
ephact run
```

Run a specific workflow triggered by a `push` event:

```sh
ephact run --workflow CI --event push
```

Run a single job within a workflow:

```sh
ephact run --workflow CI --job test
```

Pass workflow inputs and read a secret directly from the host environment:

```sh
ephact run --workflow CI --event workflow_dispatch --input greeting=World --secret GITHUB_TOKEN --verbose
```

Run against a specific repository path and preserve the temporary workspace for debugging:

```sh
ephact run /path/to/repo --workflow CI --preserve
```

## Listing Workflows (`ephact list-workflows`)

Inspects workflow definitions found in supported directories (`.forgejo/workflows` and `.github/workflows`) and prints their names.

### Syntax

```sh
ephact list-workflows [PATH]
```

### Examples

List workflows in the current directory:

```sh
ephact list-workflows
```

List workflows in an external repository:

```sh
ephact list-workflows /path/to/repo
```

## Listing Actions (`ephact list-actions`)

Parses workflow files and outputs all unique actions referenced across job steps.

### Syntax

```sh
ephact list-actions [PATH]
```

### Examples

List actions referenced in the current repository:

```sh
ephact list-actions
```

List actions referenced in an external repository:

```sh
ephact list-actions /path/to/repo
```

## Safe by Default

`ephact` is designed to be completely safe to run on untrusted repositories or unverified workflows:

- **Isolated Workspace**: Repositories are copied to an ephemeral directory before execution. Changes never mutate your working directory or Git history.
- **Automatic Teardown**: Ephemeral directories and temporary containers are removed after execution completes, unless `--preserve` is explicitly supplied.
- **Simulated Execution**: By default, steps run within a safe fake runtime that blocks host filesystem writes and outbound network traffic.
- **Local Action Mirroring**: Actions are resolved from a local mirror by default. Remote forge access requires `--allow-real-fetcher`.
- **Secret Protection**: Secrets injected via `--secret` are passed directly into the container execution context and are never written to disk or recorded in permanent logs.
