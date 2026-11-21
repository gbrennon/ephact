#!/usr/bin/env bash
set -euo pipefail

source_file='.forgejo/actions/install-dependencies/action.yml'

# Cargo may live outside HOME/.cargo when the runner configures CARGO_HOME.
grep -Fq 'cargo_bin="${CARGO_HOME:-$HOME/.cargo}/bin"' "$source_file"

# GITHUB_PATH is not consistently applied by all Forgejo-compatible runners;
# persist the complete PATH as a job environment variable as well.
grep -Fq 'echo "PATH=$cargo_bin:$PATH" >> "$GITHUB_ENV"' "$source_file"
grep -Fq 'echo "$cargo_bin" >> "$GITHUB_PATH"' "$source_file"
grep -Fq 'echo "PATH=$cargo_bin:$PATH" >> "$GITHUB_ENV"' "$source_file"

grep -Fq 'echo "PATH=$cargo_bin:$PATH" >> "$GITHUB_ENV"' .forgejo/actions/install-cargo-llvm-cov/action.yml
grep -Fq 'echo "PATH=$cargo_bin:$PATH" >> "$GITHUB_ENV"' .forgejo/actions/install-git-cliff/action.yml

printf '%s\n' 'Rust dependency PATH propagation is configured'
