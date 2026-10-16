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
exec /usr/bin/git "$@"
EOF
chmod +x "$bin_dir/git"

cat > "$bin_dir/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
method=GET
output=""
data=""
while (($# > 0)); do
  case "$1" in
    --request)
      shift
      method="$1"
      ;;
    --output)
      shift
      output="$1"
      ;;
    --data)
      shift
      data="$1"
      ;;
  esac
  shift
done
if [[ "$method" == "GET" ]]; then
  if [[ "${EXISTING_PULL_REQUEST:-false}" == true ]]; then
    existing_payload='[{"head":{"ref":"chore/update-changelog-test",'
    existing_payload+=' "repo":{"full_name":"gbrennon/__ep"}}}]'
    printf '%s\n' "$existing_payload" > "$output"
  else
    printf '%s\n' '[]' > "$output"
  fi
  printf '%s' '200'
  exit 0
fi
if [[ -f "$TEST_ROOT/post-count" ]]; then
  count="$(cat "$TEST_ROOT/post-count")"
else
  count=0
fi
printf '%s\n' "$((count + 1))" > "$TEST_ROOT/post-count"
printf '%s\n' "$data" > "$TEST_ROOT/pull-request.json"
printf '%s\n' '{"number":1}' > "$output"
printf '%s' '201'
EOF
chmod +x "$bin_dir/curl"

(
  cd "$source_repo"
  PATH="$bin_dir:$PATH" \
    HOME="$test_root" \
    TEST_ROOT="$test_root" \
    TARGET_BRANCH=main \
    PULL_REQUEST_BRANCH="$target_branch" \
    BASE_BRANCH=main \
    CREATE_PULL_REQUEST=true \
    FORGEJO_API_URL=https://forgejo.invalid/api/v1 \
    FORGEJO_REPOSITORY=gbrennon/__ep \
    FORGEJO_TOKEN=test-token \
    CLIFF_CONFIG=cliff.toml \
    GIT_USER_NAME='pull-request-author' \
    GIT_USER_EMAIL='pull-request-author@users.noreply.codeberg.org' \
    MAX_PUSH_ATTEMPTS=1 \
    RELEASE_TAG_WAIT_ATTEMPTS=1 \
    bash "$repo_root/scripts/update_changelog.sh"
)

(
  cd "$source_repo"
  PATH="$bin_dir:$PATH" \
    HOME="$test_root" \
    TEST_ROOT="$test_root" \
    EXISTING_PULL_REQUEST=true \
    TARGET_BRANCH=main \
    PULL_REQUEST_BRANCH="$target_branch" \
    BASE_BRANCH=main \
    CREATE_PULL_REQUEST=true \
    FORGEJO_API_URL=https://forgejo.invalid/api/v1 \
    FORGEJO_REPOSITORY=gbrennon/__ep \
    FORGEJO_TOKEN=test-token \
    CLIFF_CONFIG=cliff.toml \
    GIT_USER_NAME='pull-request-author' \
    GIT_USER_EMAIL='pull-request-author@users.noreply.codeberg.org' \
    MAX_PUSH_ATTEMPTS=1 \
    RELEASE_TAG_WAIT_ATTEMPTS=1 \
    bash "$repo_root/scripts/update_changelog.sh"
)

test "$(git -C "$origin" rev-parse main)" = "$initial_main"
test "$(git -C "$origin" rev-parse "$target_branch")" != "$initial_main"
test -f "$test_root/pull-request.json"
test "$(jq -r '.base' "$test_root/pull-request.json")" = main
test "$(jq -r '.head' "$test_root/pull-request.json")" = "$target_branch"
test "$(jq -r '.title' "$test_root/pull-request.json")" = 'chore(release): update changelog'
test "$(cat "$test_root/post-count")" = 1
printf '%s\n' 'changelog pull request flow passed'
