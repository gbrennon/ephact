# Modularize Cargo LLVM Coverage Installation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Install the pinned `cargo-llvm-cov` tool only in Forgejo Actions that run the test suite.

**Architecture:** Extract the existing installation logic into a focused local composite action. Keep general dependency setup limited to tools needed by all jobs, and invoke the new action from the shared test action immediately before coverage tests.

**Tech Stack:** Forgejo composite actions, Bash, Rust toolchain, `cargo-llvm-cov v0.8.7`, `actionlint`.

## Global Constraints

- Keep `cargo-llvm-cov` pinned to `v0.8.7`.
- Keep all workflows on Forgejo Actions.
- Do not change runner labels, test commands, coverage thresholds, or application code.
- Reference the new installation action only from `.forgejo/actions/run-tests/action.yml`.
- Preserve the existing architecture selection, download URL, installation path, and already-installed guard.

---

### Task 1: Extract the coverage-tool installer

**Files:**
- Create: `.forgejo/actions/install-cargo-llvm-cov/action.yml`
- Modify: `.forgejo/actions/install-dependencies/action.yml`

**Interfaces:**
- Consumes: runner architecture and `$HOME/.cargo/bin`.
- Produces: an executable `cargo-llvm-cov` at `$HOME/.cargo/bin/cargo-llvm-cov`.

- [ ] **Step 1: Create the focused composite action**

Create `.forgejo/actions/install-cargo-llvm-cov/action.yml` with the existing pinned installer:

```yaml
name: Install cargo-llvm-cov
description: Install the pinned cargo-llvm-cov release

runs:
  using: composite

  steps:
    - name: Install cargo-llvm-cov
      shell: bash
      run: |
        export PATH="$HOME/.cargo/bin:$PATH"
        if command -v cargo-llvm-cov >/dev/null 2>&1; then
          echo "cargo-llvm-cov already installed: $(cargo llvm-cov --version)"
          exit 0
        fi

        arch=$(uname -m)
        case "$arch" in
          x86_64) target="x86_64-unknown-linux-gnu" ;;
          aarch64) target="aarch64-unknown-linux-gnu" ;;
          *)
            echo "unsupported architecture: $arch" >&2
            exit 1
            ;;
        esac

        curl -fsSL --proto '=https' --tlsv1.2 \
          -o /tmp/cargo-llvm-cov.tar.gz \
          "https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.8.7/cargo-llvm-cov-${target}.tar.gz"
        tar -xzf /tmp/cargo-llvm-cov.tar.gz -C "$HOME/.cargo/bin"
        rm -f /tmp/cargo-llvm-cov.tar.gz
        "$HOME/.cargo/bin/cargo" llvm-cov --version
```

- [ ] **Step 2: Remove the installer from general dependencies**

Delete only the `Install cargo-llvm-cov` step from
`.forgejo/actions/install-dependencies/action.yml`. Leave system packages, `just`, Rust,
rustfmt, and clippy installation unchanged.

- [ ] **Step 3: Verify extraction statically**

Run:

```sh
grep -RIn "cargo-llvm-cov\|cargo llvm-cov" .forgejo/actions
```

Expected: the installation logic and `v0.8.7` URL occur only in
`.forgejo/actions/install-cargo-llvm-cov/action.yml`; no installation step remains in
`install-dependencies/action.yml`.

- [ ] **Step 4: Commit the extraction**

```sh
git add .forgejo/actions/install-cargo-llvm-cov/action.yml \
  .forgejo/actions/install-dependencies/action.yml
git commit -m "ci: isolate coverage tool installation"
```

### Task 2: Install the tool only for test runs

**Files:**
- Modify: `.forgejo/actions/run-tests/action.yml`

**Interfaces:**
- Consumes: the local `install-cargo-llvm-cov` action.
- Produces: the existing `just test` behavior with its coverage tool available.

- [ ] **Step 1: Add the local installer before tests**

Change the action steps to:

```yaml
runs:
  using: composite

  steps:
    - uses: ./.forgejo/actions/install-cargo-llvm-cov
    - shell: bash
      run: just test
```

- [ ] **Step 2: Verify the consumer boundary**

Run:

```sh
grep -RIn "install-cargo-llvm-cov" .forgejo
```

Expected: exactly one reference, in `.forgejo/actions/run-tests/action.yml`.

- [ ] **Step 3: Lint the Forgejo actions**

Run:

```sh
just lint-workflows
```

Expected: `actionlint` exits successfully.

- [ ] **Step 4: Run repository verification**

Run:

```sh
cargo fmt --all -- --check
just test-local
cargo clippy --all-targets --locked -- -D warnings
```

Expected: all commands exit successfully. The full `just test` coverage command may be run when
`cargo-llvm-cov` is available locally.

- [ ] **Step 5: Commit the test-action wiring**

```sh
git add .forgejo/actions/run-tests/action.yml
git commit -m "ci: install coverage tool for tests"
```
