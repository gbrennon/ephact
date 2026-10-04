# Configurable Failure-Log Retention Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement issue #289 by making ephact failure-log retention configurable through persisted settings and a per-run CLI override while preserving the 24-hour default and ownership-safe cleanup.

**Architecture:** Keep the domain pure by storing only a validated positive hour count in `Settings`; TOML and CLI layers own parsing and error messages. Add a cloneable infrastructure retention store shared by `FailureLogHandler` and presentation wiring. Apply the resolved value before each run and prune on `RunStarted`, not during container construction, so CLI precedence is real.

**Tech Stack:** Rust 2024, clap, serde/toml, tempfile, existing domain/application/infrastructure/presentation crates, same-file Rust unit-test modules.

## Global Constraints

- The default retention is exactly 24 hours.
- Domain code must not import TOML, clap, filesystem APIs, or `std::time::Duration`.
- Domain unit tests must be in the implementation file's final `#[cfg(test)] mod tests` block.
- Invalid zero, negative, non-numeric, and overflowing configuration values fail clearly; omitted values use the default.
- Pruning must only remove ephact-owned `failure-*.log` files below the ephact temporary subtree.
- Unrelated files, including unrelated expired `.log` files, must remain.
- Do not add environment-variable configuration, per-repository policies, or background cleanup.

---

### Task 1: Add the pure domain retention value

**Files:**
- Modify: `crates/domain/src/aggregates/settings.rs`
- Test: same file, final `#[cfg(test)] mod tests` block

**Interfaces:**
- Add `Settings::failure_log_retention_hours() -> u64`.
- Add `Settings::with_failure_log_retention_hours(self, hours: u64) -> Result<Self, String>`.
- Add `Settings::DEFAULT_FAILURE_LOG_RETENTION_HOURS: u64 = 24` or an equivalent private default used by `Default`.

- [ ] **Step 1: Write the failing domain tests**

Add tests in `settings.rs`'s existing final test module:

```rust
#[test]
fn default_failure_log_retention_is_24_hours() {
    assert_eq!(Settings::default().failure_log_retention_hours(), 24);
}

#[test]
fn positive_failure_log_retention_replaces_default() {
    let settings = Settings::default()
        .with_failure_log_retention_hours(72)
        .expect("positive retention is valid");

    assert_eq!(settings.failure_log_retention_hours(), 72);
}

#[test]
fn zero_failure_log_retention_is_rejected() {
    let error = Settings::default()
        .with_failure_log_retention_hours(0)
        .expect_err("zero retention must be invalid");

    assert!(error.contains("greater than zero"));
}
```

- [ ] **Step 2: Run the domain tests and verify the expected failure**

Run:

```bash
cargo test -p ephact-domain settings::tests::default_failure_log_retention_is_24_hours
```

Expected: compilation failure because the retention accessor and builder do not exist.

- [ ] **Step 3: Implement the minimal pure setting**

Add a `failure_log_retention_hours: u64` field, initialize it to `24` in `Default`, expose the accessor, and validate the builder with a `hours == 0` error. Do not import serialization, CLI, filesystem, or duration types.

- [ ] **Step 4: Run the same-file domain tests**

Run:

```bash
cargo test -p ephact-domain settings::tests
```

Expected: all settings tests pass.

- [ ] **Step 5: Commit the domain change**

```bash
git add crates/domain/src/aggregates/settings.rs
git commit -m "feat(settings): add validated failure-log retention"
```

---

### Task 2: Persist and expose the setting through the existing settings CLI

**Files:**
- Modify: `crates/infrastructure/src/persistence/settings/toml_settings.rs`
- Modify: `crates/presentation/src/cli/settings_command.rs`
- Modify: `crates/presentation/src/cli/cli_app.rs`
- Test: existing settings tests in `crates/infrastructure/src/persistence/settings/toml_settings.rs` and `crates/presentation/src/cli/cli_app.rs` test modules

**Interfaces:**
- TOML key: `failure_log_retention_hours`.
- Settings CLI enum variant: `SettingName::FailureLogRetentionHours`, rendered as `failure-log-retention-hours` by clap.
- `ephact settings set failure-log-retention-hours <hours>` updates `Settings` or returns a clear error.

- [ ] **Step 1: Write failing persistence and CLI tests**

Add tests proving that a missing TOML key maps to 24, a TOML value round-trips, and the settings command rejects `0`, `-1`, and non-numeric values. Use the existing temporary settings store and CLI test fixtures rather than mocks that only echo arguments.

Representative assertions:

```rust
assert_eq!(Settings::default().failure_log_retention_hours(), 24);
let persisted: Settings = toml::from_str::<TomlSettings>(
    "failure_log_retention_hours = 72",
)
.expect("valid retention TOML")
.into();
assert_eq!(persisted.failure_log_retention_hours(), 72);
assert!(update_setting(
    Settings::default(),
    SettingName::FailureLogRetentionHours,
    "0",
).is_err());
assert!(update_setting(
    Settings::default(),
    SettingName::FailureLogRetentionHours,
    "not-a-number",
).is_err());
```

- [ ] **Step 2: Run the focused tests and verify the expected failures**

Run:

```bash
cargo test -p ephact-infrastructure persistence::settings
cargo test -p ephact-presentation cli::cli_app
```

Expected: compilation or assertion failures because the TOML field, setting name, and update branch do not exist.

- [ ] **Step 3: Implement TOML conversion and CLI validation**

Add the optional/defaulted TOML field, map it to/from `Settings`, add the clap `SettingName` variant, parse with `u64::parse`, reject zero, and call the domain builder. Extend settings rendering to show `failure-log-retention-hours = <value>`.

- [ ] **Step 4: Run focused persistence and CLI tests**

Run the two commands from Step 2. Expected: all focused tests pass, including malformed and overflowing input errors.

- [ ] **Step 5: Commit the settings interface**

```bash
git add crates/infrastructure/src/persistence/settings/toml_settings.rs crates/presentation/src/cli/settings_command.rs crates/presentation/src/cli/cli_app.rs
git commit -m "feat(settings): persist failure-log retention"
```

---

### Task 3: Wire runtime precedence and ownership-safe pruning

**Files:**
- Modify: `crates/infrastructure/src/logging/failure_log_handler.rs`
- Modify: `crates/infrastructure/src/di/container.rs`
- Modify: `crates/presentation/src/composition_root/root.rs`
- Modify: `crates/presentation/src/cli/cli_app.rs`
- Modify: `crates/presentation/src/cli/run_args.rs`
- Test: `crates/infrastructure/src/logging/failure_log_handler.rs` final test module and presentation CLI tests

**Interfaces:**
- Add a cloneable `FailureLogRetentionStore` with default `24` hours and `set_hours(u64)` / `hours() -> u64`.
- Add the store to `FailureLogStores` so composition passes one shared instance to both the handler and `Cli`.
- Add `RunArgs::failure_log_retention_hours() -> Option<u64>` and `--failure-log-retention-hours HOURS`.
- `Cli::execute_run` applies explicit CLI hours to the shared store after `args.apply_settings(&self.settings)` and before `RunHandler::handle_with_preflight_output_and_diagnostics`.
- `FailureLogHandler` prunes on `Event::RunStarted` using the current shared hours, not in its constructor.

- [ ] **Step 1: Write the failing runtime and ownership tests**

Add tests for:

```rust
#[test]
fn unrelated_expired_log_is_not_pruned() {
    // Create tmp/ephact/project/expired.log and tmp/ephact/project/failure-run.log.
    // Age both, construct the handler, trigger RunStarted, and assert only failure-run.log is removed.
}

#[test]
fn configured_retention_controls_pruning_boundary() {
    // Create an owned failure log whose age is older than one hour but newer than 24 hours.
    // Set the shared store to one hour, trigger RunStarted, and assert it is removed.
}
```

Add a CLI test that parses `--failure-log-retention-hours 72` and confirms the explicit value wins over persisted settings.

- [ ] **Step 2: Run the focused tests and verify the expected failures**

Run:

```bash
cargo test -p ephact-infrastructure logging::failure_log_handler
cargo test -p ephact-presentation cli::run_args
```

Expected: the new tests fail because pruning currently runs during construction, all `.log` files are considered owned, and the CLI option/store do not exist.

- [ ] **Step 3: Implement the shared runtime store and handler changes**

Create the cloneable store in the logging module, initialize it at 24 hours, move pruning into the `RunStarted` event path, convert the current hours to `Duration` only in infrastructure, and require the generated `failure-` prefix in `is_expired_log`. Change generated paths from `<run-id>.log` to `failure-<run-id>.log`.

- [ ] **Step 4: Wire persisted and explicit values**

Include the retention store in `FailureLogStores`, pass it through `Container`/`CompositionRoot` into `Cli`, set it from loaded `Settings` in `Cli::with_settings`, and replace it with the explicit run value in `execute_run` after applying persisted settings. Ensure invalid clap values fail before dispatch.

- [ ] **Step 5: Run focused runtime tests**

Run the commands from Step 2. Expected: default retention, override precedence, pruning boundary, unrelated-log preservation, and existing diagnostic-writing tests pass.

- [ ] **Step 6: Commit the runtime feature**

```bash
git add crates/infrastructure/src/logging/failure_log_handler.rs crates/infrastructure/src/di/container.rs crates/presentation/src/composition_root/root.rs crates/presentation/src/cli/cli_app.rs crates/presentation/src/cli/run_args.rs
git commit -m "feat(logging): configure failure-log retention"
```

---

### Task 4: Document the configuration and run repository verification

**Files:**
- Modify: `docs/usage.md`
- Test: documentation examples are verified through the CLI parsing and settings tests from Tasks 2 and 3

- [ ] **Step 1: Add documentation**

Document the TOML key, `ephact settings set failure-log-retention-hours <hours>`, `ephact run --failure-log-retention-hours <hours>`, precedence, 24-hour default, rejection of non-positive/invalid values, and the fact that only ephact-owned `failure-*.log` files are pruned.

- [ ] **Step 2: Run formatting and focused verification**

```bash
cargo fmt --all -- --check
cargo test -p ephact-domain
cargo test -p ephact-infrastructure
cargo test -p ephact-presentation
```

Expected: exit code 0 with no test failures.

- [ ] **Step 3: Run repository-required quality gates**

```bash
just test
just fmt-check
just lint
just semgrep
```

Expected: every command exits 0. Investigate any finding on a modified file rather than suppressing it.

- [ ] **Step 4: Commit documentation**

```bash
git add docs/usage.md
git commit -m "docs(settings): document failure-log retention"
```

- [ ] **Step 5: Confirm the final diff is scoped**

```bash
git status --porcelain
git diff origin/main...HEAD --stat
git diff origin/main...HEAD --check
```

Expected: only the retention spec, domain setting, persistence/CLI wiring, logging/runtime wiring, tests, and documentation are present.
