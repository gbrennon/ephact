#!/usr/bin/env bash
set -euo pipefail

should_update_changelog() {
  local merged="$1"
  local base_ref="$2"
  local head_label="$3"

  [[ "$merged" == true ]] || return 1
  [[ "$base_ref" == main ]] || return 1
  [[ "$head_label" != *chore/changelog/* ]] || return 1
  [[ "$head_label" != *release/v* ]]
}

assert_update_decision() {
  local expected="$1"
  shift

  if should_update_changelog "$@"; then
    actual=true
  else
    actual=false
  fi

  test "$actual" = "$expected"
}

assert_changelog_branch_has_ci_trigger() {
  grep -Fq 'branches: [main, chore/changelog/main]' .forgejo/workflows/ci.yml
}

assert_changelog_branch_has_ci_trigger

assert_update_decision true true main gbrennon:feature/workflow-fix
assert_update_decision false true main gbrennon:chore/changelog/123
assert_update_decision false true main gbrennon/chore/changelog/123
assert_update_decision false true main gbrennon:release/v1.2.3
assert_update_decision false true release/v1.2.3 gbrennon:feature/workflow-fix
assert_update_decision false false main gbrennon:feature/workflow-fix
printf '%s\n' 'changelog workflow guard behavior passed'
