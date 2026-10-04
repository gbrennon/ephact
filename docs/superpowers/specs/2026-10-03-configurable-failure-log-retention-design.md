# Configurable Failure-Log Retention

## Context

Issue #289 requests a configuration surface for failure diagnostics while preserving the existing 24-hour default. Failure logs are written under the system temporary directory by `FailureLogHandler`, which already limits cleanup to the ephact-owned temporary subtree and `.log` files.

## Scope

This change adds one retention setting and one per-run override. It does not change diagnostic file placement, but it changes generated diagnostic filenames to use an ephact-owned `failure-<run-id>.log` prefix so cleanup can distinguish owned logs from unrelated `.log` files. Cleanup ownership and the behavior of unrelated temporary files remain unchanged.

The precedence is:

1. `ephact run --failure-log-retention-hours <hours>` when supplied.
2. Persisted `failure_log_retention_hours` in `~/.config/ephact/config.toml`.
3. The built-in default of 24 hours.

The existing `ephact settings set <name> <value>` interface exposes the persisted setting as `failure-log-retention-hours`.

## Design

### Domain

`Settings` gains a validated, unit-agnostic positive hour count with a default of 24. The domain stores the value as an integer rather than importing TOML, CLI types, filesystem APIs, or `std::time::Duration`. Its implementation remains pure and agnostic. The value's same-file `#[cfg(test)] mod tests` covers the default, positive values, and rejection of zero.

### Persistence and CLI

`TomlSettings` serializes the optional non-default value as `failure_log_retention_hours`. Missing fields deserialize to 24. The settings command maps `failure-log-retention-hours` to the validated setting and returns a clear user-facing error for zero, negative, non-integer, or overflowing values.

`RunArgs` adds `--failure-log-retention-hours HOURS`. The run handler resolves this optional override against persisted settings before execution. Invalid command-line values fail before the workflow starts. Configuration and CLI conversion tests remain in their owning infrastructure/presentation test modules.

### Wiring

The resolved retention value is held in a shared, cloneable runtime configuration object created by the composition root. The `FailureLogHandler` reads that value when a run starts; startup pruning is moved from handler construction to the first run-start event so the parsed per-run CLI override is available. The CLI updates the shared value immediately before dispatching the run command. The handler converts hours to `Duration` at the infrastructure boundary and uses it for pruning. No second retention mechanism is introduced.

### Ownership safety

Pruning remains restricted to files below the handler's ephact-owned temporary root, within repository-name directories, with the generated `failure-` filename prefix, a `.log` extension, and an expired modification time. Tests will place an unrelated expired `.log` in the same temporary hierarchy and verify it survives. The configured duration changes only expiry, never the ownership filter.

Invalid values fail clearly. Zero, negative, non-numeric, and overflow values are rejected; there is no silent fallback. Omitted values use the documented 24-hour default.

## Validation

Tests will cover:

- default settings produce 24 hours;
- persisted TOML override round-trips;
- settings CLI parsing accepts the supported name;
- zero, negative, non-numeric, and overflowing values fail clearly;
- explicit run override takes precedence over persisted settings;
- configured retention changes pruning boundaries;
- unrelated files are not removed;
- existing diagnostic writing behavior remains unchanged.

Documentation will describe the TOML key, settings command, run override, precedence, default, and invalid-value behavior.

## Out of scope

- Environment-variable configuration.
- Changing the system temporary directory or the ownership-safe diagnostic filename prefix.
- Per-repository retention policies.
- Automatic disk-quota management or background cleanup.
