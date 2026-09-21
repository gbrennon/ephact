#!/usr/bin/env bash
set -euo pipefail

staged_files="$*"
if [ -z "$staged_files" ]; then
  exit 0
fi
stash_created=false
restore_non_staged_files() {
  local changed_file

  for changed_file in $(git diff --name-only); do
    if [[ " $staged_files " != *" $changed_file "* ]]; then
      git restore --worktree -- "$changed_file"
    fi
  done
}

restore_stash() {
  if [ "$stash_created" = true ]; then
    git stash pop >/dev/null
  fi
}

restore_unstaged_changes() {
  local restore_status=0

  restore_non_staged_files || restore_status=$?
  restore_stash || restore_status=$?
  return "$restore_status"
}

trap restore_unstaged_changes EXIT

if ! git diff --quiet || [ -n "$(git ls-files --others --exclude-standard)" ]; then
  git stash push --keep-index --include-untracked \
    --message "pre-commit lint-fix" >/dev/null
  stash_created=true
fi

cargo clippy --fix --allow-dirty --allow-staged
