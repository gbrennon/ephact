#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
test_root="$(mktemp -d)"
trap 'rm -rf -- "$test_root"' EXIT

origin="$test_root/origin.git"
source_repo="$test_root/source"
bin_dir="$test_root/bin"
target_branch="chore/update-changelog-test"
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
initial_main="$(git -C "$origin" rev-parse main)"
git -C "$source_repo" switch --quiet --create "$target_branch"
git -C "$source_repo" push --quiet origin "$target_branch"
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
  for argument in "$@"; do
    if [[ "$argument" == HEAD:refs/for/* ]]; then
      printf '%s\n' "$argument" > "$TEST_ROOT/agit-push"
    fi
  done
fi
exec /usr/bin/git "$@"
EOF
chmod +x "$bin_dir/git"


(
  cd "$source_repo"
  PATH="$bin_dir:$PATH" \
    HOME="$test_root" \
    TEST_ROOT="$test_root" \
    TARGET_BRANCH=main \
    PULL_REQUEST_BRANCH="$target_branch" \
    BASE_BRANCH=main \
    CREATE_PULL_REQUEST=true \
    CLIFF_CONFIG=cliff.toml \
    GIT_USER_NAME='pull-request-author' \
    GIT_USER_EMAIL='pull-request-author@users.noreply.codeberg.org' \
    MAX_PUSH_ATTEMPTS=1 \
    RELEASE_TAG_WAIT_ATTEMPTS=1 \
    bash "$repo_root/scripts/update_changelog.sh"
)

test "$(git -C "$origin" rev-parse main)" = "$initial_main"
test "$(cat "$test_root/agit-push")" = "HEAD:refs/for/main/$target_branch"
git -C "$origin" show-ref --verify --quiet "refs/for/main/$target_branch"
printf '%s\n' 'changelog pull request flow passed'
