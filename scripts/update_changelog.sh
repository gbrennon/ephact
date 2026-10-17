#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"
source scripts/lib/changelog_validation.sh

readonly CHANGELOG_FILE="${CHANGELOG_FILE:-CHANGELOG.md}"
readonly CONFIG_FILE="${CLIFF_CONFIG:-cliff.toml}"
readonly TARGET_BRANCH="${TARGET_BRANCH:-}"
readonly BASE_BRANCH="${BASE_BRANCH:-$TARGET_BRANCH}"
readonly PULL_REQUEST_BRANCH="${PULL_REQUEST_BRANCH:-}"
readonly CREATE_PULL_REQUEST="${CREATE_PULL_REQUEST:-false}"
readonly FORGEJO_API_URL="${FORGEJO_API_URL:-}"
readonly FORGEJO_REPOSITORY="${FORGEJO_REPOSITORY:-}"
readonly FORGEJO_TOKEN="${FORGEJO_TOKEN:-}"
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
synchronize_pull_request_branch() {
  [[ "$CREATE_PULL_REQUEST" == true ]] || return 0
  git fetch --quiet origin \
    "+$PULL_REQUEST_BRANCH:refs/remotes/origin/$PULL_REQUEST_BRANCH" \
    2>/dev/null || true
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

is_retryable_push_failure() {
  case "$1" in
    *non-fast-forward*|*fetch\ first*|*stale\ info*) return 0 ;;
    *) return 1 ;;
  esac
}

prepare_changelog_commit() {
  if [[ -f "$CHANGELOG_FILE" ]] && cmp --silent "$temporary_file" "$CHANGELOG_FILE"; then
    printf 'CHANGELOG.md is already current\n'
    return 3
  fi
  cp -- "$temporary_file" "$CHANGELOG_FILE"
  git add -- "$CHANGELOG_FILE"
  if git diff --cached --quiet -- "$CHANGELOG_FILE"; then
    printf 'CHANGELOG.md produced no changes\n'
    return 3
  fi
  if ! git -c user.name="$GIT_USER_NAME" \
    -c user.email="$GIT_USER_EMAIL" \
    commit --message 'chore(release): update changelog'; then
    return 1
  fi
  return 0
}

push_changelog() {
  local push_ref="$1"
  local push_output
  local expected_revision
  local -a push_command
  push_command=(git push origin "HEAD:$push_ref")
  if [[ "$CREATE_PULL_REQUEST" == true ]] && \
    git show-ref --verify --quiet "refs/remotes/origin/$PULL_REQUEST_BRANCH"; then
    expected_revision="$(git rev-parse "refs/remotes/origin/$PULL_REQUEST_BRANCH")"
    push_command=(
      git push
      "--force-with-lease=refs/heads/$PULL_REQUEST_BRANCH:$expected_revision"
      origin "HEAD:$push_ref"
    )
  fi
  if push_output="$("${push_command[@]}" 2>&1)"; then
    printf '%s\n' "$push_output"
    printf 'Updated changelog on %s\n' "$push_ref"
    return 0
  fi
  printf '%s\n' "$push_output" >&2
  if is_retryable_push_failure "$push_output"; then
    printf 'Target branch advanced while pushing; retrying\n' >&2
    return 1
  fi
  printf 'Changelog push failed without a retryable branch race\n' >&2
  return 2
}
validate_pull_request_configuration() {
  [[ "$CREATE_PULL_REQUEST" == true ]] || return 0
  [[ -n "$BASE_BRANCH" ]] || fail 'BASE_BRANCH is required for pull request mode'
  [[ "$BASE_BRANCH" =~ ^[A-Za-z0-9._/-]+$ ]] || \
    fail 'BASE_BRANCH is invalid for pull request mode'
  [[ -n "$PULL_REQUEST_BRANCH" ]] || \
    fail 'PULL_REQUEST_BRANCH is required for pull request mode'
  [[ "$PULL_REQUEST_BRANCH" =~ ^[A-Za-z0-9._/-]+$ ]] || \
    fail 'PULL_REQUEST_BRANCH is invalid for pull request mode'
  [[ "$PULL_REQUEST_BRANCH" != refs/* ]] || \
    fail 'PULL_REQUEST_BRANCH must be a branch name'
  [[ -n "$FORGEJO_API_URL" ]] || \
    fail 'FORGEJO_API_URL is required for pull request mode'
  [[ -n "$FORGEJO_REPOSITORY" ]] || \
    fail 'FORGEJO_REPOSITORY is required for pull request mode'
  [[ -n "$FORGEJO_TOKEN" ]] || \
    fail 'FORGEJO_TOKEN is required for pull request mode'
  command -v curl >/dev/null 2>&1 || fail 'curl is required for pull request mode'
  command -v jq >/dev/null 2>&1 || fail 'jq is required for pull request mode'
}

create_pull_request() {
  local payload
  local response_file
  local http_status
  local existing_url
  response_file="$(mktemp)"
  if ! http_status="$(
    curl --silent --show-error \
      --request GET \
      --url "${FORGEJO_API_URL}/repos/${FORGEJO_REPOSITORY}/pulls?state=open&limit=50" \
      --header "Authorization: token ${FORGEJO_TOKEN}" \
      --output "$response_file" \
      --write-out '%{http_code}'
  )"; then
    cat "$response_file" >&2
    rm -f -- "$response_file"
    fail 'Could not list changelog pull requests'
  fi
  if (( http_status < 200 || http_status >= 300 )); then
    printf 'Pull request list API returned HTTP status: %s\n' "$http_status" >&2
    cat "$response_file" >&2
    rm -f -- "$response_file"
    fail 'Could not list changelog pull requests'
  fi
  existing_url="$(jq -r \
    --arg head "$PULL_REQUEST_BRANCH" \
    --arg base "$BASE_BRANCH" \
    '[.[] | select(.head.ref == $head and .base.ref == $base)] |
      first | .html_url // empty' "$response_file")"
  if [[ -n "$existing_url" ]]; then
    printf 'Changelog pull request already exists: %s\n' "$existing_url"
    rm -f -- "$response_file"
    return 0
  fi
  payload="$(jq -n \
    --arg title 'chore(release): update changelog' \
    --arg head "$PULL_REQUEST_BRANCH" \
    --arg base "$BASE_BRANCH" \
    --arg body "Automated changelog update for $TARGET_BRANCH." \
    '{title: $title, head: $head, base: $base, body: $body}')"
  if ! http_status="$(
    curl --silent --show-error \
      --request POST \
      --url "${FORGEJO_API_URL}/repos/${FORGEJO_REPOSITORY}/pulls" \
      --header "Authorization: token ${FORGEJO_TOKEN}" \
      --header 'Content-Type: application/json' \
      --output "$response_file" \
      --write-out '%{http_code}' \
      --data "$payload"
  )"; then
    cat "$response_file" >&2
    rm -f -- "$response_file"
    fail 'Could not create changelog pull request'
  fi
  if (( http_status < 200 || http_status >= 300 )); then
    printf 'Pull request API returned HTTP status: %s\n' "$http_status" >&2
    cat "$response_file" >&2
    rm -f -- "$response_file"
    fail 'Could not create changelog pull request'
  fi
  cat "$response_file"
  rm -f -- "$response_file"
}

commit_and_push() {
  local push_ref="$TARGET_BRANCH"
  local preparation_status
  if [[ "$CREATE_PULL_REQUEST" == true ]]; then
    push_ref="$PULL_REQUEST_BRANCH"
  fi
  if prepare_changelog_commit; then
    preparation_status=0
  else
    preparation_status=$?
  fi
  case "$preparation_status" in
    0)
      push_changelog "$push_ref" || return $?
      if [[ "$CREATE_PULL_REQUEST" == true ]]; then
        create_pull_request
      fi
      ;;
    3) return 0 ;;
    *) return 1 ;;
  esac
}

validate_pull_request_configuration

validate_configuration \
  "$TARGET_BRANCH" "$CONFIG_FILE" "$VERSION" "$MAX_PUSH_ATTEMPTS" \
  "$PUSH_RETRY_DELAY_SECONDS" "$RELEASE_TAG_WAIT_ATTEMPTS" \
  "$RELEASE_TAG_WAIT_SECONDS"

for ((attempt = 1; attempt <= MAX_PUSH_ATTEMPTS; attempt++)); do
  printf 'Updating changelog attempt %d/%d\n' "$attempt" "$MAX_PUSH_ATTEMPTS"
  synchronize_target_branch
  synchronize_pull_request_branch
  wait_for_release_tag
  generate_changelog
  if commit_and_push; then
    exit 0
  else
    push_status=$?
  fi
  if (( push_status == 2 )); then
    fail 'Changelog push failed without a retryable branch race'
  fi
  if (( attempt < MAX_PUSH_ATTEMPTS )); then
    retry_delay=$((PUSH_RETRY_DELAY_SECONDS * attempt))
    printf 'Waiting %d seconds before retrying\n' "$retry_delay" >&2
    sleep "$retry_delay"
  fi
done

fail "Could not push changelog after $MAX_PUSH_ATTEMPTS attempts"
