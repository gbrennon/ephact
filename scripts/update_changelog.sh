#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"

readonly CHANGELOG_FILE="${CHANGELOG_FILE:-CHANGELOG.md}"
readonly CONFIG_FILE="${CLIFF_CONFIG:-cliff.toml}"
readonly TARGET_BRANCH="${TARGET_BRANCH:-}"
readonly VERSION="${VERSION:-}"
readonly MAX_PUSH_ATTEMPTS="${MAX_PUSH_ATTEMPTS:-3}"
temporary_file=""

fail() {
  printf 'ERROR: %s\n' "$1" >&2
  exit 1
}

trap 'if [[ -n "$temporary_file" ]]; then rm -f -- "$temporary_file"; fi' EXIT

validate_configuration() {
  [[ -n "$TARGET_BRANCH" ]] || fail 'TARGET_BRANCH is required'
  [[ "$TARGET_BRANCH" =~ ^[A-Za-z0-9._/-]+$ ]] || fail 'TARGET_BRANCH is invalid'
  [[ -f "$CONFIG_FILE" ]] || fail "git-cliff configuration is missing: $CONFIG_FILE"
  command -v git-cliff >/dev/null 2>&1 || fail 'git-cliff is not installed'
  [[ "$MAX_PUSH_ATTEMPTS" =~ ^[1-9][0-9]*$ ]] || \
    fail 'MAX_PUSH_ATTEMPTS must be a positive integer'
  if [[ -n "$VERSION" ]] && [[ ! "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    fail "VERSION is invalid: $VERSION"
  fi
}

synchronize_target_branch() {
  git fetch --quiet origin \
    "$TARGET_BRANCH:refs/remotes/origin/$TARGET_BRANCH"
  git reset --hard --quiet "origin/$TARGET_BRANCH"
}

validate_generated_changelog() {
  local unreleased_count
  unreleased_count="$(grep -c '^## \[Unreleased\]$' "$temporary_file" || true)"
  if [[ -n "$VERSION" ]]; then
    local release_heading="## [${VERSION#v}]"
    grep --fixed-strings --quiet "$release_heading" "$temporary_file" || \
      fail "Generated changelog is missing heading: $release_heading"
    [[ "$unreleased_count" -eq 0 ]] || \
      fail 'Versioned changelog must not contain an Unreleased heading'
    return
  fi
  [[ "$unreleased_count" -eq 1 ]] || \
    fail "Expected one Unreleased heading, found $unreleased_count"
}

generate_changelog() {
  if [[ -n "$temporary_file" ]]; then
    rm -f -- "$temporary_file"
  fi
  temporary_file="$(mktemp)"
  if [[ -n "$VERSION" ]]; then
    git cliff --config "$CONFIG_FILE" --tag "$VERSION" --no-exec \
      --output "$temporary_file"
  else
    git cliff --config "$CONFIG_FILE" --output "$temporary_file"
  fi
  validate_generated_changelog
}

commit_and_push() {
  if [[ -f "$CHANGELOG_FILE" ]] && cmp --silent "$temporary_file" "$CHANGELOG_FILE"; then
    printf 'CHANGELOG.md is already current\n'
    return 0
  fi
  cp -- "$temporary_file" "$CHANGELOG_FILE"
  git config user.name 'github-actions[bot]'
  git config user.email '41898282+github-actions[bot]@users.noreply.github.com'
  git add -- "$CHANGELOG_FILE"
  if git diff --cached --quiet -- "$CHANGELOG_FILE"; then
    printf 'CHANGELOG.md produced no changes\n'
    return 0
  fi
  git commit --message 'chore(release): update changelog'
  if git push origin "HEAD:$TARGET_BRANCH"; then
    printf 'Updated changelog on %s\n' "$TARGET_BRANCH"
    return 0
  fi
  printf 'Target branch advanced while pushing; retrying\n' >&2
  return 1
}

validate_configuration

for ((attempt = 1; attempt <= MAX_PUSH_ATTEMPTS; attempt++)); do
  printf 'Updating changelog attempt %d/%d\n' "$attempt" "$MAX_PUSH_ATTEMPTS"
  synchronize_target_branch
  generate_changelog
  if commit_and_push; then
    exit 0
  fi
done

fail "Could not push changelog after $MAX_PUSH_ATTEMPTS attempts"
