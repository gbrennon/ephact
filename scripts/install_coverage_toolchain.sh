#!/usr/bin/env bash
set -euo pipefail

readonly coverage_toolchain="nightly"
readonly coverage_component="llvm-tools-preview"

if ! command -v rustup >/dev/null 2>&1; then
  printf 'ERROR: rustup is required to install the %s coverage toolchain.\n' \
    "$coverage_toolchain" >&2
  exit 1
fi

if ! rustup toolchain install "$coverage_toolchain" \
  --profile minimal --no-self-update; then
  printf 'ERROR: failed to install %s coverage toolchain noninteractively.\n' \
    "$coverage_toolchain" >&2
  exit 1
fi

if ! rustup component add \
  --toolchain "$coverage_toolchain" "$coverage_component"; then
  printf 'ERROR: failed to install %s for the %s coverage toolchain noninteractively.\n' \
    "$coverage_component" "$coverage_toolchain" >&2
  exit 1
fi
