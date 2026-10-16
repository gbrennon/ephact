#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"
source scripts/lib/changelog_validation.sh

readonly CHANGELOG_FILE="${CHANGELOG_FILE:-CHANGELOG.md}"
readonly CONFIG_FILE="${CLIFF_CONFIG:-cliff.toml}"
readonly TARGET_BRANCH="${TARGET_BRANCH:-}"
readonly PULL_REQUEST_BRANCH="${PULL_REQUEST_BRANCH:-}"
readonly BASE_BRANCH="${BASE_BRANCH:-$TARGET_BRANCH}"
readonly CREATE_PULL_REQUEST="${CREATE_PULL_REQUEST:-false}"
readonly FORGEJO_API_URL="${FORGEJO_API_URL:-}"
readonly FORGEJO_REPOSITORY="${FORGEJO_REPOSITORY:-}"
readonly FORGEJO_TOKEN="${FORGEJO_TOKEN:-}"
readonly PULL_REQUEST_TITLE="${PULL_REQUEST_TITLE:-chore(release): update changelog}"
readonly PULL_REQUEST_BODY="${PULL_REQUEST_BODY:-Automated changelog update}"
readonly VERSION="${VERSION:-}"
readonly MAX_PUSH_ATTEMPTS="${MAX_PUSH_ATTEMPTS:-3}"
readonly PUSH_RETRY_DELAY_SECONDS="${PUSH_RETRY_DELAY_SECONDS:-5}"
readonly GIT_USER_NAME="${GIT_USER_NAME:-}"
readonly GIT_USER_EMAIL="${GIT_USER_EMAIL:-}"
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

normalize_release_spacing() {
  local generated_file="$1"
  local normalized_file
  normalized_file="$(mktemp)"
  if ! awk '
    /^## \[/ && previous_line != "" { print "" }
    { print; previous_line = $0 }
  ' "$generated_file" > "$normalized_file"; then
    rm -f -- "$normalized_file"
    fail 'Could not normalize changelog release spacing'
  fi
  mv -- "$normalized_file" "$generated_file"
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
  normalize_release_spacing "$temporary_file"
  validate_generated_changelog "$temporary_file" "$VERSION"
}
commit_and_push() {
  local push_branch="$TARGET_BRANCH"
  if [[ -n "$PULL_REQUEST_BRANCH" ]]; then
    push_branch="$PULL_REQUEST_BRANCH"
  fi
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
  if ! git -c user.name="$GIT_USER_NAME" \
    -c user.email="$GIT_USER_EMAIL" \
    commit --message 'chore(release): update changelog'; then
    return 1
  fi
  if [[ -n "$PULL_REQUEST_BRANCH" ]]; then
    if git push --force-with-lease origin "HEAD:$push_branch"; then
      printf 'Updated changelog on %s\n' "$push_branch"
      return 0
    fi
  elif git push origin "HEAD:$push_branch"; then
    printf 'Updated changelog on %s\n' "$push_branch"
    return 0
  fi
  printf 'Target branch advanced while pushing; retrying\n' >&2
  return 1
}

create_pull_request() {
  local api_url="${FORGEJO_API_URL%/}"
  local response_file
  local create_status
  local payload

  response_file="$(mktemp)"
  payload="$(jq -n \
    --arg title "$PULL_REQUEST_TITLE" \
    --arg head "$PULL_REQUEST_BRANCH" \
    --arg base "$BASE_BRANCH" \
    --arg body "$PULL_REQUEST_BODY" \
    '{title: $title, head: $head, base: $base, body: $body}')"
  create_status="$(curl --silent --show-error \
    --request POST \
    --url "$api_url/repos/$FORGEJO_REPOSITORY/pulls" \
    --header "Authorization: token $FORGEJO_TOKEN" \
    --header 'Content-Type: application/json' \
    --output "$response_file" \
    --write-out '%{http_code}' \
    --data "$payload")"
  if [[ "$create_status" == 2?? ]]; then
    printf 'Created changelog pull request for %s\n' "$PULL_REQUEST_BRANCH"
    rm -f -- "$response_file"
    return 0
  fi
  if [[ "$create_status" == 409 ]] && \
    jq -e '.message == "pull request already exists"' "$response_file" >/dev/null; then
    printf 'Changelog pull request already exists for %s\n' "$PULL_REQUEST_BRANCH"
    rm -f -- "$response_file"
    return 0
  fi
  cat "$response_file" >&2
  rm -f -- "$response_file"
  return 1
}

validate_configuration \
  "$TARGET_BRANCH" "$CONFIG_FILE" "$VERSION" "$MAX_PUSH_ATTEMPTS" \
  "$PUSH_RETRY_DELAY_SECONDS" "$RELEASE_TAG_WAIT_ATTEMPTS" \
  "$RELEASE_TAG_WAIT_SECONDS"
validate_pull_request_configuration \
  "$CREATE_PULL_REQUEST" "$BASE_BRANCH" "$PULL_REQUEST_BRANCH" \
  "$FORGEJO_API_URL" "$FORGEJO_REPOSITORY" "$FORGEJO_TOKEN"

for ((attempt = 1; attempt <= MAX_PUSH_ATTEMPTS; attempt++)); do
  printf 'Updating changelog attempt %d/%d\n' "$attempt" "$MAX_PUSH_ATTEMPTS"
  synchronize_target_branch
  wait_for_release_tag
  generate_changelog
  if commit_and_push; then
    if [[ "$CREATE_PULL_REQUEST" == true ]] && \
      ! git diff --quiet "origin/$BASE_BRANCH...HEAD"; then
      create_pull_request
    fi
    exit 0
  fi
  if (( attempt < MAX_PUSH_ATTEMPTS )); then
    retry_delay=$((PUSH_RETRY_DELAY_SECONDS * attempt))
    printf 'Waiting %d seconds before retrying\n' "$retry_delay" >&2
    sleep "$retry_delay"
  fi
done

fail "Could not push changelog after $MAX_PUSH_ATTEMPTS attempts"
