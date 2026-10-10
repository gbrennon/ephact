# Using ephact

`ephact` provides a terminal user interface by default and supports commands to
inspect workflows, manage settings, and run supported Forgejo, GitHub, and
Woodpecker workflows in Linux containers using Docker or Podman:

- No subcommand: Open the TUI using the persisted default interface.
- `run`: Execute workflows that declare the requested event. Noninteractive runs
  require `--event`; interactive mode selects a `pull_request` workflow. When
  repository writes are disabled, the repository is copied to `/workspace`.
  Use `--allow-repo-writes` to bind-mount it for workflow writes.
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

The workflow then runs in Linux containers using Docker or Podman with live
progress in the run view. Press `Esc` or `Backspace` to cancel a run in
progress. When it finishes, a run summary reports the workflow status and each
job result; press `d` to open the step-by-step run details and scroll with the
arrow keys. Interactive selection supports only workflows declaring
`pull_request`; noninteractive runs require `--event` and match that event.

### Settings screen

The settings screen edits persisted values in place. Move with `Up`/`Down` or
`j`/`k`, press `Enter` to edit a value, `s` to save, and `Esc` or `Backspace` to
go back.

### Key reference

- **Splash**: press any key to continue.
- **Home**: use `Up`/`Down`/`j`/`k` to move, `Enter` to select, or `q` to quit.
- **List workflows**: use `Up`/`Down`/`j`/`k` to move, `Esc`/`Bksp` to go
  back, or `q` to quit.
- **List actions**: use `Up`/`Down`/`j`/`k` to move, `Esc`/`Bksp` to go
  back, or `q` to quit.
- **Run workflow**: use `Up`/`Down`/`j`/`k` to move, `Enter` to configure,
  `Esc`/`Bksp` to go back, `d` for details, or `q` to quit.
- **Settings**: use `Up`/`Down`/`j`/`k` to move, `Enter` to edit, `s` to save,
  or `Esc`/`Bksp` to go back.

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

The persisted settings are:

- `default-interface`: `tui` or `cli`.
- `allow-repo-writes`, `allow-real-container`, `allow-real-fetcher`,
  `allow-network`, `preserve`, `verbose`, `interactive`, and `all-workflows`:
  boolean run defaults.
- `failure-log-retention-hours`: a positive whole number.
- `marker`: one of the built-in markers or custom text.
- `forward-ssh`: whether `run` should forward the host SSH agent by default.

SSH forwarding is infrastructure-owned and defaults to `false`. It is only
resolved for `run`; listing, settings, and other commands never enable it.
Effective forwarding also requires effective `allow-network` and a live Unix
socket in `SSH_AUTH_SOCK`. Set or inspect it with:

```sh
ephact settings set forward-ssh true
ephact settings show
```

The equivalent TOML value is:

```toml
forward_ssh = true
```

`settings reset` disables SSH forwarding as well as restoring every domain
setting to its built-in default.

### Failure-log retention

Failed runs retain failure diagnostics for 24 hours by default. Set the
persisted retention period in `~/.config/ephact/config.toml`:

```toml
failure_log_retention_hours = 48
```

You can set the persisted value from the command line:

```sh
ephact settings set failure-log-retention-hours 48
```

For a single run, use the run option instead:

```sh
ephact run --failure-log-retention-hours 6
```

The run option takes precedence over the persisted setting. If neither is
provided, `ephact` uses the 24-hour default. Retention values must be valid
positive whole numbers; invalid or non-positive values are rejected.

When pruning old diagnostics, `ephact` removes only its own
`failure-*.log` files. Unrelated files and logs are left untouched.

## Running Workflows (`ephact run`)

### Syntax

```sh
ephact run [OPTIONS] [PATH]
```

`[PATH]` is an optional positional path to an existing Git repository. It
defaults to `.`, is canonicalized, and must contain `.git` as either a directory
or a worktree file.

## Workflow options

- `[PATH]`: Existing Git repository to inspect and copy into job containers by
  default. It defaults to `.`.
- `--workflow <NAME>`: Select the workflow whose top-level `name:` matches the
  supplied value. Without it, all workflows matching the event are selected.
- `--job <JOB>`: Accepted by the parser but currently ignored; all jobs in each
  selected workflow execute.
- `--event <EVENT>`: Select the event whose workflows may run. It is required for
  noninteractive execution and forced to `pull_request` in interactive mode.
- `--input <KEY=VALUE>`: Add a string to the run's `inputs` and
  `github.event.inputs` contexts. Repeatable; later duplicate keys win.
- `--interactive`: Select a pull-request workflow by number, then enter values
  for discovered workflow and local-action inputs.
- `--secret <KEY[=VALUE]>`: Inject a secret as `${{ secrets.KEY }}`. Without
  `=VALUE`, read the value from the host environment. Repeatable.
- `--all-workflows`: Run every discovered workflow declaring the requested event.
  It is the default without `--workflow` and wins when both options are given.
- `--preserve`: Accepted by the parser but currently has no effect.
- `--allow-repo-writes`: Bind-mount the host repository so workflow steps can
  modify its working tree.
- `--verbose`: Show workflow and job lifecycle details, live step output, and
  failure diagnostics in addition to step start/finish status.
- `--allow-real-container`: Accepted but currently has no effect; a real Docker
  or Podman runtime on Linux is always auto-detected and used.
- `--allow-real-fetcher`: Accepted but currently has no effect; uncached remote
  actions are fetched from their forge by default.
- `--allow-network`: Allow steps classified as requiring network access; remote-
  mutation policy violations remain skipped.
- `--forward-ssh`: Forward the host SSH agent socket into job containers.
  Requires `--allow-network` and a valid `SSH_AUTH_SOCK` Unix socket.
- `--failure-log-retention-hours <HOURS>`: Retain failure diagnostics for the
  specified positive number of hours. The default is `24`.

## Examples

Run all discovered workflows that declare `pull_request`:

```sh
ephact run --event pull_request
```

Run a specific workflow that declares `pull_request`:

```sh
ephact run --event pull_request --workflow CI
```

Select a pull-request workflow interactively:

```sh
ephact run --interactive
```

The menu lists only workflows declaring `pull_request` and accepts a numeric
selection. It then asks once per discovered workflow or local-action input.
Enter a literal or `env:VARIABLE`; a blank keeps an existing or default value
and is rejected for an unresolved required input.

In interactive mode, the numeric selection replaces any `--workflow` value,
disables `--all-workflows`, and forces `pull_request`. In noninteractive mode,
`--event` is required and workflows declaring that event are selected.

`--job` is accepted but does not filter jobs; all jobs in each selected workflow
execute.

Pass run inputs and read a secret from the host environment while showing
verbose progress:

```sh
ephact run --event pull_request --workflow CI --input greeting=World --secret GITHUB_TOKEN --verbose
```

The selected workflow must declare the requested event. Interactive execution
always selects and simulates `pull_request`.

Run a named pull-request workflow from another Git repository:

```sh
ephact run /path/to/repo --event pull_request --workflow CI
```

## Listing Workflows (`ephact list-workflows`)

Inspects workflow definitions found in supported directories
(`.forgejo/workflows`, `.github/workflows`, and `.woodpecker`) and prints their
names.

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

`ephact` requires and auto-detects a reachable Docker or Podman runtime on Linux.
Jobs without an explicit `container:` or step image use the
runner-compatible `ghcr.io/catthehacker/ubuntu:act-24.04` image. Jobs with an
explicit image keep that image, including explicit Woodpecker step images.
When repository writes are disabled, the selected repository is copied into
`/workspace`. Pass `--allow-repo-writes` to bind-mount the host repository so
workflow steps can modify it. Runner-managed files are container-local. Pulling
job images and cloning uncached remote actions into a persistent host cache can
use the network. Containers use the runtime's default network behavior.

After a normally completed run, `ephact` makes a best-effort attempt to remove
recorded job containers. Failed runs also attempt cleanup, but failures can
leave containers behind. Failed runs produce external diagnostics under the
system temporary directory. `--preserve` currently has no effect.

Treat secrets as visible to the workflow. Workflow steps can print them or write
them into the container workspace. With `--allow-repo-writes`, those writes can
reach the host repository, and `--verbose` relays step output without secret
redaction. Review untrusted workflows before running them.
