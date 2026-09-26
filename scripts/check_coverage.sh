#!/usr/bin/env bash
set -euo pipefail

# shellcheck source=scripts/lib/common.sh
source "$(dirname "$0")/lib/common.sh"

readonly COVERAGE_THRESHOLD="${COVERAGE_THRESHOLD:-80}"

print_usage_and_exit() {
  local message="$1"
  printf 'ERROR: %s\n\n' "$message" >&2
  printf 'Usage: %s [--crate <name>]\n' "$0" >&2
  printf 'Supported crates: root, domain, application, infrastructure, presentation\n' >&2
  exit 2
}

parse_arguments() {
  coverage_package=""
  if [ "$#" -eq 0 ]; then
    return
  fi
  [ "$#" -eq 2 ] || print_usage_and_exit "expected --crate <name>"
  [ "$1" = "--crate" ] || print_usage_and_exit "unknown option: $1"

  case "$2" in
    root) coverage_package="ephact" ;;
    domain) coverage_package="ephact-domain" ;;
    application) coverage_package="ephact-application" ;;
    infrastructure) coverage_package="ephact-infrastructure" ;;
    presentation) coverage_package="ephact-presentation" ;;
    *) print_usage_and_exit "unknown crate: $2" ;;
  esac
}

run_coverage_and_emit_json() {
  echo "Cleaning the instrumented build to avoid stale coverage artifacts..."
  cargo llvm-cov clean
  echo "Running cargo-llvm-cov (generating JSON report)..."
  local coverage_command=(cargo llvm-cov --tests --json --output-path cov.json --remap-path-prefix)
  if [ -n "$coverage_package" ]; then
    coverage_command+=(--package "$coverage_package")
  else
    coverage_command+=(--workspace)
  fi
  "${coverage_command[@]}"
}

parse_arguments "$@"

run_coverage_and_emit_json
abort_if_coverage_json_is_missing
print_coverage_table
abort_if_line_coverage_is_below_threshold "$COVERAGE_THRESHOLD"
