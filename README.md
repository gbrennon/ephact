# ephact

> *"Simply to survive by avoiding the weaknesses of an unchanging system."* —
> **The Puppet Master**, *Ghost in the Shell* (1995).
> [Why this inspires `ephact`](docs/inspiration.md).

> [!NOTE]
> If you are viewing this repository elsewhere, please be aware that it may be
> a read-only mirror. The original repository lives on Codeberg:
> [https://codeberg.org/gbrennon/ephact](https://codeberg.org/gbrennon/ephact).

`ephact` is a Rust crate that runs supported Forgejo, GitHub, and Woodpecker
workflows locally in Linux containers using Docker or Podman. When repository
writes are disabled, the selected repository is copied to `/workspace`.
Pass `--allow-repo-writes` to bind-mount the host working tree so workflow
steps can modify it. Runner-managed files are container-local, and failed runs
write diagnostics under the system temporary directory.

## Quick start

Install `ephact` from crates.io:

```sh
cargo install ephact
```

Launch the terminal user interface:

```sh
ephact
```

Running `ephact` with no subcommand opens the TUI, which is the default
interface. The splash screen greets you first; press any key to reach the home
menu.

## Terminal user interface

The TUI is the primary way to use `ephact`. It discovers workflows from the
current repository and drives every action through keyboard navigation.

### Home menu

The home screen lists the available actions:

- **Run workflow** — pick a pull-request workflow, provide its inputs, and watch
  it execute in containers.
- **List workflows** — browse the named workflows discovered in the repository.
- **List actions** — browse the unique action references used across workflows.
- **Settings** — view and edit persisted settings such as the default
  interface.

Navigation from the home menu:

| Key            | Action        |
| -------------- | ------------- |
| `Up`/`Down` or `j`/`k` | Move selection |
| `Enter`        | Open selection |
| `q`            | Quit           |

### Running a workflow

Select **Run workflow** to open the workflow picker. Choose a workflow with the
arrow keys or `j`/`k`, then press `Enter` to configure it. `ephact` walks the
inputs discovered from the workflow and its local actions. For each value, enter
a literal value or `env:VARIABLE`; a blank keeps an existing or default value,
but an unresolved required input cannot be left blank.

Once configured, the workflow runs in Linux containers using Docker or Podman and
live progress streams into the run view. Press `Esc` or `Backspace` to cancel a
run in progress. When it finishes, a run summary reports the workflow status and
each job result. Press `d` to open the step-by-step run details, and use the
arrow keys to scroll.

Interactive workflow selection lists only workflows that declare `pull_request`.
Noninteractive runs require `--event` and select workflows that declare the
requested event.

### Settings

The settings screen edits persisted values in place. Move with the arrow keys or
`j`/`k`, press `Enter` to edit a value, `s` to save, and `Esc` or `Backspace` to
go back. Settings are stored in `~/.config/ephact/config.toml`; missing files
fall back to built-in defaults. The `default-interface` setting selects whether
`ephact` opens the TUI or the CLI when invoked with no subcommand.

### Key reference

| Screen         | Keys                                                  |
| -------------- | ----------------------------------------------------- |
| Splash         | any key to continue                                   |
| Home           | `Up`/`Down`/`j`/`k` move, `Enter` select, `q` quit    |
| List workflows | `Up`/`Down`/`j`/`k` move, `Esc`/`Bksp` back, `q` quit |
| List actions   | `Up`/`Down`/`j`/`k` move, `Esc`/`Bksp` back, `q` quit |
| Run workflow   | `Up`/`Down`/`j`/`k` move, `Enter` configure, `Esc`/`Bksp` back, `d` details, `q` quit |
| Settings       | `Up`/`Down`/`j`/`k` move, `Enter` edit, `s` save, `Esc`/`Bksp` back |

Supported platforms are **Forgejo**, **GitHub**, and **Woodpecker**. Workflows are
discovered automatically from `.forgejo/workflows`, `.github/workflows`, and
`.woodpecker`.

## Command-line interface

The TUI is the default, but `ephact` also exposes scriptable subcommands. An
explicit subcommand takes precedence over the persisted default interface.

- No subcommand: Open the default interface (the TUI unless changed in
  settings).
- `tui`: Open the terminal user interface explicitly.
- `run [PATH]`: Run workflows that declare the requested event from a Git
  repository. Noninteractive runs require `--event`. The repository is copied
  into job containers by default; use `--allow-repo-writes` to enable workflow
  writes to the host working tree.
- `list-workflows [PATH]`: List workflows discovered in a repository.
- `list-actions [PATH]`: List actions referenced across workflows.
- `settings`: Show, update, or reset persisted settings.

Full CLI syntax, options, interactive mode, secrets and inputs, and runtime
behavior are documented in [`docs/usage.md`](docs/usage.md).

Built on Rust edition **2024**; the toolchain comes from
[`rust-toolchain.toml`](rust-toolchain.toml).

## Documentation

- **Usage** — TUI walkthrough, CLI subcommands, supported pull-request event
  simulation, secrets and inputs, and runtime behavior in
  [`docs/usage.md`](docs/usage.md).
- **Architecture** — the crate follows hexagonal architecture, with `domain`,
  `application`, `infrastructure`, and `presentation` workspace crates. Ports,
  adapters, command bus coordination, and test suites are documented in
  [`docs/architecture.md`](docs/architecture.md).
- **Contributing and development** — prerequisites, contribution guidelines,
  development workflows, and the `just` command reference in
  [`CONTRIBUTING.md`](CONTRIBUTING.md) and
  [`docs/development.md`](docs/development.md).
