#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=/dev/null
source "$repo_root/.forgejo/scripts/install_git_cliff.sh"

test_root="$(mktemp -d)"
trap 'rm -rf -- "$test_root"' EXIT

archive="$test_root/archive.tar.gz"
printf '%s\n' 'trusted fixture' > "$archive"
actual_checksum="$(sha512sum "$archive" | cut -d ' ' -f 1)"

verify_archive_checksum "$archive" "$actual_checksum"

if verify_archive_checksum "$archive" "$(printf '%0128d' 0)"; then
  printf '%s\n' 'checksum mismatch was accepted' >&2
  exit 1
fi

test "$(expected_sha512_for_target x86_64)" = \
  26d1f7c8ea2400f2ccb0b2e7f321635f600b583bf4cd1f7afaa5e8c24068ad1088a2504e0a4d9f4333d4bb20a80329327d9462634343ee3af91e1a7fbdc6f18a
test "$(expected_sha512_for_target aarch64)" = \
  fd27768ea76fea8c3ba3f3247fef2f6604988b629f15e48bf40be5eaa08b8fef2e4087cc9c8af16bc522bc19cc369611fa7692101b513f63469ad58522708600

if expected_sha512_for_target riscv64 >/dev/null 2>&1; then
  printf '%s\n' 'unsupported architecture was accepted' >&2
  exit 1
fi

printf '%s\n' 'git-cliff checksum behavior passed'
