# Using ephact

`ephact` provides a terminal user interface by default and supports commands to
inspect workflows, manage settings, and run supported pull-request workflows in
Docker or Podman containers:

- No subcommand: Open the TUI using the persisted default interface.
- `run`: Execute pull-request workflows with the selected Git repository mounted
  read-only at `/workspace` by default. Use `--allow-repo-writes` to enable
  workflow writes.
- `list-workflows`: Discover and list named workflows in a repository.
- `list-actions`: Discover and list unique action references across workflows.
- `settings`: Show, update, or reset persisted settings.

An explicit subcommand takes precedence over the persisted default interface.
Use `ephact tui` to open the TUI explicitly.

## Terminal User Interface

The TUI is the default interface and the primary way to use `ephact`. Launch it
by running `ephact` with no subcommand, or `ephact tui` explicitly. A splash
screen appears first; press any key to reach the home menu.

### Home menu

The home menu lists the available actions: **Run workflow**, **List workflows**,
**List actions**, and **Settings**. Move the selection with `Up`/`Down` or
`j`/`k`, open it with `Enter`, and quit with `q`.

### Running a workflow

Select **Run workflow** to open the workflow picker. Choose a workflow, then
press `Enter` to configure it. `ephact` walks the inputs discovered from the
workflow and its local actions. For each value, enter a literal value or
`env:VARIABLE`; a blank keeps an existing or default value and is rejected for an
unresolved required input.

The workflow then runs in Docker or Podman containers with live progress in the
run view. Press `Esc` or `Backspace` to cancel a run in progress. When it
finishes, a run summary reports the workflow status and each job result; press
`d` to open the step-by-step run details and scroll with the arrow keys. Only
workflows declaring `pull_request` are eligible, and execution simulates that
event.

### Settings screen

The settings screen edits persisted values in place. Move with `Up`/`Down` or
`j`/`k`, press `Enter` to edit a value, `s` to save, and `Esc` or `Backspace` to
go back.

### Key reference

| Screen         | Keys                                                  |
| -------------- | ----------------------------------------------------- |
| Splash         | any key to continue                                   |
| Home           | `Up`/`Down`/`j`/`k` move, `Enter` select, `q` quit    |
| List workflows | `Up`/`Down`/`j`/`k` move, `Esc`/`Bksp` back, `q` quit |
| List actions   | `Up`/`Down`/`j`/`k` move, `Esc`/`Bksp` back, `q` quit |
| Run workflow   | `Up`/`Down`/`j`/`k` move, `Enter` configure, `Esc`/`Bksp` back, `d` details, `q` quit |
| Settings       | `Up`/`Down`/`j`/`k` move, `Enter` edit, `s` save, `Esc`/`Bksp` back |

## Settings (`ephact settings`)

Settings are stored in `~/.config/ephact/config.toml`. Missing files use the
built-in defaults. Use `settings show` to display the effective persisted
settings and the configuration path:

```sh
ephact settings show
```

Update one setting with a typed value:

```sh
ephact settings set default-interface cli
```

Restore all built-in defaults:

```sh
ephact settings reset
```

Persisted execution defaults apply when the corresponding `run` option is
absent. Explicit command-line options override them for the current invocation.

## Running Workflows (`ephact run`)

### Syntax

```sh
ephact run [OPTIONS] [PATH]
```

`[PATH]` is an optional positional path to an existing Git repository. It
defaults to `.`, is canonicalized, and must contain `.git` as either a directory
or a worktree file.

### Options

| Flag                     | Argument        | Description                                                                                                                                   | Default                        |
| ------------------------ | --------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `[PATH]`                 | Path            | Existing Git repository to inspect and mount read-only into job containers by default | `.`                            |
| `--workflow`             | `<NAME>`        | Exact value of the workflow's top-level `name:` field                                                                                         | All pull-request workflows     |
| `--job`                  | `<JOB>`         | Accepted by the parser but currently ignored; all jobs in each selected workflow execute                                                      | No effect                      |
| `--event`                | `<EVENT>`       | Accepted by the parser but currently ignored; execution supports and simulates only `pull_request`                                            | `pull_request` (forced)        |
| `--input`                | `<KEY=VALUE>`   | Add a string to the run's `inputs` and `github.event.inputs` contexts (repeatable; later duplicate keys win)                                  | None                           |
| `--interactive`          | None            | Select a pull-request workflow by number, then enter a literal or `env:VARIABLE` value for each discovered workflow or local-action input     | Disabled                       |
| `--secret`               | `<KEY[=VALUE]>` | Inject a secret as `${{ secrets.KEY }}`. If `=VALUE` is omitted, reads the value from the host environment (repeatable)                       | None                           |
| `--all-workflows`        | None            | Run every discovered workflow declaring `pull_request`; the default without `--workflow`; wins and ignores the name if both options are given | Active if `--workflow` omitted |
| `--preserve`             | None            | Accepted by the parser but currently has no effect                                                                                            | No effect                      |
| `--allow-repo-writes`    | None            | Permit workflow steps to modify the host repository through the workspace mount | Disabled                       |
| `--verbose`              | None            | Show workflow and job lifecycle details, live step output, and failure diagnostics in addition to step start/finish status                     | Step start/finish status       |
| `--allow-real-container` | None            | Accepted but currently has no effect; a real Docker or Podman runtime is always auto-detected and used                                        | No effect                      |
| `--allow-real-fetcher`   | None            | Accepted but currently has no effect; uncached remote actions are fetched from their forge by default                                         | No effect                      |
| `--allow-network`        | None            | Accepted but currently has no effect; containers use the runtime's default network behavior                                                   | No effect                      |

### Examples

Run all discovered workflows that declare `pull_request`:

```sh
ephact run
```

Run a specific workflow that declares `pull_request`:

```sh
ephact run --workflow CI
```

Select a pull-request workflow interactively:

```sh
ephact run --interactive
```

The menu lists only workflows declaring `pull_request` and accepts a numeric
selection. It then asks once per discovered workflow or local-action input.
Enter a literal or `env:VARIABLE`; a blank keeps an existing or default value
and is rejected for an unresolved required input.

In interactive mode, the numeric selection replaces any `--workflow` value and
disables `--all-workflows`. As in other modes, `--event` is ignored and
`pull_request` is forced.

`--job` is accepted but does not filter jobs; all jobs in each selected workflow
execute.

Pass run inputs and read a secret from the host environment while showing
verbose progress:

```sh
ephact run --workflow CI --input greeting=World --secret GITHUB_TOKEN --verbose
```

The selected workflow must declare `pull_request`; current execution always
simulates that event.

Run a named pull-request workflow from another Git repository:

```sh
ephact run /path/to/repo --workflow CI
```

## Listing Workflows (`ephact list-workflows`)

Inspects workflow definitions found in supported directories
(`.forgejo/workflows` and `.github/workflows`) and prints their names.

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

Parses workflow files and outputs the final action names derived from the
references used across job steps. Different references with the same final
component may therefore produce the same displayed name.

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

## Runtime and Safety

`ephact` requires and auto-detects a reachable Docker or Podman runtime. The
selected repository is bind-mounted read-only at `/workspace` by default.
Pass `--allow-repo-writes` when workflow steps must modify the host working
tree. Runner-managed files are container-local. Pulling job images and cloning
uncached remote actions into a persistent host cache can use the network.
Containers use the runtime's default network behavior.

After a normally completed run, `ephact` makes a best-effort attempt to remove
recorded job containers. Failed runs produce external diagnostics under the
system temporary directory. `--preserve` currently has no effect.

Treat secrets as visible to the workflow. Workflow steps can print them or write
them into the mounted repository, and `--verbose` relays step output without
secret redaction. Review untrusted workflows before running them.
