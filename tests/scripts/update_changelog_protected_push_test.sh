#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
test_root="$(mktemp -d)"
trap 'rm -rf -- "$test_root"' EXIT

origin="$test_root/origin.git"
source_repo="$test_root/source"
bin_dir="$test_root/bin"
mkdir -p "$bin_dir" "$source_repo/scripts/lib"
cp "$repo_root/scripts/lib/changelog_validation.sh" \
  "$source_repo/scripts/lib/changelog_validation.sh"

git init --bare --quiet "$origin"
git init --quiet --initial-branch=main "$source_repo"
git -C "$source_repo" config user.name test
git -C "$source_repo" config user.email test@example.invalid
printf '%s\n' '# Changelog' > "$source_repo/CHANGELOG.md"
printf '%s\n' '[git]' > "$source_repo/cliff.toml"
git -C "$source_repo" add CHANGELOG.md cliff.toml
git -C "$source_repo" commit --quiet --message 'chore(test): initialize changelog fixture'
git -C "$source_repo" remote add origin "$origin"
git -C "$source_repo" push --quiet origin main
git -C "$source_repo" config --unset user.name
git -C "$source_repo" config --unset user.email

cat > "$bin_dir/git-cliff" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
output=""
while (($# > 0)); do
  if [[ "$1" == "--output" ]]; then
    shift
    output="$1"
  fi
  shift
done
printf '%s\n\n' '## [Unreleased]' > "$output"
EOF
chmod +x "$bin_dir/git-cliff"

cat > "$bin_dir/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "cliff" ]]; then
  shift
  exec git-cliff "$@"
fi
if [[ "${1:-}" == "push" ]]; then
  count_file="$TEST_ROOT/push-count"
  count=0
  if [[ -f "$count_file" ]]; then
    count="$(cat "$count_file")"
  fi
  printf '%s\n' "$((count + 1))" > "$count_file"
  printf '%s\n' 'remote: error: protected branch hook declined' >&2
  exit 1
fi
exec /usr/bin/git "$@"
EOF
chmod +x "$bin_dir/git"

set +e
(
  cd "$source_repo"
  PATH="$bin_dir:$PATH" \
    HOME="$test_root" \
    TEST_ROOT="$test_root" \
    TARGET_BRANCH=main \
    CLIFF_CONFIG=cliff.toml \
    GIT_USER_NAME='pull-request-author' \
    GIT_USER_EMAIL='pull-request-author@users.noreply.codeberg.org' \
    MAX_PUSH_ATTEMPTS=3 \
    PUSH_RETRY_DELAY_SECONDS=0 \
    RELEASE_TAG_WAIT_ATTEMPTS=1 \
    bash "$repo_root/scripts/update_changelog.sh" > "$test_root/output" 2>&1
)
status=$?
set -e

test "$status" -ne 0
test "$(cat "$test_root/push-count")" = 1
! grep --fixed-strings --quiet 'Target branch advanced while pushing; retrying' "$test_root/output"
grep --fixed-strings --quiet 'protected branch hook declined' "$test_root/output"
printf '%s\n' 'protected push classification regression passed'
