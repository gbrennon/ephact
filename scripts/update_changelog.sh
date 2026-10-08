#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"
source scripts/lib/changelog_validation.sh

readonly CHANGELOG_FILE="${CHANGELOG_FILE:-CHANGELOG.md}"
readonly CONFIG_FILE="${CLIFF_CONFIG:-cliff.toml}"
readonly TARGET_BRANCH="${TARGET_BRANCH:-}"
readonly VERSION="${VERSION:-}"
readonly MAX_PUSH_ATTEMPTS="${MAX_PUSH_ATTEMPTS:-3}"
readonly RELEASE_TAG_WAIT_ATTEMPTS="${RELEASE_TAG_WAIT_ATTEMPTS:-40}"
readonly RELEASE_TAG_WAIT_SECONDS="${RELEASE_TAG_WAIT_SECONDS:-15}"
temporary_file=""

trap 'if [[ -n "$temporary_file" ]]; then rm -f -- "$temporary_file"; fi' EXIT

synchronize_target_branch() {
  git fetch --quiet --tags origin
  git fetch --quiet origin \
    "$TARGET_BRANCH:refs/remotes/origin/$TARGET_BRANCH"
  git reset --hard --quiet "origin/$TARGET_BRANCH"
}

find_pending_release_tag() {
  local subject
  local version
  local release_pattern
  local revision_range="origin/$TARGET_BRANCH"
  local latest_tag
  release_pattern='from[[:space:]]release/(v[0-9]+\.[0-9]+\.[0-9]+)[[:space:]]into[[:space:]]main$'
  latest_tag="$(git describe --tags --match 'v[0-9]*' --abbrev=0 \
    "origin/$TARGET_BRANCH" 2>/dev/null || true)"
  if [[ -n "$latest_tag" ]]; then
    revision_range="$latest_tag..origin/$TARGET_BRANCH"
  fi
  while IFS= read -r subject; do
    if [[ "$subject" =~ $release_pattern ]]; then
      version="${BASH_REMATCH[1]}"
      if ! git show-ref --tags --verify --quiet "refs/tags/$version"; then
        printf '%s\n' "$version"
        return 0
      fi
    fi
  done < <(git log --first-parent --merges --format='%s' "$revision_range")
  return 1
}

wait_for_release_tag() {
  [[ -n "$VERSION" ]] && return
  local pending_version=""
  for ((wait_attempt = 1; wait_attempt <= RELEASE_TAG_WAIT_ATTEMPTS; wait_attempt++)); do
    git fetch --quiet --tags origin
    pending_version="$(find_pending_release_tag || true)"
    [[ -z "$pending_version" ]] && return
    printf 'Waiting for release tag %s before generating changelog\n' \
      "$pending_version"
    ((wait_attempt < RELEASE_TAG_WAIT_ATTEMPTS)) || break
    sleep "$RELEASE_TAG_WAIT_SECONDS"
  done
  fail "Release tag is not available: $pending_version"
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
  validate_generated_changelog "$temporary_file" "$VERSION"
}

commit_and_push() {
  if [[ -f "$CHANGELOG_FILE" ]] && cmp --silent "$temporary_file" "$CHANGELOG_FILE"; then
    printf 'CHANGELOG.md is already current\n'
    return 0
  fi
  cp -- "$temporary_file" "$CHANGELOG_FILE"
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

validate_configuration \
  "$TARGET_BRANCH" "$CONFIG_FILE" "$VERSION" "$MAX_PUSH_ATTEMPTS" \
  "$RELEASE_TAG_WAIT_ATTEMPTS" "$RELEASE_TAG_WAIT_SECONDS"

for ((attempt = 1; attempt <= MAX_PUSH_ATTEMPTS; attempt++)); do
  printf 'Updating changelog attempt %d/%d\n' "$attempt" "$MAX_PUSH_ATTEMPTS"
  synchronize_target_branch
  wait_for_release_tag
  generate_changelog
  if commit_and_push; then
    exit 0
  fi
done

fail "Could not push changelog after $MAX_PUSH_ATTEMPTS attempts"
