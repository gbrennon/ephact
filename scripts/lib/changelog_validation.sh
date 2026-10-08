#!/usr/bin/env bash
set -euo pipefail

fail() {
  printf 'ERROR: %s\n' "$1" >&2
  exit 1
}

validate_configuration() {
  local target_branch="$1"
  local config_file="$2"
  local version="$3"
  local max_push_attempts="$4"
  local release_tag_wait_attempts="$5"
  local release_tag_wait_seconds="$6"

  [[ -n "$target_branch" ]] || fail 'TARGET_BRANCH is required'
  [[ "$target_branch" =~ ^[A-Za-z0-9._/-]+$ ]] || fail 'TARGET_BRANCH is invalid'
  [[ -f "$config_file" ]] || \
    fail "git-cliff configuration is missing: $config_file"
  command -v git-cliff >/dev/null 2>&1 || fail 'git-cliff is not installed'
  [[ "$max_push_attempts" =~ ^[1-9][0-9]*$ ]] || \
    fail 'MAX_PUSH_ATTEMPTS must be a positive integer'
  [[ "$release_tag_wait_attempts" =~ ^[1-9][0-9]*$ ]] || \
    fail 'RELEASE_TAG_WAIT_ATTEMPTS must be a positive integer'
  [[ "$release_tag_wait_seconds" =~ ^[0-9]+$ ]] || \
    fail 'RELEASE_TAG_WAIT_SECONDS must be a nonnegative integer'
  if [[ -n "$version" ]] && [[ ! "$version" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    fail "VERSION is invalid: $version"
  fi
}

validate_generated_changelog() {
  local generated_file="$1"
  local version="$2"
  local unreleased_count
  if ! awk '
    /^## \[/ && previous_line != "" { exit 1 }
    { previous_line = $0 }
  ' "$generated_file"; then
    fail 'Generated changelog is missing a blank line before a release heading'
  fi
  unreleased_count="$(grep -c '^## \[Unreleased\]$' "$generated_file" || true)"
  if [[ -n "$version" ]]; then
    local release_heading="## [${version#v}]"
    grep --fixed-strings --quiet "$release_heading" "$generated_file" || \
      fail "Generated changelog is missing heading: $release_heading"
    [[ "$unreleased_count" -eq 0 ]] || \
      fail 'Versioned changelog must not contain an Unreleased heading'
    return
  fi
  [[ "$unreleased_count" -eq 1 ]] || \
    fail "Expected one Unreleased heading, found $unreleased_count"
}
