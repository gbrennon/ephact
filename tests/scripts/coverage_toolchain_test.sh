#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
test_root="$(mktemp -d)"
trap 'rm -rf -- "$test_root"' EXIT

bin_dir="$test_root/bin"
log_file="$test_root/rustup.log"
mkdir -p "$bin_dir"

cat > "$bin_dir/rustup" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$RUSTUP_LOG"
if [ "${RUSTUP_FAIL:-}" = "$1" ]; then
  exit 1
fi
EOF
chmod +x "$bin_dir/rustup"

PATH="$bin_dir:$PATH" RUSTUP_LOG="$log_file" \
  "$repo_root/scripts/install_coverage_toolchain.sh"

grep --fixed-strings --quiet \
  'toolchain install nightly --profile minimal --no-self-update' "$log_file"
grep --fixed-strings --quiet \
  'component add --toolchain nightly llvm-tools-preview' "$log_file"

failure_log="$test_root/failure.log"
if PATH="$bin_dir:$PATH" RUSTUP_LOG="$log_file" RUSTUP_FAIL=toolchain \
  "$repo_root/scripts/install_coverage_toolchain.sh" >"$failure_log" 2>&1; then
  echo 'expected toolchain provisioning to fail' >&2
  exit 1
fi
grep --fixed-strings --quiet \
  'ERROR: failed to install nightly coverage toolchain noninteractively.' \
  "$failure_log"

component_failure_log="$test_root/component-failure.log"
if PATH="$bin_dir:$PATH" RUSTUP_LOG="$log_file" RUSTUP_FAIL=component \
  "$repo_root/scripts/install_coverage_toolchain.sh" >"$component_failure_log" 2>&1; then
  echo 'expected coverage component provisioning to fail' >&2
  exit 1
fi
grep --fixed-strings --quiet \
  'ERROR: failed to install llvm-tools-preview for the nightly coverage toolchain noninteractively.' \
  "$component_failure_log"

printf '%s\n' 'coverage toolchain provisioning passed'
