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
printf '%s\n' "$*" >> "$TEST_ROOT/git-args"
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
output=""
request=""
url=""
data=""
while (($# > 0)); do
  case "$1" in
    --output)
      output="$2"
      shift 2
      ;;
    --request)
      request="$2"
      shift 2
      ;;
    --url)
      url="$2"
      shift 2
      ;;
    --data)
      data="$2"
      shift 2
      ;;
    --write-out)
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done
printf '%s\n' "$request" >> "$TEST_ROOT/curl-methods"
if [[ "$request" == GET ]]; then
  if [[ -f "$TEST_ROOT/pull-request-created" ]]; then
    printf '%s' '[{"html_url":"https://codeberg.org/gbrennon/ephact/pulls/400","head":{"ref":"chore/changelog/main"},"base":{"ref":"main"}}]' > "$output"
  else
    printf '%s' '[]' > "$output"
  fi
  printf '%s' '200'
  exit 0
fi
printf '%s\n%s\n%s\n%s\n' "$request" "$url" "$data" > "$TEST_ROOT/curl-request"
touch "$TEST_ROOT/pull-request-created"
printf '%s' '{"number": 400}' > "$output"
printf '%s' '201'
EOF
chmod +x "$bin_dir/curl"

initial_main="$(git -C "$origin" rev-parse refs/heads/main)"
run_update() {
  (
    cd "$source_repo"
    PATH="$bin_dir:$PATH" \
      HOME="$test_root" \
      TEST_ROOT="$test_root" \
      TARGET_BRANCH=main \
      CREATE_PULL_REQUEST=true \
      PULL_REQUEST_BRANCH='chore/changelog/main' \
      FORGEJO_API_URL='https://codeberg.org/api/v1' \
      FORGEJO_REPOSITORY='gbrennon/ephact' \
      FORGEJO_TOKEN='test-token' \
      CLIFF_CONFIG=cliff.toml \
      GIT_USER_NAME='pull-request-author' \
      GIT_USER_EMAIL='pull-request-author@users.noreply.codeberg.org' \
      RELEASE_TAG_WAIT_ATTEMPTS=1 \
      bash "$repo_root/scripts/update_changelog.sh"
  )
}
run_update
run_update

[ "$(git -C "$origin" rev-parse refs/heads/main)" = "$initial_main" ]
[ "$(git -C "$origin" rev-parse refs/heads/chore/changelog/main)" != "$initial_main" ]
test "$(grep --count '^POST$' "$test_root/curl-methods")" = 1
test "$(grep --count '^GET$' "$test_root/curl-methods")" = 2
grep --fixed-strings --quiet \
  'chore/changelog/main:refs/remotes/origin/chore/changelog/main' \
  "$test_root/git-args"
grep --fixed-strings --quiet -- \
  '--force-with-lease=refs/heads/chore/changelog/main:' \
  "$test_root/git-args"
grep --fixed-strings --quiet \
  'https://codeberg.org/api/v1/repos/gbrennon/ephact/pulls' "$test_root/curl-request"
grep --fixed-strings --quiet '"head": "chore/changelog/main"' "$test_root/curl-request"
grep --fixed-strings --quiet '"base": "main"' "$test_root/curl-request"
printf '%s\n' 'changelog pull request flow passed'
