# TUI First Screen Design

## Scope

Add the first TUI slice to `ephact`, launched with `ephact tui`. The TUI uses `ratatui` and is isolated under a top-level Rust `tui` module with screen submodules.

## User flow

- `ephact tui` switches to the terminal alternate screen and enables raw input.
- The splash screen displays `assets/project_emblem.txt`, centered with the project name.
- Any key advances from the splash screen to the home screen.
- The home screen displays the project title, a short description, and the initial menu entries:
  - Run workflow
  - List workflows
  - List actions
- The menu is presentational for this first slice; actions are not implemented yet.
- `q` exits from either screen.
- Terminal raw mode and alternate-screen state are restored on both normal exit and errors.

## Module structure

```text
src/tui/
├── mod.rs
├── app.rs
├── event.rs
└── screen/
    ├── mod.rs
    ├── splash.rs
    └── home.rs
```

`app.rs` owns the screen state and event loop. `event.rs` owns terminal input polling. Each screen module owns only its rendering function. `mod.rs` exposes the TUI entry point.

## Integration

Add `ratatui` as a dependency. Add a `Tui`/`TuiArgs` command to the existing Clap command model and route it from the application entry point. Existing non-TUI commands retain their current behavior.

The emblem is loaded using a compile-time path relative to the crate source so running the installed binary does not depend on the current working directory.

## Testing

- Test that CLI parsing recognizes `tui`.
- Test screen state transitions: splash advances on a key and home exits on `q`.
- Test screen render functions with a ratatui test backend for the emblem, title, and menu labels.
- Run the existing test suite and formatting/lint checks.

## Non-goals

Workflow execution, menu selection, mouse input, resizing behavior, and persistent TUI settings are outside this first slice.
